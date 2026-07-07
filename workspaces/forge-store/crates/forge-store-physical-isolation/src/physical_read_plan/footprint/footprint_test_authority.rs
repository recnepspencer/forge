use super::PhysicalReadProtectedFootprintBasis;

impl PhysicalReadProtectedFootprintBasis {
    pub(crate) const fn for_certification_test(protected_references: u64) -> Self {
        Self {
            protected_references,
            protected_ranges: protected_references,
            canonical_digest: protected_references.wrapping_mul(0x1000_0000_01b3),
        }
    }
}
