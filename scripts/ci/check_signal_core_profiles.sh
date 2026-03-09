#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

echo "[signal-core-profiles] checking standard profile"
cargo check -p forge-signal
cargo check -p forge-signal --features parallel

echo "[signal-core-profiles] checking compact profile"
cargo check -p forge-signal --no-default-features --features profile-compact
cargo check -p forge-signal --no-default-features --features "parallel,profile-compact"
cargo test -p forge-signal --lib --no-default-features --features "parallel,profile-compact" harness_adapter -- --nocapture

echo "[signal-core-profiles] checking extended profile"
cargo check -p forge-signal --no-default-features --features profile-extended
cargo check -p forge-signal --no-default-features --features "parallel,profile-extended"
cargo test -p forge-signal --lib --no-default-features --features "parallel,profile-extended" harness_adapter -- --nocapture

echo "[signal-core-profiles] PASS"
