/// Counts and byte accounting retained with a product-unpublished record.
/// These are Runtime World metadata metrics, not relabeled owner byte totals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ProductUnpublishedOwnerEffectSummary {
    pub(crate) owner_effect_count: usize,
    pub(crate) live_obligation_count: usize,
    pub(crate) metadata_bytes: usize,
}

impl ProductUnpublishedOwnerEffectSummary {
    pub(crate) fn from_progress(
        progress: &crate::publication::CompositeAttemptProgress,
        live_obligation_count: usize,
        metadata_bytes: usize,
    ) -> Self {
        Self {
            owner_effect_count: progress.owner_effect_count(),
            live_obligation_count,
            metadata_bytes,
        }
    }
}
