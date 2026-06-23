use super::types::ValidationMixedReloadStormProof;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationMixedReloadStormReplayArtifact {
    scenario_digest: u64,
    final_active_artifact_digest: u64,
    final_active_plan_digest: u64,
    final_capability_snapshot_digest: u64,
    final_authoring_snapshot_digest: Option<u64>,
    final_last_valid_artifact_digest: u64,
    final_last_valid_plan_digest: u64,
    projection_frame_digest: u64,
    step_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationMixedReloadStormReplayCertification {
    scenario_digest: u64,
    final_active_artifact_digest: u64,
    final_active_plan_digest: u64,
    final_capability_snapshot_digest: u64,
    final_authoring_snapshot_digest: Option<u64>,
    final_last_valid_artifact_digest: u64,
    final_last_valid_plan_digest: u64,
    projection_frame_digest: u64,
    step_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationMixedReloadStormReplayDenial {
    ScenarioDigestMismatch,
    FinalActiveArtifactMismatch,
    FinalCapabilitySnapshotMismatch,
    FinalAuthoringSnapshotMismatch,
    FinalTruthMismatch,
    StepShapeMismatch,
    ProjectionFrameDigestMismatch,
}

impl ValidationMixedReloadStormProof {
    pub fn replay_artifact(&self) -> ValidationMixedReloadStormReplayArtifact {
        ValidationMixedReloadStormReplayArtifact {
            scenario_digest: self.scenario_digest(),
            final_active_artifact_digest: self.final_active_artifact_digest(),
            final_active_plan_digest: self.final_active_plan_digest(),
            final_capability_snapshot_digest: self.final_capability_snapshot_digest(),
            final_authoring_snapshot_digest: self.final_authoring_snapshot_digest(),
            final_last_valid_artifact_digest: self.final_last_valid_artifact_digest(),
            final_last_valid_plan_digest: self.final_last_valid_plan_digest(),
            projection_frame_digest: self.projection_frame_digest(),
            step_count: self.steps().len(),
        }
    }

    pub fn certify_replay(
        original: &Self,
        replayed: &Self,
    ) -> Result<ValidationMixedReloadStormReplayCertification, ValidationMixedReloadStormReplayDenial>
    {
        if original.scenario_digest != replayed.scenario_digest {
            return Err(ValidationMixedReloadStormReplayDenial::ScenarioDigestMismatch);
        }
        if original.final_active_artifact_digest != replayed.final_active_artifact_digest {
            return Err(ValidationMixedReloadStormReplayDenial::FinalActiveArtifactMismatch);
        }
        if original.final_capability_snapshot_digest != replayed.final_capability_snapshot_digest {
            return Err(ValidationMixedReloadStormReplayDenial::FinalCapabilitySnapshotMismatch);
        }
        if original.final_authoring_snapshot_digest != replayed.final_authoring_snapshot_digest {
            return Err(ValidationMixedReloadStormReplayDenial::FinalAuthoringSnapshotMismatch);
        }
        if original.final_active_plan_digest != replayed.final_active_plan_digest
            || original.final_last_valid_artifact_digest
                != replayed.final_last_valid_artifact_digest
            || original.final_last_valid_plan_digest != replayed.final_last_valid_plan_digest
        {
            return Err(ValidationMixedReloadStormReplayDenial::FinalTruthMismatch);
        }
        if original
            .steps
            .iter()
            .map(|step| step.digest_basis())
            .collect::<Vec<_>>()
            != replayed
                .steps
                .iter()
                .map(|step| step.digest_basis())
                .collect::<Vec<_>>()
        {
            return Err(ValidationMixedReloadStormReplayDenial::StepShapeMismatch);
        }
        let projection_frame_digest = original.projection_frame_digest();
        if projection_frame_digest != replayed.projection_frame_digest() {
            return Err(ValidationMixedReloadStormReplayDenial::ProjectionFrameDigestMismatch);
        }
        Ok(ValidationMixedReloadStormReplayCertification {
            scenario_digest: original.scenario_digest,
            final_active_artifact_digest: original.final_active_artifact_digest,
            final_active_plan_digest: original.final_active_plan_digest,
            final_capability_snapshot_digest: original.final_capability_snapshot_digest,
            final_authoring_snapshot_digest: original.final_authoring_snapshot_digest,
            final_last_valid_artifact_digest: original.final_last_valid_artifact_digest,
            final_last_valid_plan_digest: original.final_last_valid_plan_digest,
            projection_frame_digest,
            step_count: original.steps.len(),
        })
    }
}

impl ValidationMixedReloadStormReplayCertification {
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

    pub fn projection_frame_digest(&self) -> u64 {
        self.projection_frame_digest
    }

    pub fn step_count(&self) -> usize {
        self.step_count
    }
}

impl ValidationMixedReloadStormReplayArtifact {
    pub fn scenario_digest(self) -> u64 {
        self.scenario_digest
    }

    pub fn final_active_artifact_digest(self) -> u64 {
        self.final_active_artifact_digest
    }

    pub fn final_active_plan_digest(self) -> u64 {
        self.final_active_plan_digest
    }

    pub fn final_capability_snapshot_digest(self) -> u64 {
        self.final_capability_snapshot_digest
    }

    pub fn final_authoring_snapshot_digest(self) -> Option<u64> {
        self.final_authoring_snapshot_digest
    }

    pub fn final_last_valid_artifact_digest(self) -> u64 {
        self.final_last_valid_artifact_digest
    }

    pub fn final_last_valid_plan_digest(self) -> u64 {
        self.final_last_valid_plan_digest
    }

    pub fn projection_frame_digest(self) -> u64 {
        self.projection_frame_digest
    }

    pub fn step_count(self) -> usize {
        self.step_count
    }
}
