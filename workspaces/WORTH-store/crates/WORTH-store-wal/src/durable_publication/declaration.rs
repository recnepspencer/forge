use super::{CheckpointDurablePublicationScope, WalFrameDurablePublicationScope};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DurablePublicationScope {
    WalFrame(WalFrameDurablePublicationScope),
    Checkpoint(CheckpointDurablePublicationScope),
    Manifest(CheckpointDurablePublicationScope),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DurablePublicationDeclaration {
    scope: DurablePublicationScope,
}

impl DurablePublicationDeclaration {
    pub const fn wal_frame(scope: WalFrameDurablePublicationScope) -> Self {
        Self {
            scope: DurablePublicationScope::WalFrame(scope),
        }
    }

    pub const fn checkpoint(scope: CheckpointDurablePublicationScope) -> Self {
        Self {
            scope: DurablePublicationScope::Checkpoint(scope),
        }
    }

    pub const fn manifest(scope: CheckpointDurablePublicationScope) -> Self {
        Self {
            scope: DurablePublicationScope::Manifest(scope),
        }
    }

    pub const fn scope(&self) -> &DurablePublicationScope {
        &self.scope
    }
}
