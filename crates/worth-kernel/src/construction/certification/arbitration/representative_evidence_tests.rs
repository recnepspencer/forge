use super::{
    prepare_primitive_construction_intent_arbitration_representative_evidence,
    PrimitiveConstructionChosenIntentResolutionAuthority,
    PrimitiveConstructionIntentArbitrationBundleCase, PrimitiveConstructionPreservedIntentTruth,
};
use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_spatial::facade::arbitration::{SpatialIntentCandidate, SpatialIntentEscalation};

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
fn arbitration_representative_evidence_preserves_unresolved_conflict_truth() {
    let mut workspace = workspace("worth-kernel.arbitration-evidence.unresolved");
    let evidence = prepare_primitive_construction_intent_arbitration_representative_evidence(
        &mut workspace,
        PrimitiveConstructionIntentArbitrationBundleCase::HostPenetrationBlockedCut,
    )
    .expect("evidence");

    assert!(evidence.parity_verified());
    assert_eq!(
        evidence.preserved_row().preserved_truth(),
        PrimitiveConstructionPreservedIntentTruth::Unresolved {
            escalation: SpatialIntentEscalation::BlockedByMissingCapability(
                worth_spatial::facade::arbitration::SpatialBlockedCapability::CutOpening
            ),
            blocked_capability: Some(
                worth_spatial::facade::arbitration::SpatialBlockedCapability::CutOpening
            ),
        }
    );
    assert!(evidence.chosen_row().is_none());
}

#[test]
fn arbitration_representative_evidence_preserves_explicit_choice_truth() {
    let mut workspace = workspace("worth-kernel.arbitration-evidence.explicit");
    let evidence = prepare_primitive_construction_intent_arbitration_representative_evidence(
        &mut workspace,
        PrimitiveConstructionIntentArbitrationBundleCase::GrazingSnapExplicitChoice,
    )
    .expect("evidence");

    assert!(evidence.parity_verified());
    assert_eq!(
        evidence.chosen_row().expect("chosen row").authority(),
        PrimitiveConstructionChosenIntentResolutionAuthority::ExplicitChoice
    );
    assert_eq!(
        evidence.preserved_row().preserved_truth(),
        PrimitiveConstructionPreservedIntentTruth::Resolved {
            candidate: SpatialIntentCandidate::SnapFlush,
            authority: PrimitiveConstructionChosenIntentResolutionAuthority::ExplicitChoice,
        }
    );
}

#[test]
fn arbitration_representative_evidence_preserves_policy_auto_resolution_truth() {
    let mut workspace = workspace("worth-kernel.arbitration-evidence.policy");
    let evidence = prepare_primitive_construction_intent_arbitration_representative_evidence(
        &mut workspace,
        PrimitiveConstructionIntentArbitrationBundleCase::DirectMoveOnlyPolicy,
    )
    .expect("evidence");

    assert!(evidence.parity_verified());
    assert_eq!(
        evidence.policy_row().chosen_candidate(),
        Some(SpatialIntentCandidate::MoveOnly)
    );
    assert_eq!(
        evidence.chosen_row().expect("chosen row").authority(),
        PrimitiveConstructionChosenIntentResolutionAuthority::PolicyAutoResolve
    );
    assert_eq!(
        evidence.preserved_row().preserved_truth(),
        PrimitiveConstructionPreservedIntentTruth::Resolved {
            candidate: SpatialIntentCandidate::MoveOnly,
            authority: PrimitiveConstructionChosenIntentResolutionAuthority::PolicyAutoResolve,
        }
    );
}
