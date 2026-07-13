use super::{FutureLayoutCapabilityRequest, FutureLayoutWorkloadEnvelope};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FutureLayoutCustomizationRequest {
    authority_source: crate::AdmittedPhysicalArtifactFamily,
    capability_request: FutureLayoutCapabilityRequest,
    workload_envelope: FutureLayoutWorkloadEnvelope,
}

impl FutureLayoutCustomizationRequest {
    pub const fn new(
        authority_source: crate::AdmittedPhysicalArtifactFamily,
        capability_request: FutureLayoutCapabilityRequest,
        workload_envelope: FutureLayoutWorkloadEnvelope,
    ) -> Self {
        Self {
            authority_source,
            capability_request,
            workload_envelope,
        }
    }

    pub const fn authority_source(self) -> crate::AdmittedPhysicalArtifactFamily {
        self.authority_source
    }

    pub const fn capability_request(self) -> FutureLayoutCapabilityRequest {
        self.capability_request
    }

    pub const fn workload_envelope(self) -> FutureLayoutWorkloadEnvelope {
        self.workload_envelope
    }
}
