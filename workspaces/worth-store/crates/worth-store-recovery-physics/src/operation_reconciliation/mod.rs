mod binding_freshness;
mod denial;
mod evidence_join;
mod fate;
mod identity;
mod materialized_join;

pub use binding_freshness::{classify_binding_freshness, RecoveryBindingFreshness};
pub use denial::OperationReconciliationDenial;
pub use evidence_join::{reconcile_operation_fates, ReconciledOperationFates};
pub use fate::{ReconciledOperationFate, RecoveryOperationFate};
pub use identity::RecoveryOperationIdentity;
pub use materialized_join::reconcile_materialized_operation_fates;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryOperationEvidenceInput {
    identity: RecoveryOperationIdentity,
    request_fingerprint: [u8; 32],
    lease_issuance_generation: u64,
    lease_expiry_generation: u64,
    freshness: RecoveryBindingFreshness,
    fate: RecoveryOperationFate,
}

impl RecoveryOperationEvidenceInput {
    pub const fn new(
        identity: RecoveryOperationIdentity,
        request_fingerprint: [u8; 32],
        lease_issuance_generation: u64,
        lease_expiry_generation: u64,
        freshness: RecoveryBindingFreshness,
        fate: RecoveryOperationFate,
    ) -> Self {
        Self {
            identity,
            request_fingerprint,
            lease_issuance_generation,
            lease_expiry_generation,
            freshness,
            fate,
        }
    }
}
