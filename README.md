# Secrets Machine

Secrets Machine is a tool for injecting secrets into your environment at runtime. It is useful for CI/CD pipelines, local development, and other scenarios where you need to inject secrets into your environment.

Secrets are read from the following sources, in this order:

1. A dotenv file (.env)
2. Secret Sources (e.g. Bitwarden)
3. Environment variables
4. Keyring

Supported platforms (Contributions welcome!):

- MacOS
- Linux

## Installation

```sh
bash <(curl -sS "https://raw.githubusercontent.com/generative-ai-inc/secrets-machine/main/install.sh")
```

### Execute a command with secrets

```sh
sm run echo "My secret is $MY_SECRET"
```

### Run a pre-configured command

Set up your `secrets_machine.toml` configuration file. See [secrets_machine.toml](https://github.com/generative-ai-inc/secrets-machine/blob/main/secrets_machine.toml) for an example.

Then run:

```sh
sm run <command-name>
```

### Secret Sources

#### Bitwarden Secret Manager

To use the bitwarden secret manager, you need to have the BWS_ACCESS_TOKEN variable set. We recommend using the keyring to store this token. You can do this with the following command:

```sh
sm secret add BWS_ACCESS_TOKEN
```

#### Add Secrets to the Keyring

You can add them to the keyring with the `sm secret add` command. For example:

```sh
sm secret add GITHUB_USERNAME <github-username>
sm secret add GITHUB_TOKEN
```

## Suggestions

### Shell Autocomplete

#### Zsh

To add completions for zsh, execute the following:

```

mkdir -p ${ZDOTDIR:-~}/.zsh_functions
echo 'fpath+=${ZDOTDIR:-~}/.zsh_functions' >> ${ZDOTDIR:-~}/.zshrc
sm completions zsh > ${ZDOTDIR:-~}/.zsh_functions/\_sm

```

#### Other Shells

In general, you can generate completions for any shell with the following command:

```sh
sm completions <shell>
```

If you are not sure what to do with the output of this command, the people from Alacritty have a good [guide](https://github.com/alacritty/alacritty/blob/master/INSTALL.md#shell-completions) on how to add shell completions to your shell. In the guide it is assumed that you are adding the completions for the `alacritty` command, but the process is similar for other commands, like `sm`.
# secrets-machine
