use super::super::relational_invariant_catalog::execution_inputs::{
    relational_invariant_query_execution_input,
    relational_invariant_query_execution_input_for_loop_successor_program_slot,
};
use crate::validator_invariant_catalog::{
    WorthTopologyMilestoneNineCloseout, WorthTopologyOperatorCertificationCutoverCloseout,
    WorthTopologySelectedGraphObligationEnforcementCloseout,
};

pub(super) fn operator_cutover_closeout() -> WorthTopologyOperatorCertificationCutoverCloseout {
    let (relational_closeout, execution_input) = relational_invariant_query_execution_input();
    operator_cutover_closeout_from_parts(relational_closeout, execution_input)
}

pub(super) fn alternate_operator_cutover_closeout(
) -> WorthTopologyOperatorCertificationCutoverCloseout {
    let (relational_closeout, execution_input) =
        relational_invariant_query_execution_input_for_loop_successor_program_slot(40);
    operator_cutover_closeout_from_parts(relational_closeout, execution_input)
}

fn operator_cutover_closeout_from_parts(
    relational_closeout: crate::validator_invariant_catalog::WorthTopologyRelationalInvariantCatalogCloseout,
    execution_input: crate::validator_invariant_catalog::WorthTopologySelectedGraphObligationExecutionInput,
) -> WorthTopologyOperatorCertificationCutoverCloseout {
    let enforcement =
        WorthTopologySelectedGraphObligationEnforcementCloseout::execute_from_relational_invariant_closeout(
            &relational_closeout,
            execution_input,
        )
        .expect("Phase 6 selected graph obligation enforcement should close");
    WorthTopologyOperatorCertificationCutoverCloseout::from_selected_graph_obligation_enforcement(
        &enforcement,
    )
    .expect("Phase 7 operator certification cutover should close")
}

pub(super) fn milestone_nine_closeout() -> WorthTopologyMilestoneNineCloseout {
    let cutover = operator_cutover_closeout();
    WorthTopologyMilestoneNineCloseout::from_operator_cutover(cutover.phase_eight_seed(), &cutover)
        .expect("Milestone 9 closeout should certify the Phase 8 cutover")
}
