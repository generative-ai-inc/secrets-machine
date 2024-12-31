# Secrets Machine

Secrets Machine is a tool for injecting secrets into your environment at runtime. It is useful for CI/CD pipelines, local development, and other scenarios where you need to inject secrets into your environment.

## 😕 Without Secrets Machine

```sh
$ python3
Python 3.12.4 (main, Jun  6 2024, 18:26:44) [GCC 11.4.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> import os
>>> os.getenv("MY_SECRET")
>>>
```

## 😊 With Secrets Machine

```sh
$ sm exec python3 # <--- This is the command that injects the secrets
Python 3.12.4 (main, Jun  6 2024, 18:26:44) [GCC 11.4.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> import os
>>> os.getenv("MY_SECRET")
>>> my-secret-value
```

Secrets are read from the following sources, in this order:

1. ~~A dotenv file (.env)~~ (TODO)
2. Environment variables
3. Secret Sources (e.g. Bitwarden)
4. Keyring

Supported platforms (Contributions welcome!):

- MacOS
- Linux

## Installation

```sh
bash <(curl -sS "https://raw.githubusercontent.com/generative-ai-inc/secrets-machine/main/install.sh")
```

### Execute a command with secrets

Executing a command with `sm exec` gives the command access to the configured secrets. Use it to run commands as you would normally do.

```sh
sm exec 'cargo run'
sm exec 'python3'
sm exec 'pnpm run dev'
```

These are different ways to evaluate environment variables in your execution command. They are all equivalent:

```sh
sm exec 'echo "My secret is $MY_SECRET"' # This one is recommended
sm exec "echo \"My secret is \$MY_SECRET\""
sm exec "echo My secret is \$MY_SECRET"
# OUTPUT: My secret is my-secret-value
```

```sh
sm exec 'echo $MY_SECRET'
sm exec "echo \$MY_SECRET"
sm exec "echo \${MY_SECRET}"
# OUTPUT: My secret is my-secret-value
```

### Run a pre-configured command

Create a `secrets_machine.toml` configuration file. See [secrets_machine.toml](https://github.com/generative-ai-inc/secrets-machine/blob/main/secrets_machine.toml) for an example.

Then run:

```sh
sm run <command-name>
```

For example, given the following configuration:

```toml
[commands]
  dev  = "cargo run"
  test = "cargo test"
```

You can run:

```sh
sm run dev
sm run test
```

### Configuration

The configuration file is located at `~/.config/secrets-machine/config.toml`.
For now, only Bitwarden is supported as a secrets source.

#### Secrets Sources

##### Keyring

You can always add secrets to the keyring with the `sm secret add` command. For example:

```sh
sm secret add GITHUB_USERNAME <github-username>
sm secret add GITHUB_TOKEN
```

##### Bitwarden Secret Manager

To use the bitwarden secret manager, you need to have the BWS_ACCESS_TOKEN variable set. We recommend using the keyring to store this token. You can do this with the following command:

```sh
sm secret add BWS_ACCESS_TOKEN
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
