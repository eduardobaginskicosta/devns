#!/usr/bin/env bash
set -euo pipefail

# please, support me on ko-fi: https://ko-fi.com/baginskistudio
# and follow me on social media:
# - github (personal): eduardobaginskicosta
# - github (business): baginskistudios
# - instagram: eduardobaginskicosta
# - linkedin: eduardobaginskicosta
#
# build.sh                     : native (debug)
# build.sh --release           : native (release)
# build.sh --linux             : linux x86_64 gnu (debug)
# build.sh --linyx --release   : linux x86_64 gnu (release)
# build.sh --windows           : windows x86_64 gnu (debug)
# build.sh --windows --release : windows x86_64 gnu (release)

CARGO_FLAGS=()
TARGET=""
MODE="debug"

usage() {
  echo "Usage: $0 [--windows|--linux|--any] [--release]"
  exit 1
}

for arg in "$@"; do
  case "$arg" in
    --windows)
      TARGET="x86_64-pc-windows-gnu"
      ;;
    --linux)
      TARGET="x86_64-unknown-linux-gnu"
      ;;
    --any)
      TARGET=""
      ;;
    --release)
      MODE="release"
      CARGO_FLAGS+=("--release")
      ;;
    *)
      echo "Unknown argument: $arg"
      usage
      ;;
  esac
done

echo "Building mode: $MODE"
echo "Target: ${TARGET:-native}"

CMD=(cargo build "${CARGO_FLAGS[@]}")

if [ -n "$TARGET" ]; then
  CMD+=(--target "$TARGET")
fi

"${CMD[@]}"
