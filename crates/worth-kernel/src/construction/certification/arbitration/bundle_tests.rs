use super::{
    prepare_primitive_construction_intent_arbitration_report_bundle,
    PrimitiveConstructionChosenIntentResolutionAuthority,
    PrimitiveConstructionIntentArbitrationBundleCase,
};
use crate::construction::{
    PrimitiveConstructionIntentChosenTruth, PrimitiveConstructionPreservedIntentTruth,
};
use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_spatial::facade::{SpatialIntentCandidate, SpatialIntentEscalation};

fn workspace(name: &str) -> forge_query::facade::ForgeQueryWorkspace {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        name.to_string(),
    )
    .expect("workspace")
}

#[test]
fn arbitration_report_bundle_verifies_policy_dx_and_query_truth_for_unresolved_conflicts() {
    let mut workspace = workspace("worth-kernel.arbitration-bundle.unresolved");
    let bundle = prepare_primitive_construction_intent_arbitration_report_bundle(
        &mut workspace,
        PrimitiveConstructionIntentArbitrationBundleCase::HostPenetrationBlockedCut,
    )
    .expect("bundle");

    assert_eq!(
        bundle.truth().preserved_truth(),
        PrimitiveConstructionPreservedIntentTruth::Unresolved {
            escalation: SpatialIntentEscalation::BlockedByMissingCapability(
                worth_spatial::facade::SpatialBlockedCapability::CutOpening
            ),
            blocked_capability: Some(worth_spatial::facade::SpatialBlockedCapability::CutOpening),
        }
    );
    assert!(bundle.replay_parity_report().parity_verified());
    assert!(bundle.chosen_row().is_none());
    assert_eq!(
        bundle.policy_row().escalation(),
        SpatialIntentEscalation::BlockedByMissingCapability(
            worth_spatial::facade::SpatialBlockedCapability::CutOpening
        )
    );
    assert_eq!(
        bundle.query_inspection_parity_report().chosen_truth(),
        PrimitiveConstructionIntentChosenTruth::Unresolved
    );
}

#[test]
fn arbitration_report_bundle_preserves_explicit_choice_truth() {
    let mut workspace = workspace("worth-kernel.arbitration-bundle.explicit");
    let bundle = prepare_primitive_construction_intent_arbitration_report_bundle(
        &mut workspace,
        PrimitiveConstructionIntentArbitrationBundleCase::GrazingSnapExplicitChoice,
    )
    .expect("bundle");

    assert_eq!(
        bundle.truth().preserved_truth(),
        PrimitiveConstructionPreservedIntentTruth::Resolved {
            candidate: SpatialIntentCandidate::SnapFlush,
            authority: PrimitiveConstructionChosenIntentResolutionAuthority::ExplicitChoice,
        }
    );
    assert!(bundle.replay_parity_report().parity_verified());
    assert_eq!(
        bundle.chosen_row().expect("chosen row").authority(),
        PrimitiveConstructionChosenIntentResolutionAuthority::ExplicitChoice
    );
    assert_eq!(
        bundle.query_projection_receipt_report().chosen_truth(),
        PrimitiveConstructionIntentChosenTruth::Resolved {
            candidate: SpatialIntentCandidate::SnapFlush,
            authority: PrimitiveConstructionChosenIntentResolutionAuthority::ExplicitChoice,
        }
    );
}

#[test]
fn arbitration_report_bundle_preserves_policy_auto_resolution_truth() {
    let mut workspace = workspace("worth-kernel.arbitration-bundle.policy");
    let bundle = prepare_primitive_construction_intent_arbitration_report_bundle(
        &mut workspace,
        PrimitiveConstructionIntentArbitrationBundleCase::DirectMoveOnlyPolicy,
    )
    .expect("bundle");

    assert_eq!(
        bundle.truth().preserved_truth(),
        PrimitiveConstructionPreservedIntentTruth::Resolved {
            candidate: SpatialIntentCandidate::MoveOnly,
            authority: PrimitiveConstructionChosenIntentResolutionAuthority::PolicyAutoResolve,
        }
    );
    assert!(bundle.replay_parity_report().parity_verified());
    assert_eq!(
        bundle.query_inspection_parity_report().chosen_truth(),
        PrimitiveConstructionIntentChosenTruth::Resolved {
            candidate: SpatialIntentCandidate::MoveOnly,
            authority: PrimitiveConstructionChosenIntentResolutionAuthority::PolicyAutoResolve,
        }
    );
    assert_eq!(
        bundle.query_projection_receipt_report().chosen_truth(),
        PrimitiveConstructionIntentChosenTruth::Resolved {
            candidate: SpatialIntentCandidate::MoveOnly,
            authority: PrimitiveConstructionChosenIntentResolutionAuthority::PolicyAutoResolve,
        }
    );
}
