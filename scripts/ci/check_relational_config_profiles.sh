#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

echo "[relational-config-profiles] checking certification core profile"
cargo test -p worth-relational profile_resolution_and_provenance_are_explicit -- --nocapture

echo "[relational-config-profiles] checking mvcc retention semantics"
cargo test -p worth-relational snapshot_pins_block_reclaim_until_release -- --nocapture
cargo test -p worth-relational read_records_expose_visibility_metadata -- --nocapture
cargo test -p worth-relational chunked_storage_summary_tracks_visibility_boundaries -- --nocapture
cargo test -p worth-relational chunk_diagnostics_and_packet_plans_are_public_and_stable -- --nocapture

echo "[relational-config-profiles] PASS"
