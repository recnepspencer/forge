use super::{RecoveryBindingFreshness, RecoveryOperationIdentity};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryOperationFate {
    AcknowledgedDurable,
    DurableUnacknowledged,
    ProvenNoEffect,
    Indeterminate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReconciledOperationFate {
    identity: RecoveryOperationIdentity,
    request_fingerprint: [u8; 32],
    lease_issuance_generation: u64,
    lease_expiry_generation: u64,
    freshness: RecoveryBindingFreshness,
    fate: RecoveryOperationFate,
}

impl ReconciledOperationFate {
    pub(super) const fn new(
        input: super::RecoveryOperationEvidenceInput,
        freshness: RecoveryBindingFreshness,
    ) -> Self {
        Self {
            identity: input.identity,
            request_fingerprint: input.request_fingerprint,
            lease_issuance_generation: input.lease_issuance_generation,
            lease_expiry_generation: input.lease_expiry_generation,
            freshness,
            fate: input.fate,
        }
    }

    pub const fn identity(&self) -> RecoveryOperationIdentity {
        self.identity
    }
    pub const fn request_fingerprint(&self) -> [u8; 32] {
        self.request_fingerprint
    }
    pub const fn lease_issuance_generation(&self) -> u64 {
        self.lease_issuance_generation
    }
    pub const fn lease_expiry_generation(&self) -> u64 {
        self.lease_expiry_generation
    }
    pub const fn freshness(&self) -> RecoveryBindingFreshness {
        self.freshness
    }
    pub const fn fate(&self) -> RecoveryOperationFate {
        self.fate
    }

    pub(super) const fn with_fate(mut self, fate: RecoveryOperationFate) -> Self {
        self.fate = fate;
        self
    }
}
