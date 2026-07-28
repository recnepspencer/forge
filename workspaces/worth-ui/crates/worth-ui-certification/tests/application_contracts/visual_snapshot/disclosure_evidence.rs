use worth_ui::facade::inspection::{
    UiPixelsRequired, UiVisualCapturePoll, UiVisualInspectionAudience,
    UiVisualInspectionByteBudget, UiVisualInspectionCapacity, UiVisualInspectionDisclosure,
    UiVisualInspectionPolicy, UiVisualInspectionRegionCapacity, UiVisualPixelCaptureSource,
    UiVisualPixelRedaction, UiVisualSnapshotArtifactPosture, UiVisualSnapshotDenial,
    UiVisualSnapshotEvidence, UiVisualSnapshotOutcome, UiVisualSnapshotRequest,
};

use super::support::{
    capture_host, current_target, pixel_artifact, publish_one_frame, visual_transform,
};
use crate::mounted_application_lifecycle::in_flight_presentation_world::{
    mounted_session, mounted_session_with_visual_policy,
};

#[test]
fn disclosure_mismatch_denies_before_capture_or_retention_effects() {
    let host = capture_host();
    let (mut session, _) = mounted_session(host.clone(), "visual-disclosure-denial", 1);
    publish_one_frame(&mut session, &host);
    let target = current_target(&session);
    let grant = session.visual_inspection_authority().issue_pixel_grant();
    let request = UiVisualSnapshotRequest::for_frame(
        target,
        UiVisualInspectionDisclosure::redacted(UiVisualInspectionAudience::LocalDevelopment),
    )
    .artifacts(UiPixelsRequired::policy());

    assert_eq!(
        session.begin_visual_pixel_snapshot(&grant, request).err(),
        Some(UiVisualSnapshotDenial::Disclosure)
    );
    assert!(host.visual_capture_calls().is_empty());
    assert_eq!(
        session
            .mounted_retention_report()
            .class(worth_ui_runtime::facade::mounted::UiMountedRetentionClass::VisualSnapshot)
            .active_leases(),
        0
    );
    let shutdown = session.shutdown();
    assert_eq!(shutdown.visual_capture().cancelled_capture_count(), 0);
    assert_eq!(shutdown.visual_capture().disposed_snapshot_count(), 0);
}

#[test]
fn redacted_capture_seals_explicit_derived_pixels_and_immutable_evidence() {
    let disclosure =
        UiVisualInspectionDisclosure::redacted(UiVisualInspectionAudience::DiagnosticAgent);
    let policy = UiVisualInspectionPolicy::bounded(
        disclosure,
        UiVisualInspectionCapacity::bounded(2, 8, 16),
        UiVisualInspectionRegionCapacity::bounded(65_536, 65_536),
        UiVisualInspectionByteBudget::bounded(1_024, 2_048, 64 << 10, 128 << 10),
    )
    .expect("the redacted scenario policy is valid");
    let host = capture_host();
    let (mut session, _) =
        mounted_session_with_visual_policy(host.clone(), "visual-redacted-evidence", 1, policy);
    publish_one_frame(&mut session, &host);
    let target = current_target(&session);
    let grant = session.visual_inspection_authority().issue_pixel_grant();
    host.push_visual_capture(visual_transform(), Some(pixel_artifact()));
    let pending = session
        .begin_visual_pixel_snapshot(
            &grant,
            UiVisualSnapshotRequest::for_frame(target, disclosure)
                .artifacts(UiPixelsRequired::policy()),
        )
        .expect("matching redacted disclosure admits capture");
    let receipt = match session.poll_visual_snapshot(pending, 0) {
        UiVisualCapturePoll::Completed(UiVisualSnapshotOutcome::Captured(receipt)) => receipt,
        _ => panic!("the scripted redacted capture completes"),
    };

    assert_eq!(
        receipt.pixel_artifact().capture_source(),
        UiVisualPixelCaptureSource::RedactedNativePresentation
    );
    assert_eq!(
        receipt.pixel_artifact().redaction(),
        UiVisualPixelRedaction::OpaqueBlack
    );
    assert_eq!(
        receipt.pixel_artifact().bytes(),
        &[0, 0, 0, 255, 0, 0, 0, 255]
    );
    assert_snapshot_evidence(&receipt, disclosure);

    let disposed = session.dispose_visual_snapshot(receipt);
    assert!(disposed.released_registered_resource());
    assert_eq!(
        session
            .shutdown()
            .visual_capture()
            .disposed_snapshot_count(),
        0
    );
}

fn assert_snapshot_evidence(
    receipt: &worth_ui::facade::inspection::UiVisualSnapshotReceipt<UiPixelsRequired>,
    disclosure: UiVisualInspectionDisclosure,
) {
    let evidence = receipt.evidence();
    assert_eq!(
        evidence.schema_version(),
        UiVisualSnapshotEvidence::SCHEMA_VERSION
    );
    assert_eq!(evidence.affinity(), receipt.affinity());
    assert_eq!(evidence.coordinates(), receipt.coordinates());
    assert_eq!(
        evidence.visible_index(),
        receipt.visible_region_index_identity()
    );
    assert_eq!(
        evidence.hit_test_index(),
        receipt.hit_test_region_index_identity()
    );
    assert_eq!(
        evidence.artifact(),
        UiVisualSnapshotArtifactPosture::PixelsRequiredCaptured
    );
    assert_eq!(evidence.disclosure(), disclosure);
    assert_eq!(evidence.cost(), receipt.cost());
}
