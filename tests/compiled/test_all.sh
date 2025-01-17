#!/bin/bash

set -e
EXEC_FILE=$1

# Config file tests
./tests/compiled/eval_command.sh "$EXEC_FILE run print_hello --config ./tests/assets/secrets_machine_2.toml" "Hello" "Run print_hello"
./tests/compiled/eval_command.sh "$EXEC_FILE run print_spider --config ./tests/assets/secrets_machine_2.toml" "Peter" "Run print_spider"
./tests/compiled/eval_command.sh "$EXEC_FILE run print_man --config ./tests/assets/secrets_machine_2.toml" "" "Run print_man"
./tests/compiled/eval_command.sh "$EXEC_FILE run print_man --config ./tests/assets/secrets_machine_22.toml" "Error reading project config file: No such file or directory (os error 2)" "Run wrong config"
