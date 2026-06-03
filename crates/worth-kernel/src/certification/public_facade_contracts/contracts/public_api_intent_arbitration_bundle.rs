use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_kernel::facade::{certification::arbitration::*, diagnostics::arbitration::*};

#[test]
fn kernel_public_facade_exports_intent_arbitration_diagnostics_without_bundle_lane() {
    let preserved =
        prepare_primitive_construction_preserved_intent_resolution_report().expect("preserved");
    let unresolved = preserved
        .row(PrimitiveConstructionPreservedIntentResolutionCase::HostPenetrationBlockedCut)
        .expect("unresolved");
    let chosen = prepare_primitive_chosen_intent_resolution_report().expect("chosen");
    let explicit = chosen
        .row(PrimitiveConstructionChosenIntentResolutionCase::ExplicitSnapFlush)
        .expect("explicit");
    let dx = prepare_primitive_intent_conflict_dx_surface_report().expect("dx");

    assert_eq!(
        unresolved.preserved_truth(),
        PrimitiveConstructionPreservedIntentTruth::Unresolved {
            escalation: worth_spatial::facade::arbitration::SpatialIntentEscalation::BlockedByMissingCapability(
                worth_spatial::facade::arbitration::SpatialBlockedCapability::CutOpening
            ),
            blocked_capability: Some(
                worth_spatial::facade::arbitration::SpatialBlockedCapability::CutOpening
            ),
        }
    );
    assert_eq!(
        explicit.chosen_candidate(),
        worth_spatial::facade::arbitration::SpatialIntentCandidate::SnapFlush
    );
    assert_eq!(
        explicit.authority(),
        PrimitiveConstructionChosenIntentResolutionAuthority::ExplicitChoice
    );
    assert!(dx.rows().iter().any(|row| {
        row.case() == PrimitiveConstructionIntentArbitrationPolicyCase::HostPenetrationBlockedCut
    }));
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
        .evidence(PrimitiveConstructionIntentArbitrationBundleCase::HostPenetrationBlockedCut)
        .is_some());
}
