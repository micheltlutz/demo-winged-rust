#!/usr/bin/env bash
# Serves dist/ over HTTP. WebAssembly does not load from file://.
set -euo pipefail

cd "$(dirname "$0")/.."
port="${PORT:-8000}"

if [ ! -f dist/index.html ]; then
    echo "dist/ está vazio — rode ./scripts/build.sh primeiro." >&2
    exit 1
fi

echo "http://localhost:$port"
exec python3 -m http.server "$port" --directory dist
