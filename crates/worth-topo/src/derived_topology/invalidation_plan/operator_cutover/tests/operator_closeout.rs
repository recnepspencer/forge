use super::super::super::migrated_products::CoveredDerivedProductMigrationError;
use super::super::{
    close_derived_invalidation_operator_cutover, DerivedInvalidationOperatorCutoverErrorKind,
    DerivedInvalidationOperatorCutoverReceipt,
};
use super::support::{
    admitted_operator_evidence, execution_receipt, full_phase_six_closeout,
    matching_operator_touch_proof, mismatched_operator_touch_proof,
    missing_graph_obligation_evidence, partial_phase_six_closeout, real_operator_artifact,
    selected_plan, selected_plan_from_operator_artifact,
};

#[test]
fn operator_cutover_binds_real_operator_artifact_phase_six_sweep_and_execution_receipt() {
    let artifact = real_operator_artifact();
    let selected_plan = selected_plan_from_operator_artifact(&artifact);
    let execution_receipt = execution_receipt(&selected_plan);
    let phase_six_closeout = full_phase_six_closeout(&selected_plan);

    let operator_cutover = artifact
        .bind_derived_invalidation_cutover(&phase_six_closeout, &selected_plan, &execution_receipt)
        .expect("operator cutover should bind complete proofs");
    let closeout = close_derived_invalidation_operator_cutover(operator_cutover)
        .expect("operator cutover closeout");

    assert_eq!(closeout.counters().denied_product_count(), 0);
    assert_eq!(closeout.counters().projection_dirty_expansion_count(), 0);
    assert_eq!(
        closeout.operator_cutover().operator_touched_basis_digest(),
        artifact.declared_touched_basis().basis_digest()
    );
    assert_eq!(
        closeout
            .phase_eight_seed()
            .operator_cutover_receipt_digest(),
        closeout.operator_cutover().receipt_digest()
    );
    assert_eq!(
        closeout.phase_eight_seed().selected_plan_digest(),
        closeout.operator_cutover().selected_plan_digest()
    );
    assert_eq!(
        closeout.phase_eight_seed().execution_receipt_digest(),
        closeout.operator_cutover().execution_receipt_digest()
    );
    assert_eq!(
        closeout.phase_eight_seed().touched_closure_digest(),
        closeout.operator_cutover().touched_closure_digest()
    );
    assert_eq!(
        closeout.phase_eight_seed().query_support_digest(),
        closeout.operator_cutover().query_support_digest()
    );
    assert_eq!(
        closeout.phase_eight_seed().legality_support_digest(),
        closeout.operator_cutover().legality_support_digest()
    );
}

#[test]
fn partial_phase_six_product_sweep_cannot_start_operator_cutover() {
    let selected_plan = selected_plan();
    let error = partial_phase_six_closeout(&selected_plan).expect_err("partial sweep cannot close");

    assert_eq!(
        error,
        CoveredDerivedProductMigrationError::RequiredFamilyNotOrdinaryConsumable
    );
}

#[test]
fn operator_cutover_rejects_touched_basis_that_does_not_match_execution_closure() {
    let selected_plan = selected_plan();
    let execution_receipt = execution_receipt(&selected_plan);
    let phase_six_closeout = full_phase_six_closeout(&selected_plan);
    let error = DerivedInvalidationOperatorCutoverReceipt::bind_operator_cutover(
        &phase_six_closeout,
        &selected_plan,
        &execution_receipt,
        &mismatched_operator_touch_proof(),
        &admitted_operator_evidence(),
    )
    .expect_err("mismatched touched authority must deny");

    assert_eq!(
        error.kind(),
        DerivedInvalidationOperatorCutoverErrorKind::OperatorTouchedBasisDoesNotMatchExecutionReceipt
    );
}

#[test]
fn operator_cutover_rejects_missing_graph_obligation_proof() {
    let selected_plan = selected_plan();
    let execution_receipt = execution_receipt(&selected_plan);
    let phase_six_closeout = full_phase_six_closeout(&selected_plan);
    let error = DerivedInvalidationOperatorCutoverReceipt::bind_operator_cutover(
        &phase_six_closeout,
        &selected_plan,
        &execution_receipt,
        &matching_operator_touch_proof(),
        &missing_graph_obligation_evidence(),
    )
    .expect_err("operator cutover requires graph-obligation evidence");

    assert_eq!(
        error.kind(),
        DerivedInvalidationOperatorCutoverErrorKind::MissingOperatorGraphObligationProof
    );
}
