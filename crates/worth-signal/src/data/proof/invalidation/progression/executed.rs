use worth_proof::{
    ActionMarker, Binding, ExecuteReadyRecipeTransition, ExecutedRecipe, Performed, Transition,
    TransitionOutcome,
};

use super::resolved::{CurrentOriginBasis, InvalidationWorkBatch};
use super::{InvalidationWorkBindingAxes, ReadyInvalidationBatch};

worth_proof::authority_marker!(InvalidationExecutionAuthority);

struct ExecuteInvalidationBatch;
impl ActionMarker for ExecuteInvalidationBatch {}

type PerformedInvalidationExecution<Outcome> =
    Performed<ExecuteInvalidationBatch, InvalidationExecutionAuthority, Outcome>;

/// Executed invalidation work plus the performed Signal outcome.
#[derive(Debug)]
pub(crate) struct ExecutedInvalidationBatch<Outcome> {
    _recipe: ExecutedRecipe<InvalidationWorkBatch, CurrentOriginBasis>,
    binding: Binding<InvalidationWorkBindingAxes>,
    performed: PerformedInvalidationExecution<Outcome>,
}

impl<Outcome> ExecutedInvalidationBatch<Outcome> {
    pub(super) fn execute(
        ready: ReadyInvalidationBatch,
        effect: impl FnOnce(&InvalidationWorkBatch) -> Result<Outcome, crate::data::error::SignalError>,
    ) -> TransitionOutcome<
        Self,
        std::convert::Infallible,
        std::convert::Infallible,
        std::convert::Infallible,
        std::convert::Infallible,
        crate::data::error::SignalError,
    > {
        let (recipe, binding) = ready.into_parts();
        let outcome = match effect(recipe.payload()) {
            Ok(outcome) => outcome,
            Err(error) => return TransitionOutcome::failed(error),
        };
        let recipe = ExecuteReadyRecipeTransition.transition(recipe).into_value();
        let performed = Performed::record(&InvalidationExecutionAuthority::witness(), outcome);
        TransitionOutcome::success(Self {
            _recipe: recipe,
            binding,
            performed,
        })
    }

    pub(super) fn binding(&self) -> &Binding<InvalidationWorkBindingAxes> {
        &self.binding
    }

    pub(super) fn outcome(&self) -> &Outcome {
        self.performed.outcome()
    }

    pub(super) fn into_outcome(self) -> Outcome {
        self.performed.into_outcome()
    }
}
