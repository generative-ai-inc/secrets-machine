use clap::{ArgMatches, Command};
use clap_complete::{generate, Generator, Shell};
use lazy_static::lazy_static;
use serde_json::{json, Value};
use sm::library::commands::{execute, run};
use sm::library::config::commands;
use sm::library::secrets::{generic, keyring};
use sm::library::secrets_sources;
use sm::library::system::{commands_config, config};
use sm::library::utils::{env_vars, logging, updater};
use std::io;
use std::path::PathBuf;

mod cli;

// Use lazy_static to avoid leaking string in an uncontrolled way
lazy_static! {
    pub static ref COMMANDS_CONFIG_PATH: PathBuf = PathBuf::from("secrets_machine.toml");
    pub static ref COMMANDS_CONFIG_PATH_STR: &'static str = Box::leak(
        COMMANDS_CONFIG_PATH
            .to_str()
            .unwrap()
            .to_string()
            .into_boxed_str()
    );
    pub static ref CONFIG_PATH: Option<PathBuf> = None;
    pub static ref BIND_ADDRESS: String = "127.0.0.1:8000".to_string();
    pub static ref BIND_ADDRESS_STR: &'static str =
        Box::leak(BIND_ADDRESS.clone().into_boxed_str());
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

async fn handle_run_mode(matches: ArgMatches) {
    // Run options
    let commands_config_path = COMMANDS_CONFIG_PATH.clone();

    let mut command_args = String::new();

    let mut command_name: String = String::new();
    if let Some(run_matches) = matches.subcommand_matches("run") {
        if let Some(passed_command_name) = run_matches.get_one::<String>("command_name") {
            passed_command_name.clone_into(&mut command_name);
            logging::info(&format!("Command: {passed_command_name}")).await;
        } else {
            logging::error("Unable to get command from run mode").await;
            std::process::exit(1);
        }

        if let Some(passed_command_args) = run_matches.get_many::<String>("command_args") {
            for arg in passed_command_args {
                command_args = command_args + arg + " ";
            }
        }
    }

    let commands_config = commands_config::parse(commands_config_path).await;

    let config = config::parse(None).await;

    // Check that the command is in the config
    commands::check(&commands_config, &command_name).await;

    let secrets = keyring::get_secrets().await;

    secrets_sources::check(&config, &secrets).await;

    match run(commands_config, config, secrets, command_name, command_args).await {
        Ok(()) => (),
        Err(e) => {
            // Only print the error log if there was an error on our side
            let formatted_error = format!("{e}");
            if formatted_error != "Command failed" {
                logging::error(&format!("{e}")).await;
            }
            std::process::exit(1);
        }
    }
}

async fn handle_exec_mode(matches: ArgMatches) {
    let mut command_to_run: String = String::new();

    if let Some(exec_matches) = matches.subcommand_matches("exec") {
        if let Some(passed_command_to_run) = exec_matches.get_one::<String>("command") {
            passed_command_to_run.clone_into(&mut command_to_run);
        } else {
            logging::error("Unable to get command from exec mode").await;
            std::process::exit(1);
        }
    }

    let config = config::parse(None).await;

    let secrets = keyring::get_secrets().await;

    secrets_sources::check(&config, &secrets).await;

    match execute(config, secrets, command_to_run.as_str()).await {
        Ok(()) => (),
        Err(e) => {
            // Only print the error log if there was an error on our side
            let formatted_error = format!("{e}");
            if formatted_error != "Command failed" {
                logging::error(&format!("{e}")).await;
            }
            std::process::exit(1);
        }
    }
}

#[tokio::main]
async fn main() {
    let matches = cli::build().get_matches();

    let run_mode = matches.subcommand_matches("run").is_some();
    let exec_mode = matches.subcommand_matches("exec").is_some();
    let update_mode = matches.subcommand_matches("update").is_some();
    let completions_mode = matches.subcommand_matches("completions").is_some();
    let secrets_mode = matches.subcommand_matches("secret").is_some();

    if run_mode {
        handle_run_mode(matches).await;
    } else if exec_mode {
        handle_exec_mode(matches).await;
    } else if update_mode {
        updater::update().await;
    } else if completions_mode {
        if let Some(completions_matches) = matches.subcommand_matches("completions") {
            if let Some(shell) = completions_matches.get_one::<Shell>("shell").copied() {
                let mut cmd = cli::build();
                print_completions(shell, &mut cmd);
            }
        }
    } else if secrets_mode {
        if let Some(secrets_matches) = matches.subcommand_matches("secret") {
            if let Some(add_matches) = secrets_matches.subcommand_matches("add") {
                if let Some(name) = add_matches.get_one::<String>("name") {
                    let upper_name = name.to_uppercase();
                    env_vars::verify_name(upper_name.clone()).await;

                    let mut secrets = keyring::get_secrets().await;
                    let secret;
                    if let Some(value) = add_matches.get_one::<String>("value") {
                        secret = value.to_owned();
                    } else {
                        secret = generic::ask_for_secret(&upper_name).await;
                    }
                    secrets[upper_name] = json!(secret);
                    keyring::set_secret(secrets).await;
                }
            } else if let Some(remove_matches) = secrets_matches.subcommand_matches("remove") {
                if remove_matches.get_flag("all") {
                    let secrets = json!({});
                    keyring::set_secret(secrets).await;
                } else if let Some(name) = remove_matches.get_one::<String>("name") {
                    let upper_name = name.to_uppercase();
                    env_vars::verify_name(upper_name.clone()).await;
                    let mut secrets = keyring::get_secrets().await;
                    if let Value::Object(ref mut map) = secrets {
                        map.remove(&upper_name);
                    }
                    keyring::set_secret(secrets).await;
                }
            } else if secrets_matches.subcommand_matches("list").is_some() {
                let credentials = keyring::get_secrets().await;
                for (key, _) in credentials.as_object().unwrap() {
                    println!("{key}");
                }
            }
        }
    } else {
        cli::build().print_help().unwrap();
        std::process::exit(1);
    }
}
