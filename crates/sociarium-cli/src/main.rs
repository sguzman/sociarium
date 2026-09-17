use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::time::{Duration as StdDuration, Instant};

use chrono::{Duration, Utc};
use clap::{Parser, Subcommand};
use sociarium_adapter::SocialAdapter;
use sociarium_adapter_x::{XAdapter, XOAuthConfig, XOAuthSession, XStoredTokens};
use sociarium_config::SociariumConfig;
use sociarium_core::TrackedProfile;
use sociarium_credentials::{CredentialKey, CredentialStore, NativeCredentialStore};
use sociarium_search::{PostHit, SearchIndex};
use sociarium_store::{CorpusStore, ProfileBinding};
use sociarium_sync::{SyncOptions, sync_profile};
use url::Url;

const X_ACCESS_TOKEN_ENV: &str = "SOCIARIUM_X_ACCESS_TOKEN";
const X_REFRESH_LEEWAY_MINUTES: i64 = 5;
const CORPUS_CONFIG_FILE: &str = "sociarium.toml";
const CORPUS_CONFIG_TEMPLATE: &str = r#"schema_version = 1

# Add non-secret surface settings and one or more [[profiles]] entries here.
# Bearer credentials, refresh tokens, client secrets, passwords, and session
# cookies never belong in this file.
"#;

#[derive(Debug, Parser)]
#[command(name = "sociarium", version, about = "User-sovereign social corpus")]
struct Cli {
    /// Path to the non-secret Sociarium corpus configuration.
    #[arg(long, global = true, default_value = "sociarium.toml")]
    config: PathBuf,

    /// Root directory of the durable corpus repository.
    #[arg(long, global = true, default_value = "corpus")]
    corpus: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Check the installation and adapter registry.
    Doctor,
    /// Initialize and inspect durable corpus repositories.
    Corpus {
        #[command(subcommand)]
        command: CorpusCommand,
    },
    /// Validate corpus configuration.
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },
    /// Inspect configured synchronization profiles.
    Profiles {
        #[command(subcommand)]
        command: ProfilesCommand,
    },
    /// Manage profile-scoped remote authorization.
    Auth {
        #[command(subcommand)]
        command: AuthCommand,
    },
    /// Synchronize one configured profile through its surface adapter.
    Sync {
        /// Local profile id from sociarium.toml.
        profile: String,
        /// Skip rebuilding the disposable search index after a successful sync.
        #[arg(long)]
        no_index: bool,
        /// Safety cap for pages acquired in one invocation.
        #[arg(long, default_value_t = 10_000)]
        max_pages: usize,
    },
    /// Manage disposable local indexes derived from durable corpus files.
    Index {
        #[command(subcommand)]
        command: IndexCommand,
    },
    /// Query normalized posts through the local search index.
    Posts {
        #[command(subcommand)]
        command: PostsCommand,
    },
}

#[derive(Debug, Subcommand)]
enum CorpusCommand {
    /// Initialize a dedicated Git-safe Sociarium corpus directory.
    Init { path: PathBuf },
}

#[derive(Debug, Subcommand)]
enum ConfigCommand {
    /// Parse and validate the configuration file.
    Check,
}

#[derive(Debug, Subcommand)]
enum ProfilesCommand {
    /// List configured profiles without contacting remote surfaces.
    List,
}

#[derive(Debug, Subcommand)]
enum AuthCommand {
    /// Authorize a configured profile and persist its token set in the native credential store.
    Login { profile: String },
    /// Report credential presence and expiry without printing secrets.
    Status { profile: String },
    /// Delete the profile's persisted credential from the native credential store.
    Logout { profile: String },
}

#[derive(Debug, Subcommand)]
enum IndexCommand {
    /// Delete-and-rebuild the local search projection from durable acquisitions.
    Rebuild,
}

#[derive(Debug, Subcommand)]
enum PostsCommand {
    /// List most recent indexed posts.
    List {
        /// Restrict results to one configured profile id.
        #[arg(long)]
        profile: Option<String>,
        /// Maximum number of results.
        #[arg(long, default_value_t = 20)]
        limit: usize,
    },
    /// Full-text search indexed posts.
    Search {
        /// Literal text query. Whitespace-separated terms are all required.
        query: String,
        /// Restrict results to one configured profile id.
        #[arg(long)]
        profile: Option<String>,
        /// Maximum number of results.
        #[arg(long, default_value_t = 20)]
        limit: usize,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Doctor => doctor(),
        Command::Corpus {
            command: CorpusCommand::Init { path },
        } => corpus_init(&path)?,
        Command::Config {
            command: ConfigCommand::Check,
        } => config_check(&cli.config)?,
        Command::Profiles {
            command: ProfilesCommand::List,
        } => profiles_list(&cli.config)?,
        Command::Auth {
            command: AuthCommand::Login { profile },
        } => auth_login(&cli.config, &profile).await?,
        Command::Auth {
            command: AuthCommand::Status { profile },
        } => auth_status(&cli.config, &profile)?,
        Command::Auth {
            command: AuthCommand::Logout { profile },
        } => auth_logout(&cli.config, &profile)?,
        Command::Sync {
            profile,
            no_index,
            max_pages,
        } => sync_one_profile(&cli.config, &cli.corpus, &profile, no_index, max_pages).await?,
        Command::Index {
            command: IndexCommand::Rebuild,
        } => index_rebuild(&cli.corpus)?,
        Command::Posts {
            command: PostsCommand::List { profile, limit },
        } => posts_list(&cli.corpus, profile.as_deref(), limit)?,
        Command::Posts {
            command:
                PostsCommand::Search {
                    query,
                    profile,
                    limit,
                },
        } => posts_search(&cli.corpus, &query, profile.as_deref(), limit)?,
    }

    Ok(())
}

fn corpus_init(path: &Path) -> Result<(), Box<dyn Error>> {
    let store = CorpusStore::initialize(path)?;
    let config_path = store.layout().root().join(CORPUS_CONFIG_FILE);
    if !config_path.exists() {
        fs::write(&config_path, CORPUS_CONFIG_TEMPLATE)?;
    }
    SociariumConfig::load(&config_path)?;

    println!(
        "initialized Sociarium corpus: {}",
        store.layout().root().display()
    );
    println!("corpus marker: {}", store.layout().marker_path().display());
    println!("corpus config: {}", config_path.display());
    println!("Git is optional; no repository was initialized or pushed automatically.");
    Ok(())
}

fn doctor() {
    let x = XAdapter::new();
    let capabilities = x
        .capabilities()
        .into_iter()
        .map(|capability| format!("{capability:?}"))
        .collect::<Vec<_>>()
        .join(", ");
    let x_token_state = if env::var_os(X_ACCESS_TOKEN_ENV).is_some() {
        "present"
    } else {
        "missing"
    };
    let native_store_state = match NativeCredentialStore::new() {
        Ok(_) => "available",
        Err(_) => "unavailable",
    };

    println!("sociarium: bootstrap healthy");
    println!("registered adapter: {} [{capabilities}]", x.surface_id());
    println!("native credential store: {native_store_state}");
    println!("X emergency access-token environment: {x_token_state}");
    println!("current M0 slice: auth + profile sync -> durable corpus -> rebuildable search");
}

fn config_check(path: &Path) -> Result<(), Box<dyn Error>> {
    let config = SociariumConfig::load(path)?;
    println!(
        "configuration valid: schema={} surfaces={} profiles={} enabled={}",
        config.schema_version,
        config.surfaces.len(),
        config.profiles.len(),
        config.enabled_profiles().count()
    );
    Ok(())
}

fn profiles_list(path: &Path) -> Result<(), Box<dyn Error>> {
    let config = SociariumConfig::load(path)?;
    for profile in &config.profiles {
        println!(
            "{}\tsurface={}\thandle={}\townership={:?}\tenabled={}",
            profile.id,
            profile.surface,
            profile.handle.as_deref().unwrap_or("-"),
            profile.ownership,
            profile.enabled
        );
    }
    Ok(())
}

async fn auth_login(config_path: &Path, profile_id: &str) -> Result<(), Box<dyn Error>> {
    let config = SociariumConfig::load(config_path)?;
    let profile = configured_profile(&config, profile_id)?;
    match profile.surface.as_str() {
        "x" => auth_login_x(&config, profile).await,
        surface => Err(io::Error::new(
            io::ErrorKind::Unsupported,
            format!("authorization is not implemented for surface {surface}"),
        )
        .into()),
    }
}

async fn auth_login_x(
    config: &SociariumConfig,
    profile: &TrackedProfile,
) -> Result<(), Box<dyn Error>> {
    let (oauth, redirect_uri) = x_oauth_config(config)?;
    let credential_store = NativeCredentialStore::new()?;
    let credential_key = oauth_credential_key(profile);
    let listener = OAuthCallbackListener::bind(&redirect_uri)?;
    let session = oauth.begin()?;

    println!("Authorize profile {} at:", profile.id);
    println!("{}", session.authorize_url());
    println!("waiting for loopback OAuth callback...");

    let code = listener.wait_for_code(&session)?;
    let token_set = oauth.exchange_code(&session, &code).await?;
    let stored = token_set.into_stored(Utc::now(), None);
    credential_store.save(&credential_key, &stored.to_secret_bytes()?)?;

    println!(
        "authorization stored: profile={} surface={} refresh_token={}",
        profile.id,
        profile.surface,
        stored.refresh_token().is_some()
    );
    Ok(())
}

fn auth_status(config_path: &Path, profile_id: &str) -> Result<(), Box<dyn Error>> {
    let config = SociariumConfig::load(config_path)?;
    let profile = configured_profile(&config, profile_id)?;
    let key = oauth_credential_key(profile);
    let environment_override =
        env::var_os(X_ACCESS_TOKEN_ENV).is_some() && profile.surface.as_str() == "x";

    match NativeCredentialStore::new() {
        Ok(store) => match store.load(&key)? {
            Some(secret) if profile.surface.as_str() == "x" => {
                let tokens = XStoredTokens::from_secret_bytes(&secret)?;
                println!(
                    "profile={} native_credential=present expires_at={} refresh_token={} env_override={}",
                    profile.id,
                    tokens
                        .expires_at()
                        .map(|value| value.to_rfc3339())
                        .unwrap_or_else(|| "unknown".to_owned()),
                    tokens.refresh_token().is_some(),
                    environment_override
                );
            }
            Some(_) => {
                println!(
                    "profile={} native_credential=present env_override={}",
                    profile.id, environment_override
                );
            }
            None => {
                println!(
                    "profile={} native_credential=missing env_override={}",
                    profile.id, environment_override
                );
            }
        },
        Err(error) => {
            println!(
                "profile={} native_credential=unavailable ({error}) env_override={}",
                profile.id, environment_override
            );
        }
    }
    Ok(())
}

fn auth_logout(config_path: &Path, profile_id: &str) -> Result<(), Box<dyn Error>> {
    let config = SociariumConfig::load(config_path)?;
    let profile = configured_profile(&config, profile_id)?;
    let store = NativeCredentialStore::new()?;
    let deleted = store.delete(&oauth_credential_key(profile))?;
    println!(
        "authorization removed: profile={} existed={} env_override_still_present={}",
        profile.id,
        deleted,
        env::var_os(X_ACCESS_TOKEN_ENV).is_some() && profile.surface.as_str() == "x"
    );
    Ok(())
}

async fn sync_one_profile(
    config_path: &Path,
    corpus: &Path,
    profile_id: &str,
    no_index: bool,
    max_pages: usize,
) -> Result<(), Box<dyn Error>> {
    let config = SociariumConfig::load(config_path)?;
    let configured = configured_profile(&config, profile_id)?;
    let store = CorpusStore::open_initialized(corpus)?;
    let profile = profile_with_durable_binding(configured, &store)?;

    let report = match profile.surface.as_str() {
        "x" => {
            let access_token = x_access_token(&config, &profile).await?;
            let adapter = XAdapter::authenticated(access_token)?;
            sync_profile(&adapter, &profile, &store, SyncOptions { max_pages }).await?
        }
        surface => {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                format!("no runnable adapter is registered for surface {surface}"),
            )
            .into());
        }
    };

    println!(
        "sync complete: profile={} pages={} records={} raw={} prior_state={} checkpoint={}",
        profile.id,
        report.pages_persisted,
        report.records_persisted,
        report.raw_evidence_objects,
        report.started_from_prior_state,
        if report.final_cursor.is_some() {
            "stored"
        } else {
            "none"
        }
    );

    if !no_index {
        index_rebuild(corpus)?;
    }
    Ok(())
}

async fn x_access_token(
    config: &SociariumConfig,
    profile: &TrackedProfile,
) -> Result<String, Box<dyn Error>> {
    if let Ok(access_token) = env::var(X_ACCESS_TOKEN_ENV) {
        if !access_token.trim().is_empty() {
            return Ok(access_token);
        }
    }

    let store = NativeCredentialStore::new().map_err(|error| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "X credential missing and native credential storage is unavailable: {error}; run `sociarium auth login {}` or set {X_ACCESS_TOKEN_ENV}",
                profile.id
            ),
        )
    })?;
    let key = oauth_credential_key(profile);
    let secret = store.load(&key)?.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "X credential missing for profile {}; run `sociarium auth login {}` or set {X_ACCESS_TOKEN_ENV}",
                profile.id, profile.id
            ),
        )
    })?;
    let mut tokens = XStoredTokens::from_secret_bytes(&secret)?;

    if tokens.should_refresh(Utc::now(), Duration::minutes(X_REFRESH_LEEWAY_MINUTES)) {
        let prior_refresh_token = tokens.refresh_token().map(str::to_owned).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!(
                    "stored X access token for profile {} is expiring but has no refresh token; run `sociarium auth login {}`",
                    profile.id, profile.id
                ),
            )
        })?;
        let (oauth, _) = x_oauth_config(config)?;
        let refreshed = oauth.refresh(&prior_refresh_token).await?;
        tokens = refreshed.into_stored(Utc::now(), Some(&prior_refresh_token));
        store.save(&key, &tokens.to_secret_bytes()?)?;
    }

    Ok(tokens.access_token().to_owned())
}

fn configured_profile<'a>(
    config: &'a SociariumConfig,
    profile_id: &str,
) -> Result<&'a TrackedProfile, Box<dyn Error>> {
    config
        .profiles
        .iter()
        .find(|profile| profile.id.as_str() == profile_id)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("configured profile not found: {profile_id}"),
            )
            .into()
        })
}

fn profile_with_durable_binding(
    configured: &TrackedProfile,
    store: &CorpusStore,
) -> Result<TrackedProfile, Box<dyn Error>> {
    let mut resolved = configured.clone();
    match store.profile_binding(configured)? {
        ProfileBinding::Unbound => {}
        ProfileBinding::Bound(durable_remote_id) => {
            if let Some(configured_remote_id) = &configured.remote_id {
                if configured_remote_id != &durable_remote_id {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "configured remote profile id conflicts with durable binding for {}: configured={} durable={}",
                            configured.id, configured_remote_id, durable_remote_id
                        ),
                    )
                    .into());
                }
            } else {
                resolved.remote_id = Some(durable_remote_id);
            }
        }
        ProfileBinding::Conflicted(remote_ids) => {
            let ids = remote_ids
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ");
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "durable remote profile binding is conflicted for {}: [{}]",
                    configured.id, ids
                ),
            )
            .into());
        }
    }
    Ok(resolved)
}

fn oauth_credential_key(profile: &TrackedProfile) -> CredentialKey {
    CredentialKey::oauth_tokens(profile.surface.clone(), profile.id.clone())
}

fn x_oauth_config(config: &SociariumConfig) -> Result<(XOAuthConfig, String), Box<dyn Error>> {
    let client_id = required_surface_setting(config, "x", "client_id")?;
    let redirect_uri = required_surface_setting(config, "x", "redirect_uri")?.to_owned();
    Ok((XOAuthConfig::new(client_id, &redirect_uri)?, redirect_uri))
}

fn required_surface_setting<'a>(
    config: &'a SociariumConfig,
    surface: &str,
    key: &str,
) -> Result<&'a str, Box<dyn Error>> {
    config.surface_setting(surface, key).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("missing configuration setting: [surfaces.{surface}] {key}"),
        )
        .into()
    })
}

const DEFAULT_OAUTH_CALLBACK_TIMEOUT: StdDuration = StdDuration::from_secs(180);
const OAUTH_CALLBACK_POLL_INTERVAL: StdDuration = StdDuration::from_millis(20);
const OAUTH_CONNECTION_READ_TIMEOUT: StdDuration = StdDuration::from_secs(2);
const MAX_OAUTH_REQUEST_LINE_BYTES: usize = 8_192;

struct OAuthCallbackListener {
    listener: TcpListener,
    expected_path: String,
}

enum CallbackOutcome {
    Ignored,
    Authorized(String),
}

impl OAuthCallbackListener {
    fn bind(redirect_uri: &str) -> Result<Self, Box<dyn Error>> {
        let redirect = Url::parse(redirect_uri)?;
        if redirect.scheme() != "http" || redirect.host_str() != Some("127.0.0.1") {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "M0 OAuth redirect_uri must use loopback http://127.0.0.1",
            )
            .into());
        }
        if redirect.query().is_some() || redirect.fragment().is_some() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "OAuth redirect_uri must not contain a query or fragment",
            )
            .into());
        }
        let port = redirect.port().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "OAuth redirect_uri must specify an explicit loopback port",
            )
        })?;
        let listener = TcpListener::bind(("127.0.0.1", port))?;
        listener.set_nonblocking(true)?;
        Ok(Self {
            listener,
            expected_path: redirect.path().to_owned(),
        })
    }

    fn wait_for_code(&self, session: &XOAuthSession) -> Result<String, Box<dyn Error>> {
        self.wait_for_code_with_timeout(session, DEFAULT_OAUTH_CALLBACK_TIMEOUT)
    }

    fn wait_for_code_with_timeout(
        &self,
        session: &XOAuthSession,
        timeout: StdDuration,
    ) -> Result<String, Box<dyn Error>> {
        let deadline = Instant::now() + timeout;

        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                return Err(oauth_callback_timeout().into());
            }

            match self.listener.accept() {
                Ok((mut stream, _)) => {
                    let remaining = deadline.saturating_duration_since(Instant::now());
                    if remaining.is_zero() {
                        return Err(oauth_callback_timeout().into());
                    }
                    stream.set_read_timeout(Some(remaining.min(OAUTH_CONNECTION_READ_TIMEOUT)))?;

                    match self.handle_connection(&mut stream, session)? {
                        CallbackOutcome::Ignored => {}
                        CallbackOutcome::Authorized(code) => return Ok(code),
                    }
                }
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                    std::thread::sleep(
                        deadline
                            .saturating_duration_since(Instant::now())
                            .min(OAUTH_CALLBACK_POLL_INTERVAL),
                    );
                }
                Err(error) => return Err(error.into()),
            }
        }
    }

    fn handle_connection(
        &self,
        stream: &mut TcpStream,
        session: &XOAuthSession,
    ) -> Result<CallbackOutcome, Box<dyn Error>> {
        let mut request_line = Vec::with_capacity(512);
        let mut reader =
            BufReader::new(stream.try_clone()?).take((MAX_OAUTH_REQUEST_LINE_BYTES + 1) as u64);

        match reader.read_until(b'\n', &mut request_line) {
            Ok(0) => return Ok(CallbackOutcome::Ignored),
            Ok(_) => {}
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut
                ) =>
            {
                let _ = send_callback_response(stream, 408, "OAuth callback request timed out.");
                return Ok(CallbackOutcome::Ignored);
            }
            Err(error) => return Err(error.into()),
        }

        if request_line.len() > MAX_OAUTH_REQUEST_LINE_BYTES {
            let _ = send_callback_response(stream, 400, "OAuth callback request was too large.");
            return Ok(CallbackOutcome::Ignored);
        }

        let request_line = match std::str::from_utf8(&request_line) {
            Ok(line) => line,
            Err(_) => {
                let _ = send_callback_response(stream, 400, "Malformed OAuth callback request.");
                return Ok(CallbackOutcome::Ignored);
            }
        };

        let mut parts = request_line.split_whitespace();
        if parts.next() != Some("GET") {
            let _ = send_callback_response(stream, 405, "Expected an OAuth GET callback.");
            return Ok(CallbackOutcome::Ignored);
        }
        let Some(target) = parts.next() else {
            let _ = send_callback_response(stream, 400, "Malformed OAuth callback request.");
            return Ok(CallbackOutcome::Ignored);
        };
        let callback = match Url::parse(&format!("http://127.0.0.1{target}")) {
            Ok(callback) => callback,
            Err(_) => {
                let _ = send_callback_response(stream, 400, "Malformed OAuth callback request.");
                return Ok(CallbackOutcome::Ignored);
            }
        };
        if callback.path() != self.expected_path {
            let _ = send_callback_response(stream, 404, "Unexpected OAuth callback path.");
            return Ok(CallbackOutcome::Ignored);
        }

        let parameters = callback
            .query_pairs()
            .into_owned()
            .collect::<std::collections::BTreeMap<_, _>>();

        if parameters.contains_key("error") {
            let _ = send_callback_response(stream, 400, "X authorization was not granted.");
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "X OAuth callback returned an authorization error",
            )
            .into());
        }

        let state = match parameters.get("state") {
            Some(state) => state,
            None => {
                let _ = send_callback_response(stream, 400, "OAuth state validation failed.");
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "OAuth callback state missing",
                )
                .into());
            }
        };
        if !session.state_matches(state) {
            let _ = send_callback_response(stream, 400, "OAuth state validation failed.");
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "OAuth callback state mismatch",
            )
            .into());
        }

        let code = match parameters.get("code") {
            Some(code) if !code.trim().is_empty() => code.clone(),
            _ => {
                let _ = send_callback_response(
                    stream,
                    400,
                    "OAuth callback authorization code missing.",
                );
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "OAuth callback authorization code missing",
                )
                .into());
            }
        };

        let _ = send_callback_response(
            stream,
            200,
            "Sociarium received the authorization. You may close this tab and return to the terminal.",
        );
        Ok(CallbackOutcome::Authorized(code))
    }
}

fn oauth_callback_timeout() -> io::Error {
    io::Error::new(
        io::ErrorKind::TimedOut,
        "OAuth callback timed out before authorization completed",
    )
}

fn send_callback_response(stream: &mut TcpStream, status: u16, message: &str) -> io::Result<()> {
    let reason = match status {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        405 => "Method Not Allowed",
        408 => "Request Timeout",
        _ => "Error",
    };
    let body =
        format!("<!doctype html><meta charset=\"utf-8\"><title>Sociarium</title><p>{message}</p>");
    write!(
        stream,
        "HTTP/1.1 {status} {reason}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )?;
    stream.flush()
}

fn index_rebuild(corpus: &Path) -> Result<(), Box<dyn Error>> {
    CorpusStore::open_initialized(corpus)?;
    let index = SearchIndex::for_corpus(corpus);
    let stats = index.rebuild()?;
    println!(
        "rebuilt {}: acquisitions={} records={} posts={}",
        index.path().display(),
        stats.acquisitions_scanned,
        stats.records_scanned,
        stats.posts_indexed
    );
    Ok(())
}

fn posts_list(corpus: &Path, profile: Option<&str>, limit: usize) -> Result<(), Box<dyn Error>> {
    CorpusStore::open_initialized(corpus)?;
    let index = SearchIndex::for_corpus(corpus);
    for hit in index.list_posts(profile, limit)? {
        print_hit(&hit);
    }
    Ok(())
}

fn posts_search(
    corpus: &Path,
    query: &str,
    profile: Option<&str>,
    limit: usize,
) -> Result<(), Box<dyn Error>> {
    CorpusStore::open_initialized(corpus)?;
    let index = SearchIndex::for_corpus(corpus);
    for hit in index.search_posts(query, profile, limit)? {
        print_hit(&hit);
    }
    Ok(())
}

fn print_hit(hit: &PostHit) {
    let timestamp = hit.created_at.as_deref().unwrap_or(&hit.observed_at);
    let text = hit.text.replace(['\r', '\n', '\t'], " ");
    println!(
        "{}\t{}\t{}\t{}\t{}",
        timestamp,
        hit.profile_id,
        hit.object_id,
        hit.canonical_url.as_deref().unwrap_or("-"),
        text
    );
}

#[cfg(test)]
mod tests {
    use std::net::{SocketAddr, TcpStream};
    use std::thread;

    use super::*;

    #[test]
    fn corpus_init_writes_parseable_nonsecret_config_template() {
        let path =
            std::env::temp_dir().join(format!("sociarium-cli-corpus-init-{}", std::process::id()));
        let _ = fs::remove_dir_all(&path);

        corpus_init(&path).unwrap();
        let config_path = path.join(CORPUS_CONFIG_FILE);
        let config = SociariumConfig::load(&config_path).unwrap();
        let config_text = fs::read_to_string(&config_path).unwrap();

        assert_eq!(config.schema_version, 1);
        assert!(path.join("sociarium-corpus.json").is_file());
        assert!(path.join(".gitignore").is_file());
        assert!(!config_text.contains("access_token"));
        assert!(!config_text.contains("refresh_token"));
        assert!(!config_text.contains("client_secret"));

        fs::remove_dir_all(path).unwrap();
    }

    fn test_session() -> XOAuthSession {
        XOAuthConfig::new("test-client", "http://127.0.0.1:49152/oauth/x/callback")
            .unwrap()
            .begin()
            .unwrap()
    }

    fn test_listener() -> OAuthCallbackListener {
        OAuthCallbackListener::bind("http://127.0.0.1:0/oauth/x/callback").unwrap()
    }

    fn send_request(address: SocketAddr, request: &str) {
        let mut stream = TcpStream::connect(address).unwrap();
        stream
            .set_read_timeout(Some(StdDuration::from_secs(1)))
            .unwrap();
        stream.write_all(request.as_bytes()).unwrap();
        let mut response = String::new();
        let _ = stream.read_to_string(&mut response);
    }

    #[test]
    fn callback_times_out_when_no_request_arrives() {
        let listener = test_listener();
        let session = test_session();

        let error = listener
            .wait_for_code_with_timeout(&session, StdDuration::from_millis(40))
            .unwrap_err();

        assert!(error.to_string().contains("timed out"));
    }

    #[test]
    fn unrelated_path_then_valid_callback_succeeds() {
        let listener = test_listener();
        let address = listener.listener.local_addr().unwrap();
        let session = test_session();
        let state = session.state().to_owned();

        let worker = thread::spawn(move || {
            send_request(
                address,
                "GET /favicon.ico HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n",
            );
            send_request(
                address,
                &format!(
                    "GET /oauth/x/callback?state={state}&code=secret-code HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n"
                ),
            );
        });

        let code = listener
            .wait_for_code_with_timeout(&session, StdDuration::from_secs(2))
            .unwrap();
        worker.join().unwrap();

        assert_eq!(code, "secret-code");
    }

    #[test]
    fn unsupported_method_then_valid_callback_succeeds() {
        let listener = test_listener();
        let address = listener.listener.local_addr().unwrap();
        let session = test_session();
        let state = session.state().to_owned();

        let worker = thread::spawn(move || {
            send_request(
                address,
                "POST /oauth/x/callback HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n",
            );
            send_request(
                address,
                &format!(
                    "GET /oauth/x/callback?state={state}&code=secret-code HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n"
                ),
            );
        });

        assert_eq!(
            listener
                .wait_for_code_with_timeout(&session, StdDuration::from_secs(2))
                .unwrap(),
            "secret-code"
        );
        worker.join().unwrap();
    }

    #[test]
    fn oversized_request_then_valid_callback_succeeds() {
        let listener = test_listener();
        let address = listener.listener.local_addr().unwrap();
        let session = test_session();
        let state = session.state().to_owned();

        let worker = thread::spawn(move || {
            let oversized = format!(
                "GET /{} HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n",
                "x".repeat(MAX_OAUTH_REQUEST_LINE_BYTES + 256)
            );
            send_request(address, &oversized);
            send_request(
                address,
                &format!(
                    "GET /oauth/x/callback?state={state}&code=secret-code HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n"
                ),
            );
        });

        assert_eq!(
            listener
                .wait_for_code_with_timeout(&session, StdDuration::from_secs(2))
                .unwrap(),
            "secret-code"
        );
        worker.join().unwrap();
    }

    #[test]
    fn state_mismatch_is_terminal_without_echoing_callback_secrets() {
        let listener = test_listener();
        let address = listener.listener.local_addr().unwrap();
        let session = test_session();

        let worker = thread::spawn(move || {
            send_request(
                address,
                "GET /oauth/x/callback?state=wrong-secret-state&code=secret-code HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n",
            );
        });

        let rendered = listener
            .wait_for_code_with_timeout(&session, StdDuration::from_secs(2))
            .unwrap_err()
            .to_string();
        worker.join().unwrap();

        assert!(rendered.contains("state mismatch"));
        assert!(!rendered.contains("wrong-secret-state"));
        assert!(!rendered.contains("secret-code"));
    }

    #[test]
    fn explicit_oauth_error_is_terminal_without_echoing_remote_query() {
        let listener = test_listener();
        let address = listener.listener.local_addr().unwrap();
        let session = test_session();

        let worker = thread::spawn(move || {
            send_request(
                address,
                "GET /oauth/x/callback?error=access_denied&error_description=remote-secret-description HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n",
            );
        });

        let rendered = listener
            .wait_for_code_with_timeout(&session, StdDuration::from_secs(2))
            .unwrap_err()
            .to_string();
        worker.join().unwrap();

        assert!(rendered.contains("authorization error"));
        assert!(!rendered.contains("access_denied"));
        assert!(!rendered.contains("remote-secret-description"));
    }
}
