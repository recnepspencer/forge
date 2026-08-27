use crate::history::data::PositionedCanonicalCommit;
use crate::publication::patch::data::PatchStreamPosition;

pub(crate) fn select_execution_envelopes(
    source: &[PositionedCanonicalCommit],
    start_after_position: Option<PatchStreamPosition>,
    max_commits: usize,
) -> Vec<PositionedCanonicalCommit> {
    source
        .iter()
        .filter(|envelope| {
            start_after_position.is_none_or(|position| envelope.position() > position)
        })
        .take(max_commits)
        .cloned()
        .collect()
}
