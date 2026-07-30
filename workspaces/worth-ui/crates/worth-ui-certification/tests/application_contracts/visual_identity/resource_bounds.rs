use worth_ui::facade::inspection::{
    UiGeometryOnly, UiVisualInspectionByteBudget, UiVisualInspectionCapacity,
    UiVisualInspectionDisclosure, UiVisualInspectionPolicy, UiVisualInspectionRegionCapacity,
    UiVisualSnapshotDenial, UiVisualSnapshotRequest,
};
use worth_ui_host_egui::WorthUiHostEgui;
use worth_ui_runtime::facade::mounted::{
    UiMountedFrameOutcome, UiMountedInspectionReceipt, UiMountedInspectionRequest,
    UiPresentationDeadline,
};
use worth_ui_test_support::WorthUiMountedPublicationCertificationExt;

use super::super::filesystem_mounted_world::{
    establish_allocation, launch_native_world_with_policy, prepare_frame,
};

#[test]
fn visible_region_capacity_denies_before_host_capture_effects() {
    assert_pre_effect_denial(
        policy(
            UiVisualInspectionRegionCapacity::bounded(1, 65_536),
            64 << 20,
            256 << 20,
        ),
        UiVisualSnapshotDenial::VisibleRegionCapacityExceeded,
    );
}

#[test]
fn hit_test_region_capacity_denies_before_host_capture_effects() {
    assert_pre_effect_denial(
        policy(
            UiVisualInspectionRegionCapacity::bounded(65_536, 1),
            64 << 20,
            256 << 20,
        ),
        UiVisualSnapshotDenial::HitTestRegionCapacityExceeded,
    );
}

#[test]
fn per_receipt_structure_denies_before_host_capture_effects() {
    assert_pre_effect_denial(
        policy(
            UiVisualInspectionRegionCapacity::bounded(65_536, 65_536),
            1,
            u64::MAX,
        ),
        UiVisualSnapshotDenial::RetainedStructurePerReceiptCapacityExceeded,
    );
}

#[test]
fn per_session_structure_denies_before_host_capture_effects() {
    assert_pre_effect_denial(
        policy(
            UiVisualInspectionRegionCapacity::bounded(65_536, 65_536),
            u64::MAX,
            1,
        ),
        UiVisualSnapshotDenial::RetainedStructurePerSessionCapacityExceeded,
    );
}

fn assert_pre_effect_denial(policy: UiVisualInspectionPolicy, expected: UiVisualSnapshotDenial) {
    let context = egui::Context::default();
    let host = WorthUiHostEgui::new(context.clone());
    let mut session = launch_native_world_with_policy(host, policy);
    let _ = context.run_ui(super::raw_input(), |_| {
        establish_allocation(&mut session, 3);
        let prepared = prepare_frame(&mut session).expect("the four-way projection completes");
        assert!(matches!(
            session.present_prepared_mounted_frame(
                prepared,
                UiPresentationDeadline::at_tick(10),
                0,
            ),
            UiMountedFrameOutcome::Published(_)
        ));
    });
    let target = match session.inspect_mounted_frame(UiMountedInspectionRequest::current()) {
        UiMountedInspectionReceipt::Available(frame) => frame
            .current_visual_target()
            .expect("the real world presents exactly one surface"),
        other => panic!("the published frame is inspectable, got {other:?}"),
    };
    let grant = session.visual_inspection_authority().issue_geometry_grant();
    let request = UiVisualSnapshotRequest::for_local_development_unredacted_frame(target)
        .artifacts(UiGeometryOnly::policy());
    let mut request = Some(request);
    let mut observed_denial = None;
    let output = context.run_ui(super::raw_input(), |_| {
        observed_denial = session
            .begin_visual_geometry_snapshot(
                &grant,
                request
                    .take()
                    .expect("the egui proof submits the request exactly once"),
            )
            .err();
    });

    assert_eq!(observed_denial, Some(expected));
    let screenshot_commands = output
        .viewport_output
        .values()
        .flat_map(|viewport| &viewport.commands)
        .filter(|command| matches!(command, egui::ViewportCommand::Screenshot(_)))
        .count();
    assert_eq!(screenshot_commands, 0);
    assert_eq!(
        session
            .shutdown()
            .visual_capture()
            .cancelled_capture_count(),
        0
    );
}

fn policy(
    regions: UiVisualInspectionRegionCapacity,
    structure_per_receipt: u64,
    structure_per_session: u64,
) -> UiVisualInspectionPolicy {
    UiVisualInspectionPolicy::bounded(
        UiVisualInspectionDisclosure::local_development_unredacted(),
        UiVisualInspectionCapacity::bounded(2, 32, 4_096),
        regions,
        UiVisualInspectionByteBudget::bounded(
            64 << 20,
            256 << 20,
            structure_per_receipt,
            structure_per_session,
        ),
    )
    .expect("the capacity proof policy is valid")
}
