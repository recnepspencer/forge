use crate::observation_rebind::support::RebindExecutionWorld;
use worth_ui::facade::inspection::{
    UiPixelsRequired, UiUnbudgetedVisualSnapshotComparisonRequest, UiVisualCaptureDeadline,
    UiVisualCapturePoll, UiVisualComparisonPixelPolicy, UiVisualIdentityContinuity,
    UiVisualSnapshotComparisonBudget, UiVisualSnapshotComparisonIncompatibility,
    UiVisualSnapshotComparisonOutcome, UiVisualSnapshotOutcome, UiVisualSnapshotReceipt,
    UiVisualSnapshotRequest,
};
use worth_ui::facade::rebind::{UiIdentityLifecycleDecision, UiRebindOutcome};
use worth_ui_runtime::facade::mounted::{UiMountedInspectionReceipt, UiMountedInspectionRequest};

#[test]
fn comparison_borrows_exact_rebind_snapshots_without_recapture_or_pixel_identity_inference() {
    let mut world = RebindExecutionWorld::new("phase-312-visual-comparison");
    let predecessor = capture_current(&mut world, [242, 204, 96, 255]);
    world.host.push_presented();
    let prepared = world.prepare_changed();
    let rebind = match prepared.execute(1) {
        UiRebindOutcome::Published(receipt) => receipt,
        _ => panic!("the changed generation publishes"),
    };
    let successor = capture_current(&mut world, [76, 175, 80, 255]);
    let capture_call_count = world.host.visual_capture_calls().len();
    let grant = world
        .session
        .visual_inspection_authority()
        .issue_comparison_grant();
    let request =
        UiUnbudgetedVisualSnapshotComparisonRequest::between(&predecessor, &successor, &rebind)
            .with_pixel_observation(UiVisualComparisonPixelPolicy::IfAlreadyRetained)
            .with_budget(UiVisualSnapshotComparisonBudget::bounded(128).unwrap());
    let comparison = world.session.compare_visual_snapshots(&grant, request);

    let compared = match comparison {
        UiVisualSnapshotComparisonOutcome::Compared(compared) => compared,
        other => panic!("exact borrowed evidence compares, got {other:?}"),
    };
    let mounted = rebind.mounted_publication().unwrap();
    assert_eq!(
        compared.frame_identities(),
        [
            mounted.predecessor().unwrap().diagnostic_value(),
            mounted.frame().diagnostic_value(),
        ]
    );
    assert!(rebind.plan().identity_decisions().iter().any(|entry| {
        matches!(
            entry.decision(),
            UiIdentityLifecycleDecision::Create
                | UiIdentityLifecycleDecision::Retire
                | UiIdentityLifecycleDecision::Rebind
                | UiIdentityLifecycleDecision::Remount
        )
    }));
    assert_eq!(compared.continuity(), UiVisualIdentityContinuity::Rebound);
    assert_eq!(compared.retained_pixels_differ(), Some(true));
    assert_eq!(compared.cost().retained_pixel_bytes_examined(), 16);
    assert_eq!(
        world.host.visual_capture_calls().len(),
        capture_call_count,
        "comparison must not recapture either frame"
    );
    let omitted_pixels = world.session.compare_visual_snapshots(
        &grant,
        UiUnbudgetedVisualSnapshotComparisonRequest::between(&predecessor, &successor, &rebind)
            .with_pixel_observation(UiVisualComparisonPixelPolicy::Omit)
            .with_budget(UiVisualSnapshotComparisonBudget::bounded(128).unwrap()),
    );
    match omitted_pixels {
        UiVisualSnapshotComparisonOutcome::Compared(compared) => {
            assert_eq!(compared.retained_pixels_differ(), None)
        }
        other => panic!("pixel omission must not relabel structural evidence: {other:?}"),
    }
    let foreign = RebindExecutionWorld::new("phase-312-visual-comparison-foreign");
    let foreign_grant = foreign
        .session
        .visual_inspection_authority()
        .issue_comparison_grant();
    let incompatible = world.session.compare_visual_snapshots(
        &foreign_grant,
        UiUnbudgetedVisualSnapshotComparisonRequest::between(&predecessor, &successor, &rebind)
            .with_budget(UiVisualSnapshotComparisonBudget::bounded(128).unwrap()),
    );
    assert!(matches!(
        incompatible,
        UiVisualSnapshotComparisonOutcome::Incompatible(
            UiVisualSnapshotComparisonIncompatibility::ForeignSession
        )
    ));
    foreign.close();
    assert_eq!(world.host.visual_capture_calls().len(), capture_call_count);
    world.session.dispose_visual_snapshot(predecessor);
    world.session.dispose_visual_snapshot(successor);
    drop(rebind);
    world.close();
}

fn capture_current(
    world: &mut RebindExecutionWorld,
    rgba: [u8; 4],
) -> UiVisualSnapshotReceipt<UiPixelsRequired> {
    let target = match world
        .session
        .inspect_mounted_frame(UiMountedInspectionRequest::current())
    {
        UiMountedInspectionReceipt::Available(frame) => frame
            .current_visual_target()
            .expect("one current presented surface is unambiguous"),
        other => panic!("the current frame is inspectable, got {other:?}"),
    };
    let grant = world
        .session
        .visual_inspection_authority()
        .issue_pixel_grant();
    world.host.push_visual_capture(
        visual_transform(),
        Some(worth_ui_host_contract::UiHostPixelArtifact::copied_by_host(
            [2, 1],
            8,
            [rgba, rgba].concat().into_boxed_slice(),
            worth_ui_host_contract::UiHostPixelColorSpace::Srgb,
        )),
    );
    let pending = world
        .session
        .begin_visual_pixel_snapshot(
            &grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(target)
                .artifacts(UiPixelsRequired::policy())
                .deadline(UiVisualCaptureDeadline::at_tick(20)),
        )
        .expect("the exact current frame capture admits");
    match world.session.poll_visual_snapshot(pending, 1) {
        UiVisualCapturePoll::Completed(UiVisualSnapshotOutcome::Captured(receipt)) => receipt,
        UiVisualCapturePoll::Completed(UiVisualSnapshotOutcome::Superseded(outcome)) => {
            panic!("capture superseded: {outcome:?}")
        }
        UiVisualCapturePoll::Completed(UiVisualSnapshotOutcome::Omitted(outcome)) => {
            panic!("capture omitted: {outcome:?}")
        }
        UiVisualCapturePoll::Completed(UiVisualSnapshotOutcome::Denied(outcome)) => {
            panic!("capture denied: {outcome:?}")
        }
        UiVisualCapturePoll::Completed(UiVisualSnapshotOutcome::Indeterminate(outcome)) => {
            panic!("capture indeterminate: {outcome:?}")
        }
        UiVisualCapturePoll::Pending(_) => panic!("capture unexpectedly remained pending"),
    }
}

fn visual_transform() -> worth_ui_host_contract::UiHostCoordinateTransform {
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
