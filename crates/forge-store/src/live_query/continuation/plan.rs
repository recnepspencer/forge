use crate::live_query::basis::StableBasisHandle;
use crate::live_query::compatibility::ContinuationCompatibilityWitness;
use serde::Serialize;

use super::budget::ContinuationBatchBudget;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ContinuationStrategy {
    AdmittedLayoutNarrow,
    ExplicitBroadened,
    AuthorityReplayControlLane,
}

#[derive(Debug, Clone)]
pub struct CursorContinuationPlan {
    witness: ContinuationCompatibilityWitness,
    strategy: ContinuationStrategy,
}

impl CursorContinuationPlan {
    pub(crate) fn new(
        witness: ContinuationCompatibilityWitness,
        strategy: ContinuationStrategy,
    ) -> Self {
        Self { witness, strategy }
    }

    pub fn witness(&self) -> &ContinuationCompatibilityWitness {
        &self.witness
    }

    pub fn strategy(&self) -> ContinuationStrategy {
        self.strategy
    }

    pub fn stable_basis(&self) -> &StableBasisHandle {
        self.witness.stable_basis()
    }

    pub fn batch_budget(&self) -> &ContinuationBatchBudget {
        self.witness.batch_budget()
    }
}
