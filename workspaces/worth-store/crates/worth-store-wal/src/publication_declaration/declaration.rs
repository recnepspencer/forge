use super::{CheckpointPublicationScope, WalFramePublicationScope};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PublicationScope {
    WalFrame(WalFramePublicationScope),
    Checkpoint(CheckpointPublicationScope),
    Manifest(CheckpointPublicationScope),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicationDeclaration {
    scope: PublicationScope,
}

impl PublicationDeclaration {
    pub const fn wal_frame(scope: WalFramePublicationScope) -> Self {
        Self {
            scope: PublicationScope::WalFrame(scope),
        }
    }

    pub const fn checkpoint(scope: CheckpointPublicationScope) -> Self {
        Self {
            scope: PublicationScope::Checkpoint(scope),
        }
    }

    pub const fn manifest(scope: CheckpointPublicationScope) -> Self {
        Self {
            scope: PublicationScope::Manifest(scope),
        }
    }

    pub const fn scope(&self) -> &PublicationScope {
        &self.scope
    }
}
