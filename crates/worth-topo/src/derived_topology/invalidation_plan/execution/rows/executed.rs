use serde::Serialize;

use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::execution::{
    DerivedInvalidationExecutionOutcome, DerivedInvalidationProductExecutionReport,
};
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedRow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationExecutedProductRow {
    family_identity: DerivedTopologyProductFamilyIdentity,
    family_digest: String,
    source_selected_row_digest: String,
    outcome: DerivedInvalidationExecutionOutcome,
    query_receipt_digest: Option<String>,
    legality_receipt_digest: Option<String>,
    execution_work_count: usize,
    caller_owned_graph_work_count: usize,
    whole_view_fallback_count: usize,
    materialization_report_digest: Option<String>,
    product_output_digest: Option<String>,
    execution_report_digest: String,
    row_digest: String,
}

impl DerivedInvalidationExecutedProductRow {
    pub(in crate::derived_topology::invalidation_plan::execution) fn from_selected_row_report(
        row: &DerivedInvalidationSelectedRow,
        report: &DerivedInvalidationProductExecutionReport,
    ) -> Self {
        let row_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-executed-product-row:v1".to_string(),
            format!("family:{}", row.family_identity().as_str()),
            format!("family-digest:{}", row.family_digest()),
            format!("source-selected-row:{}", row.row_digest()),
            format!("execution-report:{}", report.report_digest()),
            format!("outcome:{}", report.outcome().as_str()),
            format!(
                "query-receipt:{}",
                row.query_receipt_digest().unwrap_or("not-required")
            ),
            format!(
                "legality-receipt:{}",
                row.legality_receipt_digest().unwrap_or("not-required")
            ),
            format!("execution-work:{}", report.execution_work_count()),
            format!(
                "caller-owned-graph-work:{}",
                report.caller_owned_graph_work_count()
            ),
            format!(
                "whole-view-fallbacks:{}",
                report.whole_view_fallback_count()
            ),
            format!(
                "materialization-report:{}",
                report
                    .materialization_report_digest()
                    .unwrap_or("not-materialized")
            ),
            format!(
                "product-output:{}",
                report.product_output_digest().unwrap_or("not-bound")
            ),
        ]);
        Self {
            family_identity: row.family_identity(),
            family_digest: row.family_digest().to_string(),
            source_selected_row_digest: report.source_selected_row_digest().to_string(),
            outcome: report.outcome(),
            query_receipt_digest: row.query_receipt_digest().map(str::to_string),
            legality_receipt_digest: row.legality_receipt_digest().map(str::to_string),
            execution_work_count: report.execution_work_count(),
            caller_owned_graph_work_count: report.caller_owned_graph_work_count(),
            whole_view_fallback_count: report.whole_view_fallback_count(),
            materialization_report_digest: report
                .materialization_report_digest()
                .map(str::to_string),
            product_output_digest: report.product_output_digest().map(str::to_string),
            execution_report_digest: report.report_digest().to_string(),
            row_digest,
        }
    }

    pub const fn family_identity(&self) -> DerivedTopologyProductFamilyIdentity {
        self.family_identity
    }

    pub fn source_selected_row_digest(&self) -> &str {
        &self.source_selected_row_digest
    }

    pub fn family_digest(&self) -> &str {
        &self.family_digest
    }

    pub const fn outcome(&self) -> DerivedInvalidationExecutionOutcome {
        self.outcome
    }

    pub fn query_receipt_digest(&self) -> Option<&str> {
        self.query_receipt_digest.as_deref()
    }

    pub fn legality_receipt_digest(&self) -> Option<&str> {
        self.legality_receipt_digest.as_deref()
    }

    pub const fn execution_work_count(&self) -> usize {
        self.execution_work_count
    }

    pub const fn caller_owned_graph_work_count(&self) -> usize {
        self.caller_owned_graph_work_count
    }

    pub const fn whole_view_fallback_count(&self) -> usize {
        self.whole_view_fallback_count
    }

    pub fn materialization_report_digest(&self) -> Option<&str> {
        self.materialization_report_digest.as_deref()
    }

    pub fn product_output_digest(&self) -> Option<&str> {
        self.product_output_digest.as_deref()
    }

    pub fn execution_report_digest(&self) -> &str {
        &self.execution_report_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
