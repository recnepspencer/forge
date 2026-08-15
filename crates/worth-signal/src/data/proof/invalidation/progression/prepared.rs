use worth_proof::{Recipe, Unresolved};

use crate::data::graph::DirectInvalidationPreparationReceipt;

use super::super::output_commit::ProducedAspectDelta;

/// A direct invalidation decision prepared from current producer output.
///
/// This form proves preparation only. It cannot authorize cause publication,
/// scheduling, or execution. Atomic output publication must consume it before
/// committed direct invalidation truth can exist.
#[derive(Debug)]
pub(crate) struct PreparedDirectInvalidation(Recipe<Unresolved, ProducedAspectDelta>);

impl PreparedDirectInvalidation {
    pub(crate) fn from_semantic_decision(
        delta: ProducedAspectDelta,
        _receipt: DirectInvalidationPreparationReceipt,
    ) -> Self {
        Self(Recipe::new(delta))
    }

    pub(crate) fn delta(&self) -> &ProducedAspectDelta {
        self.0.payload()
    }

    pub(super) fn into_recipe(self) -> Recipe<Unresolved, ProducedAspectDelta> {
        self.0
    }
}
