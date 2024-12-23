use clap::{Parser, Subcommand};
use glazy::{commands, config::read_config, gitlab};
use miette::Result;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Path to the configuration file")]
    config: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize the workspace with a layout file
    Bootstrap { file: String },
    /// Fuzzy find a repository and open it locally. It will be cloned if it doesn't exist locally.
    Open { group: Option<String> },
    /// Update all locally checked out repositories
    Update,
}

fn main() -> Result<()> {
    // Initialize tracing subscriber with environment configuration
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    let config = read_config(args.config)?;
    let gitlab_client = gitlab::Client::new(config.gitlab.host, config.gitlab.token)?;

    match args.command {
        Commands::Bootstrap { file } => {
            commands::bootstrap(&gitlab_client, &file, &config.local.project_dir)?;
        }
        Commands::Open { group } => {
            commands::open(&gitlab_client, group, &config.local.project_dir)?
        }
        Commands::Update => commands::update()?,
    }

    Ok(())
}
