use worth_ui::facade::inspection::{
    UiClientPhysicalRect, UiGeometryOnly, UiPixelsRequired, UiVisualCapturePoll,
    UiVisualPixelCaptureSource, UiVisualSnapshotOmission, UiVisualSnapshotOutcome,
    UiVisualSnapshotReceipt, UiVisualSnapshotRequest,
};
use worth_ui_runtime::facade::mounted::{
    UiMountedInspectionReceipt, UiMountedInspectionRequest, UiMountedVisualTargetDenial,
};
use worth_ui_test_support::WorthUiMountedIdentityCertificationExt;

use super::support::{capture_host, pixel_artifact, publish_one_frame, visual_transform};
use super::*;
use crate::mounted_application_lifecycle::in_flight_presentation_world::mounted_session_with_visual_policy;

#[test]
fn selected_mounted_node_is_the_only_public_node_target_authority() {
    let host = capture_host();
    let (mut session, _) = mounted_session(host.clone(), "visual-node-target", 1);
    publish_one_frame(&mut session, &host);
    let instance = session.inspect_mounted_identity().mounted_instances()[0].identity();
    let no_selection = match session.inspect_mounted_frame(UiMountedInspectionRequest::current()) {
        UiMountedInspectionReceipt::Available(frame) => frame,
        other => panic!("the current frame is inspectable, got {other:?}"),
    };
    assert!(matches!(
        no_selection.node_visual_target(),
        Err(UiMountedVisualTargetDenial::NodeSelectionRequired)
    ));
    let selected = match session
        .inspect_mounted_frame(UiMountedInspectionRequest::current().for_instance(instance))
    {
        UiMountedInspectionReceipt::Available(frame) => frame,
        other => panic!("the selected instance is retained, got {other:?}"),
    };
    let expected_receipt = selected
        .selected_node_receipt()
        .expect("instance selection resolves one mounted receipt");
    let target = selected
        .node_visual_target()
        .expect("a selected receipt on one surface seals a node target");
    assert_eq!(target.receipt(), expected_receipt);

    host.push_visual_capture(visual_transform(), None);
    let grant = session.visual_inspection_authority().issue_geometry_grant();
    let pending = session
        .begin_visual_geometry_snapshot(
            &grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(target)
                .artifacts(UiGeometryOnly::policy()),
        )
        .expect("the live node target admits a host-backed geometry capture");
    assert!(matches!(
        session.poll_visual_snapshot(pending, 0),
        UiVisualCapturePoll::Completed(UiVisualSnapshotOutcome::Captured(_))
    ));
    assert_eq!(host.visual_capture_calls().len(), 1);
}

#[test]
fn retained_region_crop_mints_child_provenance_without_host_recapture() {
    let host = capture_host();
    let (mut session, parent) = required_parent(&host, "visual-derived-crop");
    let parent_identity = parent.identity();
    let target = parent
        .into_client_region_target(|scope| {
            scope.client_region(UiClientPhysicalRect::new(1, 0, 2, 1).unwrap())
        })
        .expect("the crop lies inside the retained parent pixels");
    assert_eq!(target.snapshot(), parent_identity);
    let host_calls_before = host.visual_capture_calls().len();
    let grant = session.visual_inspection_authority().issue_pixel_grant();
    let pending = session
        .begin_visual_pixel_snapshot(
            &grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(target)
                .artifacts(UiPixelsRequired::policy()),
        )
        .expect("the retained one-pixel crop fits the admitted budget");
    let child = match session.poll_visual_snapshot(pending, 0) {
        UiVisualCapturePoll::Completed(UiVisualSnapshotOutcome::Captured(receipt)) => receipt,
        _ => panic!("retained crop completion is local and immediate"),
    };

    assert_eq!(host.visual_capture_calls().len(), host_calls_before);
    assert_derived_crop(parent_identity, &child);
    assert!(session
        .dispose_visual_snapshot(child)
        .released_registered_resource());
    assert!(matches!(
        session.inspect_mounted_frame(UiMountedInspectionRequest::current()),
        UiMountedInspectionReceipt::Available(_)
    ));
}

fn assert_derived_crop(
    parent_identity: worth_ui::facade::inspection::UiVisualSnapshotIdentity,
    child: &UiVisualSnapshotReceipt<UiPixelsRequired>,
) {
    assert_eq!(child.parent_snapshot(), Some(parent_identity));
    assert_eq!(
        child.captured_client_extent(),
        UiClientPhysicalRect::new(1, 0, 2, 1).unwrap()
    );
    assert_ne!(child.identity(), parent_identity);
    assert_eq!(child.pixel_artifact().dimensions(), [1, 1]);
    assert_eq!(child.pixel_artifact().stride(), 4);
    assert_eq!(child.pixel_artifact().bytes(), &[4, 5, 6, 255]);
    assert_eq!(
        child.pixel_artifact().capture_source(),
        UiVisualPixelCaptureSource::DerivedSnapshotCrop {
            parent_snapshot: parent_identity.diagnostic_value(),
            client_origin: [1, 0],
        }
    );
    assert_eq!(child.cost().pixel_bytes_requested(), 4);
    assert_eq!(child.cost().pixel_bytes_transferred(), 0);
    assert_eq!(child.cost().pixel_bytes_retained(), 4);
    assert_eq!(child.cost().coordinate_transforms(), 0);
    assert_eq!(
        child.visible_region_index_identity().diagnostic_value(),
        child.identity().diagnostic_value()
    );
    assert_eq!(
        child.hit_test_region_index_identity().diagnostic_value(),
        child.identity().diagnostic_value()
    );
    child.with_coordinate_scope(|scope| {
        let outside = worth_ui::facade::inspection::UiClientPhysicalPixel::new(0, 0).unwrap();
        assert!(matches!(
            scope.client_pixel(outside),
            Err(worth_ui::facade::inspection::UiVisualSnapshotDenial::OutsideCapturedPixelExtent)
        ));
    });
}

#[test]
fn required_region_pixels_are_typed_unavailable_when_parent_kept_only_geometry() {
    let host = capture_host();
    let (mut session, parent) = geometry_parent(&host, "visual-derived-no-pixels");
    let target = parent
        .into_client_region_target(|scope| {
            scope.client_region(UiClientPhysicalRect::new(0, 0, 1, 1).unwrap())
        })
        .unwrap();
    let host_calls_before = host.visual_capture_calls().len();
    let grant = session.visual_inspection_authority().issue_pixel_grant();
    let pending = session
        .begin_visual_pixel_snapshot(
            &grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(target)
                .artifacts(UiPixelsRequired::policy()),
        )
        .expect("geometry authority admits the derived attempt");
    assert!(matches!(
        session.poll_visual_snapshot(pending, 0),
        UiVisualCapturePoll::Completed(UiVisualSnapshotOutcome::Omitted(
            UiVisualSnapshotOmission::HistoricalPixelsUnavailable
        ))
    ));
    assert_eq!(host.visual_capture_calls().len(), host_calls_before);
}

#[test]
fn cancelling_a_derived_region_releases_parent_without_host_effect() {
    let host = capture_host();
    let (mut session, parent) = required_parent(&host, "visual-derived-cancel");
    let target = parent
        .into_client_region_target(|scope| {
            scope.client_region(UiClientPhysicalRect::new(0, 0, 1, 1).unwrap())
        })
        .unwrap();
    let host_calls_before = host.visual_capture_calls().len();
    let grant = session.visual_inspection_authority().issue_pixel_grant();
    let pending = session
        .begin_visual_pixel_snapshot(
            &grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(target)
                .artifacts(UiPixelsRequired::policy()),
        )
        .unwrap();
    let cancelled = session.cancel_visual_snapshot(pending);
    assert_eq!(
        cancelled.posture(),
        worth_ui::facade::inspection::UiVisualCancellationPosture::BeforeHostRequest
    );
    assert_eq!(host.visual_capture_calls().len(), host_calls_before);
}

fn required_parent(
    host: &ScriptedPresentationHost,
    label: &str,
) -> (
    worth_ui::facade::app::WorthUiActiveApplicationSession,
    UiVisualSnapshotReceipt<UiPixelsRequired>,
) {
    let policy = worth_ui::facade::inspection::UiVisualInspectionPolicy::bounded(
        worth_ui::facade::inspection::UiVisualInspectionDisclosure::local_development_unredacted(),
        worth_ui::facade::inspection::UiVisualInspectionCapacity::bounded(1, 32, 4_096),
        worth_ui::facade::inspection::UiVisualInspectionRegionCapacity::bounded(65_536, 65_536),
        worth_ui::facade::inspection::UiVisualInspectionByteBudget::bounded(
            1_024,
            2_048,
            64 << 20,
            64 << 20,
        ),
    )
    .expect("the derived transfer world admits exactly one snapshot");
    let (mut session, _) = mounted_session_with_visual_policy(host.clone(), label, 1, policy);
    assert_eq!(
        session
            .visual_inspection_authority()
            .policy()
            .maximum_snapshot_count(),
        1
    );
    publish_one_frame(&mut session, host);
    host.push_visual_capture(visual_transform(), Some(pixel_artifact()));
    let grant = session.visual_inspection_authority().issue_pixel_grant();
    let pending = session
        .begin_visual_pixel_snapshot(
            &grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(current_target(
                &session,
            ))
            .artifacts(UiPixelsRequired::policy()),
        )
        .unwrap();
    let receipt = match session.poll_visual_snapshot(pending, 0) {
        UiVisualCapturePoll::Completed(UiVisualSnapshotOutcome::Captured(receipt)) => receipt,
        _ => panic!("the scripted parent capture completes"),
    };
    (session, receipt)
}

fn geometry_parent(
    host: &ScriptedPresentationHost,
    label: &str,
) -> (
    worth_ui::facade::app::WorthUiActiveApplicationSession,
    UiVisualSnapshotReceipt<UiGeometryOnly>,
) {
    let (mut session, _) = mounted_session(host.clone(), label, 1);
    publish_one_frame(&mut session, host);
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
        .unwrap();
    let receipt = match session.poll_visual_snapshot(pending, 0) {
        UiVisualCapturePoll::Completed(UiVisualSnapshotOutcome::Captured(receipt)) => receipt,
        _ => panic!("the scripted geometry parent capture completes"),
    };
    (session, receipt)
}
