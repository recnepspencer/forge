use crate::history::data::CanonicalCommitEnvelope;
use crate::publication::patch::data::PatchStreamPosition;

pub(crate) fn select_execution_envelopes(
    source: &[CanonicalCommitEnvelope],
    start_after_position: Option<PatchStreamPosition>,
    max_commits: usize,
) -> Vec<CanonicalCommitEnvelope> {
    source
        .iter()
        .filter(|envelope| {
            start_after_position.is_none_or(|position| envelope.patch.position > position)
        })
        .take(max_commits)
        .cloned()
        .collect()
}
