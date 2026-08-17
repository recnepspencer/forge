use std::collections::HashSet;

use worth_ui_host_contract::{UiMountedInstanceIdentity, UiSemanticSurfaceIdentity};

pub(super) fn assert_exact_attribution(
    transcript: &worth_ui_host_headless::UiHeadlessMountedFrameTranscript,
    expected_instances: &[UiMountedInstanceIdentity],
    expected_surface: UiSemanticSurfaceIdentity,
) {
    let rows = ordered_rows(transcript);
    assert_eq!(rows.len(), expected_instances.len());
    for (row, expected_instance) in rows.iter().zip(expected_instances) {
        assert_eq!(row.mounted_instance(), *expected_instance);
    }
    assert_internal_attribution(transcript, &rows, expected_surface);
}

pub(super) fn assert_initial_attribution(
    transcript: &worth_ui_host_headless::UiHeadlessMountedFrameTranscript,
    expected_instances: &[UiMountedInstanceIdentity],
    expected_surface: UiSemanticSurfaceIdentity,
) {
    assert_exact_attribution(transcript, expected_instances, expected_surface);
    for row in transcript.filled_rects() {
        assert_eq!(row.frame(), transcript.frame());
    }
}

pub(super) fn assert_exact_frame_attribution(
    transcript: &worth_ui_host_headless::UiHeadlessMountedFrameTranscript,
    expected_frames: &[worth_ui_host_contract::UiMountedFrameIdentity],
) {
    let rows = ordered_rows(transcript);
    assert_eq!(rows.len(), expected_frames.len());
    for (row, expected_frame) in rows.iter().zip(expected_frames) {
        assert_eq!(row.frame(), *expected_frame);
    }
}

fn ordered_rows(
    transcript: &worth_ui_host_headless::UiHeadlessMountedFrameTranscript,
) -> Vec<worth_ui_host_headless::UiHeadlessFilledRectMechanic> {
    let mut rows = transcript.filled_rects().to_vec();
    rows.sort_by_key(|row| row.layer_semantic_order());
    rows
}

fn assert_internal_attribution(
    transcript: &worth_ui_host_headless::UiHeadlessMountedFrameTranscript,
    rows: &[worth_ui_host_headless::UiHeadlessFilledRectMechanic],
    expected_surface: UiSemanticSurfaceIdentity,
) {
    let mut commands = HashSet::with_capacity(rows.len());
    let mut instances = HashSet::with_capacity(rows.len());
    let mut receipts = HashSet::with_capacity(rows.len());
    for row in rows {
        assert_eq!(row.surface(), expected_surface);
        assert_eq!(row.binding(), transcript.binding());
        assert_eq!(
            row.command_identity().mounted_instance(),
            row.mounted_instance()
        );
        assert_eq!(
            row.node_receipt().mounted_instance(),
            row.mounted_instance()
        );
        assert!(
            commands.insert(row.command_identity()),
            "duplicate command attribution"
        );
        assert!(
            instances.insert(row.mounted_instance()),
            "duplicate mounted attribution"
        );
        assert!(
            receipts.insert(row.node_receipt()),
            "duplicate receipt attribution"
        );
    }
}
