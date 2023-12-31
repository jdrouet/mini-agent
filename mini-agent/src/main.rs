pub(crate) mod config;
pub(crate) mod sink;
pub(crate) mod source;
pub(crate) mod transform;

use std::path::PathBuf;

use clap::Parser;

/// Simple program to monitor your server
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the configuration file
    #[arg(short, long, default_value = "/etc/mini-agent/config.toml")]
    config: PathBuf,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let args = Args::parse();

    let config = crate::config::Config::from_file(&args.config);
    let handles = config.build();

    for (name, handle) in handles {
        if let Err(err) = handle.await {
            eprintln!("something went wrong with {name}: {err:?}");
        }
    }

    println!("done...");
}
