use super::support::{current_target, pixel_artifact, publish_one_frame, visual_transform};
use super::*;

type ActiveSession = worth_ui::facade::app::WorthUiActiveApplicationSession;
type CurrentTarget = worth_ui::facade::inspection::UiCurrentPresentedSurfaceTarget;

#[test]
fn deadline_elapsed_before_host_request_has_no_host_effect() {
    let (mut session, host, target) = capture_world("visual-deadline-before-effect");
    let grant = session.visual_inspection_authority().issue_pixel_grant();
    let pending = session
        .begin_visual_pixel_snapshot(
            &grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(target)
                .artifacts(UiPixelsRequired::policy())
                .deadline(UiVisualCaptureDeadline::at_tick(0)),
        )
        .expect("capture admission itself is within capacity");

    let outcome = completed(session.poll_visual_snapshot(pending, 1));
    assert!(matches!(
        outcome,
        UiVisualSnapshotOutcome::Denied(UiVisualSnapshotDenial::DeadlineAlreadyElapsed)
    ));
    assert!(host.visual_capture_calls().is_empty());
    assert_eq!(visual_snapshot_leases(&session), 0);
}

#[test]
fn artifact_policy_matrix_preserves_geometry_optional_and_required_truth() {
    let (mut geometry_session, geometry_host, geometry_target) =
        capture_world("visual-geometry-only");
    geometry_host.set_visual_capture_capability(
        worth_ui_host_contract::UiHostCaptureCapability::GeometryOnly,
    );
    geometry_host.push_visual_capture(visual_transform(), None);
    let geometry_grant = geometry_session
        .visual_inspection_authority()
        .issue_geometry_grant();
    let geometry_pending = geometry_session
        .begin_visual_geometry_snapshot(
            &geometry_grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(geometry_target),
        )
        .expect("geometry capture is supported");
    assert!(matches!(
        completed(geometry_session.poll_visual_snapshot(geometry_pending, 0)),
        UiVisualSnapshotOutcome::Captured(_)
    ));

    let (mut optional_session, optional_host, optional_target) =
        capture_world("visual-optional-pixels");
    optional_host.push_visual_capture(visual_transform(), None);
    let optional_grant = optional_session
        .visual_inspection_authority()
        .issue_pixel_grant();
    let optional_pending = optional_session
        .begin_visual_pixel_snapshot(
            &optional_grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(optional_target)
                .artifacts(UiPixelsOptional::policy()),
        )
        .expect("optional capture is admitted");
    let optional = match completed(optional_session.poll_visual_snapshot(optional_pending, 0)) {
        UiVisualSnapshotOutcome::Captured(receipt) => receipt,
        _ => panic!("missing optional pixels remain a successful typed capture"),
    };
    assert!(optional.optional_pixel_artifact().is_none());

    let required = required_outcome(
        "visual-required-pixels-missing",
        ScriptedPresentationHost::push_visual_capture_unsupported_payload,
    );
    assert!(matches!(
        required,
        UiVisualSnapshotOutcome::Denied(UiVisualSnapshotDenial::ProtocolIncompatible)
    ));
}

#[test]
fn mismatched_request_and_epoch_are_capture_affinity_indeterminate() {
    let wrong_request = required_outcome(
        "visual-wrong-request",
        ScriptedPresentationHost::push_visual_capture_wrong_request_payload,
    );
    assert!(matches!(
        wrong_request,
        UiVisualSnapshotOutcome::Indeterminate(UiVisualSnapshotIndeterminate::CaptureAffinity)
    ));

    let wrong_epoch = required_outcome(
        "visual-wrong-epoch",
        ScriptedPresentationHost::push_visual_capture_wrong_epoch_payload,
    );
    assert!(matches!(
        wrong_epoch,
        UiVisualSnapshotOutcome::Indeterminate(UiVisualSnapshotIndeterminate::CaptureAffinity)
    ));
}

#[test]
fn host_region_mismatch_is_protocol_incompatible() {
    let result = required_outcome(
        "visual-region-mismatch",
        ScriptedPresentationHost::push_visual_capture_region_mismatch,
    );
    assert!(matches!(
        result,
        UiVisualSnapshotOutcome::Denied(UiVisualSnapshotDenial::ProtocolIncompatible)
    ));
}

#[test]
fn invalid_transform_and_pixel_shape_are_rejected_before_receipt_seal() {
    let transform = required_outcome(
        "visual-invalid-transform",
        ScriptedPresentationHost::push_visual_capture_invalid_transform,
    );
    assert!(matches!(
        transform,
        UiVisualSnapshotOutcome::Denied(UiVisualSnapshotDenial::InvalidCoordinateTransform)
    ));

    let pixels = required_outcome(
        "visual-invalid-pixels",
        ScriptedPresentationHost::push_visual_capture_invalid_pixels,
    );
    assert!(matches!(
        pixels,
        UiVisualSnapshotOutcome::Denied(UiVisualSnapshotDenial::ProtocolIncompatible)
    ));
}

#[test]
fn pixel_capability_without_exact_epoch_is_affinity_indeterminate_before_effect() {
    let (mut session, host, target) = capture_world("visual-unproved-host-epoch");
    host.set_visual_capture_capability(worth_ui_host_contract::UiHostCaptureCapability::Pixels {
        maximum_bytes: 1_024,
        exact_presentation_epoch: false,
    });
    let grant = session.visual_inspection_authority().issue_pixel_grant();
    let pending = session
        .begin_visual_pixel_snapshot(
            &grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(target)
                .artifacts(UiPixelsRequired::policy()),
        )
        .expect("runtime admission precedes host capability adjudication");
    assert!(matches!(
        completed(session.poll_visual_snapshot(pending, 0)),
        UiVisualSnapshotOutcome::Indeterminate(UiVisualSnapshotIndeterminate::CaptureAffinity)
    ));
    assert!(host.visual_capture_calls().is_empty());
}

#[test]
fn terminal_host_outcomes_remain_distinct() {
    let superseded = required_outcome(
        "visual-superseded",
        ScriptedPresentationHost::push_visual_capture_superseded,
    );
    assert!(matches!(
        superseded,
        UiVisualSnapshotOutcome::Superseded(value)
            if !value.predecessor_artifact_copied()
    ));

    let affinity = required_outcome(
        "visual-affinity-indeterminate",
        ScriptedPresentationHost::push_visual_capture_affinity_indeterminate,
    );
    assert!(matches!(
        affinity,
        UiVisualSnapshotOutcome::Indeterminate(UiVisualSnapshotIndeterminate::CaptureAffinity)
    ));

    let unsupported = required_outcome(
        "visual-host-unsupported",
        ScriptedPresentationHost::push_visual_capture_unsupported,
    );
    assert!(matches!(
        unsupported,
        UiVisualSnapshotOutcome::Omitted(
            worth_ui::facade::inspection::UiVisualSnapshotOmission::HostCapabilityUnsupported
        )
    ));

    let capacity = required_outcome(
        "visual-host-capacity",
        ScriptedPresentationHost::push_visual_capture_capacity_exceeded,
    );
    assert!(matches!(
        capacity,
        UiVisualSnapshotOutcome::Denied(UiVisualSnapshotDenial::CapacityExceeded)
    ));
}

fn capture_world(label: &str) -> (ActiveSession, ScriptedPresentationHost, CurrentTarget) {
    let host = ScriptedPresentationHost::default();
    host.set_visual_capture_capability(worth_ui_host_contract::UiHostCaptureCapability::Pixels {
        maximum_bytes: 1_024,
        exact_presentation_epoch: true,
    });
    let (mut session, _) = mounted_session(host.clone(), label, 1);
    publish_one_frame(&mut session, &host);
    let target = current_target(&session);
    (session, host, target)
}

fn required_outcome(
    label: &str,
    script: fn(&ScriptedPresentationHost),
) -> UiVisualSnapshotOutcome<UiPixelsRequired> {
    let (mut session, host, target) = capture_world(label);
    script(&host);
    let grant = session.visual_inspection_authority().issue_pixel_grant();
    let pending = session
        .begin_visual_pixel_snapshot(
            &grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(target)
                .artifacts(UiPixelsRequired::policy()),
        )
        .expect("the required capture is admitted");
    completed(session.poll_visual_snapshot(pending, 0))
}

fn completed<Target, Policy>(
    poll: UiVisualCapturePoll<Target, Policy>,
) -> UiVisualSnapshotOutcome<Policy::CapturedPosture>
where
    Target: worth_ui::facade::inspection::UiVisualTarget,
    Policy: UiVisualArtifactPolicy,
{
    match poll {
        UiVisualCapturePoll::Completed(outcome) => outcome,
        UiVisualCapturePoll::Pending(_) => panic!("the script names a terminal observation"),
    }
}

fn visual_snapshot_leases(session: &ActiveSession) -> usize {
    session
        .mounted_retention_report()
        .class(worth_ui_runtime::facade::mounted::UiMountedRetentionClass::VisualSnapshot)
        .active_leases()
}

trait ScriptedVisualPayloads {
    fn push_visual_capture_unsupported_payload(&self);
    fn push_visual_capture_wrong_request_payload(&self);
    fn push_visual_capture_wrong_epoch_payload(&self);
    fn push_visual_capture_region_mismatch(&self);
    fn push_visual_capture_invalid_transform(&self);
    fn push_visual_capture_invalid_pixels(&self);
}

impl ScriptedVisualPayloads for ScriptedPresentationHost {
    fn push_visual_capture_unsupported_payload(&self) {
        self.push_visual_capture(visual_transform(), None);
    }

    fn push_visual_capture_wrong_request_payload(&self) {
        self.push_visual_capture_with_wrong_request(visual_transform(), Some(pixel_artifact()));
    }

    fn push_visual_capture_wrong_epoch_payload(&self) {
        self.push_visual_capture_with_wrong_epoch(visual_transform(), Some(pixel_artifact()));
    }

    fn push_visual_capture_region_mismatch(&self) {
        let bounds = worth_ui_host_contract::UiMountedCanonicalBox::canonicalize(
            worth_ui_host_contract::UiMountedCanonicalBoxInput {
                x: 0.0,
                y: 0.0,
                width: 1.0,
                height: 1.0,
                coordinate_space: worth_ui_host_contract::UiMountedCoordinateSpace::HostSurface,
            },
        )
        .expect("finite host region");
        let region = worth_ui_host_contract::UiHostRealizedRegion::observed_by_host(
            worth_ui_host_contract::UiMountedNodeReceiptIdentity::mint_unbound()
                .expect("test identity capacity"),
            worth_ui_host_contract::UiHostRealizedGeometry::observed_by_host(bounds, bounds),
            worth_ui_host_contract::UiHostRealizedOrdering::observed_by_host(
                0,
                worth_ui_host_contract::UiHostRealizedRegionParticipation::Paint,
            ),
        );
        self.push_visual_capture_with_regions(
            visual_transform(),
            vec![region],
            Some(pixel_artifact()),
        );
    }

    fn push_visual_capture_invalid_transform(&self) {
        let transform = worth_ui_host_contract::UiHostCoordinateTransform::observed_by_host(
            worth_ui_host_contract::UiHostClientAreaObservation::observed_by_host([17, 23], [2, 1]),
            worth_ui_host_contract::UiHostViewportTransformObservation::observed_by_host(
                [f32::NAN, 0.5],
                [1.6, 2.0],
                [0.25, 0.5],
            ),
            worth_ui_host_contract::UiHostCoordinatePosture::observed_by_host(
                worth_ui_host_contract::UiHostCoordinateOrientation::TopLeftOrigin,
                worth_ui_host_contract::UiHostCoordinateRounding::PixelCenterNearest,
            ),
        );
        self.push_visual_capture(transform, Some(pixel_artifact()));
    }

    fn push_visual_capture_invalid_pixels(&self) {
        let pixels = worth_ui_host_contract::UiHostPixelArtifact::copied_by_host(
            [2, 1],
            4,
            vec![1, 2, 3, 255, 4, 5, 6, 255].into_boxed_slice(),
            worth_ui_host_contract::UiHostPixelColorSpace::Srgb,
        );
        self.push_visual_capture(visual_transform(), Some(pixels));
    }
}
