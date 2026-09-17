use std::env;
use std::error::Error;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};

use chrono::{Duration, Utc};
use clap::{Parser, Subcommand};
use sociarium_adapter::SocialAdapter;
use sociarium_adapter_x::{XAdapter, XOAuthConfig, XOAuthSession, XStoredTokens};
use sociarium_config::SociariumConfig;
use sociarium_core::TrackedProfile;
use sociarium_credentials::{CredentialKey, CredentialStore, NativeCredentialStore};
use sociarium_search::{PostHit, SearchIndex};
use sociarium_store::CorpusStore;
use sociarium_sync::{SyncOptions, sync_profile};
use url::Url;

const X_ACCESS_TOKEN_ENV: &str = "SOCIARIUM_X_ACCESS_TOKEN";
const X_REFRESH_LEEWAY_MINUTES: i64 = 5;

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
    let profile = configured_profile(&config, profile_id)?;
    let store = CorpusStore::open(corpus)?;

    let report = match profile.surface.as_str() {
        "x" => {
            let access_token = x_access_token(&config, profile).await?;
            let adapter = XAdapter::authenticated(access_token)?;
            sync_profile(&adapter, profile, &store, SyncOptions { max_pages }).await?
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

struct OAuthCallbackListener {
    listener: TcpListener,
    expected_path: String,
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
        Ok(Self {
            listener,
            expected_path: redirect.path().to_owned(),
        })
    }

    fn wait_for_code(&self, session: &XOAuthSession) -> Result<String, Box<dyn Error>> {
        let (mut stream, _) = self.listener.accept()?;
        let mut request_line = String::new();
        BufReader::new(stream.try_clone()?).read_line(&mut request_line)?;
        if request_line.len() > 8_192 {
            send_callback_response(&mut stream, 400, "OAuth callback request was too large.")?;
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "OAuth callback request line exceeded 8192 bytes",
            )
            .into());
        }

        let mut parts = request_line.split_whitespace();
        if parts.next() != Some("GET") {
            send_callback_response(&mut stream, 405, "Expected an OAuth GET callback.")?;
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "OAuth callback did not use GET",
            )
            .into());
        }
        let target = parts.next().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "OAuth callback target missing")
        })?;
        let callback = Url::parse(&format!("http://127.0.0.1{target}"))?;
        if callback.path() != self.expected_path {
            send_callback_response(&mut stream, 404, "Unexpected OAuth callback path.")?;
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "OAuth callback path did not match configured redirect URI",
            )
            .into());
        }

        let parameters = callback
            .query_pairs()
            .into_owned()
            .collect::<std::collections::BTreeMap<_, _>>();
        if let Some(remote_error) = parameters.get("error") {
            send_callback_response(&mut stream, 400, "X authorization was not granted.")?;
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!("X OAuth callback returned error: {remote_error}"),
            )
            .into());
        }
        let state = parameters.get("state").ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "OAuth callback state missing")
        })?;
        if !session.state_matches(state) {
            send_callback_response(&mut stream, 400, "OAuth state validation failed.")?;
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "OAuth callback state mismatch",
            )
            .into());
        }
        let code = parameters.get("code").cloned().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "OAuth callback authorization code missing",
            )
        })?;
        send_callback_response(
            &mut stream,
            200,
            "Sociarium received the authorization. You may close this tab and return to the terminal.",
        )?;
        Ok(code)
    }
}

fn send_callback_response(
    stream: &mut std::net::TcpStream,
    status: u16,
    message: &str,
) -> io::Result<()> {
    let reason = match status {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        405 => "Method Not Allowed",
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
