#!/usr/bin/env bash

set -euo pipefail

echo "[signal-resource-bounds] stale-state / boundedness lane"

cargo test -p forge-signal --lib --features parallel adversarial_edges -- --nocapture
cargo test -p forge-signal --lib --features parallel adversarial_diagnostics -- --nocapture
cargo test -p forge-signal --lib --features parallel lifecycle -- --nocapture
cargo test -p forge-signal --lib --features parallel transaction_stress -- --nocapture
cargo test -p forge-signal --lib --features parallel phase5_state -- --nocapture

echo "[signal-resource-bounds] stale-state / boundedness lane is green"
