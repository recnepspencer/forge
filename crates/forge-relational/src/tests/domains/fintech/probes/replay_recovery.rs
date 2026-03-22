use std::collections::BTreeMap;

use crate::facade::history::BranchId;
use crate::facade::replay::{
    RelationalReplayOutcome, RelationalReplayRequest, ReplayExecutionMode,
    ReplayVerificationMode,
};
use crate::facade::runtime::RelationalRuntime;
use crate::logic::runtime::RecoveryOutcome;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ReplayProbe {
    pub(crate) branch_name: String,
    pub(crate) commit_id: Option<u64>,
    pub(crate) mismatch_count: usize,
    pub(crate) failure: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RecoveryProbe {
    pub(crate) latest_commit_id: Option<u64>,
    pub(crate) branch_heads: BTreeMap<String, u64>,
    pub(crate) recovered_commits: usize,
    pub(crate) skipped_corrupt_checkpoints: usize,
}

pub(crate) fn capture_replay_probe(
    world: &mut super::super::fixture::FintechWorld,
    branch_id: BranchId,
) -> ReplayProbe {
    let latest = world
        .runtime
        .history_access()
        .latest_commit()
        .map(|commit| commit.commit_id);
    let replay = latest.map(|commit_id| {
        world
            .runtime
            .replay_authority()
            .replay_commit(RelationalReplayRequest {
                commit_id,
                branch_id: branch_id.clone(),
                execution_mode: ReplayExecutionMode::SerialDeterministic,
                verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
            })
    });
    replay_probe_from_outcome(branch_id.0.clone(), replay.as_ref())
}

pub(crate) fn replay_probe_from_outcome(
    branch_name: String,
    replay: Option<&RelationalReplayOutcome>,
) -> ReplayProbe {
    ReplayProbe {
        branch_name,
        commit_id: replay
            .and_then(|outcome| outcome.commit.as_ref().map(|commit| commit.commit_id.0)),
        mismatch_count: replay
            .map(|outcome| outcome.mismatches.len())
            .unwrap_or_default(),
        failure: replay.and_then(|outcome| {
            outcome
                .failure
                .as_ref()
                .map(|failure| format!("{failure:?}"))
        }),
    }
}

pub(crate) fn capture_recovery_probe(
    runtime: &RelationalRuntime,
    outcome: &RecoveryOutcome,
) -> RecoveryProbe {
    RecoveryProbe {
        latest_commit_id: runtime
            .history_access()
            .latest_commit()
            .map(|commit| commit.commit_id.0),
        branch_heads: runtime
            .history_access()
            .branches()
            .into_iter()
            .filter_map(|head| {
                head.head
                    .map(|commit| (head.branch_id.0, commit.commit_id.0))
            })
            .collect(),
        recovered_commits: outcome.recovered_commits,
        skipped_corrupt_checkpoints: outcome.integrity_report.skipped_corrupt_checkpoints.len(),
    }
}
