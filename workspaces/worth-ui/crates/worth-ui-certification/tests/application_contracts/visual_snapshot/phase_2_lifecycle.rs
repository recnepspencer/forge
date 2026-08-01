use super::support::{current_target, pixel_artifact, publish_one_frame, visual_transform};
use super::*;

#[test]
fn current_target_requires_one_unambiguous_presented_surface() {
    let host = ScriptedPresentationHost::default();
    let (mut session, _) = mounted_session(host.clone(), "visual-two-surface-target", 2);
    host.push_presented();
    host.push_presented();
    let frame = prepared(&mut session);
    let outcome =
        session.present_prepared_mounted_frame(frame, UiPresentationDeadline::at_tick(10), 0);
    assert!(matches!(outcome, UiMountedFrameOutcome::Published(_)));
    let inspected = match session.inspect_mounted_frame(UiMountedInspectionRequest::current()) {
        UiMountedInspectionReceipt::Available(frame) => frame,
        other => panic!("two-surface frame remains inspectable, got {other:?}"),
    };
    assert!(matches!(
        inspected.current_visual_target(),
        Err(worth_ui_runtime::facade::mounted::UiMountedVisualTargetDenial::SurfaceSelectionRequired)
    ));
}

#[test]
fn foreign_session_grant_denies_before_host_capture() {
    let host_a = pixel_host();
    let host_b = pixel_host();
    let (mut session_a, _) = mounted_session(host_a.clone(), "visual-grant-a", 1);
    let (mut session_b, _) = mounted_session(host_b.clone(), "visual-grant-b", 1);
    publish_one_frame(&mut session_a, &host_a);
    publish_one_frame(&mut session_b, &host_b);
    let foreign_grant = session_a.visual_inspection_authority().issue_pixel_grant();
    let target_b = current_target(&session_b);

    let result = session_b.begin_visual_pixel_snapshot(
        &foreign_grant,
        UiVisualSnapshotRequest::for_local_development_unredacted_frame(target_b)
            .artifacts(UiPixelsRequired::policy()),
    );
    assert!(matches!(
        result,
        Err(UiVisualSnapshotDenial::ForeignSession)
    ));
    assert!(host_b.visual_capture_calls().is_empty());
}

#[test]
fn snapshot_and_overlay_leases_use_independent_retention_classes() {
    let host = pixel_host();
    let (mut session, _) = mounted_session(host.clone(), "visual-independent-leases", 1);
    publish_one_frame(&mut session, &host);
    let overlay_target = current_target(&session);
    let overlay = session
        .acquire_visual_overlay_lease(overlay_target.frame(), overlay_target.binding())
        .expect("the overlay class admits its own lease");
    assert_eq!(overlay.frame(), overlay_target.frame());
    assert!(overlay.structural_bytes() > 0);

    let grant = session.visual_inspection_authority().issue_pixel_grant();
    let pending = session
        .begin_visual_pixel_snapshot(
            &grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(current_target(
                &session,
            ))
            .artifacts(UiPixelsRequired::policy()),
        )
        .expect("the snapshot class independently admits a lease");
    let report = session.mounted_retention_report();
    assert_eq!(
        report
            .class(worth_ui_runtime::facade::mounted::UiMountedRetentionClass::VisualSnapshot)
            .active_leases(),
        1
    );
    assert_eq!(
        report
            .class(worth_ui_runtime::facade::mounted::UiMountedRetentionClass::VisualOverlay)
            .active_leases(),
        1
    );

    drop(overlay);
    let _ = session.cancel_visual_snapshot(pending);
    assert_eq!(snapshot_leases(&session), 0);
    assert_eq!(
        session
            .mounted_retention_report()
            .class(worth_ui_runtime::facade::mounted::UiMountedRetentionClass::VisualOverlay)
            .active_leases(),
        0
    );
}

#[test]
fn retained_snapshot_reports_its_live_relation_without_a_hit_test_target() {
    let host = pixel_host();
    let (mut session, _) = mounted_session(host.clone(), "visual-live-relation", 1);
    publish_one_frame(&mut session, &host);
    let grant = session.visual_inspection_authority().issue_pixel_grant();
    host.push_visual_capture(visual_transform(), Some(pixel_artifact()));
    let pending = session
        .begin_visual_pixel_snapshot(
            &grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(current_target(
                &session,
            ))
            .artifacts(UiPixelsRequired::policy()),
        )
        .expect("current capture is admitted");
    let receipt = match session.poll_visual_snapshot(pending, 0) {
        UiVisualCapturePoll::Completed(UiVisualSnapshotOutcome::Captured(receipt)) => receipt,
        _ => panic!("scripted current capture completes"),
    };
    assert_eq!(
        receipt.relation(),
        Ok(worth_ui::facade::inspection::UiVisualSnapshotRelation::Current)
    );

    publish_one_frame(&mut session, &host);
    assert_eq!(
        receipt.relation(),
        Ok(worth_ui::facade::inspection::UiVisualSnapshotRelation::RetainedPredecessor)
    );
}

#[test]
fn copied_predecessor_completes_with_retained_relation_after_successor_publication() {
    let host = pixel_host();
    let (mut session, _) = mounted_session(host.clone(), "visual-copy-before-successor", 1);
    publish_one_frame(&mut session, &host);
    let predecessor = current_target(&session).frame();
    let grant = session.visual_inspection_authority().issue_pixel_grant();
    host.push_visual_capture_pending();
    host.push_visual_capture(visual_transform(), Some(pixel_artifact()));
    let pending = session
        .begin_visual_pixel_snapshot(
            &grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(current_target(
                &session,
            ))
            .artifacts(UiPixelsRequired::policy()),
        )
        .expect("predecessor capture is admitted");
    let pending = expect_pending(session.poll_visual_snapshot(pending, 0));

    publish_one_frame(&mut session, &host);
    let successor = current_target(&session).frame();
    assert_ne!(predecessor, successor);
    let receipt = match session.poll_visual_snapshot(pending, 1) {
        UiVisualCapturePoll::Completed(UiVisualSnapshotOutcome::Captured(receipt)) => receipt,
        _ => panic!("the pre-copied observation completes exactly"),
    };
    assert_eq!(receipt.affinity().frame(), predecessor.diagnostic_value());
    assert_eq!(
        receipt.affinity().relation(),
        worth_ui::facade::inspection::UiVisualSnapshotRelation::RetainedPredecessor
    );
    assert_eq!(receipt.pixel_artifact().bytes(), pixel_artifact().bytes());
    assert_eq!(current_target(&session).frame(), successor);
}

#[test]
fn successor_before_copy_returns_superseded_without_receipt() {
    let host = pixel_host();
    let (mut session, _) = mounted_session(host.clone(), "visual-successor-before-copy", 1);
    publish_one_frame(&mut session, &host);
    let grant = session.visual_inspection_authority().issue_pixel_grant();
    host.push_visual_capture_pending();
    let pending = session
        .begin_visual_pixel_snapshot(
            &grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(current_target(
                &session,
            ))
            .artifacts(UiPixelsRequired::policy()),
        )
        .expect("predecessor capture is admitted");
    let pending = expect_pending(session.poll_visual_snapshot(pending, 0));

    publish_one_frame(&mut session, &host);
    host.push_visual_capture_superseded();
    let outcome = match session.poll_visual_snapshot(pending, 1) {
        UiVisualCapturePoll::Completed(outcome) => outcome,
        UiVisualCapturePoll::Pending(_) => panic!("supersession is terminal"),
    };
    assert!(matches!(
        outcome,
        UiVisualSnapshotOutcome::Superseded(value)
            if !value.predecessor_artifact_copied()
    ));
}

#[test]
fn snapshot_cost_and_disposal_leave_mounted_publication_live() {
    let host = pixel_host();
    let (mut session, _) = mounted_session(host.clone(), "visual-dispose-keeps-mounted", 1);
    publish_one_frame(&mut session, &host);
    let receipt = immediate_required_capture(&mut session, &host);
    let cost = receipt.cost();
    assert_eq!(cost.pixel_bytes_requested(), 8);
    assert_eq!(cost.pixel_bytes_transferred(), 8);
    assert_eq!(cost.pixel_bytes_retained(), 8);
    assert_eq!(cost.coordinate_transforms(), 1);
    assert_eq!(cost.lease_count(), 1);
    assert!(cost.retained_structural_bytes() > 0);
    assert_eq!(
        worth_ui::facade::inspection::UiVisualInspectionCostReceipt::default().counters(),
        [0; 11]
    );

    let disposal = session.dispose_visual_snapshot(receipt);
    assert!(disposal.released_registered_resource());
    assert_eq!(snapshot_leases(&session), 0);
    assert!(matches!(
        session.inspect_mounted_frame(UiMountedInspectionRequest::current()),
        UiMountedInspectionReceipt::Available(_)
    ));
    publish_one_frame(&mut session, &host);
    assert!(matches!(
        session.inspect_mounted_frame(UiMountedInspectionRequest::current()),
        UiMountedInspectionReceipt::Available(_)
    ));
}

fn pixel_host() -> ScriptedPresentationHost {
    let host = ScriptedPresentationHost::default();
    host.set_visual_capture_capability(worth_ui_host_contract::UiHostCaptureCapability::Pixels {
        maximum_bytes: 1_024,
        exact_presentation_epoch: true,
    });
    host
}

fn expect_pending<Target, Policy>(
    poll: UiVisualCapturePoll<Target, Policy>,
) -> worth_ui::facade::inspection::UiPendingVisualCapture<Target, Policy>
where
    Target: worth_ui::facade::inspection::UiVisualTarget,
    Policy: UiVisualArtifactPolicy,
{
    match poll {
        UiVisualCapturePoll::Pending(pending) => pending,
        UiVisualCapturePoll::Completed(_) => panic!("the scripted first poll remains pending"),
    }
}

fn immediate_required_capture(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    host: &ScriptedPresentationHost,
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
        .expect("capture is admitted");
    match session.poll_visual_snapshot(pending, 0) {
        UiVisualCapturePoll::Completed(UiVisualSnapshotOutcome::Captured(receipt)) => receipt,
        _ => panic!("the scripted capture completes immediately"),
    }
}

fn snapshot_leases(session: &worth_ui::facade::app::WorthUiActiveApplicationSession) -> usize {
    session
        .mounted_retention_report()
        .class(worth_ui_runtime::facade::mounted::UiMountedRetentionClass::VisualSnapshot)
        .active_leases()
}
