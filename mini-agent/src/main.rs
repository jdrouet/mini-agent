pub(crate) mod config;
pub(crate) mod sink;
pub(crate) mod source;

use std::path::PathBuf;

use clap::Parser;

/// Simple program to monitor your server
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    config: PathBuf,
}

#[tokio::main]
async fn main() {
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
