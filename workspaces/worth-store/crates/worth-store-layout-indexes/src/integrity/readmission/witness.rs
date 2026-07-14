use worth_store_recovery_physics::{LogSequenceNumber, RecoveryLayoutReadmissionIdentity};

use super::{LayoutReadmissionIdentity, LayoutReadmissionSource};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutReadmissionWitness {
    family: crate::AdmittedPhysicalArtifactFamily,
    source: LayoutReadmissionSource,
    identity: LayoutReadmissionIdentity,
    replay_frontier: Option<LogSequenceNumber>,
}

impl LayoutReadmissionWitness {
    pub(super) fn issue(
        family: crate::AdmittedPhysicalArtifactFamily,
        source: LayoutReadmissionSource,
        identity: &RecoveryLayoutReadmissionIdentity,
        replay_frontier: Option<LogSequenceNumber>,
    ) -> Self {
        Self {
            family,
            source,
            identity: LayoutReadmissionIdentity::from_recovery(identity),
            replay_frontier,
        }
    }

    pub const fn family(self) -> crate::PhysicalArtifactFamily {
        self.family.lifecycle().declaration().family()
    }
    pub const fn security_identity(self) -> worth_store_security::StoreSecurityScopeIdentity {
        self.family.security_identity()
    }
    pub const fn store_authority_identity(
        self,
    ) -> worth_store_authority::StoreCurrentAuthorityIdentity {
        self.family.authority_identity()
    }
    pub const fn source(self) -> LayoutReadmissionSource {
        self.source
    }
    pub const fn identity(self) -> LayoutReadmissionIdentity {
        self.identity
    }
    pub const fn replay_frontier(self) -> Option<LogSequenceNumber> {
        self.replay_frontier
    }
}
