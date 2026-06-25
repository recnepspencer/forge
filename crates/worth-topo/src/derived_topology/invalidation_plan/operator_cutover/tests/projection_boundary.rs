use super::super::{
    DerivedInvalidationOperatorCutoverErrorKind, DerivedInvalidationOperatorCutoverReceipt,
    DerivedInvalidationProjectionReadStageReceipt, ProjectionReadStageConsumptionScope,
};
use super::support::{
    admitted_operator_evidence, execution_receipt, full_phase_six_closeout,
    matching_operator_touch_proof, selected_plan,
};
use crate::projection::runtime_boundary::read_stage::consume_derived_invalidation_for_projection_read_stage;

#[test]
fn projection_read_stage_consumes_cutover_receipt_without_expanding_dirty_scope() {
    let selected_plan = selected_plan();
    let execution_receipt = execution_receipt(&selected_plan);
    let operator_cutover = DerivedInvalidationOperatorCutoverReceipt::bind_operator_cutover(
        &full_phase_six_closeout(&selected_plan),
        &selected_plan,
        &execution_receipt,
        &matching_operator_touch_proof(),
        &admitted_operator_evidence(),
    )
    .expect("operator cutover");

    let projection_receipt =
        consume_derived_invalidation_for_projection_read_stage(&operator_cutover)
            .expect("projection read-stage receipt");

    assert_eq!(
        projection_receipt.execution_receipt_digest(),
        operator_cutover.execution_receipt_digest()
    );
    assert_eq!(projection_receipt.projection_dirty_expansion_count(), 0);
}

#[test]
fn projection_read_stage_cannot_expand_dirty_scope_after_selected_plan() {
    let selected_plan = selected_plan();
    let execution_receipt = execution_receipt(&selected_plan);
    let operator_cutover = DerivedInvalidationOperatorCutoverReceipt::bind_operator_cutover(
        &full_phase_six_closeout(&selected_plan),
        &selected_plan,
        &execution_receipt,
        &matching_operator_touch_proof(),
        &admitted_operator_evidence(),
    )
    .expect("operator cutover");

    let error = DerivedInvalidationProjectionReadStageReceipt::consume_operator_cutover(
        &operator_cutover,
        ProjectionReadStageConsumptionScope::CommittedRead,
        1,
    )
    .expect_err("projection dirty expansion must deny");

    assert_eq!(
        error.kind(),
        DerivedInvalidationOperatorCutoverErrorKind::ProjectionReadStageScopeExpandedDirtyProducts
    );
}
