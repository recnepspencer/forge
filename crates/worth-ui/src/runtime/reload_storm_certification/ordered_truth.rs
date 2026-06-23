use crate::runtime::{
    WorthUiActiveRuntimeObservation, WorthUiLastValidObservation,
    WorthUiReloadStormIterationOutcome,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiReloadStormOrderedTruth {
    initial_active_artifact_digest: u64,
    initial_active_plan_digest: u64,
    initial_capability_snapshot_digest: u64,
    initial_authoring_snapshot_digest: Option<u64>,
    final_active_artifact_digest: u64,
    final_active_plan_digest: u64,
    final_capability_snapshot_digest: u64,
    final_authoring_snapshot_digest: Option<u64>,
    final_last_valid_artifact_digest: u64,
    final_last_valid_plan_digest: u64,
    activated_plan_digests: Vec<u64>,
    denied_preservation_plan_digests: Vec<u64>,
    no_op_plan_digests: Vec<u64>,
}

impl WorthUiReloadStormOrderedTruth {
    pub(crate) fn from_outcomes(
        initial_active: WorthUiActiveRuntimeObservation,
        final_active: WorthUiActiveRuntimeObservation,
        final_last_valid: WorthUiLastValidObservation,
        outcomes: &[WorthUiReloadStormIterationOutcome],
    ) -> Self {
        let mut activated_plan_digests = Vec::new();
        let mut denied_preservation_plan_digests = Vec::new();
        let mut no_op_plan_digests = Vec::new();
        for outcome in outcomes {
            match outcome {
                WorthUiReloadStormIterationOutcome::Activated(iteration) => {
                    activated_plan_digests.push(iteration.active_plan_digest_after());
                }
                WorthUiReloadStormIterationOutcome::EquivalentNoOp(iteration) => {
                    no_op_plan_digests.push(iteration.active_plan_digest());
                }
                WorthUiReloadStormIterationOutcome::DeniedPreserved(iteration) => {
                    denied_preservation_plan_digests.push(iteration.active_plan_digest_after());
                }
            }
        }
        Self {
            initial_active_artifact_digest: initial_active.artifact_digest(),
            initial_active_plan_digest: initial_active.active_plan_digest(),
            initial_capability_snapshot_digest: initial_active.snapshot_digest(),
            initial_authoring_snapshot_digest: initial_active.authoring_snapshot_digest(),
            final_active_artifact_digest: final_active.artifact_digest(),
            final_active_plan_digest: final_active.active_plan_digest(),
            final_capability_snapshot_digest: final_active.snapshot_digest(),
            final_authoring_snapshot_digest: final_active.authoring_snapshot_digest(),
            final_last_valid_artifact_digest: final_last_valid.artifact_digest(),
            final_last_valid_plan_digest: final_last_valid.active_plan_digest(),
            activated_plan_digests,
            denied_preservation_plan_digests,
            no_op_plan_digests,
        }
    }

    pub fn initial_active_artifact_digest(&self) -> u64 {
        self.initial_active_artifact_digest
    }

    pub fn initial_active_plan_digest(&self) -> u64 {
        self.initial_active_plan_digest
    }

    pub fn initial_capability_snapshot_digest(&self) -> u64 {
        self.initial_capability_snapshot_digest
    }

    pub fn initial_authoring_snapshot_digest(&self) -> Option<u64> {
        self.initial_authoring_snapshot_digest
    }

    pub fn final_active_artifact_digest(&self) -> u64 {
        self.final_active_artifact_digest
    }

    pub fn final_active_plan_digest(&self) -> u64 {
        self.final_active_plan_digest
    }

    pub fn final_capability_snapshot_digest(&self) -> u64 {
        self.final_capability_snapshot_digest
    }

    pub fn final_authoring_snapshot_digest(&self) -> Option<u64> {
        self.final_authoring_snapshot_digest
    }

    pub fn final_last_valid_artifact_digest(&self) -> u64 {
        self.final_last_valid_artifact_digest
    }

    pub fn final_last_valid_plan_digest(&self) -> u64 {
        self.final_last_valid_plan_digest
    }

    pub fn activated_plan_digests(&self) -> &[u64] {
        &self.activated_plan_digests
    }

    pub fn denied_preservation_plan_digests(&self) -> &[u64] {
        &self.denied_preservation_plan_digests
    }

    pub fn no_op_plan_digests(&self) -> &[u64] {
        &self.no_op_plan_digests
    }
}
