use serde::{Deserialize, Serialize};

use crate::data::handle::NodeId;
use crate::data::resource::{
    ResourcePayloadContractDigest, ResourcePolicyDigest, ResourceRequestHandle,
};
use crate::data::temporal::TemporalDuration;

use super::{AsyncNodeRequestIntent, AsyncNodeRevalidationIntent};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AsyncCapableNode {
    node: NodeId,
    registry_digest: ResourcePolicyDigest,
    bundle_digest: ResourcePolicyDigest,
    payload_contract_digest: ResourcePayloadContractDigest,
}

impl AsyncCapableNode {
    pub(crate) fn new(
        node: NodeId,
        registry_digest: ResourcePolicyDigest,
        bundle_digest: ResourcePolicyDigest,
        payload_contract_digest: ResourcePayloadContractDigest,
    ) -> Self {
        Self {
            node,
            registry_digest,
            bundle_digest,
            payload_contract_digest,
        }
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

    pub fn request_intent(&self) -> AsyncNodeRequestIntent {
        AsyncNodeRequestIntent::new(self.node)
    }

    pub fn request_intent_with_transaction_deadline(
        &self,
        deadline: TemporalDuration,
    ) -> AsyncNodeRequestIntent {
        AsyncNodeRequestIntent::with_transaction_deadline(self.node, deadline)
    }

    pub fn revalidation_intent(&self) -> AsyncNodeRevalidationIntent {
        AsyncNodeRevalidationIntent::new(self.node)
    }

    pub fn revalidation_intent_with_expected_active(
        &self,
        expected_active: ResourceRequestHandle,
    ) -> AsyncNodeRevalidationIntent {
        AsyncNodeRevalidationIntent::with_expected_active(self.node, expected_active)
    }

    pub fn revalidation_intent_with_transaction_deadline(
        &self,
        deadline: TemporalDuration,
    ) -> AsyncNodeRevalidationIntent {
        AsyncNodeRevalidationIntent::with_transaction_deadline(self.node, deadline)
    }
}
