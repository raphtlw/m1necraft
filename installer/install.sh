#!/usr/bin/env bash

# ----------------------------------------------------------------------------
# install.sh: Simple script to install M1necraft on macOS
#
# Author: Raphael Tang <raphpb1912@gmail.com>
# ----------------------------------------------------------------------------
# Dependencies:
#  - curl

CLEAR='\033[0m'
RED='\033[0;31m'
DARKGRAY='\033[1;30m'
LIGHTRED='\033[1;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
LIGHTPURPLE='\033[1;35m'
CYAN='\033[0;36m'
WHITE='\033[1;37m'

APP_NAME="M1necraft.app"

WORKDIR="$TMPDIR/m1necraft-$RANDOM"

function usage() {
  if [ -n "$1" ]; then
    echo -e "${RED}👉 $1${CLEAR}\n";
  fi

  cat << EOF
  Usage: $(basename $0)
    -h, --help     Show this message
    -v, --version  Version to download, if specified

  Examples: $(basename $0) --version 0.2.0
EOF
}

# parse params
while [[ "$#" > 0 ]]; do case $1 in
      -h|--help)    usage; exit 0 ;;
      -v|--version) INSTALL_VERSION="$2"; shift; shift ;;
      *)            usage "Unknown parameter passed: $1"; exit 1 ;;
esac; done

trash() {
  osascript -e "tell application \"Finder\" to delete POSIX file \"$1\""
}

cleanup() {
  rm -rf "$WORKDIR"
}

main() {
  mkdir -p "$WORKDIR"
  pushd "$WORKDIR" >/dev/null

  echo -e "\n    ${DARKGRAY}=== Begin install ===${CLEAR}"

  echo -e "\n${GREEN} * Downloading $APP_NAME.zip${CLEAR}"

  if [ -n "$INSTALL_VERSION" ]; then
    # Download specific version
    curl -L "https://github.com/raphtlw/m1necraft/releases/download/v$INSTALL_VERSION/$APP_NAME.zip" --output "$APP_NAME.zip"
  else
    # Download latest version
    curl -L "https://github.com/raphtlw/m1necraft/releases/latest/download/$APP_NAME.zip" --output "$APP_NAME.zip"
  fi

  # Check if not found
  (zip -T "$APP_NAME.zip" >/dev/null 2>&1) || {
    echo -e "\n${RED}  ERROR: Version not found${CLEAR}"
    cleanup
    exit 1
  }

  echo -e "${GREEN} * Extracting $APP_NAME.zip${CLEAR}"
  unzip $APP_NAME.zip >/dev/null
  chmod a+x $APP_NAME/Contents/MacOS/M1necraft
  [ -d "/Applications/$APP_NAME" ] && trash "/Applications/$APP_NAME"
  mv "$APP_NAME" "/Applications/$APP_NAME"

  echo -e "${GREEN} * Unlocking $APP_NAME so that it can run properly${CLEAR}"
  spctl --add "/Applications/$APP_NAME"

  echo -e "\n    ${GREEN}=== Install complete 🚀 ===${CLEAR}"

  popd >/dev/null
}

main
cleanup
