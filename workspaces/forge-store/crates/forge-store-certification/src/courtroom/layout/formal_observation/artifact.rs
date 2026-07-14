#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutDurableArtifactKind {
    BTreeStableRoot,
    BTreeSelectedReference,
    LsmValue,
    LsmGenerationPublication,
    LsmTombstone,
    LsmReplacementOutput,
    LsmActivationManifest,
    PhysicalCompactionOldRoot,
    PhysicalCompactionNewRoot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutDurableArtifactObservation {
    PhysicalRoot {
        kind: LayoutDurableArtifactKind,
        root: forge_store_physical_isolation::CurrentPhysicalRoot,
    },
    PhysicalReference {
        kind: LayoutDurableArtifactKind,
        reference: forge_store_physical_format::PhysicalReference,
    },
    WalRecord {
        kind: LayoutDurableArtifactKind,
        identity: forge_store_wal::BlobWalRecordIdentity,
    },
    CheckpointManifest {
        kind: LayoutDurableArtifactKind,
        scope: forge_store_wal::CheckpointDurablePublicationScope,
    },
}

impl LayoutDurableArtifactObservation {
    pub const fn kind(&self) -> LayoutDurableArtifactKind {
        match self {
            Self::PhysicalRoot { kind, .. }
            | Self::PhysicalReference { kind, .. }
            | Self::WalRecord { kind, .. }
            | Self::CheckpointManifest { kind, .. } => *kind,
        }
    }
}
