#!/usr/bin/env bash

RELEASE_URL="https://github.com/mufeedvh/binserve/releases/download"
RELEASE_VERSION=$1
UNAME=$(uname)
OSX="binserve-${RELEASE_VERSION}-x86_64-macos"
BIN_LOCATION="/usr/local/bin/binserve"

if [[ $UNAME -eq "Darwin" ]]; then
  OS=${OSX}
fi

download_and_install() {
  echo "🚀 install and download binserve..."
  echo "✅ ${RELEASE_URL}/${RELEASE_VERSION}/${OS}"
  # download
  command curl -fsSL "${RELEASE_URL}/${RELEASE_VERSION}/${OS}" -o $HOME/binserve
  # move to $BIN_LOCATION
  command mv $HOME/binserve $BIN_LOCATION
  command chmod +x $BIN_LOCATION
}

download_and_install
