use worth_proof::{
    AdmitExecutionReadyRecipeTransition, Binding, ContextualTransition, ExecutionReadinessContext,
    ExecutionReadyRecipe, TransitionOutcome,
};

use super::resolved::{CurrentOriginBasis, InvalidationWorkBatch};
use super::{InvalidationProgressionDenial, InvalidationWorkBindingAxes, LoweredInvalidationBatch};

worth_proof::authority_marker!(InvalidationReadinessAuthority);

/// The only invalidation form accepted by execution.
#[derive(Debug)]
pub(crate) struct ReadyInvalidationBatch {
    recipe: ExecutionReadyRecipe<InvalidationWorkBatch, CurrentOriginBasis>,
    binding: Binding<InvalidationWorkBindingAxes>,
}

impl ReadyInvalidationBatch {
    pub(super) fn work(&self) -> &InvalidationWorkBatch {
        self.recipe.payload()
    }

    pub(super) fn admit(
        lowered: LoweredInvalidationBatch,
        current: Binding<InvalidationWorkBindingAxes>,
    ) -> TransitionOutcome<
        Self,
        InvalidationProgressionDenial,
        InvalidationProgressionDenial,
        InvalidationProgressionDenial,
        InvalidationProgressionDenial,
        crate::data::error::SignalError,
    > {
        let (recipe, binding) = lowered.into_parts();
        if let Err(drift) = binding.ensure_matches(&current) {
            use super::binding::InvalidationWorkBindingDrift;
            let denial = match drift {
                InvalidationWorkBindingDrift::GraphInstance => {
                    InvalidationProgressionDenial::StaleGraphInstance
                }
                InvalidationWorkBindingDrift::DependencyRevision => {
                    InvalidationProgressionDenial::StaleDependencyRevision
                }
                InvalidationWorkBindingDrift::Origin => {
                    InvalidationProgressionDenial::StaleOriginGeneration
                }
                InvalidationWorkBindingDrift::ReadinessEpoch => {
                    InvalidationProgressionDenial::StaleReadinessEpoch
                }
                InvalidationWorkBindingDrift::StageOrder => {
                    InvalidationProgressionDenial::StaleStageOrder
                }
                InvalidationWorkBindingDrift::Target => {
                    InvalidationProgressionDenial::RebindRequired
                }
            };
            return TransitionOutcome::stale(denial);
        }
        let recipe = AdmitExecutionReadyRecipeTransition
            .transition(
                recipe,
                ExecutionReadinessContext::new((), InvalidationReadinessAuthority::witness()),
            )
            .into_value();
        TransitionOutcome::success(Self { recipe, binding })
    }

    pub(super) fn binding(&self) -> &Binding<InvalidationWorkBindingAxes> {
        &self.binding
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        ExecutionReadyRecipe<InvalidationWorkBatch, CurrentOriginBasis>,
        Binding<InvalidationWorkBindingAxes>,
    ) {
        (self.recipe, self.binding)
    }
}
