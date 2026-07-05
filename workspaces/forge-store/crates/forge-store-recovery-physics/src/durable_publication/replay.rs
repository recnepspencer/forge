use forge_store_physical_backend::{BackendTargetProfile, StoreDurabilityPublicationKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DurabilityReplayKind {
    WalFrame,
    Checkpoint,
    Manifest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DurabilityReplayIdentity {
    kind: DurabilityReplayKind,
    profile: BackendTargetProfile,
    digest: String,
    first_lsn: u64,
    last_lsn: u64,
}

impl DurabilityReplayIdentity {
    pub fn new(
        publication: StoreDurabilityPublicationKind,
        profile: BackendTargetProfile,
        digest: impl Into<String>,
        first_lsn: u64,
        last_lsn: u64,
    ) -> Self {
        let kind = match publication {
            StoreDurabilityPublicationKind::WalFrame => DurabilityReplayKind::WalFrame,
            StoreDurabilityPublicationKind::Checkpoint => DurabilityReplayKind::Checkpoint,
            StoreDurabilityPublicationKind::Manifest => DurabilityReplayKind::Manifest,
        };
        Self {
            kind,
            profile,
            digest: digest.into(),
            first_lsn,
            last_lsn,
        }
    }

    pub const fn kind(&self) -> DurabilityReplayKind {
        self.kind
    }

    pub const fn profile(&self) -> BackendTargetProfile {
        self.profile
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub const fn first_lsn(&self) -> u64 {
        self.first_lsn
    }

    pub const fn last_lsn(&self) -> u64 {
        self.last_lsn
    }
}
