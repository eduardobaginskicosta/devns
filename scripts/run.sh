#!/usr/bin/env bash
set -euo pipefail

# please, support me on ko-fi: https://ko-fi.com/baginskistudio
# and follow me on social media:
# - github (personal): eduardobaginskicosta
# - github (business): baginskistudios
# - instagram: eduardobaginskicosta
# - linkedin: eduardobaginskicosta
#
# run.sh                     : native (debug)
# run.sh --release           : native(release)
# run.sh --linux             : linux x86_64 musl (debug)
# run.sh --linux --release   : linux x86_64 musl (release)
# run.sh --windows           : windows x86_64 gnu (debug)
# run.sh --windows --release : windows x86_64 gnu (release)

# require sudo
if [ "$EUID" -ne 0 ]; then
  echo "run.sh must be executed with sudo"
  echo "Usage: sudo $0 [args]"
  exit 1
fi

TARGET=""
MODE="debug"

# default env
: "${PORT:=53}"
: "${DEBUG_MODE:=false}"
: "${MAX_WORKERS:=10}"
: "${MAX_MESSAGES:=20}"
: "${DNS_SERVERS:=1.1.1.1;1.0.0.1;8.8.8.8}"

for arg in "$@"; do
  case "$arg" in
    --windows)
      TARGET="x86_64-pc-windows-gnu"
      ;;
    --linux)
      TARGET="x86_64-unknown-linux-musl"
      ;;
    --any)
      TARGET=""
      ;;
    --release)
      MODE="release"
      ;;
    *)
      echo "Unknown argument: $arg"
      exit 1
      ;;
  esac
done

if [ "$MODE" == "release" ]; then
  BUILD_DIR="release"
else
  BUILD_DIR="debug"
fi

BIN_NAME="devns"

if [ -z "$TARGET" ]; then
  BIN_PATH="target/${BUILD_DIR}/${BIN_NAME}"
else
  BIN_PATH="target/${TARGET}/${BUILD_DIR}/${BIN_NAME}"
fi

echo "Running DevNS..."
echo "Binary: $BIN_PATH"

if [ ! -f "$BIN_PATH" ]; then
  echo "binary not found (did you run build.sh?)"
  exit 1
fi

export PORT DEBUG_MODE MAX_WORKERS MAX_MESSAGES DNS_SERVERS
exec "$BIN_PATH"
