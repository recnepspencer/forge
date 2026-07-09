#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobPhysicalManifestDenialKind {
    MissingExternalChunk,
    StaleGenerationRow,
    OrphanedPlacementResidue,
    WrongRowKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobPhysicalManifestDenial {
    kind: BlobPhysicalManifestDenialKind,
    row_digest: String,
}

impl BlobPhysicalManifestDenial {
    pub(crate) fn new(kind: BlobPhysicalManifestDenialKind, row_digest: impl Into<String>) -> Self {
        Self {
            kind,
            row_digest: row_digest.into(),
        }
    }

    pub const fn kind(&self) -> BlobPhysicalManifestDenialKind {
        self.kind
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
