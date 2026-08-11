use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TierMoveBreadthSummary {
    candidate_count: u64,
    admitted_count: u64,
    locality_group_count: u64,
}

impl TierMoveBreadthSummary {
    pub(crate) fn new(
        candidate_count: u64,
        admitted_count: u64,
        locality_group_count: u64,
    ) -> Self {
        Self {
            candidate_count,
            admitted_count,
            locality_group_count,
        }
    }

    pub fn candidate_count(&self) -> u64 {
        self.candidate_count
    }

    pub fn admitted_count(&self) -> u64 {
        self.admitted_count
    }

    pub fn locality_group_count(&self) -> u64 {
        self.locality_group_count
    }
}
