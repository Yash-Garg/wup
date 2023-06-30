use clap::Command;

pub fn cli() -> Command {
    Command::new("wup")
        .about("A CLI tools version manager for Windows")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
}
