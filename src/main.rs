use uuidtools::args::Args;
use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    uuidtools::run(args).await
}