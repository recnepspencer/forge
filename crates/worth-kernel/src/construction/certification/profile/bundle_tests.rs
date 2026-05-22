use super::{
    prepare_primitive_construction_policy_profile_bundle_from_combined_hostility_suite,
    prepare_primitive_construction_policy_profile_bundle_from_hostility_suites,
    prepare_primitive_construction_policy_profile_report_bundle,
};
use crate::construction::{
    prepare_primitive_construction_continuity_hostility_suite_report,
    prepare_primitive_construction_preview_continuity_hostility_suite_report,
    prepare_primitive_construction_preview_hostility_suite_report,
    PrimitiveConstructionPolicyProfileCase,
};
use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};

#[test]
fn policy_profile_bundle_binds_profile_preview_and_continuity_truth() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.profile-bundle".to_string(),
    )
    .expect("workspace");

    let bundle = prepare_primitive_construction_policy_profile_report_bundle(
        &mut workspace,
        PrimitiveConstructionPolicyProfileCase::BimHostFriendly,
    )
    .expect("bundle");

    assert!(bundle.parity_verified());
    assert_eq!(
        bundle.profile_row().profile_name(),
        bundle.preview_bundle().preview_row().profile_name()
    );
    assert!(bundle.continuity_bundle().is_some());
    assert_eq!(
        bundle.profile_row().proximity_posture(),
        bundle.replay_report().direct_row().proximity_posture()
    );
    assert_eq!(
        bundle.profile_row().alignment_posture(),
        bundle.replay_report().direct_row().alignment_posture()
    );
    assert_eq!(
        bundle.replay_report().direct_row().profile_name(),
        bundle.branch_runtime_report().profile_row().profile_name()
    );
    assert_eq!(
        bundle
            .continuity_bundle()
            .expect("continuity bundle")
            .continuity_row()
            .profile_name(),
        bundle
            .preview_bundle()
            .branch_runtime_report()
            .preview_row()
            .profile_name()
    );
    assert_eq!(
        bundle.profile_row().representative_preview_case(),
        bundle.preview_bundle().branch_runtime_report().case()
    );
}

#[test]
fn policy_profile_bundle_reuses_hostility_suite_rows_directly() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.profile-bundle-suite".to_string(),
    )
    .expect("workspace");
    let preview_suite =
        prepare_primitive_construction_preview_hostility_suite_report().expect("preview suite");
    let continuity_suite = prepare_primitive_construction_continuity_hostility_suite_report()
        .expect("continuity suite");

    let bundle = prepare_primitive_construction_policy_profile_bundle_from_hostility_suites(
        &preview_suite,
        &continuity_suite,
        &mut workspace,
        PrimitiveConstructionPolicyProfileCase::AggressiveSnap,
    )
    .expect("bundle");

    assert!(bundle.parity_verified());
    assert_eq!(
        bundle.profile_row().profile_name(),
        bundle.preview_bundle().preview_row().profile_name()
    );
    assert_ne!(
        bundle.report_digest(),
        bundle.preview_bundle().report_digest()
    );
    assert_ne!(
        bundle.report_digest(),
        bundle
            .continuity_bundle()
            .expect("continuity bundle")
            .report_digest()
    );
}

#[test]
fn policy_profile_bundle_reuses_combined_hostility_suite_rows_directly() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.profile-bundle-combined-suite".to_string(),
    )
    .expect("workspace");
    let suite = prepare_primitive_construction_preview_continuity_hostility_suite_report()
        .expect("combined suite");

    let bundle =
        prepare_primitive_construction_policy_profile_bundle_from_combined_hostility_suite(
            &suite,
            &mut workspace,
            PrimitiveConstructionPolicyProfileCase::ConservativeExactModeling,
        )
        .expect("bundle");

    assert!(bundle.parity_verified());
    assert!(bundle.continuity_bundle().is_some());
    assert_eq!(
        bundle.profile_row().profile_name(),
        bundle
            .continuity_bundle()
            .expect("continuity bundle")
            .branch_runtime_report()
            .continuity_row()
            .profile_name()
    );
}
