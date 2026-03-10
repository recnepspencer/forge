#!/usr/bin/env bash

set -euo pipefail

echo "[signal-phase5] snapshot / branch / lineage contract lane"

cargo test -p forge-signal --lib --features parallel phase5_state -- --nocapture
cargo test -p forge-signal --lib --features parallel phase5_workflows -- --nocapture

echo "[signal-phase5] phase 5 contract lane is green"
