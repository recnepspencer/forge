//! Replay logging utilities.
//!
//! DOMAIN: Generic utilities for tracking and validating pipeline replay snapshots.

use forge_core::DecisionLog;
use forge_topo::provenance::OpSignature;
use forge_topo::provenance::{ReplayLog, ReplayEntry};

use crate::core::ModelingContext;

/// Record a replay entry with pre/post hashes and auto-computed decision delta.
pub fn record_replay(
    log: &mut ReplayLog,
    seq: &mut u64,
    name: &'static str,
    payload: String,
    pre_hash: u128,
    post_hash: u128,
    ctx: &ModelingContext,
    prev_snapshot: &mut Option<DecisionLog>,
) {
    *seq += 1;
    let mut entry = ReplayEntry::new(
        OpSignature::with_id(name, *seq), payload.into_bytes(), *seq, pre_hash,
    );
    entry.set_post_hash(post_hash);

    let current_log = ctx.get_decision_log();
    if let Some(prev) = prev_snapshot.as_ref() {
        let delta = forge_core::tracing::checkpoint_diff::diff_decision_logs(prev, current_log);
        entry.set_decision_delta(delta);
    }
    *prev_snapshot = Some(current_log.clone());

    log.record(entry);
}
