use super::ProtocolFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OwnerBoundaryGapKind {
    CheckedProtocolModelPending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OwnerBoundaryGap {
    protocol: ProtocolFamily,
    reason: OwnerBoundaryGapKind,
}

impl OwnerBoundaryGap {
    pub const fn protocol(self) -> ProtocolFamily {
        self.protocol
    }

    pub const fn reason(self) -> OwnerBoundaryGapKind {
        self.reason
    }
}
