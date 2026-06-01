use crate::identity::data::{VersionBound, VersionId};
use crate::storage::logic::state::HistoricalMetadata;

pub(in super::super) fn visible_metadata<M: HistoricalMetadata>(
    history: &[M],
    version_id: VersionId,
) -> Option<&M> {
    let bound = VersionBound::new(version_id);
    let end = history.partition_point(|entry| bound.includes_created(entry.effective_at()));
    history[..end].iter().rev().find(|entry| {
        bound.includes_created(entry.effective_at())
            && entry
                .retired_at()
                .is_none_or(|retired| bound.retains_retired(retired))
    })
}
