use std::collections::BTreeSet;

use super::super::relational_invariant_catalog::execution_inputs::{
    relational_invariant_query_execution_input,
    relational_invariant_query_execution_input_for_loop_successor_program_slot,
};
use crate::validator_invariant_catalog::{
    WorthTopologyOperatorCertificationCutoverCloseout,
    WorthTopologySelectedGraphObligationEnforcementCloseout,
};

pub(super) fn rewire_operator_enforcement_closeout(
) -> WorthTopologySelectedGraphObligationEnforcementCloseout {
    let (relational_closeout, execution_input) = relational_invariant_query_execution_input();
    WorthTopologySelectedGraphObligationEnforcementCloseout::execute_from_relational_invariant_closeout(
        &relational_closeout,
        execution_input,
    )
    .expect("Phase 6 enforcement should close")
}

pub(super) fn loop_successor_operator_enforcement_closeout(
) -> WorthTopologySelectedGraphObligationEnforcementCloseout {
    let (relational_closeout, execution_input) =
        relational_invariant_query_execution_input_for_loop_successor_program_slot(40);
    WorthTopologySelectedGraphObligationEnforcementCloseout::execute_from_relational_invariant_closeout(
        &relational_closeout,
        execution_input,
    )
    .expect("loop-successor operator should close through selected obligations")
}

pub(super) fn worth_family_digests(
    cutover: &WorthTopologyOperatorCertificationCutoverCloseout,
) -> BTreeSet<String> {
    cutover
        .selected_obligation_closeout_rows()
        .iter()
        .map(|row| row.worth_family_identity_digest().to_string())
        .collect()
}
