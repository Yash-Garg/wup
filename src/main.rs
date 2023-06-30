#![allow(dead_code)]
mod cli;
mod constants;
mod download;
mod models;

use download::start_update;
use figment::{
    providers::{Format, Yaml},
    Figment,
};

use color_eyre::eyre::Result;
use models::config::VersionStore;

use crate::{constants::CLI_CONFIG, models::config::CliConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;

    let matches = cli::cli().get_matches();

    match matches.subcommand() {
        Some(("config", _)) => {
            let config = get_config();
            println!("{:#?}", config);
        }

        Some(("vstores", _)) => {
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

        Some(("update", _)) => {
            let config = get_config();
            start_update(config).await.unwrap_or_else(|e| {
                println!("Error: {}", e);
            });
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn get_config() -> CliConfig {
    let config: CliConfig = Figment::new()
        .merge(Yaml::file(CLI_CONFIG))
        .extract()
        .unwrap_or_else(|_| {
            panic!("Failed to load config.yml. Please make sure it exists and is valid YAML.");
        });

    config
}
