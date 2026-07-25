use worth_signal::facade::{
    AdmittedResourceRetry, ResourceAttemptId, ResourcePayloadContractDigest, ResourcePolicyDigest,
    ResourceRequestHandle, ResourceSupersessionRecord,
};

pub(in crate::physical_runtime) struct PhysicalSignalReadinessEvidence {
    pub(in crate::physical_runtime) signal_request: ResourceRequestHandle,
    pub(in crate::physical_runtime) supersession: Option<ResourceSupersessionRecord>,
    pub(in crate::physical_runtime) replaces: Option<ResourceRequestHandle>,
    pub(in crate::physical_runtime) attempt: ResourceAttemptId,
    pub(in crate::physical_runtime) capability_registry: ResourcePolicyDigest,
    pub(in crate::physical_runtime) capability_bundle: ResourcePolicyDigest,
    pub(in crate::physical_runtime) payload_contract: ResourcePayloadContractDigest,
}

impl PhysicalSignalReadinessEvidence {
    pub(in crate::physical_runtime) const fn payload_contract(
        &self,
    ) -> &ResourcePayloadContractDigest {
        &self.payload_contract
    }

    pub(in crate::physical_runtime) fn for_retry(&self, admitted: AdmittedResourceRetry) -> Self {
        let predecessor = admitted.scheduled().previous();
        let request = admitted.admitted_request();
        Self {
            signal_request: request.handle(),
            supersession: None,
            replaces: Some(predecessor),
            attempt: request.attempt(),
            capability_registry: self.capability_registry.clone(),
            capability_bundle: self.capability_bundle.clone(),
            payload_contract: self.payload_contract.clone(),
        }
    }
}
