#!/usr/bin/env bash
# Builds dist/ — the native render first (it wipes dist/), then the wasm module into it.
set -euo pipefail

cd "$(dirname "$0")/.."

# Homebrew's rustup keeps its shims off the default PATH on macOS; harmless elsewhere.
if ! command -v cargo >/dev/null 2>&1 && [ -d /opt/homebrew/opt/rustup/bin ]; then
    export PATH="/opt/homebrew/opt/rustup/bin:$PATH"
fi

if ! command -v wasm-pack >/dev/null 2>&1; then
    echo "wasm-pack não encontrado — 'brew install wasm-pack' ou 'cargo install wasm-pack'" >&2
    exit 1
fi

echo "==> render nativo"
cargo run --quiet --bin build-site

echo "==> render para WebAssembly"
wasm-pack build \
    --release \
    --target web \
    --out-dir dist/pkg \
    --out-name winged_demo \
    --no-pack \
    -- --no-default-features --features wasm

# wasm-pack writes packaging metadata we do not serve.
rm -f dist/pkg/package.json dist/pkg/.gitignore dist/pkg/README.md

raw=$(wc -c < dist/pkg/winged_demo_bg.wasm | tr -d ' ')
gzipped=$(gzip -9 -c dist/pkg/winged_demo_bg.wasm | wc -c | tr -d ' ')
printf '==> módulo: %s bytes (%s gzipped)\n' "$raw" "$gzipped"
echo "==> dist/ pronto — ./scripts/serve.sh para abrir"
