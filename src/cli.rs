use clap::{arg, command, value_parser, Command};
use clap::{ArgAction, ValueHint};
use clap_complete::Shell;
use dotenv::dotenv;
use std::path::PathBuf;

use crate::PROJECT_CONFIG_PATH_STR;

pub fn build() -> Command {
    dotenv().ok();

    command!()
    .subcommand_value_name("command")
    .about("🔑 Secrets Machine is a tool for injecting secrets at runtime")
    .subcommand(Command::new("run")
        .about("Run a command defined in the secrets_machine.toml configuration file")
        .arg(
            arg!([command_name] "Name of the command to run, as defined in the configuration file")
            .required(true)
            .value_parser(value_parser!(String))
        )
        .arg(
            arg!(
                -c --config <file> "Override the project configuration file to use."
            )
            .default_value(*PROJECT_CONFIG_PATH_STR)
            .required(false)
            .value_parser(value_parser!(PathBuf))
            .value_hint(ValueHint::AnyPath),
        )
        // Allow passing direct args to the command
        .arg(
            arg!(
                [command_args] ... "Arguments passed after --"
            )
            .required(false)
            .value_parser(value_parser!(String))
            .value_hint(ValueHint::Other)
            .allow_hyphen_values(true)
            .last(true)
        )
    )
    .subcommand(Command::new("exec")
        .about("Execute an arbitrary command using the Secrets Machine environment")
        .arg(
            arg!(
                -c --config <file> "Override the project configuration file to use."
            )
            .default_value(*PROJECT_CONFIG_PATH_STR)
            .required(false)
            .value_parser(value_parser!(PathBuf))
            .value_hint(ValueHint::AnyPath),
        )
        .arg(
            arg!(<command> "Command to execute")
            .required(true)
            .value_parser(value_parser!(String))
            .value_hint(ValueHint::Other)
            .allow_hyphen_values(true)
        )
    )
    .subcommand(Command::new("secret")
        .subcommand_value_name("action")
        .about("Add or remove a secret")
        .subcommand_required(true)
        .subcommand(Command::new("add")
            .about("Add a secret")
            .arg_required_else_help(true)
            .arg(
                arg!(<name> "Name of the secret")
                .required(true)
                .value_parser(value_parser!(String))
                .value_hint(ValueHint::Other),
            )
            .arg(
                arg!([value] "Value of the secret. If not provided, you will be prompted for it.")
                .required(false)
                .value_parser(value_parser!(String))
                .value_hint(ValueHint::Other),
            )
        )
        .subcommand(Command::new("remove")
            .arg_required_else_help(true)
            .about("Remove a secret")
            .arg(
                arg!(<name> "Name of the secret")
                .required(true)
                .value_parser(value_parser!(String))
                .value_hint(ValueHint::Other),
            )
            .arg(
                arg!(-a --all "Remove all secrets")
                .action(ArgAction::SetTrue)
            )
        )
        .subcommand(Command::new("list")
            .about("List your secrets")
        )
    )
    .subcommand(Command::new("update")
        .about("Update Secrets Machine")
    )
    .subcommand(Command::new("completions")
        .about("Generate shell completions. Place the output in your shell's completions directory")
        .arg_required_else_help(true)
        .arg(
            arg!(<shell> "Shell to generate completions for.")
            .required(true)
            .value_parser(value_parser!(Shell))
        )
    )
}
