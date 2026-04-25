use serde::{Deserialize, Serialize};

/// Canonical runtime-owned lifecycle classification for resource-backed nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ResourceLifecycleClass {
    Unrequested,
    Pending,
    Fulfilled,
    Rejected,
    Cancelled,
    TimedOut,
    Stale,
    Superseded,
    Disposed,
    RetainedHistoryUnavailable,
}

impl ResourceLifecycleClass {
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Fulfilled
                | Self::Rejected
                | Self::Cancelled
                | Self::TimedOut
                | Self::Superseded
                | Self::Disposed
                | Self::RetainedHistoryUnavailable
        )
    }

    pub fn is_runtime_truth(self) -> bool {
        !matches!(self, Self::RetainedHistoryUnavailable)
    }
}

/// Monotonic lifecycle transition ordinal assigned by the resource subsystem.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourceLifecycleOrdinal(u64);

impl ResourceLifecycleOrdinal {
    pub const ZERO: Self = Self(0);

    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

/// Output-continuity posture kept separate from resource lifecycle truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceOutputContinuity {
    NoPriorOutput,
    PriorOutputPreserved,
    OutputReplaced,
    OutputUnavailableByPolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceLifecycleTransitionKind {
    DeclarationInitialized,
    RequestAdmitted,
    RequestSuperseded,
    RequestCancelled,
    RequestTimedOut,
    CompletionAdmitted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceLifecycleTransition {
    node: super::request::ResourceNodeId,
    from: ResourceLifecycleClass,
    to: ResourceLifecycleClass,
    kind: ResourceLifecycleTransitionKind,
    ordinal: ResourceLifecycleOrdinal,
    output_continuity: ResourceOutputContinuity,
}

impl ResourceLifecycleTransition {
    pub(crate) fn new(
        node: super::request::ResourceNodeId,
        from: ResourceLifecycleClass,
        to: ResourceLifecycleClass,
        kind: ResourceLifecycleTransitionKind,
        ordinal: ResourceLifecycleOrdinal,
        output_continuity: ResourceOutputContinuity,
    ) -> Self {
        Self {
            node,
            from,
            to,
            kind,
            ordinal,
            output_continuity,
        }
    }

    pub fn node(self) -> super::request::ResourceNodeId {
        self.node
    }

    pub fn from(self) -> ResourceLifecycleClass {
        self.from
    }

    pub fn to(self) -> ResourceLifecycleClass {
        self.to
    }

    pub fn kind(self) -> ResourceLifecycleTransitionKind {
        self.kind
    }

    pub fn ordinal(self) -> ResourceLifecycleOrdinal {
        self.ordinal
    }

    pub fn output_continuity(self) -> ResourceOutputContinuity {
        self.output_continuity
    }
}
