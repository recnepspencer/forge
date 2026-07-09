use worth_store_readiness::{
    accept_s5_1_admitted_security_scope_readiness, S51SecurityScopeReadinessReservation,
};
use worth_store_security::StoreSecurityScopeProofProgressionIdentity;

fn main() {
    let proof_progression_identity: StoreSecurityScopeProofProgressionIdentity = unimplemented!();
    accept_s5_1_admitted_security_scope_readiness(
        S51SecurityScopeReadinessReservation::security_foundation(),
        proof_progression_identity,
    );
}
