use super::super::relational_invariant_catalog::execution_inputs::{
    envelope_from_input_rows, relational_invariant_query_execution_input,
    relational_invariant_query_execution_input_for_rewire_slot_with_rows,
};
use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalogError, WorthTopologySelectedGraphObligationEnforcementCloseout,
    WorthTopologySelectedGraphObligationEnforcementDenialKind,
    WorthTopologySelectedGraphObligationEnforcementOutcome,
};
use forge_query::facade::{
    ForgeQueryGraphObligationExecutionResultRow, ForgeQueryGraphObligationExecutionStatus,
};

#[test]
fn selected_invariant_families_close_with_query_execution_backed_receipts() {
    let (relational_closeout, execution_input) = relational_invariant_query_execution_input();
    let closeout =
        WorthTopologySelectedGraphObligationEnforcementCloseout::execute_from_relational_invariant_closeout(
            &relational_closeout,
            execution_input,
        )
        .expect("Phase 6 should close from Query execution-backed evidence");

    assert_eq!(
        closeout.enforcement_receipts().len(),
        relational_closeout.selected_validator_family_rows().len()
            + relational_closeout.selected_invariant_family_rows().len()
    );
    assert_eq!(
        closeout.counters().selected_validator_family_count(),
        relational_closeout.selected_validator_family_rows().len()
    );
    assert_eq!(
        closeout.counters().selected_invariant_family_count(),
        relational_closeout.selected_invariant_family_rows().len()
    );
    assert_eq!(
        closeout.counters().selected_obligation_family_count(),
        closeout.enforcement_receipts().len()
    );
    assert_eq!(
        closeout.counters().passed_count(),
        closeout.enforcement_receipts().len()
    );
    assert_eq!(
        closeout.counters().enforcement_receipt_count(),
        closeout.enforcement_receipts().len()
    );
    assert_eq!(
        closeout.counters().executor_row_count(),
        closeout.query_execution_rows().len()
    );
    assert_eq!(closeout.counters().support_pin_count(), 1);
    assert_eq!(closeout.counters().adoption_manifest_count(), 1);
    assert_eq!(closeout.counters().residue_manifest_count(), 1);
    assert_eq!(closeout.counters().budget_denial_count(), 0);
    assert_eq!(closeout.counters().caller_owned_graph_work_count(), 0);
    assert_eq!(closeout.counters().violation_count(), 0);
    assert_eq!(closeout.counters().denied_before_execution_count(), 0);
    assert_eq!(
        closeout.phase_seven_seed().receipt_count(),
        closeout.enforcement_receipts().len()
    );
    assert_eq!(
        closeout
            .phase_seven_seed()
            .query_execution_envelope_digest(),
        closeout.query_execution_envelope_digest()
    );
    assert!(
        closeout.proof_projection().executor_row_count() >= closeout.enforcement_receipts().len()
    );
    assert_eq!(
        closeout.phase_seven_seed().support_pin_digest(),
        closeout.proof_projection().support_pin_digest()
    );
    assert_eq!(
        closeout.phase_seven_seed().support_matrix_digest(),
        closeout.proof_projection().support_matrix_digest()
    );
    assert_eq!(
        closeout.phase_seven_seed().residue_manifest_digest(),
        closeout.proof_projection().residue_manifest_digest()
    );
    assert_eq!(
        closeout.phase_seven_seed().local_ceremony_audit_digest(),
        closeout.proof_projection().local_ceremony_audit_digest()
    );
    assert_eq!(
        closeout.phase_seven_seed().in_memory_proof_digest(),
        closeout.proof_projection().in_memory_proof_digest()
    );
    assert_eq!(
        closeout.phase_seven_seed().execution_proof_digest(),
        closeout.proof_projection().execution_proof_digest()
    );
    assert!(closeout.source_firewall().is_clean());
    assert!(closeout.enforcement_receipts().iter().all(|receipt| {
        matches!(
            receipt.outcome(),
            WorthTopologySelectedGraphObligationEnforcementOutcome::Passed
        ) && receipt.query_execution_envelope_digest() == closeout.query_execution_envelope_digest()
            && receipt.query_execution_status() == "executed"
            && receipt.query_support_lane() == "worth-topo-operator-catalog"
            && receipt.query_support_status() == "supported"
            && receipt.query_execution_cost_class() == "selection-only"
            && receipt.query_execution_scope() == "selection-only"
            && receipt.query_budget_exceeded_policy() == "deferred-to-backstop"
            && receipt.query_diagnostic_materialization() == "bounded-evidence-only"
            && !receipt.query_support_posture_digest().is_empty()
            && !receipt.query_execution_budget_digest().is_empty()
    }));
}

#[test]
fn swapped_query_execution_envelope_is_rejected_before_receipt_projection() {
    let (relational_closeout, execution_input) = relational_invariant_query_execution_input();
    let (_, mismatched_input) =
        relational_invariant_query_execution_input_for_rewire_slot_with_rows(31, |dispatch| {
            envelope_from_input_rows(dispatch, |input| {
                ForgeQueryGraphObligationExecutionResultRow::status_only(
                    input,
                    ForgeQueryGraphObligationExecutionStatus::SuppressedByPolicy,
                )
            })
        });
    let mismatched_input =
        crate::validator_invariant_catalog::WorthTopologySelectedGraphObligationExecutionInput::from_query_authority(
            mismatched_input.query_execution_envelope().clone(),
            execution_input.execution_backed_adoption_proof().clone(),
        );

    let error =
        WorthTopologySelectedGraphObligationEnforcementCloseout::execute_from_relational_invariant_closeout(
            &relational_closeout,
            mismatched_input,
        )
        .unwrap_err();

    let WorthTopologyLegalityCatalogError::PhaseSixGraphObligationEnforcement(denial) = error
    else {
        panic!("expected Phase 6 graph obligation enforcement denial");
    };
    assert_eq!(
        denial.kind(),
        WorthTopologySelectedGraphObligationEnforcementDenialKind::ExecutionEnvelopeMismatch
    );
}
