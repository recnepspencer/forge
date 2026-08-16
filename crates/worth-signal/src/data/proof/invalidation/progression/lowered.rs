use worth_proof::{Binding, LowerRecipeTransition, Lowered, Recipe, Transition};

use super::resolved::{CurrentOriginBasis, InvalidationWorkBatch};
use super::{
    InvalidationReadinessEpoch, InvalidationStageOrder, InvalidationWorkBindingAxes,
    ResolvedInvalidationWork,
};

worth_proof::capability_marker!(InvalidationTopologyLoweringCapability);

/// Current invalidation work bound to one readiness epoch and stage order.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct LoweredInvalidationBatch {
    recipe: Recipe<Lowered, InvalidationWorkBatch, CurrentOriginBasis>,
    binding: Binding<InvalidationWorkBindingAxes>,
}

impl LoweredInvalidationBatch {
    pub(super) fn lower(
        resolved: ResolvedInvalidationWork,
        readiness_epoch: InvalidationReadinessEpoch,
        stage_order: InvalidationStageOrder,
    ) -> Self {
        let resolved = resolved.into_recipe();
        let origin_binding = resolved.basis().basis().value().clone();
        let binding = Binding::new(origin_binding.into_work_binding(readiness_epoch, stage_order));
        let recipe = LowerRecipeTransition::new(InvalidationTopologyLoweringCapability::witness())
            .transition(resolved)
            .into_value();
        Self { recipe, binding }
    }

    pub(super) fn binding(&self) -> &Binding<InvalidationWorkBindingAxes> {
        &self.binding
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        Recipe<Lowered, InvalidationWorkBatch, CurrentOriginBasis>,
        Binding<InvalidationWorkBindingAxes>,
    ) {
        (self.recipe, self.binding)
    }
}
