use forge_query::facade::ForgeQueryAuthorityLane;
use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_kernel::facade::{
    authoring::{intents::*, policy::*},
    certification::{continuity::*, query::*},
    diagnostics::{arbitration::*, continuity::*},
};

#[test]
fn kernel_public_facade_exports_continuity_surface_and_query_parity() {
    let report = prepare_primitive_construction_continuity_surface_report().expect("report");
    let row = report
        .row(PrimitiveConstructionContinuityCase::ExplicitMergeIdentityMerged)
        .expect("row")
        .clone();
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-continuity".to_string(),
    )
    .expect("workspace");
    let query = prepare_primitive_construction_query_continuity_inspection_parity_report(
        &mut workspace,
        row,
    )
    .expect("query");

    assert_eq!(
        query.continuity_class(),
        SpatialIdentityContinuityClass::IdentityMerged
    );
    assert!(query.parity_verified());
}

#[test]
fn kernel_public_facade_exports_continuity_hostility_suite_and_preview_inspection() {
    let suite = prepare_primitive_construction_continuity_hostility_suite_report()
        .expect("continuity suite");
    let preview = PrimitiveIntentPreviewAssessment::analyze_with_capabilities(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::GrazingContact],
        SpatialIntentCapabilitySet::blocked_defaults(),
        SpatialIntentPolicyProfile::aggressive_snap(),
    )
    .continuity()
    .clone();

    assert!(suite.suite_verified());
    assert_eq!(
        preview.continuity_class(),
        SpatialIdentityContinuityClass::AnchorContinuityPreserved
    );
}

#[test]
fn kernel_public_facade_exports_continuity_replay_and_branch_runtime_surface() {
    let replay = prepare_primitive_construction_continuity_replay_parity_report(
        PrimitiveConstructionContinuityCase::OverlapBlockedPendingChoice,
    )
    .expect("replay");
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-continuity-runtime".to_string(),
    )
    .expect("workspace");
    let branch = prepare_primitive_construction_continuity_branch_preview_runtime_report(
        &mut workspace,
        PrimitiveConstructionContinuityCase::HostAttachReinterpreted,
    )
    .expect("branch");

    assert!(replay.parity_verified());
    assert_eq!(
        branch.continuity_row().continuity_class(),
        SpatialIdentityContinuityClass::IdentityReinterpreted
    );
    assert_eq!(
        branch.preview_lane().authority_lane(),
        ForgeQueryAuthorityLane::PreviewTruth
    );
    assert_eq!(
        branch.branch_lane().authority_lane(),
        ForgeQueryAuthorityLane::BranchLocalTruth
    );
}

#[test]
fn kernel_public_facade_exports_continuity_bundle_surfaces() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-continuity-bundle".to_string(),
    )
    .expect("workspace");
    let direct = prepare_primitive_construction_continuity_report_bundle(
        &mut workspace,
        PrimitiveConstructionContinuityCase::GrazingSnapAnchorContinuity,
    )
    .expect("direct bundle");
    let suite = prepare_primitive_construction_continuity_hostility_suite_report()
        .expect("continuity suite");
    let reused = prepare_primitive_construction_continuity_bundle_from_hostility_suite(
        &suite,
        &mut workspace,
        PrimitiveConstructionContinuityCase::OverlapBlockedPendingChoice,
    )
    .expect("reused bundle");

    assert!(direct.parity_verified());
    assert!(reused.parity_verified());
}
