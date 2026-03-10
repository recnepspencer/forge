#!/usr/bin/env bash

set -euo pipefail

echo "[signal-adversarial] workflow and property certification lane"

cargo test -p forge-signal --lib --features parallel adversarial_properties -- --nocapture
cargo test -p forge-signal --lib --features parallel adversarial_workflows -- --nocapture

if [[ "${1:-}" == "--long" ]]; then
  cargo test -p forge-signal --lib --features parallel adversarial_workflows -- --ignored --nocapture
fi

echo "[signal-adversarial] lane is green"
