use crate::discovery_loop::ReactivationCondition;

use super::equivalence_errors::{require_equivalence_non_empty, TilingEquivalenceError};
use super::suppression_proofs::TilingCandidateSuppressionProof;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TilingReactivationRequest {
    reactivation_id: String,
    suppression: TilingCandidateSuppressionProof,
    reactivation_condition: ReactivationCondition,
}

impl TilingReactivationRequest {
    pub fn new(
        reactivation_id: impl Into<String>,
        suppression: TilingCandidateSuppressionProof,
        reactivation_condition: ReactivationCondition,
    ) -> Result<Self, TilingEquivalenceError> {
        Ok(Self {
            reactivation_id: require_equivalence_non_empty(reactivation_id, "reactivation_id")?,
            suppression,
            reactivation_condition,
        })
    }

    pub(crate) fn reactivation_id(&self) -> &str {
        &self.reactivation_id
    }

    pub(crate) fn suppression(&self) -> &TilingCandidateSuppressionProof {
        &self.suppression
    }

    pub(crate) fn reactivation_condition(&self) -> &ReactivationCondition {
        &self.reactivation_condition
    }
}
