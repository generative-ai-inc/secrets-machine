#!/bin/bash

set -e

abort() {
  printf "%s\n" "$@" >&2
  exit 1
}

SM_DIR="$HOME/.local/bin"
SM_PATH="${SM_DIR}/sm"

echo "Installing Secrets Machine to $SM_DIR"

OS="$(/usr/bin/uname -s)"
ARCH="$(/usr/bin/uname -m)"

# Make os lowercase
OS="$(echo ${OS} | tr '[:upper:]' '[:lower:]')"
ASSET_NAME="secrets-machine-${ARCH}-${OS}"

# Get the latest release information
RELEASE_INFO=$(curl -fsSL https://api.github.com/repos/generative-ai-inc/secrets-machine/releases/latest)

# Extract the asset download URL
ASSET_URL=$(echo $RELEASE_INFO | python3 -c "import sys, json; data = json.load(sys.stdin); print(next(asset['url'] for asset in data['assets'] if asset['name'] == '$ASSET_NAME'))")

# Check if the asset URL is found
if [ -z "$ASSET_URL" ]; then
  echo "Asset not found: $ASSET_NAME"
  exit 1
fi

TEMP_FILE=$(mktemp)

# Add accepts header to the request
curl -H "Accept: application/octet-stream" -fsSL "${ASSET_URL}" -o "${TEMP_FILE}"

# Check if the file was downloaded successfully
if [ ! -s "${TEMP_FILE}" ]; then
  echo "Failed to download the asset: ${ASSET_URL}"
  exit 1
fi

# Create the directory if it doesn't exist
mkdir -p "${SM_DIR}"

# Move the file to the correct location
mv "${TEMP_FILE}" "${SM_PATH}"

# Make the file executable
chmod +x "${SM_PATH}"

mkdir -p "$HOME/.config" "$HOME/.config/secrets-machine"

if [ ! -f "$HOME/.config/secrets-machine/config.toml" ]; then
  touch "$HOME/.config/secrets-machine/config.toml"
  echo "Created config.toml in $HOME/.config/secrets-machine"
else
  echo "config.toml already exists in $HOME/.config/secrets-machine"
fi

echo "🔑 Secrets Machine installed. Run 'sm' to get started."
