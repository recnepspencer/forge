use worth_foundational::facade::CanonicalDigestWorkEvidence;

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryCanonicalWorkEvidence {
    basis_preparations: u32,
    digest_derivations: u32,
    canonical_entries: u32,
    canonical_encoded_bytes: usize,
    canonical_material_allocation_bytes: usize,
    sha256_input_bytes: usize,
    sha256_compression_blocks: usize,
    digest_text_materializations: u32,
}

impl WorthQueryCanonicalWorkEvidence {
    pub const fn zero() -> Self {
        Self {
            basis_preparations: 0,
            digest_derivations: 0,
            canonical_entries: 0,
            canonical_encoded_bytes: 0,
            canonical_material_allocation_bytes: 0,
            sha256_input_bytes: 0,
            sha256_compression_blocks: 0,
            digest_text_materializations: 0,
        }
    }

    pub const fn one_digest(work: CanonicalDigestWorkEvidence) -> Self {
        Self {
            basis_preparations: 1,
            digest_derivations: 1,
            canonical_entries: work.canonical_entry_count(),
            canonical_encoded_bytes: work.canonical_encoded_bytes(),
            canonical_material_allocation_bytes: work.canonical_material_allocation_bytes(),
            sha256_input_bytes: work.sha256_input_bytes(),
            sha256_compression_blocks: work.sha256_compression_block_count(),
            digest_text_materializations: 0,
        }
    }

    pub const fn combine(self, other: Self) -> Self {
        Self {
            basis_preparations: self
                .basis_preparations
                .saturating_add(other.basis_preparations),
            digest_derivations: self
                .digest_derivations
                .saturating_add(other.digest_derivations),
            canonical_entries: self
                .canonical_entries
                .saturating_add(other.canonical_entries),
            canonical_encoded_bytes: self
                .canonical_encoded_bytes
                .saturating_add(other.canonical_encoded_bytes),
            canonical_material_allocation_bytes: self
                .canonical_material_allocation_bytes
                .saturating_add(other.canonical_material_allocation_bytes),
            sha256_input_bytes: self
                .sha256_input_bytes
                .saturating_add(other.sha256_input_bytes),
            sha256_compression_blocks: self
                .sha256_compression_blocks
                .saturating_add(other.sha256_compression_blocks),
            digest_text_materializations: self
                .digest_text_materializations
                .saturating_add(other.digest_text_materializations),
        }
    }

    pub const fn basis_preparations(self) -> u32 {
        self.basis_preparations
    }

    pub const fn digest_derivations(self) -> u32 {
        self.digest_derivations
    }

    pub const fn canonical_entries(self) -> u32 {
        self.canonical_entries
    }

    pub const fn canonical_encoded_bytes(self) -> usize {
        self.canonical_encoded_bytes
    }

    pub const fn canonical_material_allocation_bytes(self) -> usize {
        self.canonical_material_allocation_bytes
    }

    pub const fn sha256_input_bytes(self) -> usize {
        self.sha256_input_bytes
    }

    pub const fn sha256_compression_blocks(self) -> usize {
        self.sha256_compression_blocks
    }

    pub const fn digest_text_materializations(self) -> u32 {
        self.digest_text_materializations
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryCanonicalWorkPhases {
    installation: WorthQueryCanonicalWorkEvidence,
    admission: WorthQueryCanonicalWorkEvidence,
    execution: WorthQueryCanonicalWorkEvidence,
    provider_commit: WorthQueryCanonicalWorkEvidence,
    projection: WorthQueryCanonicalWorkEvidence,
    live_delivery: WorthQueryCanonicalWorkEvidence,
    retry_resolution: WorthQueryCanonicalWorkEvidence,
    recovery_inspection: WorthQueryCanonicalWorkEvidence,
    publication: WorthQueryCanonicalWorkEvidence,
}

impl WorthQueryCanonicalWorkPhases {
    pub const fn new(
        installation: WorthQueryCanonicalWorkEvidence,
        admission: WorthQueryCanonicalWorkEvidence,
    ) -> Self {
        Self {
            installation,
            admission,
            execution: WorthQueryCanonicalWorkEvidence::zero(),
            provider_commit: WorthQueryCanonicalWorkEvidence::zero(),
            projection: WorthQueryCanonicalWorkEvidence::zero(),
            live_delivery: WorthQueryCanonicalWorkEvidence::zero(),
            retry_resolution: WorthQueryCanonicalWorkEvidence::zero(),
            recovery_inspection: WorthQueryCanonicalWorkEvidence::zero(),
            publication: WorthQueryCanonicalWorkEvidence::zero(),
        }
    }

    pub const fn installation(self) -> WorthQueryCanonicalWorkEvidence {
        self.installation
    }

    pub const fn admission(self) -> WorthQueryCanonicalWorkEvidence {
        self.admission
    }

    pub const fn execution(self) -> WorthQueryCanonicalWorkEvidence {
        self.execution
    }

    pub const fn provider_commit(self) -> WorthQueryCanonicalWorkEvidence {
        self.provider_commit
    }

    pub const fn projection(self) -> WorthQueryCanonicalWorkEvidence {
        self.projection
    }

    pub const fn live_delivery(self) -> WorthQueryCanonicalWorkEvidence {
        self.live_delivery
    }

    pub const fn retry_resolution(self) -> WorthQueryCanonicalWorkEvidence {
        self.retry_resolution
    }

    pub const fn recovery_inspection(self) -> WorthQueryCanonicalWorkEvidence {
        self.recovery_inspection
    }

    pub const fn publication(self) -> WorthQueryCanonicalWorkEvidence {
        self.publication
    }
}
