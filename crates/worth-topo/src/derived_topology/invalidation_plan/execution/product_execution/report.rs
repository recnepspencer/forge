use super::materialization_evidence::{
    materialization_report_digest, materialization_report_is_whole_view_fallback,
};
use crate::derived_topology::invalidation_plan::execution::{
    admit_materialization_report_for_execution_outcome, DerivedInvalidationExecutionError,
    DerivedInvalidationExecutionErrorKind, DerivedInvalidationExecutionOutcome,
};
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedRow;
use crate::derived_topology::materialized_graph::MaterializationReport;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DerivedInvalidationProductExecutionReport {
    source_selected_row_digest: String,
    outcome: DerivedInvalidationExecutionOutcome,
    execution_work_count: usize,
    caller_owned_graph_work_count: usize,
    whole_view_fallback_count: usize,
    materialization_report_digest: Option<String>,
    product_output_digest: Option<String>,
    report_digest: String,
}

impl DerivedInvalidationProductExecutionReport {
    pub(crate) fn from_selected_row(
        row: &DerivedInvalidationSelectedRow,
        execution_work_count: usize,
        caller_owned_graph_work_count: usize,
        materialization_report: Option<&MaterializationReport>,
    ) -> Result<Self, DerivedInvalidationExecutionError> {
        Self::from_selected_row_with_product_output(
            row,
            execution_work_count,
            caller_owned_graph_work_count,
            materialization_report,
            None,
        )
    }

    pub(crate) fn from_selected_row_with_product_output(
        row: &DerivedInvalidationSelectedRow,
        execution_work_count: usize,
        caller_owned_graph_work_count: usize,
        materialization_report: Option<&MaterializationReport>,
        product_output_digest: Option<&str>,
    ) -> Result<Self, DerivedInvalidationExecutionError> {
        let outcome = DerivedInvalidationExecutionOutcome::from_planned_disposition(
            row.planned_disposition(),
        );
        reject_caller_owned_graph_work(caller_owned_graph_work_count)?;
        admit_report_for_planned_execution_outcome(outcome, materialization_report)?;

        let materialization_report_digest =
            materialization_report.map(materialization_report_digest);
        let whole_view_fallback_count = count_whole_view_fallback_report(materialization_report);
        let report_digest = execution_report_digest(
            row,
            outcome,
            execution_work_count,
            caller_owned_graph_work_count,
            whole_view_fallback_count,
            materialization_report_digest.as_deref(),
            product_output_digest,
        );

        Ok(Self {
            source_selected_row_digest: row.row_digest().to_string(),
            outcome,
            execution_work_count,
            caller_owned_graph_work_count,
            whole_view_fallback_count,
            materialization_report_digest,
            product_output_digest: product_output_digest.map(str::to_string),
            report_digest,
        })
    }

    pub(crate) fn source_selected_row_digest(&self) -> &str {
        &self.source_selected_row_digest
    }

    pub(crate) const fn outcome(&self) -> DerivedInvalidationExecutionOutcome {
        self.outcome
    }

    pub(crate) const fn execution_work_count(&self) -> usize {
        self.execution_work_count
    }

    pub(crate) const fn caller_owned_graph_work_count(&self) -> usize {
        self.caller_owned_graph_work_count
    }

    pub(crate) const fn whole_view_fallback_count(&self) -> usize {
        self.whole_view_fallback_count
    }

    pub(crate) fn materialization_report_digest(&self) -> Option<&str> {
        self.materialization_report_digest.as_deref()
    }

    pub(crate) fn product_output_digest(&self) -> Option<&str> {
        self.product_output_digest.as_deref()
    }

    pub(crate) fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

fn reject_caller_owned_graph_work(
    caller_owned_graph_work_count: usize,
) -> Result<(), DerivedInvalidationExecutionError> {
    if caller_owned_graph_work_count > 0 {
        return Err(DerivedInvalidationExecutionError::new(
            DerivedInvalidationExecutionErrorKind::CallerOwnedGraphWorkNotAdmitted,
        ));
    }
    Ok(())
}

fn admit_report_for_planned_execution_outcome(
    outcome: DerivedInvalidationExecutionOutcome,
    materialization_report: Option<&MaterializationReport>,
) -> Result<(), DerivedInvalidationExecutionError> {
    if let Some(report) = materialization_report {
        admit_materialization_report_for_execution_outcome(outcome, report)?;
    }
    Ok(())
}

fn count_whole_view_fallback_report(
    materialization_report: Option<&MaterializationReport>,
) -> usize {
    materialization_report
        .filter(|report| materialization_report_is_whole_view_fallback(report))
        .map_or(0, |_| 1)
}

fn execution_report_digest(
    row: &DerivedInvalidationSelectedRow,
    outcome: DerivedInvalidationExecutionOutcome,
    execution_work_count: usize,
    caller_owned_graph_work_count: usize,
    whole_view_fallback_count: usize,
    materialization_report_digest: Option<&str>,
    product_output_digest: Option<&str>,
) -> String {
    super::super::super::catalog::catalog_digest([
        "worth-topo:derived-invalidation-product-execution-report:v1".to_string(),
        format!("source-selected-row:{}", row.row_digest()),
        format!("family:{}", row.family_identity().as_str()),
        format!("family-digest:{}", row.family_digest()),
        format!("outcome:{}", outcome.as_str()),
        format!("execution-work:{execution_work_count}"),
        format!("caller-owned-graph-work:{caller_owned_graph_work_count}"),
        format!("whole-view-fallbacks:{whole_view_fallback_count}"),
        format!(
            "materialization-report:{}",
            materialization_report_digest.unwrap_or("not-materialized")
        ),
        format!(
            "product-output:{}",
            product_output_digest.unwrap_or("not-bound")
        ),
    ])
}
