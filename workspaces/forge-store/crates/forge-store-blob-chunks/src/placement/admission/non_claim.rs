#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobPlacementNonClaim {
    BackupSemantics,
    RestoreSemantics,
    ArchivalSemantics,
}

impl BlobPlacementNonClaim {
    pub const fn required() -> [Self; 3] {
        [
            Self::BackupSemantics,
            Self::RestoreSemantics,
            Self::ArchivalSemantics,
        ]
    }
}
