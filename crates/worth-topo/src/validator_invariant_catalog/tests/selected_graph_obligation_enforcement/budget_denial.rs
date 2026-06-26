use forge_query::facade::{
    ForgeQueryGraphObligationExecutionResultRow, ForgeQueryGraphObligationExecutionStatus,
    ForgeQueryGraphObligationStateLoadCounters,
};

use super::super::relational_invariant_catalog::execution_inputs::{
    envelope_from_input_rows, relational_invariant_query_execution_input_for_rewire_slot_with_rows,
};
use crate::validator_invariant_catalog::{
    WorthTopologySelectedGraphObligationEnforcementCloseout,
    WorthTopologySelectedGraphObligationEnforcementOutcome,
};

#[test]
fn budget_exceeded_query_rows_are_counted_as_budget_denials() {
    let (relational_closeout, execution_input) =
        relational_invariant_query_execution_input_for_rewire_slot_with_rows(35, |dispatch| {
            envelope_from_input_rows(dispatch, |input| {
                ForgeQueryGraphObligationExecutionResultRow::new(
                    input,
                    ForgeQueryGraphObligationExecutionStatus::BudgetExceeded,
                    None,
                    ForgeQueryGraphObligationStateLoadCounters::new(0, 0, 0),
                )
            })
        });

    let closeout =
        WorthTopologySelectedGraphObligationEnforcementCloseout::execute_from_relational_invariant_closeout(
            &relational_closeout,
            execution_input,
        )
        .expect("budget exceeded Query execution envelope should close with denial receipts");

    assert_eq!(
        closeout.counters().budget_denial_count(),
        closeout.enforcement_receipts().len()
    );
    assert_eq!(
        closeout.counters().denied_before_execution_count(),
        closeout.enforcement_receipts().len()
    );
    assert_eq!(closeout.counters().passed_count(), 0);
    assert!(closeout.enforcement_receipts().iter().all(|receipt| {
        matches!(
            receipt.outcome(),
            WorthTopologySelectedGraphObligationEnforcementOutcome::DeniedBeforeExecution(_)
        ) && receipt.query_execution_status() == "budget-exceeded"
            && !receipt.query_execution_budget_digest().is_empty()
            && !receipt.query_support_posture_digest().is_empty()
    }));
}
