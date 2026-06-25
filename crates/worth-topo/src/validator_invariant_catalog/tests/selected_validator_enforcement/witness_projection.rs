use crate::validator_invariant_catalog::{
    WorthTopologyLoopWiringViolationKind, WorthTopologySelectedValidatorEnforcementCloseout,
    WorthTopologySelectedValidatorEnforcementOutcome,
};

use super::selected_cases::{
    duplicate_half_edge_admitted_facts, passing_admitted_facts, selected_loop_wiring_closeout,
    selected_loop_wiring_row,
};

#[test]
fn loop_wiring_violation_receipt_carries_touched_witness_identity() {
    let selection = selected_loop_wiring_closeout();
    let selected_row = selected_loop_wiring_row(&selection);
    let admitted_facts = duplicate_half_edge_admitted_facts(selected_row);

    let closeout =
        WorthTopologySelectedValidatorEnforcementCloseout::execute_loop_wiring_family_from_admitted_facts(
            &selection,
            &admitted_facts,
        )
        .expect("duplicate half-edge should produce violation receipt");

    let WorthTopologySelectedValidatorEnforcementOutcome::Violation(witness) =
        closeout.enforcement_receipt().outcome()
    else {
        panic!("expected loop wiring violation witness");
    };
    assert_eq!(
        witness.violation_kind(),
        WorthTopologyLoopWiringViolationKind::DuplicateHalfEdgeInLoop
    );
    assert_eq!(witness.validator(), "loop_wiring");
    assert_eq!(
        closeout.diagnostic_projection().violation_kind(),
        Some(witness.violation_kind())
    );
    assert_eq!(
        closeout.diagnostic_projection().touched_loop_id(),
        witness.touched_loop_id()
    );
    assert_eq!(
        closeout.diagnostic_projection().touched_half_edge_id(),
        witness.touched_half_edge_id()
    );
    assert_eq!(
        closeout
            .diagnostic_projection()
            .diagnostic_projection_digest(),
        closeout
            .enforcement_receipt()
            .diagnostic_projection_digest()
    );
    assert_eq!(
        closeout.enforcement_receipt().counters().violation_count(),
        1
    );
}

#[test]
fn diagnostic_projection_digest_changes_with_witness_outcome() {
    let selection = selected_loop_wiring_closeout();
    let selected_row = selected_loop_wiring_row(&selection);
    let passing = passing_admitted_facts(selected_row);
    let violating = duplicate_half_edge_admitted_facts(selected_row);

    let passing_closeout =
        WorthTopologySelectedValidatorEnforcementCloseout::execute_loop_wiring_family_from_admitted_facts(
            &selection, &passing,
        )
        .expect("passing witness should close");
    let violating_closeout =
        WorthTopologySelectedValidatorEnforcementCloseout::execute_loop_wiring_family_from_admitted_facts(
            &selection, &violating,
        )
        .expect("violating witness should close");

    assert_eq!(
        passing_closeout.diagnostic_projection().violation_kind(),
        None
    );
    assert_eq!(
        violating_closeout.diagnostic_projection().violation_kind(),
        Some(WorthTopologyLoopWiringViolationKind::DuplicateHalfEdgeInLoop)
    );
    assert_ne!(
        passing_closeout
            .enforcement_receipt()
            .diagnostic_projection_digest(),
        violating_closeout
            .enforcement_receipt()
            .diagnostic_projection_digest()
    );
}
