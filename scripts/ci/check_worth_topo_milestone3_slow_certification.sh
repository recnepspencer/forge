#!/usr/bin/env bash
set -euo pipefail

cargo test -p worth-topo --features slow-certification topology_operator_closeout --lib -- --format terse
