//! Concrete public transition owners built from honest downstream markers.

worth_proof::authority_marker!(pub(crate) WitnessAuthority);
worth_proof::capability_marker!(pub(crate) WitnessCapability);

fn authority() -> worth_proof::AuthorityWitness<WitnessAuthority> {
    WitnessAuthority::witness()
}

fn capability() -> worth_proof::CapabilityWitness<WitnessCapability> {
    WitnessCapability::witness()
}

pub(crate) fn checked_resolve() -> worth_proof::CheckedResolveRecipeTransition {
    worth_proof::CheckedResolveRecipeTransition
}

pub(crate) fn transition_outcome(
) -> worth_proof::TransitionOutcome<u8, u16, u32, u64, u128, usize> {
    worth_proof::TransitionOutcome::success(1)
}

pub(crate) fn admit_execution_ready() -> worth_proof::AdmitExecutionReadyRecipeTransition {
    worth_proof::AdmitExecutionReadyRecipeTransition
}

pub(crate) fn checked_admit_execution_ready(
) -> worth_proof::CheckedAdmitExecutionReadyRecipeTransition {
    worth_proof::CheckedAdmitExecutionReadyRecipeTransition
}

pub(crate) fn execute_ready() -> worth_proof::ExecuteReadyRecipeTransition {
    worth_proof::ExecuteReadyRecipeTransition
}

pub(crate) fn resolve_recipe() -> worth_proof::ResolveRecipeTransition {
    worth_proof::ResolveRecipeTransition
}

pub(crate) fn pre_construction_gate() -> worth_proof::PreConstructionGate<u8, u16, u32> {
    worth_proof::PreConstructionGate::ready(1)
}

pub(crate) fn transition_readiness(
) -> worth_proof::TransitionReadiness<u8, u16, u32, u64, u128, usize> {
    worth_proof::TransitionReadiness::ready(1)
}

pub(crate) fn readmit_lowered() -> worth_proof::ReadmitLoweredForExecutionReadyTransition {
    worth_proof::ReadmitLoweredForExecutionReadyTransition
}

pub(crate) fn checked_readmit_lowered(
) -> worth_proof::CheckedReadmitLoweredForExecutionReadyTransition {
    worth_proof::CheckedReadmitLoweredForExecutionReadyTransition
}

pub(crate) fn checked_admit() -> worth_proof::CheckedAdmitRecipeTransition<WitnessAuthority> {
    worth_proof::CheckedAdmitRecipeTransition::new()
}

pub(crate) fn checked_lower() -> worth_proof::CheckedLowerRecipeTransition<WitnessCapability> {
    worth_proof::CheckedLowerRecipeTransition::new()
}

pub(crate) fn execution_readiness(
) -> worth_proof::ExecutionReadinessContext<&'static str, WitnessAuthority> {
    worth_proof::ExecutionReadinessContext::new("runtime", authority())
}

pub(crate) fn admit() -> worth_proof::AdmitRecipeTransition<WitnessAuthority> {
    worth_proof::AdmitRecipeTransition::new(authority())
}

pub(crate) fn lower() -> worth_proof::LowerRecipeTransition<WitnessCapability> {
    worth_proof::LowerRecipeTransition::new(capability())
}

pub(crate) fn resolution_context(
) -> worth_proof::RecipeResolutionContext<u8, WitnessAuthority> {
    worth_proof::RecipeResolutionContext::new(13_u8, authority())
}

pub(crate) fn lowered_readmission() -> worth_proof::LoweredReadmissionContext<
    u16,
    WitnessAuthority,
    &'static str,
    WitnessAuthority,
> {
    worth_proof::LoweredReadmissionContext::new(
        17_u16,
        authority(),
        "runtime",
        authority(),
    )
}

pub(crate) fn successful_outcome() -> worth_proof::SuccessfulTransitionOutcome<u8> {
    worth_proof::SuccessfulTransitionOutcome::new(1)
}
