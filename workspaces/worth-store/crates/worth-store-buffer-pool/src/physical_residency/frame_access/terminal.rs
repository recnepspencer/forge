use super::PhysicalFrameLoadingIdentity;

/// Terminal lower-layer fate shared by every participant in one frame fault.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalFrameLoadTerminal {
    identity: PhysicalFrameLoadingIdentity,
    kind: PhysicalFrameLoadTerminalKind,
}

impl PhysicalFrameLoadTerminal {
    pub(crate) const fn new(
        identity: PhysicalFrameLoadingIdentity,
        kind: PhysicalFrameLoadTerminalKind,
    ) -> Self {
        Self { identity, kind }
    }

    pub const fn identity(self) -> PhysicalFrameLoadingIdentity {
        self.identity
    }

    pub const fn kind(self) -> PhysicalFrameLoadTerminalKind {
        self.kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalFrameLoadTerminalKind {
    SourcePreparationFailed,
    SourceExecutionFailed,
    AllocationFailed,
    PoolClosed,
    FaultOwnerAbandoned,
}
