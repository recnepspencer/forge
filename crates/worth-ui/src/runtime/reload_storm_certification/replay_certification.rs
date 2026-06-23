use crate::runtime::WorthUiReloadStormCertification;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiReloadReplayCertification {
    scenario_digest: u64,
    final_active_artifact_digest: u64,
    final_active_plan_digest: u64,
    final_capability_snapshot_digest: u64,
    final_authoring_snapshot_digest: Option<u64>,
    final_last_valid_artifact_digest: u64,
    final_last_valid_plan_digest: u64,
    iteration_count: usize,
    foundational_meaning_digest: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiReloadReplayCertificationDenial {
    ScenarioDigestMismatch,
    InitialTruthMismatch,
    FinalActiveArtifactMismatch,
    FinalCapabilitySnapshotMismatch,
    FinalAuthoringSnapshotMismatch,
    FinalTruthMismatch,
    IterationShapeMismatch,
    FoundationalMeaningMismatch,
}

impl WorthUiReloadReplayCertification {
    pub fn certify(
        original: &WorthUiReloadStormCertification,
        replayed: &WorthUiReloadStormCertification,
    ) -> Result<Self, WorthUiReloadReplayCertificationDenial> {
        if original.scenario_digest() != replayed.scenario_digest() {
            return Err(WorthUiReloadReplayCertificationDenial::ScenarioDigestMismatch);
        }
        if original.ordered_truth().initial_active_plan_digest()
            != replayed.ordered_truth().initial_active_plan_digest()
            || original.ordered_truth().initial_active_artifact_digest()
                != replayed.ordered_truth().initial_active_artifact_digest()
            || original
                .ordered_truth()
                .initial_capability_snapshot_digest()
                != replayed
                    .ordered_truth()
                    .initial_capability_snapshot_digest()
            || original.ordered_truth().initial_authoring_snapshot_digest()
                != replayed.ordered_truth().initial_authoring_snapshot_digest()
        {
            return Err(WorthUiReloadReplayCertificationDenial::InitialTruthMismatch);
        }
        if original.ordered_truth().final_active_artifact_digest()
            != replayed.ordered_truth().final_active_artifact_digest()
            || original.ordered_truth().final_last_valid_artifact_digest()
                != replayed.ordered_truth().final_last_valid_artifact_digest()
        {
            return Err(WorthUiReloadReplayCertificationDenial::FinalActiveArtifactMismatch);
        }
        if original.ordered_truth().final_capability_snapshot_digest()
            != replayed.ordered_truth().final_capability_snapshot_digest()
        {
            return Err(WorthUiReloadReplayCertificationDenial::FinalCapabilitySnapshotMismatch);
        }
        if original.ordered_truth().final_authoring_snapshot_digest()
            != replayed.ordered_truth().final_authoring_snapshot_digest()
        {
            return Err(WorthUiReloadReplayCertificationDenial::FinalAuthoringSnapshotMismatch);
        }
        if original.ordered_truth().final_active_plan_digest()
            != replayed.ordered_truth().final_active_plan_digest()
            || original.ordered_truth().final_last_valid_plan_digest()
                != replayed.ordered_truth().final_last_valid_plan_digest()
        {
            return Err(WorthUiReloadReplayCertificationDenial::FinalTruthMismatch);
        }
        if original.ordered_truth().activated_plan_digests()
            != replayed.ordered_truth().activated_plan_digests()
            || original.ordered_truth().denied_preservation_plan_digests()
                != replayed.ordered_truth().denied_preservation_plan_digests()
            || original.ordered_truth().no_op_plan_digests()
                != replayed.ordered_truth().no_op_plan_digests()
        {
            return Err(WorthUiReloadReplayCertificationDenial::IterationShapeMismatch);
        }
        let foundational_meaning_digest = original.bundle().foundational_meaning_digest();
        if foundational_meaning_digest != replayed.bundle().foundational_meaning_digest() {
            return Err(WorthUiReloadReplayCertificationDenial::FoundationalMeaningMismatch);
        }
        Ok(Self {
            scenario_digest: original.scenario_digest(),
            final_active_artifact_digest: original.ordered_truth().final_active_artifact_digest(),
            final_active_plan_digest: original.ordered_truth().final_active_plan_digest(),
            final_capability_snapshot_digest: original
                .ordered_truth()
                .final_capability_snapshot_digest(),
            final_authoring_snapshot_digest: original
                .ordered_truth()
                .final_authoring_snapshot_digest(),
            final_last_valid_artifact_digest: original
                .ordered_truth()
                .final_last_valid_artifact_digest(),
            final_last_valid_plan_digest: original.ordered_truth().final_last_valid_plan_digest(),
            iteration_count: original.counters().iteration_count(),
            foundational_meaning_digest,
        })
    }

    pub fn scenario_digest(&self) -> u64 {
        self.scenario_digest
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

    pub fn iteration_count(&self) -> usize {
        self.iteration_count
    }

    pub fn foundational_meaning_digest(&self) -> u64 {
        self.foundational_meaning_digest
    }
}
