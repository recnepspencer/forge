use crate::validator_invariant_catalog::{
    WorthTopologyLoopWiringViolationKind, WorthTopologySelectedValidatorEnforcementCloseout,
    WorthTopologySelectedValidatorEnforcementOutcome,
};

use super::selected_cases::{
    passing_admitted_facts, passing_admitted_facts_with_outside_rejections,
    selected_loop_wiring_closeout, selected_loop_wiring_row, unreciprocated_next_admitted_facts,
    whole_view_oracle_passes_with_unrelated_broken_loop,
};

#[test]
fn bounded_witness_rows_define_loop_wiring_execution_breadth() {
    let selection = selected_loop_wiring_closeout();
    let selected_row = selected_loop_wiring_row(&selection);
    let admitted_facts = passing_admitted_facts(selected_row);

    let closeout =
        WorthTopologySelectedValidatorEnforcementCloseout::execute_loop_wiring_family_from_admitted_facts(
            &selection,
            &admitted_facts,
        )
        .expect("bounded admitted facts should close");

    assert_eq!(
        closeout
            .enforcement_receipt()
            .counters()
            .witness_half_edge_row_count(),
        2
    );
    assert_eq!(closeout.witness_intake_receipt().half_edge_fact_count(), 2);
    assert_eq!(
        closeout
            .enforcement_receipt()
            .counters()
            .direct_materialized_report_row_read_count(),
        0
    );
}

#[test]
fn unrelated_whole_view_breakage_does_not_poison_selected_loop_wiring_witness() {
    let selection = selected_loop_wiring_closeout();
    let selected_row = selected_loop_wiring_row(&selection);
    let admitted_facts = passing_admitted_facts_with_outside_rejections(selected_row);

    let closeout =
        WorthTopologySelectedValidatorEnforcementCloseout::execute_loop_wiring_family_from_admitted_facts(
            &selection,
            &admitted_facts,
        )
        .expect("selected facts should close independently of unrelated topology rows");

    assert!(!whole_view_oracle_passes_with_unrelated_broken_loop(
        &admitted_facts
    ));
    assert_eq!(
        closeout
            .witness_intake_receipt()
            .rejected_outside_loop_fact_count(),
        1
    );
    assert_eq!(
        closeout
            .witness_intake_receipt()
            .rejected_outside_half_edge_fact_count(),
        2
    );
    assert!(matches!(
        closeout.enforcement_receipt().outcome(),
        WorthTopologySelectedValidatorEnforcementOutcome::Passed
    ));
    assert_eq!(
        closeout
            .enforcement_receipt()
            .counters()
            .whole_view_validation_call_count(),
        0
    );
}

#[test]
fn touched_witness_violation_affects_result_without_global_scan() {
    let selection = selected_loop_wiring_closeout();
    let selected_row = selected_loop_wiring_row(&selection);
    let admitted_facts = unreciprocated_next_admitted_facts(selected_row);

    let closeout =
        WorthTopologySelectedValidatorEnforcementCloseout::execute_loop_wiring_family_from_admitted_facts(
            &selection,
            &admitted_facts,
        )
        .expect("local unreciprocated link should close as violation");

    let WorthTopologySelectedValidatorEnforcementOutcome::Violation(witness) =
        closeout.enforcement_receipt().outcome()
    else {
        panic!("expected violation");
    };
    assert_eq!(
        witness.violation_kind(),
        WorthTopologyLoopWiringViolationKind::UnreciprocatedNextLink
    );
    assert_eq!(
        closeout
            .enforcement_receipt()
            .counters()
            .whole_view_validation_call_count(),
        0
    );
}
