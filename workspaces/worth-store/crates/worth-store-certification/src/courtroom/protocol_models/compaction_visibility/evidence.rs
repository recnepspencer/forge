use worth_store_formal_models::CompactionVisibilityRefinementCoverageReceipt;

use super::scenarios::OrdinaryCompactionVisibilityExecutionReceipt;

pub struct CompactionVisibilityRefinementEvidence {
    exact_coverage: CompactionVisibilityRefinementCoverageReceipt,
    ordinary_execution: OrdinaryCompactionVisibilityExecutionReceipt,
}

impl CompactionVisibilityRefinementEvidence {
    pub(super) const fn from_execution(
        exact_coverage: CompactionVisibilityRefinementCoverageReceipt,
        ordinary_execution: OrdinaryCompactionVisibilityExecutionReceipt,
    ) -> Self {
        Self {
            exact_coverage,
            ordinary_execution,
        }
    }

    pub const fn exact_coverage(&self) -> CompactionVisibilityRefinementCoverageReceipt {
        self.exact_coverage
    }

    pub fn retained_owner_observation_count(&self) -> usize {
        self.ordinary_execution.retained_owner_observation_count()
    }
}
