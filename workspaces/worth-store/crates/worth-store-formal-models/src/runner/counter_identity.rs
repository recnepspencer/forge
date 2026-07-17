use worth_store_physical_backend::BackendDurabilityProfileId;

use crate::ProtocolFamily;

use super::{
    ExecutedProtocolCheck, ProtocolArtifactIdentityInspectionDenial, ProtocolCheckArtifactIdentity,
    ProtocolCheckBounds, ProtocolCheckInvocation,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolArtifactIdentityPosture {
    ExecutedAndObserved,
    DeclaredStructuralFixture,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolCounterEvidenceIdentity {
    protocol: ProtocolFamily,
    bounds: ProtocolCheckBounds,
    backend_profile: BackendDurabilityProfileId,
    artifacts: ProtocolCheckArtifactIdentity,
    artifact_posture: ProtocolArtifactIdentityPosture,
}

impl ProtocolCounterEvidenceIdentity {
    pub fn from_executed_check(
        check: &ExecutedProtocolCheck,
        backend_profile: BackendDurabilityProfileId,
    ) -> Self {
        Self {
            protocol: check.protocol(),
            bounds: check.invocation().bounds(),
            backend_profile,
            artifacts: check.artifact_identity().clone(),
            artifact_posture: ProtocolArtifactIdentityPosture::ExecutedAndObserved,
        }
    }

    pub fn from_declared_fixture(
        invocation: &ProtocolCheckInvocation,
        backend_profile: BackendDurabilityProfileId,
    ) -> Result<Self, ProtocolArtifactIdentityInspectionDenial> {
        Ok(Self {
            protocol: invocation.protocol(),
            bounds: invocation.bounds(),
            backend_profile,
            artifacts: ProtocolCheckArtifactIdentity::declared_for(invocation)?,
            artifact_posture: ProtocolArtifactIdentityPosture::DeclaredStructuralFixture,
        })
    }

    pub const fn protocol(&self) -> ProtocolFamily {
        self.protocol
    }

    pub const fn bounds(&self) -> ProtocolCheckBounds {
        self.bounds
    }

    pub const fn backend_profile(&self) -> BackendDurabilityProfileId {
        self.backend_profile
    }

    pub const fn artifacts(&self) -> &ProtocolCheckArtifactIdentity {
        &self.artifacts
    }

    pub const fn artifact_posture(&self) -> ProtocolArtifactIdentityPosture {
        self.artifact_posture
    }
}
