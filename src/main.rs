use anyhow::Result;
use clap::Parser;
use uuidtools::args::Args;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    uuidtools::run(args).await
}
