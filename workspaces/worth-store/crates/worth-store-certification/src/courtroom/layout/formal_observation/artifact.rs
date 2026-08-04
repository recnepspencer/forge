#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutDurableArtifactKind {
    BTreeStableRoot,
    BTreeSelectedReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutDurableArtifactObservation {
    PhysicalRoot {
        kind: LayoutDurableArtifactKind,
        root: worth_store_physical_isolation::CurrentPhysicalRoot,
    },
    PhysicalReference {
        kind: LayoutDurableArtifactKind,
        reference: worth_store_physical_format::PhysicalReference,
    },
}

impl LayoutDurableArtifactObservation {
    pub const fn kind(&self) -> LayoutDurableArtifactKind {
        match self {
            Self::PhysicalRoot { kind, .. } | Self::PhysicalReference { kind, .. } => *kind,
        }
    }
}
