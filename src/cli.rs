use clap::Command;

enum CliCommands {
    Config,
    Vstores,
    Update,
}

impl CliCommands {
    fn to_str(&self) -> &str {
        match self {
            CliCommands::Config => "config",
            CliCommands::Vstores => "vstores",
            CliCommands::Update => "update",
        }
    }

    fn get(&self) -> Command {
        match self {
            CliCommands::Config => {
                Command::new(CliCommands::Config.to_str()).about("Prints the current config")
            }
            CliCommands::Vstores => Command::new(CliCommands::Vstores.to_str())
                .about("Prints the current version stores"),
            CliCommands::Update => Command::new(CliCommands::Update.to_str())
                .about("Updates all the tools in the current version store"),
        }
    }
}

pub fn cli() -> Command {
    Command::new("wup")
        .about("A CLI tools version manager for Windows")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .version(env!("CARGO_PKG_VERSION"))
        .author("Yash Garg")
        .subcommand(CliCommands::Config.get())
        .subcommand(CliCommands::Vstores.get())
        .subcommand(CliCommands::Update.get())
}
