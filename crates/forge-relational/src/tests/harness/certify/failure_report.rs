use crate::facade::publication::RelationalPatchRecord;
use crate::tests::harness::observe::subscriber_stream::SubscriberView;

pub(super) fn render_window_matrix_failure(
    context: &str,
    window_size: usize,
    expected: &[RelationalPatchRecord],
    actual: &[RelationalPatchRecord],
) -> String {
    format!(
        "{context}: window size {window_size} diverged; expected {} patches with tail {:?}, got {} patches with tail {:?}",
        expected.len(),
        expected.last().map(|patch| patch.position),
        actual.len(),
        actual.last().map(|patch| patch.position)
    )
}

pub(super) fn render_multi_subscriber_failure(
    context: &str,
    view: &SubscriberView,
    expected: &[RelationalPatchRecord],
) -> String {
    format!(
        "{context}: subscriber checkpoint {:?} window {} diverged; expected {} patches with tail {:?}, got {} patches with tail {:?}",
        view.checkpoint.as_ref().map(|checkpoint| checkpoint.position()),
        view.window_size,
        expected.len(),
        expected.last().map(|patch| patch.position),
        view.patches.len(),
        view.patches.last().map(|patch| patch.position)
    )
}
