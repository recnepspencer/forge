use crate::validator_invariant_catalog::selected_validator_enforcement::loop_wiring::{
    WorthTopologyLoopWiringAdmittedLocalFacts, WorthTopologyLoopWiringWitnessInput,
    WorthTopologyLoopWiringWitnessIntakeReceipt,
};
use crate::validator_invariant_catalog::selection_from_touched_closure::WorthTopologySelectedLegalityObligationRow;
use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalogError, WorthTopologySelectedValidatorEnforcementDenial,
};

pub(in crate::validator_invariant_catalog) fn admit_loop_wiring_witness_input(
    selected_obligation: &WorthTopologySelectedLegalityObligationRow,
    admitted_facts: &WorthTopologyLoopWiringAdmittedLocalFacts,
) -> Result<
    (
        WorthTopologyLoopWiringWitnessInput,
        WorthTopologyLoopWiringWitnessIntakeReceipt,
    ),
    WorthTopologyLegalityCatalogError,
> {
    reject_unbound_admitted_facts(selected_obligation, admitted_facts)?;
    let witness_input = WorthTopologyLoopWiringWitnessInput::from_selected_obligation_and_rows(
        selected_obligation.row_digest(),
        admitted_facts.loop_rows().iter().cloned(),
        admitted_facts.half_edge_rows().iter().cloned(),
    );
    let intake_receipt =
        WorthTopologyLoopWiringWitnessIntakeReceipt::from_admitted_facts_and_witness(
            admitted_facts,
            &witness_input,
        );
    Ok((witness_input, intake_receipt))
}

fn reject_unbound_admitted_facts(
    selected_obligation: &WorthTopologySelectedLegalityObligationRow,
    admitted_facts: &WorthTopologyLoopWiringAdmittedLocalFacts,
) -> Result<(), WorthTopologyLegalityCatalogError> {
    if admitted_facts.selected_obligation_digest() == selected_obligation.row_digest() {
        return Ok(());
    }
    Err(WorthTopologyLegalityCatalogError::PhaseFourEnforcement(
        WorthTopologySelectedValidatorEnforcementDenial::witness_input_not_bound(
            "loop_wiring",
            selected_obligation.row_digest(),
            admitted_facts.selected_obligation_digest(),
        ),
    ))
}
