use worth_ui::facade::WorthUiActiveRuntimeObservation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HarnessEvidenceBasis {
    artifact_digest: u64,
    active_plan_digest: u64,
    snapshot_digest: u64,
    frame_epoch: u64,
}

impl HarnessEvidenceBasis {
    pub fn from_active_observation(observation: WorthUiActiveRuntimeObservation) -> Self {
        Self {
            artifact_digest: observation.artifact_digest(),
            active_plan_digest: observation.active_plan_digest(),
            snapshot_digest: observation.snapshot_digest(),
            frame_epoch: observation.frame_epoch().as_u64(),
        }
    }

    pub fn artifact_digest(self) -> u64 {
        self.artifact_digest
    }

    pub fn active_plan_digest(self) -> u64 {
        self.active_plan_digest
    }

    pub fn snapshot_digest(self) -> u64 {
        self.snapshot_digest
    }

    pub fn frame_epoch(self) -> u64 {
        self.frame_epoch
    }
}
