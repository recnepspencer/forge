//! Developer-facing projections reached from genuine governed values.

use worth_proof::{ContextualTransition as _, ReadyJoinRecipeDxExt as _, Transition as _};

type CurrentBasis<B> =
    worth_proof::FreshnessScopedBasis<worth_proof::CurrentValidity, worth_proof::AssumptionBasis<B>>;

pub(crate) fn proof_outcome() -> worth_proof::ProofOutcome<u8> {
    worth_proof::SuccessfulTransitionOutcome::new(1).into()
}

pub(crate) fn basis_posture_kind() -> worth_proof::BasisPostureKind {
    worth_proof::BasisPostureKind::None
}

pub(crate) fn proof_outcome_kind() -> worth_proof::ProofOutcomeKind {
    worth_proof::ProofOutcomeKind::Success
}

pub(crate) fn family_action_kind() -> worth_proof::FamilyActionKind {
    worth_proof::FamilyActionKind::Create
}

pub(crate) fn recipe_stage_kind() -> worth_proof::RecipeStageKind {
    worth_proof::RecipeStageKind::Unresolved
}

pub(crate) fn proof_flow() -> worth_proof::ProofFlow {
    worth_proof::proof_flow()
}

worth_proof::authority_marker!(WitnessAuthority);
worth_proof::capability_marker!(WitnessCapability);

fn ready_with<B>(basis: B) -> worth_proof::ExecutionReadyRecipe<u8, CurrentBasis<B>> {
    let resolved = worth_proof::ResolveRecipeTransition
        .transition(
            worth_proof::Recipe::<worth_proof::Unresolved, _>::new(5_u8),
            worth_proof::RecipeResolutionContext::new(basis, WitnessAuthority::witness()),
        )
        .into_value();
    let lowered = worth_proof::LowerRecipeTransition::new(WitnessCapability::witness())
        .transition(resolved)
        .into_value();
    worth_proof::AdmitExecutionReadyRecipeTransition
        .transition(
            lowered,
            worth_proof::ExecutionReadinessContext::new(
                "witness runtime",
                WitnessAuthority::witness(),
            ),
        )
        .into_value()
}

pub(crate) fn ready_join_summary(
    deliver: impl for<'a> FnOnce(
        worth_proof::ReadyJoinSummary<'a, u8, u8, CurrentBasis<u8>, CurrentBasis<u16>>,
    ),
) {
    let joined = worth_proof::join_ready(
        ready_with(3_u8),
        ready_with(5_u16),
    );
    deliver(joined.summary());
}
