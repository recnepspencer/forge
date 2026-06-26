use forge_query::facade::{
    ForgeQueryGraphObligationExecutionResultRow, ForgeQueryGraphObligationExecutionStatus,
    ForgeQueryGraphObligationStateLoadCounters, ForgeQueryGraphObligationVerdict,
};

use super::super::relational_invariant_catalog::execution_inputs::{
    envelope_from_input_rows, relational_invariant_query_execution_input_for_rewire_slot_with_rows,
};
use crate::validator_invariant_catalog::{
    WorthTopologySelectedGraphObligationEnforcementCloseout,
    WorthTopologySelectedGraphObligationEnforcementOutcome,
};

#[test]
fn executed_advisory_query_rows_project_to_advisory_receipts() {
    let (relational_closeout, execution_input) =
        relational_invariant_query_execution_input_for_rewire_slot_with_rows(32, |dispatch| {
            envelope_from_input_rows(dispatch, |input| {
                ForgeQueryGraphObligationExecutionResultRow::executed(
                    input,
                    ForgeQueryGraphObligationVerdict::advise("worth topo advisory proof")
                        .expect("advisory verdict should accept context"),
                    ForgeQueryGraphObligationStateLoadCounters::new(1, 2, 3),
                )
            })
        });

    let closeout =
        WorthTopologySelectedGraphObligationEnforcementCloseout::execute_from_relational_invariant_closeout(
            &relational_closeout,
            execution_input,
        )
        .expect("advisory Query execution envelope should close");

    assert_eq!(
        closeout.counters().advisory_count(),
        closeout.enforcement_receipts().len()
    );
    assert_eq!(closeout.counters().passed_count(), 0);
    assert_eq!(closeout.counters().violation_count(), 0);
    assert!(closeout.enforcement_receipts().iter().all(|receipt| {
        matches!(
            receipt.outcome(),
            WorthTopologySelectedGraphObligationEnforcementOutcome::Advisory(_)
        ) && receipt.query_execution_status() == "executed"
            && !receipt.query_state_load_counters_digest().is_empty()
            && receipt.diagnostic_witness_digest().is_some()
    }));
}

#[test]
fn executed_blocking_query_rows_project_to_violation_receipts() {
    let (relational_closeout, execution_input) =
        relational_invariant_query_execution_input_for_rewire_slot_with_rows(33, |dispatch| {
            envelope_from_input_rows(dispatch, |input| {
                ForgeQueryGraphObligationExecutionResultRow::executed(
                    input,
                    ForgeQueryGraphObligationVerdict::block("worth topo violation proof")
                        .expect("blocking verdict should accept context"),
                    ForgeQueryGraphObligationStateLoadCounters::new(1, 1, 1),
                )
            })
        });

    let closeout =
        WorthTopologySelectedGraphObligationEnforcementCloseout::execute_from_relational_invariant_closeout(
            &relational_closeout,
            execution_input,
        )
        .expect("blocking Query execution envelope should close");

    assert_eq!(
        closeout.counters().violation_count(),
        closeout.enforcement_receipts().len()
    );
    assert_eq!(closeout.counters().passed_count(), 0);
    assert_eq!(closeout.counters().advisory_count(), 0);
    assert!(closeout.enforcement_receipts().iter().all(|receipt| {
        matches!(
            receipt.outcome(),
            WorthTopologySelectedGraphObligationEnforcementOutcome::Violation(_)
        ) && receipt.query_execution_status() == "executed"
            && receipt.diagnostic_witness_digest().is_some()
    }));
}

#[test]
fn pre_execution_query_statuses_project_to_denied_receipts() {
    let (relational_closeout, execution_input) =
        relational_invariant_query_execution_input_for_rewire_slot_with_rows(34, |dispatch| {
            envelope_from_input_rows(dispatch, |input| {
                ForgeQueryGraphObligationExecutionResultRow::status_only(
                    input,
                    ForgeQueryGraphObligationExecutionStatus::SuppressedByPolicy,
                )
            })
        });

    let closeout =
        WorthTopologySelectedGraphObligationEnforcementCloseout::execute_from_relational_invariant_closeout(
            &relational_closeout,
            execution_input,
        )
        .expect("suppressed Query execution envelope should close with denied receipts");

    assert_eq!(
        closeout.counters().denied_before_execution_count(),
        closeout.enforcement_receipts().len()
    );
    assert_eq!(closeout.counters().passed_count(), 0);
    assert!(closeout.enforcement_receipts().iter().all(|receipt| {
        matches!(
            receipt.outcome(),
            WorthTopologySelectedGraphObligationEnforcementOutcome::DeniedBeforeExecution(_)
        ) && receipt.query_execution_status() == "suppressed-by-policy"
            && receipt.diagnostic_witness_digest().is_none()
    }));
}
