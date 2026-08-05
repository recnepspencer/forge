#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalDigestWorkEvidence {
    canonical_entry_count: u32,
    canonical_encoded_bytes: usize,
    canonical_material_allocation_bytes: usize,
    sha256_input_bytes: usize,
    sha256_compression_block_count: usize,
}

impl CanonicalDigestWorkEvidence {
    pub(super) const fn new(
        canonical_entry_count: u32,
        canonical_encoded_bytes: usize,
        canonical_material_allocation_bytes: usize,
    ) -> Self {
        Self {
            canonical_entry_count,
            canonical_encoded_bytes,
            canonical_material_allocation_bytes,
            sha256_input_bytes: canonical_encoded_bytes,
            sha256_compression_block_count: sha256_compression_blocks(canonical_encoded_bytes),
        }
    }

    pub const fn canonical_entry_count(self) -> u32 {
        self.canonical_entry_count
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

    pub const fn sha256_compression_block_count(self) -> usize {
        self.sha256_compression_block_count
    }
}

const fn sha256_compression_blocks(input_bytes: usize) -> usize {
    input_bytes.saturating_add(9).saturating_add(63) / 64
}
