use clap::Parser;
use example_forum_server::{run_main, Cli};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    run_main(cli).await
}
