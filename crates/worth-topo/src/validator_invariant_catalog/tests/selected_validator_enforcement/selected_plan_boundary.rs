use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalogError, WorthTopologySelectedValidatorEnforcementCloseout,
    WorthTopologySelectedValidatorEnforcementDenialKind,
};

use super::selected_cases::{
    selected_loop_wiring_closeout, wrong_selected_obligation_admitted_facts,
};

#[test]
fn loop_wiring_execution_rejects_unbound_witness_input() {
    let selection = selected_loop_wiring_closeout();
    let admitted_facts = wrong_selected_obligation_admitted_facts();

    let error =
        WorthTopologySelectedValidatorEnforcementCloseout::execute_loop_wiring_family_from_admitted_facts(
            &selection,
            &admitted_facts,
        )
    .expect_err("witness input must be bound to selected loop wiring obligation");

    let WorthTopologyLegalityCatalogError::PhaseFourEnforcement(denial) = error else {
        panic!("expected structured Phase 4 enforcement denial");
    };
    assert_eq!(
        denial.kind(),
        WorthTopologySelectedValidatorEnforcementDenialKind::WitnessInputNotBoundToSelectedObligation
    );
    assert_eq!(denial.family(), "loop_wiring");
}
