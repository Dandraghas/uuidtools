use clap::Parser;
use uuid::Uuid;

#[derive(Debug, Parser, PartialEq, Eq)]
#[command(
arg_required_else_help = true,
about,
version,
)]
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
    }
}

#[derive(Debug, Parser, PartialEq, Eq)]
pub enum GenerateSubcommand {
    Version {
        #[clap(subcommand)]
        version: GenerateSubcommandVersion,
    }
}

#[derive(Debug, Parser, PartialEq, Eq)]
pub enum GenerateSubcommandVersion {
    V1 {
        #[clap(long)]
        seconds: u64,
        #[clap(long)]
        nanos: u32,
        #[clap(long, help = "Use provided seed, if not present use random one")]
        seed: Option<u16>
    },
    V2,
    V3 {
        #[clap(long)]
        namespace: Uuid,
        #[clap(long)]
        name: String,
    },
    V4,
    V5 {
        #[clap(long)]
        namespace: Uuid,
        #[clap(long)]
        name: String,
    },
}