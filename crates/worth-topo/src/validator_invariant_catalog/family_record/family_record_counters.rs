#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(in crate::validator_invariant_catalog) struct WorthTopologyLegalityFamilyRecordCounters {
    pub(in crate::validator_invariant_catalog) validator_family_count: usize,
    pub(in crate::validator_invariant_catalog) invariant_family_count: usize,
    pub(in crate::validator_invariant_catalog) supported_family_count: usize,
    pub(in crate::validator_invariant_catalog) unsupported_family_count: usize,
}
