use forge_proof::raw::{
    checked_admit_ready_and_execute_recipe, resolve_checked_lower_and_admit_recipe, Admitted,
    AssumptionBasis, AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness,
    CurrentValidity, ExecutedRecipe, ExecutionReadinessContext, FreshnessScopedBasis, Lowered,
    PreConstructionGate, RebindRequiredBasis, Recipe, RecipeResolutionContext, Resolved,
    StaleReadableBasis, TransitionOutcome, TransitionReadiness, Unresolved,
};
use forge_proof::{ResolvedRecipeDxExt, UnresolvedRecipeDxExt};

use super::super::denial::RecoveryEvidenceDenial;
use super::trace::RecoveryProofProgressionStep;

pub(super) type CheckedRecoveryRecipeOutcome = TransitionOutcome<
    Recipe<
        Admitted,
        RecoveryProofProgressionStep,
        FreshnessScopedBasis<CurrentValidity, AssumptionBasis<&'static str>>,
    >,
    RecoveryEvidenceDenial,
    RecoveryEvidenceDenial,
    Recipe<Lowered, RecoveryProofProgressionStep, StaleReadableBasis<&'static str>>,
    Recipe<Resolved, RecoveryProofProgressionStep, RebindRequiredBasis<&'static str>>,
    RecoveryEvidenceDenial,
>;

pub(super) type CheckedExecutedRecoveryReplayOutcome = TransitionOutcome<
    ExecutedRecipe<
        RecoveryProofProgressionStep,
        FreshnessScopedBasis<CurrentValidity, AssumptionBasis<&'static str>>,
    >,
    RecoveryEvidenceDenial,
    RecoveryEvidenceDenial,
    Recipe<Lowered, RecoveryProofProgressionStep, StaleReadableBasis<&'static str>>,
    Recipe<Resolved, RecoveryProofProgressionStep, RebindRequiredBasis<&'static str>>,
    RecoveryEvidenceDenial,
>;

#[derive(Debug, Clone, Copy)]
struct StoreRecoveryResolutionAuthority;
impl AuthorityMarker for StoreRecoveryResolutionAuthority {}

#[derive(Debug, Clone, Copy)]
struct StoreRecoveryLoweringCapability;
impl CapabilityMarker for StoreRecoveryLoweringCapability {}

#[derive(Debug, Clone, Copy)]
struct StoreRecoveryAdmissionAuthority;
impl AuthorityMarker for StoreRecoveryAdmissionAuthority {}

pub(super) fn checked_recipe_outcome() -> CheckedRecoveryRecipeOutcome {
    resolve_checked_lower_and_admit_recipe(
        Recipe::<Unresolved, _>::new(RecoveryProofProgressionStep::RecoveryEntry),
        PreConstructionGate::<_, RecoveryEvidenceDenial, RecoveryEvidenceDenial>::ready(
            RecipeResolutionContext::new(
                "store.recovery.s4.executed-basis",
                AuthorityWitness::from_authority_marker(StoreRecoveryResolutionAuthority),
            ),
        ),
        TransitionReadiness::<
            _,
            RecoveryEvidenceDenial,
            RecoveryEvidenceDenial,
            _,
            _,
            RecoveryEvidenceDenial,
        >::ready(CapabilityWitness::from_capability_marker(
            StoreRecoveryLoweringCapability,
        )),
        TransitionReadiness::<
            _,
            RecoveryEvidenceDenial,
            RecoveryEvidenceDenial,
            _,
            _,
            RecoveryEvidenceDenial,
        >::ready(AuthorityWitness::from_authority_marker(
            StoreRecoveryAdmissionAuthority,
        )),
    )
}

pub(super) fn checked_executed_replay() -> CheckedExecutedRecoveryReplayOutcome {
    let lowered = Recipe::<Unresolved, _>::new(RecoveryProofProgressionStep::ExecutedReplay)
        .resolve_with(
            AuthorityWitness::from_authority_marker(StoreRecoveryResolutionAuthority),
            "store.recovery.s4.executed-basis",
        )
        .lower_with(CapabilityWitness::from_capability_marker(
            StoreRecoveryLoweringCapability,
        ));

    checked_admit_ready_and_execute_recipe(
        lowered,
        TransitionReadiness::<
            ExecutionReadinessContext<&'static str, StoreRecoveryAdmissionAuthority>,
            RecoveryEvidenceDenial,
            RecoveryEvidenceDenial,
            Recipe<Lowered, RecoveryProofProgressionStep, StaleReadableBasis<&'static str>>,
            Recipe<Resolved, RecoveryProofProgressionStep, RebindRequiredBasis<&'static str>>,
            RecoveryEvidenceDenial,
        >::ready(ExecutionReadinessContext::new(
            "store.recovery.s4.executed-replay",
            AuthorityWitness::from_authority_marker(StoreRecoveryAdmissionAuthority),
        )),
    )
}
