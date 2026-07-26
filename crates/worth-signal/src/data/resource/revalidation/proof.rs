use serde::Serialize;

use crate::data::node::NodeState;
use crate::data::resource::{
    ResourceLifecycleClass, ResourceLifecycleOrdinal, ResourceNodeId, ResourcePolicyDigest,
    ResourceRequestHandle,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ActiveResourceRevalidationProof {
    node: ResourceNodeId,
    handle: ResourceRequestHandle,
    decision_digest: ResourcePolicyDigest,
}

impl ActiveResourceRevalidationProof {
    pub(crate) fn new(
        node: ResourceNodeId,
        handle: ResourceRequestHandle,
        decision_digest: ResourcePolicyDigest,
    ) -> Self {
        Self {
            node,
            handle,
            decision_digest,
        }
    }

    pub fn node(&self) -> ResourceNodeId {
        self.node
    }

    pub fn handle(&self) -> ResourceRequestHandle {
        self.handle
    }

    pub fn decision_digest(&self) -> &ResourcePolicyDigest {
        &self.decision_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DependencyChangeResourceRevalidationProof {
    node: ResourceNodeId,
    node_state: NodeState,
    decision_digest: ResourcePolicyDigest,
}

impl DependencyChangeResourceRevalidationProof {
    pub(crate) fn new(
        node: ResourceNodeId,
        node_state: NodeState,
        decision_digest: ResourcePolicyDigest,
    ) -> Self {
        Self {
            node,
            node_state,
            decision_digest,
        }
    }

    pub fn node(&self) -> ResourceNodeId {
        self.node
    }

    pub fn node_state(&self) -> NodeState {
        self.node_state
    }

    pub fn decision_digest(&self) -> &ResourcePolicyDigest {
        &self.decision_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ObserverDemandResourceRevalidationProof {
    node: ResourceNodeId,
    observer_id: u64,
    handle_id: u64,
    observation_digest: String,
    decision_digest: ResourcePolicyDigest,
}

impl ObserverDemandResourceRevalidationProof {
    pub(crate) fn new(
        node: ResourceNodeId,
        observer_id: u64,
        handle_id: u64,
        observation_digest: String,
        decision_digest: ResourcePolicyDigest,
    ) -> Self {
        Self {
            node,
            observer_id,
            handle_id,
            observation_digest,
            decision_digest,
        }
    }

    pub fn node(&self) -> ResourceNodeId {
        self.node
    }

    pub fn observer_id(&self) -> u64 {
        self.observer_id
    }

    pub fn handle_id(&self) -> u64 {
        self.handle_id
    }

    pub fn observation_digest(&self) -> &str {
        &self.observation_digest
    }

    pub fn decision_digest(&self) -> &ResourcePolicyDigest {
        &self.decision_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TerminalStateResourceRevalidationProof {
    node: ResourceNodeId,
    lifecycle: ResourceLifecycleClass,
    lifecycle_ordinal: ResourceLifecycleOrdinal,
    decision_digest: ResourcePolicyDigest,
}

impl TerminalStateResourceRevalidationProof {
    pub(crate) fn new(
        node: ResourceNodeId,
        lifecycle: ResourceLifecycleClass,
        lifecycle_ordinal: ResourceLifecycleOrdinal,
        decision_digest: ResourcePolicyDigest,
    ) -> Self {
        Self {
            node,
            lifecycle,
            lifecycle_ordinal,
            decision_digest,
        }
    }

    pub fn node(&self) -> ResourceNodeId {
        self.node
    }

    pub fn lifecycle(&self) -> ResourceLifecycleClass {
        self.lifecycle
    }

    pub fn lifecycle_ordinal(&self) -> ResourceLifecycleOrdinal {
        self.lifecycle_ordinal
    }

    pub fn decision_digest(&self) -> &ResourcePolicyDigest {
        &self.decision_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FulfilledLifecycleResourceRevalidationProof {
    node: ResourceNodeId,
    lifecycle_ordinal: ResourceLifecycleOrdinal,
    decision_digest: ResourcePolicyDigest,
}

impl FulfilledLifecycleResourceRevalidationProof {
    pub(crate) fn new(
        node: ResourceNodeId,
        lifecycle_ordinal: ResourceLifecycleOrdinal,
        decision_digest: ResourcePolicyDigest,
    ) -> Self {
        Self {
            node,
            lifecycle_ordinal,
            decision_digest,
        }
    }

    pub fn node(&self) -> ResourceNodeId {
        self.node
    }

    pub fn lifecycle_ordinal(&self) -> ResourceLifecycleOrdinal {
        self.lifecycle_ordinal
    }

    pub fn decision_digest(&self) -> &ResourcePolicyDigest {
        &self.decision_digest
    }
}
