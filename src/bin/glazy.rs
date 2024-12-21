use clap::Parser;
use glazy::config::{config_file_path, read_config};
use miette::Result;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: Option<String>,
}

fn main() -> Result<()> {
    // Initialize tracing subscriber with environment configuration
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    let config_file_name = config_file_path(args.config)?;
    let config = read_config(&config_file_name)?;

    tracing::debug!(target: "config file", config_file = config_file_name);

    // This event will *only* be recorded by the metrics layer.
    tracing::info!(target: "metrics::cool_stuff_count", value = 42);

    // This event will only be seen by the debug log file layer:
    tracing::debug!("this is a message, and part of a system of messages");

    // This event will be seen by both the stdout log layer *and*
    // the debug log file layer, but not by the metrics layer.
    tracing::warn!("the message is a warning about danger!");

    Ok(())
}
