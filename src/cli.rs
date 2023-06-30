use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
    name = "wup",
    about = "A CLI tools version manager for Windows",
    version = env!("CARGO_PKG_VERSION"),
    author = "Yash Garg",
    arg_required_else_help = true
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    #[command(about = "Prints the current config")]
    Config,
    #[command(about = "Prints the current version stores")]
    Vstores,
    #[command(about = "Updates all the tools in the current version store")]
    Update,
}
