#![allow(dead_code)]
mod cli;
mod constants;
mod download;
mod models;

use clap::Parser;
use cli::{Cli, Commands};
use download::start_update;

use color_eyre::eyre::Result;
use models::config::{CliConfig, VersionStore};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();
    match cli.command {
        Commands::Config => {
            let config = CliConfig::get();
            println!("{:#?}", config);
        }

        Commands::Vstores => {
            let stores = VersionStore::read();
            match &stores {
                Ok(stores) => {
                    if stores.is_empty() {
                        println!("No version stores found.");
                    } else {
                        println!("{:#?}", stores);
                    }
                }

                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }

        Commands::Update => {
            let config = CliConfig::get();
            start_update(config).await.unwrap_or_else(|e| {
                println!("Error: {}", e);
            });
        }
    }

    Ok(())
}
