use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_kernel::facade::{
    prepare_primitive_construction_intent_arbitration_hostility_suite_report,
    prepare_primitive_construction_intent_arbitration_report_bundle,
    PrimitiveConstructionChosenIntentResolutionAuthority,
    PrimitiveConstructionIntentArbitrationBundleCase, PrimitiveConstructionIntentChosenTruth,
};
use worth_spatial::facade::SpatialIntentCandidate;

#[test]
fn kernel_public_facade_exports_intent_arbitration_report_bundle() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-api.intent-arbitration-bundle".to_string(),
    )
    .expect("workspace");
    let unresolved = prepare_primitive_construction_intent_arbitration_report_bundle(
        &mut workspace,
        PrimitiveConstructionIntentArbitrationBundleCase::HostPenetrationBlockedCut,
    )
    .expect("unresolved bundle");
    let explicit = prepare_primitive_construction_intent_arbitration_report_bundle(
        &mut workspace,
        PrimitiveConstructionIntentArbitrationBundleCase::GrazingSnapExplicitChoice,
    )
    .expect("explicit bundle");

    assert!(unresolved.bundle_verified());
    assert!(unresolved.replay_parity_report().parity_verified());
    assert_eq!(
        unresolved.query_inspection_parity_report().chosen_truth(),
        PrimitiveConstructionIntentChosenTruth::Unresolved
    );
    assert!(explicit.bundle_verified());
    assert!(explicit.replay_parity_report().parity_verified());
    assert_eq!(
        explicit.query_projection_receipt_report().chosen_truth(),
        PrimitiveConstructionIntentChosenTruth::Resolved {
            candidate: SpatialIntentCandidate::SnapFlush,
            authority: PrimitiveConstructionChosenIntentResolutionAuthority::ExplicitChoice,
        }
    );
}

#[test]
fn kernel_public_facade_exports_intent_arbitration_hostility_suite_surface() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-api.intent-arbitration-suite".to_string(),
    )
    .expect("workspace");
    let report =
        prepare_primitive_construction_intent_arbitration_hostility_suite_report(&mut workspace)
            .expect("suite");

    assert!(report.suite_verified());
    assert!(report
        .bundle(PrimitiveConstructionIntentArbitrationBundleCase::HostPenetrationBlockedCut)
        .is_some());
}
