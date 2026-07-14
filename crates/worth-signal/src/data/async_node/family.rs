use serde::Serialize;

use crate::data::handle::NodeId;
use crate::data::output::{ComputationFamily, ComputationKey};
use crate::data::resource::{ResourcePayloadContractDigest, ResourcePolicyDigest};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AsyncKeyedNodeCapabilityBinding {
    family: ComputationFamily,
    key: ComputationKey,
    node: NodeId,
    registry_digest: ResourcePolicyDigest,
    bundle_digest: ResourcePolicyDigest,
    payload_contract_digest: ResourcePayloadContractDigest,
}

impl AsyncKeyedNodeCapabilityBinding {
    pub(crate) fn new(
        family: ComputationFamily,
        key: ComputationKey,
        node: NodeId,
        registry_digest: ResourcePolicyDigest,
        bundle_digest: ResourcePolicyDigest,
        payload_contract_digest: ResourcePayloadContractDigest,
    ) -> Self {
        Self {
            family,
            key,
            node,
            registry_digest,
            bundle_digest,
            payload_contract_digest,
        }
    }

    pub fn family(&self) -> &ComputationFamily {
        &self.family
    }

    pub fn key(&self) -> &ComputationKey {
        &self.key
    }

    pub fn node(&self) -> NodeId {
        self.node
    }

    pub fn registry_digest(&self) -> &ResourcePolicyDigest {
        &self.registry_digest
    }

    pub fn bundle_digest(&self) -> &ResourcePolicyDigest {
        &self.bundle_digest
    }

    pub fn payload_contract_digest(&self) -> &ResourcePayloadContractDigest {
        &self.payload_contract_digest
    }
}
