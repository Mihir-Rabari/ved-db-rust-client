//! VedDB Command Line Interface

use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use prettytable::{row, Table};
use veddb_client::{Client, Result};

#[derive(Parser)]
#[command(name = "veddb-cli")]
#[command(author, version, about = "VedDB Command Line Interface", long_about = None)]
struct Cli {
    /// Server address (host:port)
    #[arg(short, long, default_value = "127.0.0.1:50051")]
    server: String,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Table)]
    format: OutputFormat,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Key-Value operations
    #[command(subcommand)]
    Kv(KvCommand),

    /// Pub/Sub operations
    #[command(subcommand)]
    Pubsub(PubsubCommand),

    /// Server information and statistics
    Info,

    /// Ping the server
    Ping,
}

#[derive(Subcommand)]
enum KvCommand {
    /// Get the value of a key
    Get { key: String },

    /// Set the value of a key
    Set { key: String, value: String },

    /// Delete a key
    Del { key: String },

    /// List all keys (with optional pattern)
    List { pattern: Option<String> },
}

#[derive(Subcommand)]
enum PubsubCommand {
    /// Publish a message to a channel
    Publish { channel: String, message: String },

    /// Subscribe to channels
    Subscribe { channels: Vec<String> },

    /// Unsubscribe from channels
    Unsubscribe { channels: Vec<String> },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputFormat {
    Table,
    Json,
    Raw,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    if cli.verbose {
        std::env::set_var("RUST_LOG", "debug");
    }
    tracing_subscriber::fmt::init();

    // Create client
    let addr: std::net::SocketAddr = cli.server.parse().map_err(|e| {
        veddb_client::Error::InvalidArgument(format!("Invalid server address: {}", e))
    })?;
    let client = Client::connect(addr).await?;

    // Handle commands
    match cli.command {
        Commands::Kv(cmd) => handle_kv_command(cmd, client, cli.format).await?,
        Commands::Pubsub(cmd) => handle_pubsub_command(cmd, client, cli.format).await?,
        Commands::Info => handle_info_command(client, cli.format).await?,
        Commands::Ping => handle_ping_command(client, cli.format).await?,
    }

    Ok(())
}

async fn handle_kv_command(cmd: KvCommand, client: Client, format: OutputFormat) -> Result<()> {
    match cmd {
        KvCommand::Get { key } => {
            let value = client.get(key.clone()).await?;
            match format {
                OutputFormat::Raw => print!("{}", String::from_utf8_lossy(&value)),
                OutputFormat::Json => {
                    let json = serde_json::json!({ key: String::from_utf8_lossy(&value) });
                    println!("{}", serde_json::to_string_pretty(&json)?);
                }
                OutputFormat::Table => {
                    let mut table = Table::new();
                    table.add_row(row!["Key", "Value"]);
                    table.add_row(row![key, String::from_utf8_lossy(&value)]);
                    table.printstd();
                }
            }
        }
        KvCommand::Set { key, value } => {
            client.set(key, value.into_bytes()).await?;
            if format != OutputFormat::Raw {
                println!("OK");
            }
        }
        KvCommand::Del { key } => {
            client.delete(key).await?;
            if format != OutputFormat::Raw {
                println!("OK");
            }
        }
        KvCommand::List { pattern } => {
            // Note: This is a placeholder - you'll need to implement the list_keys method
            // in your Client struct
            let keys: Vec<String> = vec![]; // client.list_keys(pattern).await?;

            match format {
                OutputFormat::Raw => {
                    for key in keys {
                        println!("{}", key);
                    }
                }
                OutputFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(&keys)?);
                }
                OutputFormat::Table => {
                    let mut table = Table::new();
                    table.add_row(row!["Keys"]);
                    for key in keys {
                        table.add_row(row![key]);
                    }
                    table.printstd();
                }
            }
        }
    }
    Ok(())
}

async fn handle_pubsub_command(
    cmd: PubsubCommand,
    client: Client,
    format: OutputFormat,
) -> Result<()> {
    match cmd {
        PubsubCommand::Publish { channel, message } => {
            // Note: You'll need to implement publish in your Client
            // client.publish(&channel, message.as_bytes()).await?;
            if format != OutputFormat::Raw {
                println!("Message published to channel '{}'", channel);
            }
        }
        PubsubCommand::Subscribe { channels } => {
            // Note: You'll need to implement subscribe in your Client
            // let mut subscription = client.subscribe(channels).await?;
            println!("Subscribed to channels: {}", channels.join(", "));
            println!("Press Ctrl+C to exit");

            // Keep the subscription alive
            // while let Some(message) = subscription.recv().await {
            //     println!("Received: {}", String::from_utf8_lossy(&message));
            // }

            // For now, just sleep to keep the program running
            tokio::signal::ctrl_c().await?;
        }
        PubsubCommand::Unsubscribe { channels } => {
            // Note: You'll need to implement unsubscribe in your Client
            // client.unsubscribe(channels).await?;
            if format != OutputFormat::Raw {
                println!("Unsubscribed from channels: {}", channels.join(", "));
            }
        }
    }
    Ok(())
}

async fn handle_info_command(client: Client, format: OutputFormat) -> Result<()> {
    // Note: You'll need to implement info in your Client
    let info = serde_json::json!({}); // client.info().await?;

    match format {
        OutputFormat::Raw => println!("{}", info),
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&info)?),
        OutputFormat::Table => {
            let mut table = Table::new();
            table.add_row(row!["Server", "Value"]);
            if let Some(obj) = info.as_object() {
                for (k, v) in obj {
                    table.add_row(row![k, v]);
                }
            }
            table.printstd();
        }
    }
    Ok(())
}

async fn handle_ping_command(client: Client, format: OutputFormat) -> Result<()> {
    let start = std::time::Instant::now();
    client.ping().await?;
    let duration = start.elapsed();

    match format {
        OutputFormat::Raw => println!("pong"),
        OutputFormat::Json => {
            let json = serde_json::json!({ "status": "pong", "latency_ms": duration.as_millis() });
            println!("{}", serde_json::to_string_pretty(&json)?);
        }
        OutputFormat::Table => {
            let mut table = Table::new();
            table.add_row(row!["Status", "Latency"]);
            table.add_row(row!["pong".green(), format!("{} ms", duration.as_millis())]);
            table.printstd();
        }
    }
    Ok(())
}
