use serde::Serialize;

use super::super::policy::ResourcePolicyName;
use super::super::summary::ResourceCostContractId;
use super::digest::descriptor_digest;
use super::identity::{
    ResourcePolicyCompatibilityPosture, ResourcePolicyDescriptorId, ResourcePolicyDigest,
    ResourcePolicyKind, ResourcePolicyVersion,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResourcePolicyDescriptor {
    id: ResourcePolicyDescriptorId,
    kind: ResourcePolicyKind,
    semantic_name: ResourcePolicyName,
    version: ResourcePolicyVersion,
    descriptor_digest: ResourcePolicyDigest,
    cost_contract: ResourceCostContractId,
    compatibility_posture: ResourcePolicyCompatibilityPosture,
}

impl ResourcePolicyDescriptor {
    pub(crate) fn new(
        id: ResourcePolicyDescriptorId,
        kind: ResourcePolicyKind,
        semantic_name: ResourcePolicyName,
        version: ResourcePolicyVersion,
        cost_contract: ResourceCostContractId,
        compatibility_posture: ResourcePolicyCompatibilityPosture,
    ) -> Self {
        let descriptor_digest = descriptor_digest(id, kind, &semantic_name, version, cost_contract);
        Self {
            id,
            kind,
            semantic_name,
            version,
            descriptor_digest,
            cost_contract,
            compatibility_posture,
        }
    }

    pub fn id(&self) -> ResourcePolicyDescriptorId {
        self.id
    }

    pub fn kind(&self) -> ResourcePolicyKind {
        self.kind
    }

    pub fn semantic_name(&self) -> &ResourcePolicyName {
        &self.semantic_name
    }

    pub fn version(&self) -> ResourcePolicyVersion {
        self.version
    }

    pub fn descriptor_digest(&self) -> &ResourcePolicyDigest {
        &self.descriptor_digest
    }

    pub fn cost_contract(&self) -> ResourceCostContractId {
        self.cost_contract
    }

    pub fn compatibility_posture(&self) -> ResourcePolicyCompatibilityPosture {
        self.compatibility_posture
    }
}
