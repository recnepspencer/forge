use super::prepare_primitive_construction_intent_arbitration_replay_parity_report;
use crate::construction::{
    PrimitiveConstructionChosenIntentResolutionAuthority,
    PrimitiveConstructionPreservedIntentResolutionCase, PrimitiveConstructionPreservedIntentTruth,
};
use worth_spatial::facade::{
    SpatialBlockedCapability, SpatialIntentCandidate, SpatialIntentEscalation,
};

#[test]
fn arbitration_replay_parity_preserves_policy_auto_resolve_truth() {
    let report = prepare_primitive_construction_intent_arbitration_replay_parity_report(
        PrimitiveConstructionPreservedIntentResolutionCase::PolicyMoveOnly,
    )
    .expect("report");

    assert!(report.parity_verified());
    assert_eq!(
        report.direct_row().preserved_truth(),
        PrimitiveConstructionPreservedIntentTruth::Resolved {
            candidate: SpatialIntentCandidate::MoveOnly,
            authority: PrimitiveConstructionChosenIntentResolutionAuthority::PolicyAutoResolve,
        }
    );
}

#[test]
fn arbitration_replay_parity_preserves_explicit_choice_truth() {
    let report = prepare_primitive_construction_intent_arbitration_replay_parity_report(
        PrimitiveConstructionPreservedIntentResolutionCase::ExplicitSnapFlush,
    )
    .expect("report");

    assert!(report.parity_verified());
    assert_eq!(
        report.replay_row().preserved_truth(),
        PrimitiveConstructionPreservedIntentTruth::Resolved {
            candidate: SpatialIntentCandidate::SnapFlush,
            authority: PrimitiveConstructionChosenIntentResolutionAuthority::ExplicitChoice,
        }
    );
}

#[test]
fn arbitration_replay_parity_preserves_blocked_unresolved_truth() {
    let report = prepare_primitive_construction_intent_arbitration_replay_parity_report(
        PrimitiveConstructionPreservedIntentResolutionCase::HostPenetrationBlockedCut,
    )
    .expect("report");

    assert!(report.parity_verified());
    assert_eq!(
        report.direct_row().preserved_truth(),
        PrimitiveConstructionPreservedIntentTruth::Unresolved {
            escalation: SpatialIntentEscalation::BlockedByMissingCapability(
                SpatialBlockedCapability::CutOpening
            ),
            blocked_capability: Some(SpatialBlockedCapability::CutOpening),
        }
    );
}
