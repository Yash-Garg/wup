#![allow(dead_code)]
mod cli;
mod constants;
mod download;
mod models;

use figment::{
    providers::{Format, Yaml},
    Figment,
};

use color_eyre::eyre::Result;

use crate::{constants::CLI_CONFIG, models::config::CliConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;

    let matches = cli::cli().get_matches();
    match matches.subcommand() {
        Some(("version", _)) => {
            println!("wup {}", env!("CARGO_PKG_VERSION"));
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
