use forge_proof::prelude::{Recipe, Unresolved};
use forge_store_readiness::{
    accept_s5_1_admitted_security_scope_readiness, S51SecurityScopeReadinessReservation,
};

fn main() {
    let proof_progression = Recipe::<Unresolved, _>::new("proof is not store authority");
    accept_s5_1_admitted_security_scope_readiness(
        S51SecurityScopeReadinessReservation::security_foundation(),
        proof_progression,
    );
}
