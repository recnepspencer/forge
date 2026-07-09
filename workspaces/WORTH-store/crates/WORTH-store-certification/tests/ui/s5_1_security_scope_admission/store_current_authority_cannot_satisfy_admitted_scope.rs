use worth_store_authority::StoreCurrentAuthorityWitness;
use worth_store_readiness::{
    accept_s5_1_admitted_security_scope_readiness, S51SecurityScopeReadinessReservation,
};

fn main() {
    let authority: StoreCurrentAuthorityWitness = unimplemented!();
    accept_s5_1_admitted_security_scope_readiness(
        S51SecurityScopeReadinessReservation::security_foundation(),
        authority,
    );
}
