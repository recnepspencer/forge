#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiLaneParityCertification {
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    active_plan_digest: u64,
    candidate_plan_digest: u64,
    semantic_reference_digest: u64,
}

impl WorthUiLaneParityCertification {
    pub(crate) fn new(
        active_artifact_digest: u64,
        candidate_artifact_digest: u64,
        active_plan_digest: u64,
        candidate_plan_digest: u64,
        semantic_reference_digest: u64,
    ) -> Self {
        Self {
            active_artifact_digest,
            candidate_artifact_digest,
            active_plan_digest,
            candidate_plan_digest,
            semantic_reference_digest,
        }
    }

    pub fn active_artifact_digest(self) -> u64 {
        self.active_artifact_digest
    }

    pub fn candidate_artifact_digest(self) -> u64 {
        self.candidate_artifact_digest
    }

    pub fn active_plan_digest(self) -> u64 {
        self.active_plan_digest
    }

    pub fn candidate_plan_digest(self) -> u64 {
        self.candidate_plan_digest
    }

    pub fn semantic_reference_digest(self) -> u64 {
        self.semantic_reference_digest
    }
}
