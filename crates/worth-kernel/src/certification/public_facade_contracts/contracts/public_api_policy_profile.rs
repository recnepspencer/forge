use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_kernel::facade::{
    prepare_primitive_construction_continuity_hostility_suite_report,
    prepare_primitive_construction_policy_profile_branch_preview_runtime_report,
    prepare_primitive_construction_policy_profile_bundle_from_combined_hostility_suite,
    prepare_primitive_construction_policy_profile_bundle_from_hostility_suites,
    prepare_primitive_construction_policy_profile_replay_parity_report,
    prepare_primitive_construction_policy_profile_report,
    prepare_primitive_construction_policy_profile_report_bundle,
    prepare_primitive_construction_preview_continuity_hostility_suite_report,
    prepare_primitive_construction_preview_hostility_suite_report,
    prepare_primitive_construction_query_policy_profile_inspection_parity_report,
    prepare_primitive_construction_query_policy_profile_projection_consumption_receipt_report,
    PrimitiveConstructionPolicyProfileCase, SpatialArbitrationPosture, SpatialPreviewRichness,
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
fn kernel_public_facade_exports_policy_profile_bundle_surfaces() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-policy-profile-bundle".to_string(),
    )
    .expect("workspace");
    let direct = prepare_primitive_construction_policy_profile_report_bundle(
        &mut workspace,
        PrimitiveConstructionPolicyProfileCase::AggressiveSnap,
    )
    .expect("direct bundle");
    let preview_suite =
        prepare_primitive_construction_preview_hostility_suite_report().expect("preview suite");
    let continuity_suite = prepare_primitive_construction_continuity_hostility_suite_report()
        .expect("continuity suite");
    let reused = prepare_primitive_construction_policy_profile_bundle_from_hostility_suites(
        &preview_suite,
        &continuity_suite,
        &mut workspace,
        PrimitiveConstructionPolicyProfileCase::BimHostFriendly,
    )
    .expect("reused bundle");

    assert!(direct.parity_verified());
    assert_eq!(
        direct.profile_row().arbitration_posture(),
        SpatialArbitrationPosture::PreferSnap
    );
    assert!(reused.parity_verified());
    assert!(reused.continuity_bundle().is_some());
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
    let bundle =
        prepare_primitive_construction_policy_profile_bundle_from_combined_hostility_suite(
            &suite,
            &mut workspace,
            PrimitiveConstructionPolicyProfileCase::BimHostFriendly,
        )
        .expect("bundle");

    assert!(replay.parity_verified());
    assert!(suite.suite_verified());
    assert!(bundle.parity_verified());
    assert_eq!(
        replay.direct_row().profile_name(),
        branch.profile_row().profile_name()
    );
}
