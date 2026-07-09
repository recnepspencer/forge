#!/usr/bin/env bash

set -euo pipefail

echo "[signal-contracts] adversarial contract matrix lane"

cargo test -p worth-signal --lib --features parallel phase3_semantics -- --nocapture
cargo test -p worth-signal --lib --features parallel phase4_planner -- --nocapture
cargo test -p worth-signal --lib --features parallel adversarial_parallel -- --nocapture
cargo test -p worth-signal --lib --features parallel observability -- --nocapture
cargo test -p worth-signal --lib --features parallel diagnostics -- --nocapture
cargo test -p worth-signal --lib --features parallel logic::events::tests -- --nocapture
cargo test -p worth-signal --lib --features parallel logic::transaction::tests -- --nocapture

echo "[signal-contracts] adversarial contract matrix lane is green"
