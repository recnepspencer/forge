use super::super::identity::{LiveChangeOrdinal, LiveProgressError};
use super::super::patches::LivePatchEnvelope;
use super::super::promotion::LiveQueryPlan;
use super::super::relevance::BridgeChangeSummary;
use super::super::telemetry::LivePolicyCounters;
use super::change::{execute_live_change, LiveExecutionError};
use crate::basis::ResolvedSnapshotBasis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveReplayBundle {
    pub(in crate::live) query_digest: String,
    pub(in crate::live) result_digest: String,
    pub(in crate::live) delivery_digest: String,
    pub(in crate::live) replay_digest: String,
    pub(in crate::live) basis_digest: String,
    pub(in crate::live) subscription_digest: String,
    pub(in crate::live) counter_snapshot: LivePolicyCounters,
    pub(in crate::live) patch_envelope: LivePatchEnvelope,
}

impl LiveReplayBundle {
    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn result_digest(&self) -> &str {
        &self.result_digest
    }

    pub fn delivery_digest(&self) -> &str {
        &self.delivery_digest
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn subscription_digest(&self) -> &str {
        &self.subscription_digest
    }

    pub fn counter_snapshot(&self) -> &LivePolicyCounters {
        &self.counter_snapshot
    }

    pub fn patch_envelope(&self) -> &LivePatchEnvelope {
        &self.patch_envelope
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveReplayStepInput {
    pub(in crate::live) change_summary: BridgeChangeSummary,
    pub(in crate::live) next_ordinal: LiveChangeOrdinal,
    pub(in crate::live) next_basis: ResolvedSnapshotBasis,
}

impl LiveReplayStepInput {
    pub fn new(
        change_summary: BridgeChangeSummary,
        next_ordinal: LiveChangeOrdinal,
        next_basis: ResolvedSnapshotBasis,
    ) -> Self {
        Self {
            change_summary,
            next_ordinal,
            next_basis,
        }
    }

    pub fn change_summary(&self) -> &BridgeChangeSummary {
        &self.change_summary
    }

    pub fn next_ordinal(&self) -> &LiveChangeOrdinal {
        &self.next_ordinal
    }

    pub fn next_basis(&self) -> &ResolvedSnapshotBasis {
        &self.next_basis
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveReplayRun {
    pub(in crate::live) final_plan: LiveQueryPlan,
    pub(in crate::live) bundles: Vec<LiveReplayBundle>,
}

impl LiveReplayRun {
    pub fn final_plan(&self) -> &LiveQueryPlan {
        &self.final_plan
    }

    pub fn bundles(&self) -> &[LiveReplayBundle] {
        &self.bundles
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiveReplayError {
    Progress(LiveProgressError),
    Execution(LiveExecutionError),
}

impl From<LiveProgressError> for LiveReplayError {
    fn from(value: LiveProgressError) -> Self {
        Self::Progress(value)
    }
}

impl From<LiveExecutionError> for LiveReplayError {
    fn from(value: LiveExecutionError) -> Self {
        Self::Execution(value)
    }
}

pub fn replay_live_sequence(
    live: &LiveQueryPlan,
    steps: &[LiveReplayStepInput],
) -> Result<LiveReplayRun, LiveReplayError> {
    let mut current = live.clone();
    let mut bundles = Vec::with_capacity(steps.len());

    for step in steps {
        current =
            current.advance_progress(step.next_ordinal().clone(), step.next_basis().clone())?;
        let execution = execute_live_change(&current, step.change_summary())?;
        let mut replay_bundle = execution.replay_bundle().clone();
        replay_bundle.counter_snapshot.add_replay_change_count(1);
        bundles.push(replay_bundle);
    }

    Ok(LiveReplayRun {
        final_plan: current,
        bundles,
    })
}
