use super::super::policy::ResourcePolicyName;
use super::super::summary::ResourceCostContractId;
use super::identity::{
    ResourcePolicyCompatibilityPosture, ResourcePolicyDescriptorId, ResourcePolicyKind,
    ResourcePolicyVersion,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourcePolicyRegistration {
    id: ResourcePolicyDescriptorId,
    kind: ResourcePolicyKind,
    semantic_name: ResourcePolicyName,
    version: ResourcePolicyVersion,
    cost_contract: ResourceCostContractId,
    compatibility_posture: ResourcePolicyCompatibilityPosture,
}

impl ResourcePolicyRegistration {
    pub fn new(
        id: ResourcePolicyDescriptorId,
        kind: ResourcePolicyKind,
        semantic_name: ResourcePolicyName,
        version: ResourcePolicyVersion,
        cost_contract: ResourceCostContractId,
        compatibility_posture: ResourcePolicyCompatibilityPosture,
    ) -> Self {
        Self {
            id,
            kind,
            semantic_name,
            version,
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

    pub fn cost_contract(&self) -> ResourceCostContractId {
        self.cost_contract
    }

    pub fn compatibility_posture(&self) -> ResourcePolicyCompatibilityPosture {
        self.compatibility_posture
    }
}

pub(super) fn built_in_resource_policy_registration(
    id: u64,
    kind: ResourcePolicyKind,
    name: &'static str,
    contract: u64,
) -> ResourcePolicyRegistration {
    ResourcePolicyRegistration::new(
        ResourcePolicyDescriptorId::new(id),
        kind,
        ResourcePolicyName::new(name),
        ResourcePolicyVersion::INITIAL,
        ResourceCostContractId::new(contract),
        ResourcePolicyCompatibilityPosture::ExactDescriptorMatch,
    )
}
