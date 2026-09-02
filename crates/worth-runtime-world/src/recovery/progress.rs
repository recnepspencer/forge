/// Counts and byte accounting retained with a product-unpublished record.
/// These are Runtime World metadata metrics, not relabeled owner byte totals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ProductUnpublishedOwnerEffectSummary {
    pub(crate) owner_effect_count: usize,
    pub(crate) live_obligation_count: usize,
    pub(crate) metadata_bytes: usize,
}
