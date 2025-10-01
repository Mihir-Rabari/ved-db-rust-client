//! VedDB Client CLI

use clap::{Parser, Subcommand};
use veddb_client::Client;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Server address (host:port)
    #[arg(short, long, default_value = "127.0.0.1:50051")]
    server: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the Pub/Sub example
    Pubsub,
    /// Run the connection pooling example
    Pooling,
    /// Run a simple get/set example
    Example {
        /// Key to get/set
        key: String,
        /// Value to set (omit to get)
        value: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let addr: std::net::SocketAddr = cli.server.parse()?;
    let client = Client::connect(addr).await?;

    match cli.command {
        Commands::Pubsub => {
            println!("Running Pub/Sub example - not yet implemented");
            // TODO: Implement pub/sub example
        }
        Commands::Pooling => {
            println!("Running connection pooling example - not yet implemented");
            // TODO: Implement pooling example
        }
        Commands::Example { key, value } => {
            if let Some(value) = value {
                // Set operation
                client.set(key.clone(), value.clone().into_bytes()).await?;
                println!("Set '{}' to '{}'", key, value);
            } else {
                // Get operation
                match client.get(key.clone()).await {
                    Ok(value) => {
                        println!("{}: {}", key, String::from_utf8_lossy(&value));
                    }
                    Err(e) => {
                        eprintln!("Error getting '{}': {}", key, e);
                    }
                }
            }
        }
    }

    Ok(())
}
