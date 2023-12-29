use clap::Parser;
use uuid::Uuid;

#[derive(Debug, Parser, PartialEq, Eq)]
#[command(arg_required_else_help = true, about, version)]
pub struct Args {
    #[clap(subcommand)]
    pub subcommand: Option<Subcommand>,
}

#[derive(Debug, Parser, PartialEq, Eq)]
pub enum Subcommand {
    #[clap(subcommand)]
    Generate(GenerateSubcommand),

    Validate {
        #[clap(long)]
        uuid: String,
    },
}

#[derive(Debug, Parser, PartialEq, Eq)]
pub enum GenerateSubcommand {
    Version {
        #[clap(subcommand)]
        version: GenerateSubcommandVersion,
    },
}

#[derive(Debug, Parser, PartialEq, Eq)]
pub enum GenerateSubcommandVersion {
    #[clap(about = "Generate a UUID using a timestamp and monotonic counter")]
    V1 {
        #[clap(long)]
        seconds: u64,
        #[clap(long)]
        nanos: u32,
        #[clap(long, help = "Use provided seed, if not present use random one")]
        seed: Option<u16>,
    },
    V2,
    #[clap(about = "Generate a UUID using a md5 hash")]
    V3 {
        #[clap(long)]
        namespace: Uuid,
        #[clap(long)]
        name: String,
    },
    #[clap(about = "Generate random UUID")]
    V4,
    #[clap(about = "Generate a UUID using a sha1 hash")]
    V5 {
        #[clap(long)]
        namespace: Uuid,
        #[clap(long)]
        name: String,
    },
    #[clap(about = "Generate a UUID using provided data")]
    V8 {
        #[clap(long)]
        data: String,
    },
}
