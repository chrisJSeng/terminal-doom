#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY="$SCRIPT_DIR/target/release/doom-terminal"

if [ ! -f "$BINARY" ]; then
    echo "Building release binary..."
    cd "$SCRIPT_DIR"
    cargo build --release -q
fi

exec "$BINARY" "$@"
