pub mod args;
mod cli;
pub mod utils;

use anyhow::Result;
use args::{GenerateSubcommand, GenerateSubcommandVersion};

use crate::args::{Args, Subcommand};

pub async fn run(args: Args) -> Result<()> {
    match args.subcommand {
        Some(Subcommand::Generate(GenerateSubcommand::Version { version })) => match version {
            GenerateSubcommandVersion::V1 {
                seconds,
                nanos,
                seed,
            } => {
                let uuid = cli::time(&seconds, &nanos, seed)?;

                println!("{}", uuid);
            }
            GenerateSubcommandVersion::V2 => {
                println!("V2 is deprecated");
            }
            GenerateSubcommandVersion::V3 { namespace, name } => {
                let uuid = cli::md5(&namespace, &name)?;

                println!("{}", uuid);
            }
            GenerateSubcommandVersion::V4 => {
                let uuid = cli::random()?;

                println!("{}", uuid);
            }
            GenerateSubcommandVersion::V5 { namespace, name } => {
                let uuid = cli::sha1(&namespace, &name)?;

                println!("{}", uuid);
            }
            GenerateSubcommandVersion::V8 { data } => {
                let uuid = cli::data(&data)?;

                println!("{}", uuid);
            }
        },
        Some(Subcommand::Validate { uuid }) => match uuid.parse::<uuid::Uuid>() {
            Ok(_) => {
                println!("{}", uuid);
                return Ok(());
            }
            Err(err) => {
                println!("Invalid UUID: {}", err);
                return Ok(());
            }
        },
        _ => {
            unreachable!("Unknown subcommand");
        }
    }
    Ok(())
}
