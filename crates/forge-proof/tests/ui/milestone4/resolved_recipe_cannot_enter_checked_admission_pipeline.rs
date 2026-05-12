use forge_proof::{
    apply_contextual_transition, CheckedAdmitRecipeTransition, CurrentValidity,
    FreshnessScopedBasis, Recipe, Resolved, TransitionReadiness,
};

struct AdmissionAuthority;
impl forge_proof::AuthorityMarker for AdmissionAuthority {}

fn invalid_checked_admission_pipeline(
    resolved: Recipe<Resolved, &str, FreshnessScopedBasis<CurrentValidity, forge_proof::AssumptionBasis<u8>>>,
) {
    let readiness: forge_proof::RecipeAdmissionReadiness<
        &str,
        u8,
        AdmissionAuthority,
        &'static str,
        &'static str,
        &'static str,
    > = TransitionReadiness::failed("failed");
    let _ = apply_contextual_transition(
        &CheckedAdmitRecipeTransition::<AdmissionAuthority>::new(),
        resolved,
        readiness,
    );
}

fn main() {}
