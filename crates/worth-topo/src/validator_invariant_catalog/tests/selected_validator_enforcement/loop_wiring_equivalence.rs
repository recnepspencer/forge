use crate::validation::loop_wiring_rule;
use crate::validator_invariant_catalog::{
    WorthTopologySelectedValidatorEnforcementCloseout,
    WorthTopologySelectedValidatorEnforcementOutcome,
};

use super::selected_cases::{
    duplicate_half_edge_admitted_facts, old_loop_wiring_oracle_error_validator,
    old_loop_wiring_oracle_passes, passing_admitted_facts, selected_loop_wiring_closeout,
    selected_loop_wiring_row, witness_input_from_admitted_facts,
};

#[test]
fn loop_wiring_selected_execution_preserves_registered_rule_identity() {
    let selection = selected_loop_wiring_closeout();
    let selected_row = selected_loop_wiring_row(&selection);
    let admitted_facts = passing_admitted_facts(selected_row);

    let closeout =
        WorthTopologySelectedValidatorEnforcementCloseout::execute_loop_wiring_family_from_admitted_facts(
            &selection,
            &admitted_facts,
        )
        .expect("selected loop wiring execution should close");

    assert_eq!(
        closeout.enforcement_receipt().validation_rule_identity(),
        &loop_wiring_rule()
    );
    assert_eq!(
        closeout
            .witness_intake_receipt()
            .selected_obligation_digest(),
        selected_row.row_digest()
    );
    assert!(matches!(
        closeout.enforcement_receipt().outcome(),
        WorthTopologySelectedValidatorEnforcementOutcome::Passed
    ));
    assert!(closeout.enforcement_receipt().is_execution_backed());
    assert_eq!(
        closeout
            .enforcement_receipt()
            .counters()
            .whole_view_validation_call_count(),
        0
    );
}

#[test]
fn selected_loop_wiring_matches_old_validator_pass_and_violation_semantics() {
    let selection = selected_loop_wiring_closeout();
    let selected_row = selected_loop_wiring_row(&selection);
    let passing_facts = passing_admitted_facts(selected_row);
    let violating_facts = duplicate_half_edge_admitted_facts(selected_row);
    let passing = witness_input_from_admitted_facts(&passing_facts);
    let violating = witness_input_from_admitted_facts(&violating_facts);

    let passing_closeout =
        WorthTopologySelectedValidatorEnforcementCloseout::execute_loop_wiring_family_from_admitted_facts(
            &selection, &passing_facts,
        )
        .expect("passing selected witness should close");
    let violating_closeout =
        WorthTopologySelectedValidatorEnforcementCloseout::execute_loop_wiring_family_from_admitted_facts(
            &selection, &violating_facts,
        )
        .expect("violating selected witness should close");

    assert!(old_loop_wiring_oracle_passes(&passing));
    assert!(matches!(
        passing_closeout.enforcement_receipt().outcome(),
        WorthTopologySelectedValidatorEnforcementOutcome::Passed
    ));
    assert_eq!(
        old_loop_wiring_oracle_error_validator(&violating),
        Some("loop_wiring.duplicate_half_edges")
    );
    assert!(matches!(
        violating_closeout.enforcement_receipt().outcome(),
        WorthTopologySelectedValidatorEnforcementOutcome::Violation(_)
    ));
}
