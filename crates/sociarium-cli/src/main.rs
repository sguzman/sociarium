use std::error::Error;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use sociarium_adapter::SocialAdapter;
use sociarium_adapter_x::XAdapter;
use sociarium_config::SociariumConfig;

#[derive(Debug, Parser)]
#[command(name = "sociarium", version, about = "User-sovereign social corpus")]
struct Cli {
    /// Path to the non-secret Sociarium corpus configuration.
    #[arg(long, global = true, default_value = "sociarium.toml")]
    config: PathBuf,

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

    println!("sociarium: bootstrap healthy");
    println!("registered adapter: {} [{capabilities}]", x.surface_id());
    println!("current M0 slice: config + X OAuth/API acquisition boundary");
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
