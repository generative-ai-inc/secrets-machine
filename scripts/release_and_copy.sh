#!/bin/bash

# DEVELOPMENT SCRIPT
# This is a script to release secrets machine and copy the binary to the bin dir

cargo build --release

cp target/release/sm "$HOME/.local/bin/sm"
mkdir -p "$HOME/.config" "$HOME/.config/secrets-machine"

if [ ! -f "$HOME/.config/secrets-machine/config.toml" ]; then
  touch "$HOME/.config/secrets-machine/config.toml"
  echo "Created config.toml in $HOME/.config/secrets-machine"
else
  echo "config.toml already exists in $HOME/.config/secrets-machine"
fi
