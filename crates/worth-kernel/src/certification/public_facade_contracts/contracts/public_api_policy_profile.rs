use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_kernel::facade::{
    authoring::policy::*,
    certification::{continuity::*, preview::*, query::*},
    diagnostics::policy::*,
};

#[test]
fn kernel_public_facade_exports_policy_profile_report_and_query_surfaces() {
    let report = prepare_primitive_construction_policy_profile_report();
    let row = report
        .row(PrimitiveConstructionPolicyProfileCase::HighFidelityPreview)
        .expect("row")
        .clone();
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-policy-profile".to_string(),
    )
    .expect("workspace");
    let inspection = prepare_primitive_construction_query_policy_profile_inspection_parity_report(
        &mut workspace,
        row.clone(),
    )
    .expect("inspection");
    let projection =
        prepare_primitive_construction_query_policy_profile_projection_consumption_receipt_report(
            &mut workspace,
            row,
        )
        .expect("projection");

    assert_eq!(
        inspection.preview_richness(),
        SpatialPreviewRichness::HighFidelity
    );
    assert_eq!(
        projection.preview_richness(),
        SpatialPreviewRichness::HighFidelity
    );
    assert!(inspection.parity_verified());
    assert!(projection.parity_verified());
}

#[test]
fn kernel_public_facade_exports_policy_profile_direct_reports_and_supporting_hostility_suites() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-policy-profile-bundle".to_string(),
    )
    .expect("workspace");
    let report = prepare_primitive_construction_policy_profile_report();
    let direct = report
        .row(PrimitiveConstructionPolicyProfileCase::AggressiveSnap)
        .expect("direct row");
    let preview_suite =
        prepare_primitive_construction_preview_hostility_suite_report().expect("preview suite");
    let continuity_suite = prepare_primitive_construction_continuity_hostility_suite_report()
        .expect("continuity suite");
    let branch = prepare_primitive_construction_policy_profile_branch_preview_runtime_report(
        &mut workspace,
        PrimitiveConstructionPolicyProfileCase::AggressiveSnap,
    )
    .expect("branch");

    assert_eq!(
        direct.arbitration_posture(),
        SpatialArbitrationPosture::PreferSnap
    );
    assert!(preview_suite.suite_verified());
    assert!(continuity_suite.suite_verified());
    assert_eq!(branch.profile_row().profile_name(), direct.profile_name());
}

#[test]
fn kernel_public_facade_exports_profile_replay_branch_and_combined_suite_surfaces() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-policy-profile-advanced".to_string(),
    )
    .expect("workspace");
    let replay = prepare_primitive_construction_policy_profile_replay_parity_report(
        PrimitiveConstructionPolicyProfileCase::BimHostFriendly,
    )
    .expect("replay");
    let branch = prepare_primitive_construction_policy_profile_branch_preview_runtime_report(
        &mut workspace,
        PrimitiveConstructionPolicyProfileCase::BimHostFriendly,
    )
    .expect("branch");
    let suite = prepare_primitive_construction_preview_continuity_hostility_suite_report()
        .expect("combined suite");

    assert!(replay.parity_verified());
    assert!(suite.suite_verified());
    assert_eq!(
        replay.direct_row().profile_name(),
        branch.profile_row().profile_name()
    );
}
