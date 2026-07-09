use crate::artifact_family::ArtifactFamilyLifecycleAdmission;

use super::{S8FutureLayoutCapabilityRequest, S8FutureLayoutWorkloadEnvelope};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8FutureLayoutCustomizationRequest {
    authority_source: ArtifactFamilyLifecycleAdmission,
    capability_request: S8FutureLayoutCapabilityRequest,
    workload_envelope: S8FutureLayoutWorkloadEnvelope,
}

impl S8FutureLayoutCustomizationRequest {
    pub const fn new(
        authority_source: ArtifactFamilyLifecycleAdmission,
        capability_request: S8FutureLayoutCapabilityRequest,
        workload_envelope: S8FutureLayoutWorkloadEnvelope,
    ) -> Self {
        Self {
            authority_source,
            capability_request,
            workload_envelope,
        }
    }

    pub const fn authority_source(self) -> ArtifactFamilyLifecycleAdmission {
        self.authority_source
    }

    pub const fn capability_request(self) -> S8FutureLayoutCapabilityRequest {
        self.capability_request
    }

    pub const fn workload_envelope(self) -> S8FutureLayoutWorkloadEnvelope {
        self.workload_envelope
    }
}
