use clap::{Parser, Subcommand};
use sociarium_adapter::SocialAdapter;
use sociarium_adapter_x::XAdapter;

#[derive(Debug, Parser)]
#[command(name = "sociarium", version, about = "User-sovereign social corpus")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Check the bootstrap installation and adapter registry.
    Doctor,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Doctor => doctor(),
    }
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
    println!("next milestone: authenticated X profile sync -> durable corpus");
}
