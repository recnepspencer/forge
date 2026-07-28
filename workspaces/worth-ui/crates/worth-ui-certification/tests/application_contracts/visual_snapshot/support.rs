use worth_ui::facade::inspection::{
    UiCurrentPresentedSurfaceTarget, UiPixelsRequired, UiVisualCapturePoll,
    UiVisualSnapshotOutcome, UiVisualSnapshotReceipt, UiVisualSnapshotRequest,
};
use worth_ui_runtime::facade::mounted::{
    UiMountedInspectionReceipt, UiMountedInspectionRequest, UiPresentationDeadline,
};
use worth_ui_test_support::WorthUiMountedPublicationCertificationExt;

use super::{mounted_session, prepared, ScriptedPresentationHost};

pub(super) fn current_target(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
) -> UiCurrentPresentedSurfaceTarget {
    match session.inspect_mounted_frame(UiMountedInspectionRequest::current()) {
        UiMountedInspectionReceipt::Available(frame) => frame
            .current_visual_target()
            .expect("one current presented surface is unambiguous"),
        other => panic!("the current frame is inspectable, got {other:?}"),
    }
}

pub(super) fn immediate_pixel_capture(
    label: &str,
) -> (
    worth_ui::facade::app::WorthUiActiveApplicationSession,
    UiVisualSnapshotReceipt<UiPixelsRequired>,
) {
    let host = capture_host();
    let (mut session, _) = mounted_session(host.clone(), label, 1);
    publish_one_frame(&mut session, &host);
    let target = current_target(&session);
    let grant = session.visual_inspection_authority().issue_pixel_grant();
    host.push_visual_capture(visual_transform(), Some(pixel_artifact()));
    let pending = session
        .begin_visual_pixel_snapshot(
            &grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(target)
                .artifacts(UiPixelsRequired::policy())
                .deadline(worth_ui::facade::inspection::UiVisualCaptureDeadline::at_tick(5)),
        )
        .expect("the capture registers");
    let receipt = match session.poll_visual_snapshot(pending, 1) {
        UiVisualCapturePoll::Completed(UiVisualSnapshotOutcome::Captured(receipt)) => receipt,
        _ => panic!("the scripted capture completes"),
    };
    (session, receipt)
}

pub(super) fn complete_required_pixel_capture(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    host: &ScriptedPresentationHost,
    target: UiCurrentPresentedSurfaceTarget,
) -> UiVisualSnapshotReceipt<UiPixelsRequired> {
    let grant = session.visual_inspection_authority().issue_pixel_grant();
    host.push_visual_capture_pending();
    host.push_visual_capture(visual_transform(), Some(pixel_artifact()));
    let pending = session
        .begin_visual_pixel_snapshot(
            &grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(target)
                .artifacts(UiPixelsRequired::policy())
                .deadline(worth_ui::facade::inspection::UiVisualCaptureDeadline::at_tick(20)),
        )
        .expect("the exact retained target is admitted");
    let pending = match session.poll_visual_snapshot(pending, 1) {
        UiVisualCapturePoll::Pending(next) => next,
        UiVisualCapturePoll::Completed(_) => panic!("the first host observation is pending"),
    };
    assert_eq!(
        session
            .mounted_retention_report()
            .class(worth_ui_runtime::facade::mounted::UiMountedRetentionClass::VisualSnapshot)
            .active_leases(),
        1
    );
    match session.poll_visual_snapshot(pending, 2) {
        UiVisualCapturePoll::Completed(UiVisualSnapshotOutcome::Captured(receipt)) => receipt,
        _ => panic!("the second host observation captures"),
    }
}

pub(super) fn pending_host_capture(
    label: &str,
) -> (
    worth_ui::facade::app::WorthUiActiveApplicationSession,
    ScriptedPresentationHost,
    worth_ui::facade::inspection::UiPendingVisualCapture<
        UiCurrentPresentedSurfaceTarget,
        UiPixelsRequired,
    >,
) {
    let host = capture_host();
    let (mut session, _) = mounted_session(host.clone(), label, 1);
    publish_one_frame(&mut session, &host);
    let target = current_target(&session);
    let grant = session.visual_inspection_authority().issue_pixel_grant();
    host.push_visual_capture_pending();
    let pending = session
        .begin_visual_pixel_snapshot(
            &grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(target)
                .artifacts(UiPixelsRequired::policy())
                .deadline(worth_ui::facade::inspection::UiVisualCaptureDeadline::at_tick(5)),
        )
        .expect("the capture registers");
    let pending = match session.poll_visual_snapshot(pending, 1) {
        UiVisualCapturePoll::Pending(next) => next,
        UiVisualCapturePoll::Completed(_) => panic!("the scripted host remains pending"),
    };
    (session, host, pending)
}

pub(super) fn visual_transform() -> worth_ui_host_contract::UiHostCoordinateTransform {
    worth_ui_host_contract::UiHostCoordinateTransform::observed_by_host(
        worth_ui_host_contract::UiHostClientAreaObservation::observed_by_host([17, 23], [2, 1]),
        worth_ui_host_contract::UiHostViewportTransformObservation::observed_by_host(
            [1.25, 0.5],
            [1.6, 2.0],
            [0.25, 0.5],
        ),
        worth_ui_host_contract::UiHostCoordinatePosture::observed_by_host(
            worth_ui_host_contract::UiHostCoordinateOrientation::TopLeftOrigin,
            worth_ui_host_contract::UiHostCoordinateRounding::PixelCenterNearest,
        ),
    )
}

pub(super) fn capture_host() -> ScriptedPresentationHost {
    let host = ScriptedPresentationHost::default();
    host.set_visual_capture_capability(worth_ui_host_contract::UiHostCaptureCapability::Pixels {
        maximum_bytes: 1_024,
        exact_presentation_epoch: true,
    });
    host
}

pub(super) fn publish_one_frame(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    host: &ScriptedPresentationHost,
) {
    host.push_presented();
    let frame = prepared(session);
    let _ = session.present_prepared_mounted_frame(frame, UiPresentationDeadline::at_tick(10), 0);
}

pub(super) fn pixel_artifact() -> worth_ui_host_contract::UiHostPixelArtifact {
    worth_ui_host_contract::UiHostPixelArtifact::copied_by_host(
        [2, 1],
        8,
        vec![1, 2, 3, 255, 4, 5, 6, 255].into_boxed_slice(),
        worth_ui_host_contract::UiHostPixelColorSpace::Srgb,
    )
}
