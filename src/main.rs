pub(crate) mod config;
pub(crate) mod event;
pub(crate) mod prelude;
pub(crate) mod sink;
pub(crate) mod source;

#[tokio::main]
async fn main() {
    let config = crate::config::Config::from_file("./victor.toml");
    let handles = config.build();

    for (name, handle) in handles {
        if let Err(err) = handle.await {
            eprintln!("something went wrong with {name}: {err:?}");
        }
    }

    println!("done...");
}
