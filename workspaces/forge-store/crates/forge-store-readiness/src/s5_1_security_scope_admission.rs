use forge_store_security::{
    StoreAdmittedSecurityScope, StoreCurrentSecurityScopeWitnessSet,
    StoreSecurityScopeAdmissionReceipt,
};

use crate::S51SecurityScopeReadinessReservation;

#[derive(Debug, PartialEq, Eq)]
pub struct S51AdmittedSecurityScopeReadiness {
    reservation: S51SecurityScopeReadinessReservation,
    witnesses: StoreCurrentSecurityScopeWitnessSet,
    receipt: StoreSecurityScopeAdmissionReceipt,
}

impl S51AdmittedSecurityScopeReadiness {
    pub fn from_admitted_security_scope(
        reservation: S51SecurityScopeReadinessReservation,
        admitted_scope: StoreAdmittedSecurityScope,
    ) -> Self {
        let receipt = admitted_scope.receipt();
        Self {
            reservation,
            witnesses: admitted_scope.into_witnesses_for_readiness_handoff(),
            receipt,
        }
    }

    pub const fn reservation(&self) -> S51SecurityScopeReadinessReservation {
        self.reservation
    }

    pub const fn witnesses(&self) -> &StoreCurrentSecurityScopeWitnessSet {
        &self.witnesses
    }

    pub const fn receipt(&self) -> StoreSecurityScopeAdmissionReceipt {
        self.receipt
    }
}

pub fn accept_s5_1_admitted_security_scope_readiness(
    reservation: S51SecurityScopeReadinessReservation,
    admitted_scope: StoreAdmittedSecurityScope,
) -> S51AdmittedSecurityScopeReadiness {
    S51AdmittedSecurityScopeReadiness::from_admitted_security_scope(reservation, admitted_scope)
}
