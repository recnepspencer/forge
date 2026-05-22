use super::{
    prepare_primitive_construction_preserved_intent_resolution_report,
    PrimitiveConstructionPreservedIntentResolutionCase, PrimitiveConstructionPreservedIntentTruth,
};
use crate::construction::PrimitiveConstructionChosenIntentResolutionAuthority;
use worth_spatial::facade::{
    SpatialBlockedCapability, SpatialIntentCandidate, SpatialIntentEscalation,
};

#[test]
fn preserved_intent_report_keeps_policy_auto_resolve_truth() {
    let report =
        prepare_primitive_construction_preserved_intent_resolution_report().expect("report");
    let row = report
        .row(PrimitiveConstructionPreservedIntentResolutionCase::PolicyMoveOnly)
        .expect("row");

    assert_eq!(
        row.preserved_truth(),
        PrimitiveConstructionPreservedIntentTruth::Resolved {
            candidate: SpatialIntentCandidate::MoveOnly,
            authority: PrimitiveConstructionChosenIntentResolutionAuthority::PolicyAutoResolve,
        }
    );
}

#[test]
fn preserved_intent_report_keeps_explicit_choice_truth() {
    let report =
        prepare_primitive_construction_preserved_intent_resolution_report().expect("report");
    let row = report
        .row(PrimitiveConstructionPreservedIntentResolutionCase::ExplicitSnapFlush)
        .expect("row");

    assert_eq!(
        row.preserved_truth(),
        PrimitiveConstructionPreservedIntentTruth::Resolved {
            candidate: SpatialIntentCandidate::SnapFlush,
            authority: PrimitiveConstructionChosenIntentResolutionAuthority::ExplicitChoice,
        }
    );
}

#[test]
fn preserved_intent_report_keeps_unresolved_blocked_truth() {
    let report =
        prepare_primitive_construction_preserved_intent_resolution_report().expect("report");
    let row = report
        .row(PrimitiveConstructionPreservedIntentResolutionCase::HostPenetrationBlockedCut)
        .expect("row");

    assert_eq!(
        row.preserved_truth(),
        PrimitiveConstructionPreservedIntentTruth::Unresolved {
            escalation: SpatialIntentEscalation::BlockedByMissingCapability(
                SpatialBlockedCapability::CutOpening
            ),
            blocked_capability: Some(SpatialBlockedCapability::CutOpening),
        }
    );
}

#[test]
fn preserved_intent_report_keeps_unresolved_ask_truth() {
    let report =
        prepare_primitive_construction_preserved_intent_resolution_report().expect("report");
    let row = report
        .row(PrimitiveConstructionPreservedIntentResolutionCase::GrazingClarificationRequired)
        .expect("row");

    assert_eq!(
        row.preserved_truth(),
        PrimitiveConstructionPreservedIntentTruth::Unresolved {
            escalation: SpatialIntentEscalation::AskForClarification,
            blocked_capability: None,
        }
    );
}
