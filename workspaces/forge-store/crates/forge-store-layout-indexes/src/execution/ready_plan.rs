use forge_proof::raw::{
    CheckedAdmitExecutionReadyRecipeTransition, ContextualTransition, ExecutionReadinessContext,
    ExecutionReadyAdmissionReadiness, ExecutionReadyRecipe, TransitionOutcome,
};

use super::denial::S8AccessLoweringDeferred;
use super::freshness::readiness_authority;
use super::lowered_plan::{S8AccessLoweringBasis, S8LoweredAccessPayload, S8LoweredAccessReceipt};

type ReadyRecipe = ExecutionReadyRecipe<
    S8LoweredAccessPayload,
    forge_proof::raw::FreshnessScopedBasis<
        forge_proof::raw::CurrentValidity,
        forge_proof::raw::AssumptionBasis<S8AccessLoweringBasis>,
    >,
>;

#[derive(Debug, PartialEq, Eq)]
pub struct S8ExecutionReadyAccessReceipt {
    recipe: ReadyRecipe,
}

impl S8ExecutionReadyAccessReceipt {
    pub(crate) const fn from_recipe(recipe: ReadyRecipe) -> Self {
        Self { recipe }
    }

    pub(crate) fn admit(lowered: S8LoweredAccessReceipt) -> Self {
        let ready = CheckedAdmitExecutionReadyRecipeTransition.transition(
            lowered.into_recipe(),
            ExecutionReadyAdmissionReadiness::<
                S8LoweredAccessPayload,
                S8AccessLoweringBasis,
                &'static str,
                super::freshness::S8ExecutionReadinessAuthority,
                S8AccessLoweringDeferred,
                S8AccessLoweringDeferred,
                S8AccessLoweringDeferred,
            >::ready(ExecutionReadinessContext::new(
                "store-ready",
                readiness_authority(),
            )),
        );

        let ready = match ready {
            TransitionOutcome::Success(ready) => ready,
            TransitionOutcome::Denied(_)
            | TransitionOutcome::Deferred(_)
            | TransitionOutcome::Stale(_)
            | TransitionOutcome::RebindRequired(_)
            | TransitionOutcome::Failed(_) => {
                unreachable!("exact lowering readiness admission is pre-validated")
            }
        };

        Self::from_recipe(ready)
    }

    pub(crate) fn recipe(self) -> ReadyRecipe {
        self.recipe
    }

    pub fn selected(&self) -> crate::planning::S8SelectedAccessPlan {
        self.recipe.payload().selected()
    }

    pub fn path_kind(&self) -> super::path_kind::S8AccessPathKind {
        self.recipe.payload().path_kind()
    }

    pub fn basis(&self) -> S8AccessLoweringBasis {
        *self.recipe.strong_basis().value()
    }
}
