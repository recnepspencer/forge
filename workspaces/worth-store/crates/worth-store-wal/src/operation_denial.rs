#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalOperationDenialKind {
    WrongPublicationKind,
    NonReplayTailRecord,
    ReplayTopologyDenied,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalOperationDenial {
    kind: WalOperationDenialKind,
}

impl WalOperationDenial {
    pub(crate) const fn new(kind: WalOperationDenialKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> WalOperationDenialKind {
        self.kind
    }
}
