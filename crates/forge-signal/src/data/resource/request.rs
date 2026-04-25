use serde::{Deserialize, Serialize};

use crate::data::handle::NodeId;
use crate::state::SignalBranchId;

/// Resource-node identity is distinct from the signal node handle that owns it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourceNodeId(NodeId);

impl ResourceNodeId {
    pub fn from_node(node: NodeId) -> Self {
        Self(node)
    }

    pub fn node(self) -> NodeId {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourceRequestId(u64);

impl ResourceRequestId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourceGeneration(u64);

impl ResourceGeneration {
    pub const ZERO: Self = Self(0);

    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourceAttemptId(u64);

impl ResourceAttemptId {
    pub const ZERO: Self = Self(0);

    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }

    pub(crate) fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourceBranchEpoch {
    branch_id: SignalBranchId,
    restore_epoch: u64,
}

impl ResourceBranchEpoch {
    pub fn new(branch_id: SignalBranchId, restore_epoch: u64) -> Self {
        Self {
            branch_id,
            restore_epoch,
        }
    }

    pub fn branch_id(self) -> SignalBranchId {
        self.branch_id
    }

    pub fn restore_epoch(self) -> u64 {
        self.restore_epoch
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourceCompletionOrdinal(u64);

impl ResourceCompletionOrdinal {
    pub const ZERO: Self = Self(0);

    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourceCancellationOrdinal(u64);

impl ResourceCancellationOrdinal {
    pub const ZERO: Self = Self(0);

    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourceTimeoutOrdinal(u64);

impl ResourceTimeoutOrdinal {
    pub const ZERO: Self = Self(0);

    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourceSupersessionOrdinal(u64);

impl ResourceSupersessionOrdinal {
    pub const ZERO: Self = Self(0);

    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourceRetryOrdinal(u64);

impl ResourceRetryOrdinal {
    pub const ZERO: Self = Self(0);

    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

/// User-visible request intent carries no admission authority by itself.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceRequestIntent {
    node: ResourceNodeId,
}

impl ResourceRequestIntent {
    pub fn new(node: ResourceNodeId) -> Self {
        Self { node }
    }

    pub fn node(&self) -> ResourceNodeId {
        self.node
    }
}

/// Facade-safe handle for a request already admitted by the runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourceRequestHandle {
    request_id: ResourceRequestId,
    generation: ResourceGeneration,
    branch_epoch: ResourceBranchEpoch,
}

impl ResourceRequestHandle {
    #[allow(dead_code)]
    pub(crate) fn new(
        request_id: ResourceRequestId,
        generation: ResourceGeneration,
        branch_epoch: ResourceBranchEpoch,
    ) -> Self {
        Self {
            request_id,
            generation,
            branch_epoch,
        }
    }

    pub fn request_id(self) -> ResourceRequestId {
        self.request_id
    }

    pub fn generation(self) -> ResourceGeneration {
        self.generation
    }

    pub fn branch_epoch(self) -> ResourceBranchEpoch {
        self.branch_epoch
    }

    pub(crate) fn with_branch_epoch(self, branch_epoch: ResourceBranchEpoch) -> Self {
        Self {
            request_id: self.request_id,
            generation: self.generation,
            branch_epoch,
        }
    }
}
