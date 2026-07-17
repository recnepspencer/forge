use std::path::Path;

use super::{
    release_lease, RecoverySourceLeaseDenial, RecoverySourceLeaseReleaseReceipt,
    RecoverySourceReachabilityLease,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedBootstrapSourceCut(pub(super) RecoverySourceReachabilityLease);

impl ResolvedBootstrapSourceCut {
    pub fn lease(self) -> BootstrapReachabilityLease {
        BootstrapReachabilityLease(self.0)
    }

    pub fn source_root(&self) -> &Path {
        self.0.source_root()
    }

    pub const fn source_identity(&self) -> [u8; 32] {
        self.0.source_identity()
    }

    pub const fn source_evidence_identity(&self) -> [u8; 32] {
        self.0.source_evidence_identity()
    }

    pub fn artifact_names(&self) -> &[String] {
        self.0.artifact_names()
    }

    pub fn binding_fingerprint(&self) -> [u8; 32] {
        self.0.binding_fingerprint()
    }

    pub const fn operation_identity(&self) -> [u8; 32] {
        self.0.operation_identity()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapReachabilityLease(pub(super) RecoverySourceReachabilityLease);

impl BootstrapReachabilityLease {
    pub fn source_root(&self) -> &Path {
        self.0.source_root()
    }

    pub const fn source_identity(&self) -> [u8; 32] {
        self.0.source_identity()
    }

    pub const fn source_evidence_identity(&self) -> [u8; 32] {
        self.0.source_evidence_identity()
    }

    pub fn artifact_names(&self) -> &[String] {
        self.0.artifact_names()
    }

    pub fn binding_fingerprint(&self) -> [u8; 32] {
        self.0.binding_fingerprint()
    }

    pub const fn operation_identity(&self) -> [u8; 32] {
        self.0.operation_identity()
    }

    pub fn release(self) -> Result<RecoverySourceLeaseReleaseReceipt, RecoverySourceLeaseDenial> {
        release_lease(self.0)
    }
}
