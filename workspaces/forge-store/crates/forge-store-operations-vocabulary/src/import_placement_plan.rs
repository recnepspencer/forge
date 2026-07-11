#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportPlacementSource {
    InlineInBundle,
    ExternalByReference,
    ColdUnavailable,
    ScopeDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportPlacementDisposition {
    AlreadyPresentLocally,
    DedupedLocally,
    RequiresFetch,
    ScopeDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImportPlacementPlan {
    source: ImportPlacementSource,
    disposition: ImportPlacementDisposition,
    declared_chunks: u64,
    local_chunks: u64,
}

impl ImportPlacementPlan {
    pub const fn already_present_locally(
        source: ImportPlacementSource,
        declared_chunks: u64,
    ) -> Self {
        Self {
            source,
            disposition: ImportPlacementDisposition::AlreadyPresentLocally,
            declared_chunks,
            local_chunks: declared_chunks,
        }
    }

    pub const fn deduped_locally(source: ImportPlacementSource, declared_chunks: u64) -> Self {
        Self {
            source,
            disposition: ImportPlacementDisposition::DedupedLocally,
            declared_chunks,
            local_chunks: declared_chunks,
        }
    }

    pub const fn requires_fetch(
        source: ImportPlacementSource,
        declared_chunks: u64,
        local_chunks: u64,
    ) -> Self {
        Self {
            source,
            disposition: ImportPlacementDisposition::RequiresFetch,
            declared_chunks,
            local_chunks,
        }
    }

    pub const fn scope_denied(
        source: ImportPlacementSource,
        declared_chunks: u64,
        local_chunks: u64,
    ) -> Self {
        Self {
            source,
            disposition: ImportPlacementDisposition::ScopeDenied,
            declared_chunks,
            local_chunks,
        }
    }

    pub const fn source(self) -> ImportPlacementSource {
        self.source
    }

    pub const fn disposition(self) -> ImportPlacementDisposition {
        self.disposition
    }

    pub const fn declared_chunks(self) -> u64 {
        self.declared_chunks
    }

    pub const fn local_chunks(self) -> u64 {
        self.local_chunks
    }
}
