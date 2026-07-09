#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobPhysicalManifestRowKind {
    Reachability,
    Placement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobPhysicalManifestRow {
    kind: BlobPhysicalManifestRowKind,
    row_digest: String,
    generation_sequence: u64,
    external_chunk_present: bool,
}

impl BlobPhysicalManifestRow {
    #[cfg(test)]
    pub(crate) fn new(
        kind: BlobPhysicalManifestRowKind,
        row_digest: impl Into<String>,
        generation_sequence: u64,
        external_chunk_present: bool,
    ) -> Option<Self> {
        Self::admit(
            kind,
            row_digest,
            generation_sequence,
            external_chunk_present,
        )
    }

    #[cfg(test)]
    fn admit(
        kind: BlobPhysicalManifestRowKind,
        row_digest: impl Into<String>,
        generation_sequence: u64,
        external_chunk_present: bool,
    ) -> Option<Self> {
        let row_digest = row_digest.into();
        if row_digest.is_empty() || generation_sequence == 0 {
            return None;
        }
        Some(Self {
            kind,
            row_digest,
            generation_sequence,
            external_chunk_present,
        })
    }

    pub const fn kind(&self) -> BlobPhysicalManifestRowKind {
        self.kind
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }

    pub const fn generation_sequence(&self) -> u64 {
        self.generation_sequence
    }

    pub const fn external_chunk_present(&self) -> bool {
        self.external_chunk_present
    }
}
