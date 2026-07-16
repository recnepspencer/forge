#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobImportPlacementSource {
    InlineInBundle,
    ExternalByReference,
    ColdUnavailable,
    ScopeDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobImportPlacementDisposition {
    AlreadyPresentLocally,
    DedupedLocally,
    RequiresFetch,
    ScopeDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobImportPlacementPlan {
    source: BlobImportPlacementSource,
    disposition: BlobImportPlacementDisposition,
    declared_chunks: u64,
    local_chunks: u64,
}

impl BlobImportPlacementPlan {
    pub const fn already_present_locally(
        source: BlobImportPlacementSource,
        declared_chunks: u64,
    ) -> Self {
        Self {
            source,
            disposition: BlobImportPlacementDisposition::AlreadyPresentLocally,
            declared_chunks,
            local_chunks: declared_chunks,
        }
    }

    pub const fn deduped_locally(source: BlobImportPlacementSource, declared_chunks: u64) -> Self {
        Self {
            source,
            disposition: BlobImportPlacementDisposition::DedupedLocally,
            declared_chunks,
            local_chunks: declared_chunks,
        }
    }

    pub const fn requires_fetch(
        source: BlobImportPlacementSource,
        declared_chunks: u64,
        local_chunks: u64,
    ) -> Self {
        Self {
            source,
            disposition: BlobImportPlacementDisposition::RequiresFetch,
            declared_chunks,
            local_chunks,
        }
    }

    pub const fn scope_denied(
        source: BlobImportPlacementSource,
        declared_chunks: u64,
        local_chunks: u64,
    ) -> Self {
        Self {
            source,
            disposition: BlobImportPlacementDisposition::ScopeDenied,
            declared_chunks,
            local_chunks,
        }
    }

    pub const fn source(self) -> BlobImportPlacementSource {
        self.source
    }
    pub const fn disposition(self) -> BlobImportPlacementDisposition {
        self.disposition
    }
    pub const fn declared_chunks(self) -> u64 {
        self.declared_chunks
    }
    pub const fn local_chunks(self) -> u64 {
        self.local_chunks
    }
}
