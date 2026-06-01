use super::failure_report::{render_multi_subscriber_failure, render_window_matrix_failure};
use crate::facade::publication::{PublishedAuthoritativePatchEnvelope, SubscriberCheckpoint};
use crate::tests::harness::model::truth_model::VisibleTruthSummary;
use crate::tests::harness::observe::subscriber_stream::SubscriberView;

pub(crate) fn assert_window_matrix_matches(
    context: &str,
    expected: &[PublishedAuthoritativePatchEnvelope],
    matrix: &[(usize, Vec<PublishedAuthoritativePatchEnvelope>)],
) {
    for (window_size, patches) in matrix {
        assert!(
            patches == expected,
            "{}",
            render_window_matrix_failure(context, *window_size, expected, patches)
        );
    }
}

pub(crate) fn assert_multi_subscriber_converges(
    context: &str,
    views: &[SubscriberView],
    expected_from_head: &[PublishedAuthoritativePatchEnvelope],
) {
    for view in views {
        let expected = match &view.checkpoint {
            None => expected_from_head.to_vec(),
            Some(checkpoint) => suffix_after_checkpoint(expected_from_head, checkpoint),
        };
        assert!(
            view.patches == expected,
            "{}",
            render_multi_subscriber_failure(context, view, &expected)
        );
    }
}

pub(crate) fn assert_visible_truth_matches(
    context: &str,
    expected: &VisibleTruthSummary,
    actual: &VisibleTruthSummary,
) {
    assert_eq!(
        actual, expected,
        "{context} visible truth drifted\nexpected: {expected:#?}\nactual: {actual:#?}"
    );
}

fn suffix_after_checkpoint(
    patches: &[PublishedAuthoritativePatchEnvelope],
    checkpoint: &SubscriberCheckpoint,
) -> Vec<PublishedAuthoritativePatchEnvelope> {
    patches
        .iter()
        .filter(|patch| patch.position.0 > checkpoint.position().0)
        .cloned()
        .collect()
}
