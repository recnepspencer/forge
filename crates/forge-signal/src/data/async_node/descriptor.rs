use serde::{Deserialize, Serialize};

use crate::data::handle::NodeId;
use crate::data::resource::{
    FrozenResourcePolicyDescriptorSet, LoweredResourcePolicyBundle, ResourcePayloadContractDigest,
    ResourcePolicyDigest,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FrozenAsyncNodeCapabilityDescriptor {
    node: NodeId,
    payload_contract_digest: ResourcePayloadContractDigest,
    frozen: FrozenResourcePolicyDescriptorSet,
}

impl FrozenAsyncNodeCapabilityDescriptor {
    pub(crate) fn new(
        node: NodeId,
        payload_contract_digest: ResourcePayloadContractDigest,
        frozen: FrozenResourcePolicyDescriptorSet,
    ) -> Self {
        Self {
            node,
            payload_contract_digest,
            frozen,
        }
    }

    pub fn node(&self) -> NodeId {
        self.node
    }

    pub fn registry_digest(&self) -> &ResourcePolicyDigest {
        self.frozen.registry_digest()
    }

    pub fn payload_contract_digest(&self) -> &ResourcePayloadContractDigest {
        &self.payload_contract_digest
    }

    pub(crate) fn frozen(&self) -> &FrozenResourcePolicyDescriptorSet {
        &self.frozen
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LoweredAsyncNodeCapabilityBundle {
    node: NodeId,
    payload_contract_digest: ResourcePayloadContractDigest,
    lowered: LoweredResourcePolicyBundle,
}

impl LoweredAsyncNodeCapabilityBundle {
    pub(crate) fn new(
        node: NodeId,
        payload_contract_digest: ResourcePayloadContractDigest,
        lowered: LoweredResourcePolicyBundle,
    ) -> Self {
        Self {
            node,
            payload_contract_digest,
            lowered,
        }
    }

    pub fn node(&self) -> NodeId {
        self.node
    }

    pub fn registry_digest(&self) -> &ResourcePolicyDigest {
        self.lowered.registry_digest()
    }

    pub fn bundle_digest(&self) -> &ResourcePolicyDigest {
        self.lowered.bundle_digest()
    }

    pub fn payload_contract_digest(&self) -> &ResourcePayloadContractDigest {
        &self.payload_contract_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AsyncNodeCapabilityAliasLoweringProof {
    node: NodeId,
    capability_registry_digest: ResourcePolicyDigest,
    legacy_registry_digest: ResourcePolicyDigest,
    capability_bundle_digest: ResourcePolicyDigest,
    legacy_bundle_digest: ResourcePolicyDigest,
    capability_payload_contract_digest: ResourcePayloadContractDigest,
    legacy_payload_contract_digest: ResourcePayloadContractDigest,
    compared_width: u32,
}

impl AsyncNodeCapabilityAliasLoweringProof {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        node: NodeId,
        capability_registry_digest: ResourcePolicyDigest,
        legacy_registry_digest: ResourcePolicyDigest,
        capability_bundle_digest: ResourcePolicyDigest,
        legacy_bundle_digest: ResourcePolicyDigest,
        capability_payload_contract_digest: ResourcePayloadContractDigest,
        legacy_payload_contract_digest: ResourcePayloadContractDigest,
        compared_width: u32,
    ) -> Self {
        Self {
            node,
            capability_registry_digest,
            legacy_registry_digest,
            capability_bundle_digest,
            legacy_bundle_digest,
            capability_payload_contract_digest,
            legacy_payload_contract_digest,
            compared_width,
        }
    }

    pub fn node(&self) -> NodeId {
        self.node
    }

    pub fn capability_registry_digest(&self) -> &ResourcePolicyDigest {
        &self.capability_registry_digest
    }

    pub fn legacy_registry_digest(&self) -> &ResourcePolicyDigest {
        &self.legacy_registry_digest
    }

    pub fn capability_bundle_digest(&self) -> &ResourcePolicyDigest {
        &self.capability_bundle_digest
    }

    pub fn legacy_bundle_digest(&self) -> &ResourcePolicyDigest {
        &self.legacy_bundle_digest
    }

    pub fn capability_payload_contract_digest(&self) -> &ResourcePayloadContractDigest {
        &self.capability_payload_contract_digest
    }

    pub fn legacy_payload_contract_digest(&self) -> &ResourcePayloadContractDigest {
        &self.legacy_payload_contract_digest
    }

    pub fn compared_width(&self) -> u32 {
        self.compared_width
    }
}
