use worth_ui::facade::inspection::{
    UiGeometryOnly, UiPixelsRequired, UiVisualCapturePoll, UiVisualInspectionByteBudget,
    UiVisualInspectionCapacity, UiVisualInspectionDisclosure, UiVisualInspectionPolicy,
    UiVisualInspectionRegionCapacity, UiVisualSnapshotDenial, UiVisualSnapshotOutcome,
    UiVisualSnapshotRequest,
};

use super::support::{
    capture_host, current_target, pixel_artifact, publish_one_frame, visual_transform,
};
use crate::mounted_application_lifecycle::in_flight_presentation_world::mounted_session_with_visual_policy;

#[test]
fn retained_pixel_capacity_denies_a_second_capture_before_host_effects() {
    let policy = policy(2, 12);
    let host = capture_host();
    let (mut session, _) =
        mounted_session_with_visual_policy(host.clone(), "visual-pixel-capacity", 1, policy);
    publish_one_frame(&mut session, &host);
    let first = complete_pixel_capture(&mut session, &host);
    let host_calls_before = host.visual_capture_calls().len();
    let grant = session.visual_inspection_authority().issue_pixel_grant();
    let denial = session
        .begin_visual_pixel_snapshot(
            &grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(current_target(
                &session,
            ))
            .artifacts(UiPixelsRequired::policy()),
        )
        .err();

    assert_eq!(
        denial,
        Some(UiVisualSnapshotDenial::RetainedPixelCapacityExceeded)
    );
    assert_eq!(host.visual_capture_calls().len(), host_calls_before);
    assert!(session
        .dispose_visual_snapshot(first)
        .released_registered_resource());
    let _ = session.shutdown();
}

#[test]
fn snapshot_count_capacity_denies_after_one_retained_capture() {
    let policy = policy(1, 1_024);
    let host = capture_host();
    let (mut session, _) =
        mounted_session_with_visual_policy(host.clone(), "visual-snapshot-capacity", 1, policy);
    publish_one_frame(&mut session, &host);
    host.push_visual_capture(visual_transform(), None);
    let grant = session.visual_inspection_authority().issue_geometry_grant();
    let pending = session
        .begin_visual_geometry_snapshot(
            &grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(current_target(
                &session,
            ))
            .artifacts(UiGeometryOnly::policy()),
        )
        .expect("the first snapshot is admitted");
    let first = match session.poll_visual_snapshot(pending, 0) {
        UiVisualCapturePoll::Completed(UiVisualSnapshotOutcome::Captured(receipt)) => receipt,
        _ => panic!("the first geometry capture completes"),
    };
    let host_calls_before = host.visual_capture_calls().len();
    let denial = session
        .begin_visual_geometry_snapshot(
            &grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(current_target(
                &session,
            ))
            .artifacts(UiGeometryOnly::policy()),
        )
        .err();

    assert_eq!(
        denial,
        Some(UiVisualSnapshotDenial::SnapshotCapacityExceeded)
    );
    assert_eq!(host.visual_capture_calls().len(), host_calls_before);
    assert!(session
        .dispose_visual_snapshot(first)
        .released_registered_resource());
    let _ = session.shutdown();
}

fn complete_pixel_capture(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    host: &super::ScriptedPresentationHost,
) -> worth_ui::facade::inspection::UiVisualSnapshotReceipt<UiPixelsRequired> {
    host.push_visual_capture(visual_transform(), Some(pixel_artifact()));
    let grant = session.visual_inspection_authority().issue_pixel_grant();
    let pending = session
        .begin_visual_pixel_snapshot(
            &grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(current_target(
                session,
            ))
            .artifacts(UiPixelsRequired::policy()),
        )
        .expect("the first retained pixel capture is admitted");
    match session.poll_visual_snapshot(pending, 0) {
        UiVisualCapturePoll::Completed(UiVisualSnapshotOutcome::Captured(receipt)) => receipt,
        _ => panic!("the scripted retained pixel capture completes"),
    }
}

fn policy(snapshot_count: u8, retained_pixels: u64) -> UiVisualInspectionPolicy {
    UiVisualInspectionPolicy::bounded(
        UiVisualInspectionDisclosure::local_development_unredacted(),
        UiVisualInspectionCapacity::bounded(snapshot_count, 32, 4_096),
        UiVisualInspectionRegionCapacity::bounded(65_536, 65_536),
        UiVisualInspectionByteBudget::bounded(1_024, retained_pixels, 64 << 20, 256 << 20),
    )
    .expect("the resource-bound policy is valid")
}
