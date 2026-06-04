#!/usr/bin/env bash
set -euo pipefail

# please, support me on ko-fi: https://ko-fi.com/baginskistudio
# and follow me on social media:
# - github (personal): eduardobaginskicosta
# - github (business): baginskistudios
# - instagram: eduardobaginskicosta
# - linkedin: eduardobaginskicosta
#
# docker.sh              : cargo build (linux release) + docker build + docker compose up
# docker.sh --build-only : cargo build (linux release) + docker build
# docker.sh --down       : docker compose down

BUILD_ONLY=false
DOWN_ONLY=false

usage() {
echo "Usage: $0 [--build-only] [--down]"
exit 1
}

for arg in "$@"; do
case "$arg" in
--build-only)
BUILD_ONLY=true
;;
--down)
DOWN_ONLY=true
;;
*)
echo "Unknown argument: $arg"
usage
;;
esac
done

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

BUILD_SCRIPT="$PROJECT_ROOT/scripts/build.sh"
DOCKER_DIR="$PROJECT_ROOT/docker"

# --down não faz build

if $DOWN_ONLY; then
echo "Stopping DevNS..."

cd "$DOCKER_DIR"
docker compose down

echo "DevNS stopped."
exit 0
fi

echo "Building DevNS binary..."
"$BUILD_SCRIPT" --linux --release

echo "Building Docker image..."
cd "$DOCKER_DIR"

docker compose build

if $BUILD_ONLY; then
echo "Docker image built successfully."
echo "Build-only mode enabled. Container not started."
exit 0
fi

echo "Starting container..."
docker compose up -d

echo "DevNS started successfully."
