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
            #[cfg(target_os = "windows")]
            set_path().unwrap_or_else(|e| {
                println!("Error: {}", e);
            });

            start_update(config).await.unwrap_or_else(|e| {
                println!("Error: {}", e);
            });
        }
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn set_path() -> Result<(), Box<dyn std::error::Error>> {
    use winreg::{enums::HKEY_CURRENT_USER, RegKey};

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = hkcu.create_subkey("Environment").unwrap();

    let current_path = env
        .get_value::<String, _>("PATH")
        .unwrap_or_else(|_| "".to_string());

    if !current_path.contains("WUP_PATH") {
        let new_path = format!("{};{}", current_path, "%WUP_PATH%");
        env.set_value("PATH", &new_path)?;
    }

    Ok(())
}
