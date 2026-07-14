use forge_proof::raw::{
    AssumptionBasis, AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness,
    ContextualTransition, CurrentValidity, ExecuteReadyRecipeTransition, ExecutedRecipe,
    ExecutionReadinessContext, ExecutionReadyRecipe, FreshnessScopedBasis, LowerRecipeTransition,
    Recipe, RecipeResolutionContext, ResolveRecipeTransition, Transition,
};

use super::QueueExecutionReplayIdentity;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QueueExecutionLoweringAuthority {
    _sealed: (),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct QueueExecutionProofBasis {
    replay_identity: QueueExecutionReplayIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueueExecutionProgression {
    Lowered,
    ExecutionReady,
    Executed,
}

pub const fn queue_execution_lowering_authority() -> QueueExecutionLoweringAuthority {
    QueueExecutionLoweringAuthority { _sealed: () }
}

struct QueueExecutionResolutionAuthority;
impl AuthorityMarker for QueueExecutionResolutionAuthority {}

struct QueueExecutionLoweringCapability;
impl CapabilityMarker for QueueExecutionLoweringCapability {}

struct QueueExecutionReadinessAuthority;
impl AuthorityMarker for QueueExecutionReadinessAuthority {}

pub(crate) type QueueReadyRecipe = ExecutionReadyRecipe<
    QueueExecutionReplayIdentity,
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<QueueExecutionProofBasis>>,
>;

pub(crate) type QueueExecutedRecipe = ExecutedRecipe<
    QueueExecutionReplayIdentity,
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<QueueExecutionProofBasis>>,
>;

pub(crate) fn ready_queue_execution_proof(
    replay_identity: QueueExecutionReplayIdentity,
) -> QueueReadyRecipe {
    let basis = QueueExecutionProofBasis { replay_identity };
    let resolved = ResolveRecipeTransition
        .transition(
            Recipe::new(replay_identity),
            RecipeResolutionContext::new(
                basis,
                AuthorityWitness::from_authority_marker(QueueExecutionResolutionAuthority),
            ),
        )
        .into_value();
    let lowered = LowerRecipeTransition::new(CapabilityWitness::from_capability_marker(
        QueueExecutionLoweringCapability,
    ))
    .transition(resolved)
    .into_value();

    forge_proof::raw::AdmitExecutionReadyRecipeTransition
        .transition(
            lowered,
            ExecutionReadinessContext::new(
                replay_identity,
                AuthorityWitness::from_authority_marker(QueueExecutionReadinessAuthority),
            ),
        )
        .into_value()
}

pub(crate) fn execute_queue_execution_proof(proof: QueueReadyRecipe) -> QueueExecutedRecipe {
    ExecuteReadyRecipeTransition.transition(proof).into_value()
}
