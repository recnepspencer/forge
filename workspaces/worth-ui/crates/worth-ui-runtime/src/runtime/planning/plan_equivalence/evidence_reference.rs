#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPlanEquivalenceEvidenceReference {
    predecessor_artifact_digest: u64,
    candidate_artifact_digest: u64,
    predecessor_plan_digest: u64,
    candidate_plan_digest: u64,
    transition_count: usize,
}

impl WorthUiPlanEquivalenceEvidenceReference {
    pub(crate) fn new(
        predecessor_artifact_digest: u64,
        candidate_artifact_digest: u64,
        predecessor_plan_digest: u64,
        candidate_plan_digest: u64,
        transition_count: usize,
    ) -> Self {
        Self {
            predecessor_artifact_digest,
            candidate_artifact_digest,
            predecessor_plan_digest,
            candidate_plan_digest,
            transition_count,
        }
    }

    pub fn predecessor_artifact_digest(self) -> u64 {
        self.predecessor_artifact_digest
    }

    pub fn candidate_artifact_digest(self) -> u64 {
        self.candidate_artifact_digest
    }

    pub fn predecessor_plan_digest(self) -> u64 {
        self.predecessor_plan_digest
    }

    pub fn candidate_plan_digest(self) -> u64 {
        self.candidate_plan_digest
    }

    pub fn transition_count(self) -> usize {
        self.transition_count
    }
}
