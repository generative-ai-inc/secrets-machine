#!/bin/bash

set -e

COMMAND=$1
EXPECTED_OUTPUT=$2
TEST_NAME=$3

# Function to capture the output of a command
capture_output() {
  local command="$1"
  # Capture the output, include lines after "Executing command", "Running command", or "ERROR", exclude lines starting with "INFO    Executing:" or "INFO    Running:", and remove empty lines
  echo "$($command)" | awk '
    /Executing command|Running command/ {found=1; next}
    /\x1b\[1;31mERROR   \x1b\[0m/ {found=1; gsub(/\x1b\[1;31mERROR   \x1b\[0m/, ""); print; next}
    found && !/^\x1b\[1;32mINFO    \x1b\[0m(Executing|Running): / && NF
  '
}
# Function to assert the output
assert_output() {
  local output="$1"
  local expected="$2"
  local test_name="$3"

  if [ "$output" == "$expected" ]; then
    printf "\x1b[1;32mINFO    \x1b[0mTest '%s' passed\n" "$test_name"
  else
    printf "\x1b[1;31mERROR   \x1b[0mTest '%s' failed\n" "$test_name"
    echo "Expected: $expected"
    echo "Got: $output"
    exit 1
  fi
}

output=$(capture_output "$COMMAND" | sed 's/\x1b\[[0-9;]*m//g')
assert_output "$output" "$EXPECTED_OUTPUT" "$TEST_NAME"
