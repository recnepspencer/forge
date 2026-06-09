use super::super::intent::PrimitiveConstructionIntent;
use super::super::outcome::{
    prepare_primitive_construction_outcome, GeometryRecoveryAction, GeometryRecoverySourcePosture,
    GeometryRecoveryTargetScope,
};
use super::super::specs::WireBodySpec;

#[test]
fn rejected_construction_outcome_exposes_geometry_recovery_receipts() {
    let prepared = prepare_primitive_construction_outcome(PrimitiveConstructionIntent::wire_body(
        WireBodySpec { edge_count: 2 },
    ));
    let receipts = prepared.recovery_fact_receipts();

    assert_eq!(receipts.len(), 1);
    assert_eq!(
        receipts[0].recovery_action_kind(),
        GeometryRecoveryAction::CorrectRequestFamilyOrCounts
    );
    assert_eq!(
        receipts[0].source_posture(),
        GeometryRecoverySourcePosture::RejectedConstructionOutcome
    );
    assert_eq!(receipts[0].source_family(), prepared.family());
    assert_eq!(
        receipts[0].recovery_target_scope(),
        GeometryRecoveryTargetScope::RequestFamilyOrCounts
    );
    assert!(receipts[0].resulting_binding_identity().is_none());
    assert!(receipts[0].resulting_target_identity().is_none());
    assert!(!receipts[0].fact_digest().is_empty());
}
