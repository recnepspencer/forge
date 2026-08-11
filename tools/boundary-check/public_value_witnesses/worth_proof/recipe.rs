//! Recipes progressed only through the ordinary public transition protocol.

use worth_proof::{ContextualTransition as _, Transition as _};

worth_proof::authority_marker!(pub(crate) WitnessAuthority);
worth_proof::capability_marker!(pub(crate) WitnessCapability);

type CurrentBasis<B> =
    worth_proof::FreshnessScopedBasis<worth_proof::CurrentValidity, worth_proof::AssumptionBasis<B>>;

fn lowered<B>(basis: B) -> worth_proof::Recipe<worth_proof::Lowered, u8, CurrentBasis<B>> {
    let unresolved = worth_proof::Recipe::<worth_proof::Unresolved, _>::new(5_u8);
    let resolved = worth_proof::ResolveRecipeTransition
        .transition(
            unresolved,
            worth_proof::RecipeResolutionContext::new(basis, WitnessAuthority::witness()),
        )
        .into_value();
    worth_proof::LowerRecipeTransition::new(WitnessCapability::witness())
        .transition(resolved)
        .into_value()
}

pub(crate) fn unresolved() -> worth_proof::Unresolved {
    worth_proof::Unresolved
}

pub(crate) fn resolved() -> worth_proof::Resolved {
    worth_proof::Resolved
}

pub(crate) fn lowered_stage() -> worth_proof::Lowered {
    worth_proof::Lowered
}

pub(crate) fn admitted() -> worth_proof::Admitted {
    worth_proof::Admitted
}

pub(crate) fn recipe() -> worth_proof::Recipe<worth_proof::Unresolved, u8> {
    worth_proof::Recipe::new(5)
}

pub(crate) fn execution_ready_recipe(
) -> worth_proof::ExecutionReadyRecipe<u8, CurrentBasis<u8>> {
    worth_proof::AdmitExecutionReadyRecipeTransition
        .transition(
            lowered(11_u8),
            worth_proof::ExecutionReadinessContext::new(
                "witness runtime",
                WitnessAuthority::witness(),
            ),
        )
        .into_value()
}

pub(crate) fn executed_recipe() -> worth_proof::ExecutedRecipe<u8, CurrentBasis<u8>> {
    worth_proof::ExecuteReadyRecipeTransition
        .transition(execution_ready_recipe())
        .into_value()
}
