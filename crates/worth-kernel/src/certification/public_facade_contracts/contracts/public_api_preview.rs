use forge_query::facade::ForgeQueryAuthorityLane;
use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_kernel::facade::{
    authoring::{intents::*, policy::*},
    certification::{preview::*, query::*},
    diagnostics::{arbitration::*, preview::*},
};

#[test]
fn kernel_public_facade_exports_preview_surface_and_query_parity() {
    let report = prepare_primitive_construction_preview_surface_report().expect("report");
    let row = report
        .row(PrimitiveConstructionPreviewCase::GrazingAggressiveSnap)
        .expect("row")
        .clone();
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-preview".to_string(),
    )
    .expect("workspace");
    let query =
        prepare_primitive_construction_query_preview_inspection_parity_report(&mut workspace, row)
            .expect("query");

    assert_eq!(query.profile_name(), "aggressive_snap");
    assert!(query.parity_verified());
}

#[test]
fn kernel_public_facade_exports_preview_hostility_suite() {
    let suite = prepare_primitive_construction_preview_hostility_suite_report().expect("suite");

    assert!(suite.suite_verified());
}

#[test]
fn kernel_public_facade_exports_preview_replay_branch_and_bundle_surfaces() {
    let replay = prepare_primitive_construction_preview_replay_parity_report(
        PrimitiveConstructionPreviewCase::OverlapBlockedMerge,
    )
    .expect("replay");
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-preview-runtime".to_string(),
    )
    .expect("workspace");
    let branch = prepare_primitive_construction_preview_branch_preview_runtime_report(
        &mut workspace,
        PrimitiveConstructionPreviewCase::HostFaceBimAttach,
    )
    .expect("branch");
    let bundle = prepare_primitive_construction_preview_report_bundle(
        &mut workspace,
        PrimitiveConstructionPreviewCase::GrazingAggressiveSnap,
    )
    .expect("bundle");
    let suite = prepare_primitive_construction_preview_hostility_suite_report().expect("suite");
    let bundle_from_suite = prepare_primitive_construction_preview_bundle_from_hostility_suite(
        &suite,
        &mut workspace,
        PrimitiveConstructionPreviewCase::OverlapBlockedMerge,
    )
    .expect("bundle from suite");

    assert!(replay.parity_verified());
    assert_eq!(
        branch.preview_lane().authority_lane(),
        ForgeQueryAuthorityLane::PreviewTruth
    );
    assert_eq!(
        branch.branch_lane().authority_lane(),
        ForgeQueryAuthorityLane::BranchLocalTruth
    );
    assert!(bundle.parity_verified());
    assert!(bundle_from_suite.parity_verified());
}

#[test]
fn kernel_public_facade_exports_preview_assessment_envelope_and_profile_override() {
    let profile = SpatialIntentPolicyProfile::aggressive_snap().derive(
        SpatialIntentPolicyProfileOverride::new()
            .with_name("aggressive_snap_high_fidelity")
            .with_preview_richness(SpatialPreviewRichness::HighFidelity)
            .with_arbitration_posture(SpatialArbitrationPosture::PreferSnap),
    );
    let assessment = PrimitiveIntentPreviewAssessment::analyze(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::GrazingContact],
        profile,
    );

    assert_eq!(assessment.profile().name(), "aggressive_snap_high_fidelity");
    assert_eq!(
        assessment.commit_disposition(),
        SpatialIntentPreviewCommitDisposition::WouldAutoResolve(SpatialIntentCandidate::SnapFlush)
    );
    assert_eq!(
        assessment.preview_richness(),
        SpatialPreviewRichness::HighFidelity
    );
    assert_eq!(
        assessment.continuity().candidate(),
        Some(SpatialIntentCandidate::SnapFlush)
    );
    assert!(assessment.clarification_request().is_err());
    assert!(assessment
        .warnings()
        .contains(&SpatialIntentPreviewWarning::HighFidelityPreview));
    assert_eq!(
        assessment.commit_disposition(),
        SpatialIntentPreviewCommitDisposition::WouldAutoResolve(SpatialIntentCandidate::SnapFlush)
    );
}
