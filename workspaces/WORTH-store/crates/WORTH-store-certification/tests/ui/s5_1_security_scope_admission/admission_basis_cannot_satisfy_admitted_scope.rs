use worth_store_readiness::{
    accept_s5_1_admitted_security_scope_readiness, S51SecurityScopeReadinessReservation,
};
use worth_store_security::StoreSecurityScopeAdmissionBasis;

fn main() {
    let basis: StoreSecurityScopeAdmissionBasis = unimplemented!();
    accept_s5_1_admitted_security_scope_readiness(
        S51SecurityScopeReadinessReservation::security_foundation(),
        basis,
    );
}
