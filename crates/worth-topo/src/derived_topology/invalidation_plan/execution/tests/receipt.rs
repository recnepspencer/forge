use super::super::{
    DerivedInvalidationDiagnosticProjection, DerivedInvalidationExecutionErrorKind,
    DerivedInvalidationExecutionOutcome, DerivedInvalidationExecutionReceipt,
};
use super::support::{
    selected_loop_cycles_plan, whole_view_materialization_report, MeasuredExecutionExecutor,
};
use crate::derived_topology::invalidation_plan::catalog::{
    DerivedTopologyLegalityReceiptPosture, DerivedTopologyQueryReceiptPosture,
};
use crate::derived_topology::invalidation_plan::selection::selection_test_fixtures::{
    admitted_query_support, catalog_closeout_with_loop_cycles_postures,
    legality_support_missing_selected_legality_plan, loop_cycles_touched_closure,
};
use crate::derived_topology::invalidation_plan::selection::{
    DerivedInvalidationDensityPolicy, DerivedInvalidationSelectedPlan,
};

#[test]
fn execution_receipt_is_deterministic_from_selected_plan() {
    let plan = selected_loop_cycles_plan();
    let first = DerivedInvalidationExecutionReceipt::execute_selected_plan(&plan).unwrap();
    let second = DerivedInvalidationExecutionReceipt::execute_selected_plan(&plan).unwrap();
    let first_diagnostics = DerivedInvalidationDiagnosticProjection::from_execution_receipt(&first);
    let second_diagnostics =
        DerivedInvalidationDiagnosticProjection::from_execution_receipt(&second);

    assert_eq!(
        first.execution_receipt_digest(),
        second.execution_receipt_digest()
    );
    assert_eq!(
        first.counters().counters_digest(),
        second.counters().counters_digest()
    );
    assert_eq!(
        first_diagnostics.diagnostic_projection_digest(),
        second_diagnostics.diagnostic_projection_digest()
    );
    assert_eq!(first_diagnostics.rows(), second_diagnostics.rows());
}

#[test]
fn selected_plan_rows_project_into_execution_receipt_without_reselection() {
    let plan = selected_loop_cycles_plan();
    let receipt = DerivedInvalidationExecutionReceipt::execute_selected_plan(&plan).unwrap();

    assert_eq!(receipt.selected_plan_digest(), plan.selected_plan_digest());
    assert_eq!(
        receipt.phase_four_seed_digest(),
        plan.phase_four_seed().seed_digest()
    );
    assert_eq!(receipt.executed_rows().len(), plan.selected_rows().len());
    assert_eq!(
        receipt.unaffected_rows().len(),
        plan.unaffected_rows().len()
    );
    assert_eq!(receipt.denied_rows().len(), plan.denied_rows().len());
    assert_eq!(receipt.residue_rows().len(), plan.residue_rows().len());
    assert_eq!(
        receipt.counters().incremental_update_count(),
        plan.counters().incremental_update_count()
    );
    assert_eq!(
        receipt.counters().bounded_rebuild_count(),
        plan.counters().bounded_rebuild_count()
    );
}

#[test]
fn execution_receipt_binds_product_execution_report_work() {
    let plan = selected_loop_cycles_plan();
    let default_receipt = DerivedInvalidationExecutionReceipt::execute_selected_plan_with_executor(
        &plan,
        &MeasuredExecutionExecutor::new(1, 0, None),
    )
    .unwrap();
    let higher_work_receipt =
        DerivedInvalidationExecutionReceipt::execute_selected_plan_with_executor(
            &plan,
            &MeasuredExecutionExecutor::new(2, 0, None),
        )
        .unwrap();

    assert_ne!(
        default_receipt.execution_receipt_digest(),
        higher_work_receipt.execution_receipt_digest()
    );
    assert_eq!(
        higher_work_receipt.executed_rows()[0].execution_work_count(),
        2
    );
    assert!(!higher_work_receipt.executed_rows()[0]
        .execution_report_digest()
        .is_empty());
}

#[test]
fn denied_selected_plan_produces_denial_proof_without_successful_execution_rows() {
    let catalog = catalog_closeout_with_loop_cycles_postures(
        DerivedTopologyQueryReceiptPosture::NativeReadReceiptRequired,
        DerivedTopologyLegalityReceiptPosture::SelectedLegalityReceiptRequired,
    );
    let plan = DerivedInvalidationSelectedPlan::lower(
        &catalog,
        &loop_cycles_touched_closure("loop-touch"),
        &admitted_query_support(),
        &legality_support_missing_selected_legality_plan(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap();
    let receipt = DerivedInvalidationExecutionReceipt::execute_selected_plan(&plan).unwrap();

    assert!(receipt.executed_rows().is_empty());
    assert!(!receipt.denied_rows().is_empty());
    assert_eq!(receipt.counters().executed_product_count(), 0);
    assert_eq!(
        receipt.counters().denied_product_count(),
        plan.denied_rows().len()
    );
}

#[test]
fn execution_receipt_proves_no_hidden_graph_work_or_whole_view_fallback() {
    let receipt =
        DerivedInvalidationExecutionReceipt::execute_selected_plan(&selected_loop_cycles_plan())
            .unwrap();

    assert_eq!(receipt.counters().caller_owned_graph_work_count(), 0);
    assert_eq!(receipt.counters().whole_view_fallback_count(), 0);
}

#[test]
fn execution_path_rejects_incremental_whole_view_fallback() {
    let error = DerivedInvalidationExecutionReceipt::execute_selected_plan_with_executor(
        &selected_loop_cycles_plan(),
        &MeasuredExecutionExecutor::new(1, 0, Some(whole_view_materialization_report())),
    )
    .unwrap_err();

    assert_eq!(
        error.kind(),
        DerivedInvalidationExecutionErrorKind::OrdinaryWholeViewFallbackNotAdmitted
    );
}

#[test]
fn execution_path_rejects_caller_owned_graph_work() {
    let error = DerivedInvalidationExecutionReceipt::execute_selected_plan_with_executor(
        &selected_loop_cycles_plan(),
        &MeasuredExecutionExecutor::new(1, 1, None),
    )
    .unwrap_err();

    assert_eq!(
        error.kind(),
        DerivedInvalidationExecutionErrorKind::CallerOwnedGraphWorkNotAdmitted
    );
}

#[test]
fn diagnostics_localize_every_execution_outcome_to_a_source_row() {
    let receipt =
        DerivedInvalidationExecutionReceipt::execute_selected_plan(&selected_loop_cycles_plan())
            .unwrap();
    let diagnostics = DerivedInvalidationDiagnosticProjection::from_execution_receipt(&receipt);

    assert_eq!(
        diagnostics.rows().len(),
        receipt.counters().diagnostic_row_count()
    );
    assert!(diagnostics.rows().iter().all(|row| {
        row.selected_plan_digest() == receipt.selected_plan_digest()
            && row.execution_receipt_digest() == receipt.execution_receipt_digest()
            && row.touched_closure_digest() == receipt.touched_closure_digest()
            && row.query_support_digest() == receipt.query_support_digest()
            && row.legality_support_digest() == receipt.legality_support_digest()
            && !row.source_row_digest().is_empty()
            && !row.reason().is_empty()
    }));
    assert!(diagnostics.rows().iter().any(|row| row.outcome()
        == DerivedInvalidationExecutionOutcome::ResidueCapped
        && row.residue_label().is_some()));
    assert!(diagnostics
        .rows()
        .iter()
        .any(|row| row.family_digest().is_some()
            && row.query_receipt_digest().is_some()
            && row.legality_receipt_digest().is_some()
            && row.execution_report_digest().is_some()));
}
