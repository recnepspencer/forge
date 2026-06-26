use topology::derived_invalidation_execution::{
    DerivedInvalidationDeniedProductExecutionRow, DerivedInvalidationDiagnosticProjection,
    DerivedInvalidationDiagnosticRow, DerivedInvalidationExecutionCounters,
    DerivedInvalidationExecutionError, DerivedInvalidationExecutionErrorKind,
    DerivedInvalidationExecutionOutcome, DerivedInvalidationExecutionReceipt,
    DerivedInvalidationExecutedProductRow, DerivedInvalidationResidueExecutionRow,
    DerivedInvalidationUnaffectedProductExecutionRow,
};
use topology::derived_invalidation_family_catalog::DerivedTopologyProductFamilyIdentity as ExecutionContractFamilyIdentity;
use topology::derived_invalidation_selected_plan::DerivedInvalidationSelectedPlan as ExecutionContractSelectedPlan;

fn _derived_invalidation_execution_contract() {
    let _: fn(
        &ExecutionContractSelectedPlan,
    ) -> Result<DerivedInvalidationExecutionReceipt, DerivedInvalidationExecutionError> =
        DerivedInvalidationExecutionReceipt::execute_selected_plan;

    let _: fn(&DerivedInvalidationExecutionReceipt) -> &str =
        DerivedInvalidationExecutionReceipt::execution_receipt_digest;
    let _: fn(&DerivedInvalidationExecutionReceipt) -> &str =
        DerivedInvalidationExecutionReceipt::selected_plan_digest;
    let _: fn(&DerivedInvalidationExecutionReceipt) -> &str =
        DerivedInvalidationExecutionReceipt::phase_four_seed_digest;
    let _: fn(
        &DerivedInvalidationExecutionReceipt,
    ) -> &[DerivedInvalidationExecutedProductRow] =
        DerivedInvalidationExecutionReceipt::executed_rows;
    let _: fn(
        &DerivedInvalidationExecutionReceipt,
    ) -> &[DerivedInvalidationUnaffectedProductExecutionRow] =
        DerivedInvalidationExecutionReceipt::unaffected_rows;
    let _: fn(&DerivedInvalidationExecutionReceipt) -> &[DerivedInvalidationDeniedProductExecutionRow] =
        DerivedInvalidationExecutionReceipt::denied_rows;
    let _: fn(&DerivedInvalidationExecutionReceipt) -> &[DerivedInvalidationResidueExecutionRow] =
        DerivedInvalidationExecutionReceipt::residue_rows;
    let _: fn(&DerivedInvalidationExecutionReceipt) -> &DerivedInvalidationExecutionCounters =
        DerivedInvalidationExecutionReceipt::counters;

    let _: fn(DerivedInvalidationExecutionOutcome) -> &'static str =
        DerivedInvalidationExecutionOutcome::as_str;
    let _: fn(&DerivedInvalidationExecutionError) -> DerivedInvalidationExecutionErrorKind =
        DerivedInvalidationExecutionError::kind;

    let _: fn(&DerivedInvalidationExecutedProductRow) -> ExecutionContractFamilyIdentity =
        DerivedInvalidationExecutedProductRow::family_identity;
    let _: fn(&DerivedInvalidationExecutedProductRow) -> DerivedInvalidationExecutionOutcome =
        DerivedInvalidationExecutedProductRow::outcome;
    let _: fn(&DerivedInvalidationExecutedProductRow) -> &str =
        DerivedInvalidationExecutedProductRow::family_digest;
    let _: fn(&DerivedInvalidationExecutedProductRow) -> usize =
        DerivedInvalidationExecutedProductRow::execution_work_count;
    let _: fn(&DerivedInvalidationExecutedProductRow) -> usize =
        DerivedInvalidationExecutedProductRow::caller_owned_graph_work_count;
    let _: fn(&DerivedInvalidationExecutedProductRow) -> usize =
        DerivedInvalidationExecutedProductRow::whole_view_fallback_count;
    let _: fn(&DerivedInvalidationExecutedProductRow) -> Option<&str> =
        DerivedInvalidationExecutedProductRow::materialization_report_digest;
    let _: fn(&DerivedInvalidationExecutedProductRow) -> Option<&str> =
        DerivedInvalidationExecutedProductRow::product_output_digest;
    let _: fn(&DerivedInvalidationExecutedProductRow) -> &str =
        DerivedInvalidationExecutedProductRow::execution_report_digest;
    let _: fn(&DerivedInvalidationExecutedProductRow) -> &str =
        DerivedInvalidationExecutedProductRow::row_digest;

    let _: fn(&DerivedInvalidationUnaffectedProductExecutionRow) -> ExecutionContractFamilyIdentity =
        DerivedInvalidationUnaffectedProductExecutionRow::family_identity;
    let _: fn(&DerivedInvalidationDeniedProductExecutionRow) -> ExecutionContractFamilyIdentity =
        DerivedInvalidationDeniedProductExecutionRow::family_identity;
    let _: fn(&DerivedInvalidationResidueExecutionRow) -> &str =
        DerivedInvalidationResidueExecutionRow::residue_label;

    let _: fn(&DerivedInvalidationExecutionCounters) -> usize =
        DerivedInvalidationExecutionCounters::executed_product_count;
    let _: fn(&DerivedInvalidationExecutionCounters) -> usize =
        DerivedInvalidationExecutionCounters::caller_owned_graph_work_count;
    let _: fn(&DerivedInvalidationExecutionCounters) -> usize =
        DerivedInvalidationExecutionCounters::whole_view_fallback_count;
    let _: fn(&DerivedInvalidationExecutionCounters) -> &str =
        DerivedInvalidationExecutionCounters::counters_digest;

    let _: fn(&DerivedInvalidationExecutionReceipt) -> DerivedInvalidationDiagnosticProjection =
        DerivedInvalidationDiagnosticProjection::from_execution_receipt;
    let _: fn(&DerivedInvalidationDiagnosticProjection) -> &[DerivedInvalidationDiagnosticRow] =
        DerivedInvalidationDiagnosticProjection::rows;
    let _: fn(&DerivedInvalidationDiagnosticProjection) -> &str =
        DerivedInvalidationDiagnosticProjection::diagnostic_projection_digest;
    let _: fn(&DerivedInvalidationDiagnosticRow) -> DerivedInvalidationExecutionOutcome =
        DerivedInvalidationDiagnosticRow::outcome;
    let _: fn(&DerivedInvalidationDiagnosticRow) -> &str =
        DerivedInvalidationDiagnosticRow::source_row_digest;
    let _: fn(&DerivedInvalidationDiagnosticRow) -> &str =
        DerivedInvalidationDiagnosticRow::touched_closure_digest;
    let _: fn(&DerivedInvalidationDiagnosticRow) -> Option<&str> =
        DerivedInvalidationDiagnosticRow::family_digest;
    let _: fn(&DerivedInvalidationDiagnosticRow) -> Option<&str> =
        DerivedInvalidationDiagnosticRow::query_receipt_digest;
    let _: fn(&DerivedInvalidationDiagnosticRow) -> Option<&str> =
        DerivedInvalidationDiagnosticRow::legality_receipt_digest;
    let _: fn(&DerivedInvalidationDiagnosticRow) -> Option<&str> =
        DerivedInvalidationDiagnosticRow::execution_report_digest;
    let _: fn(&DerivedInvalidationDiagnosticRow) -> Option<&str> =
        DerivedInvalidationDiagnosticRow::materialization_report_digest;
}
