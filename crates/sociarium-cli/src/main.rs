use std::env;
use std::error::Error;
use std::io;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use sociarium_adapter::SocialAdapter;
use sociarium_adapter_x::XAdapter;
use sociarium_config::SociariumConfig;
use sociarium_search::{PostHit, SearchIndex};
use sociarium_store::CorpusStore;
use sociarium_sync::{SyncOptions, sync_profile};

const X_ACCESS_TOKEN_ENV: &str = "SOCIARIUM_X_ACCESS_TOKEN";

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

    println!("sociarium: bootstrap healthy");
    println!("registered adapter: {} [{capabilities}]", x.surface_id());
    println!("X access token environment: {x_token_state}");
    println!("current M0 slice: profile sync -> durable corpus -> rebuildable search");
}

fn config_check(path: &Path) -> Result<(), Box<dyn Error>> {
    let config = SociariumConfig::load(path)?;
    println!(
        "configuration valid: schema={} profiles={} enabled={}",
        config.schema_version,
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

async fn sync_one_profile(
    config_path: &Path,
    corpus: &Path,
    profile_id: &str,
    no_index: bool,
    max_pages: usize,
) -> Result<(), Box<dyn Error>> {
    let config = SociariumConfig::load(config_path)?;
    let profile = config
        .profiles
        .iter()
        .find(|profile| profile.id.as_str() == profile_id)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("configured profile not found: {profile_id}"),
            )
        })?;
    let store = CorpusStore::open(corpus)?;

    let report = match profile.surface.as_str() {
        "x" => {
            let access_token = env::var(X_ACCESS_TOKEN_ENV).map_err(|_| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!(
                        "X credential missing; set {X_ACCESS_TOKEN_ENV} for this temporary M0 credential bridge"
                    ),
                )
            })?;
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
