#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobChunkRootCounterSnapshot {
    root_publications: u64,
    canonical_basis_entries: u64,
    canonical_digest_derivations: u64,
    canonical_comparisons: u64,
    denials: u64,
}

impl BlobChunkRootCounterSnapshot {
    pub(crate) const fn start() -> Self {
        Self {
            root_publications: 0,
            canonical_basis_entries: 0,
            canonical_digest_derivations: 0,
            canonical_comparisons: 0,
            denials: 0,
        }
    }

    pub(crate) const fn record_root_publication(self, entries: u64) -> Self {
        Self {
            root_publications: self.root_publications + 1,
            canonical_basis_entries: self.canonical_basis_entries + entries,
            ..self
        }
    }

    pub(crate) const fn record_canonical_digest_derivation(self) -> Self {
        Self {
            canonical_digest_derivations: self.canonical_digest_derivations + 1,
            ..self
        }
    }

    pub(crate) const fn record_canonical_comparison(self) -> Self {
        Self {
            canonical_comparisons: self.canonical_comparisons + 1,
            ..self
        }
    }

    pub(crate) const fn record_denial(self) -> Self {
        Self {
            denials: self.denials + 1,
            ..self
        }
    }

    pub const fn root_publications(self) -> u64 {
        self.root_publications
    }

    pub const fn canonical_basis_entries(self) -> u64 {
        self.canonical_basis_entries
    }

    pub const fn canonical_digest_derivations(self) -> u64 {
        self.canonical_digest_derivations
    }

    pub const fn canonical_comparisons(self) -> u64 {
        self.canonical_comparisons
    }

    pub const fn denials(self) -> u64 {
        self.denials
    }
}
