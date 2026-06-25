use serde::Serialize;

use crate::derived_topology::invalidation_plan::catalog::{
    DerivedTopologyLegalityReceiptPosture, DerivedTopologyProductFamilyIdentity,
    DerivedTopologyQueryReceiptPosture,
};
use crate::derived_topology::invalidation_plan::execution::{
    DerivedInvalidationExecutionOutcome, DerivedInvalidationExecutionReceipt,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationDiagnosticRow {
    selected_plan_digest: String,
    execution_receipt_digest: String,
    touched_closure_digest: String,
    query_support_digest: String,
    legality_support_digest: String,
    source_row_digest: String,
    outcome: DerivedInvalidationExecutionOutcome,
    family_identity: Option<DerivedTopologyProductFamilyIdentity>,
    family_digest: Option<String>,
    residue_label: Option<String>,
    query_receipt_digest: Option<String>,
    legality_receipt_digest: Option<String>,
    execution_report_digest: Option<String>,
    materialization_report_digest: Option<String>,
    required_query_posture: Option<DerivedTopologyQueryReceiptPosture>,
    required_legality_posture: Option<DerivedTopologyLegalityReceiptPosture>,
    reason: String,
    row_digest: String,
}

impl DerivedInvalidationDiagnosticRow {
    pub(super) fn from_execution_receipt(
        receipt: &DerivedInvalidationExecutionReceipt,
    ) -> Vec<Self> {
        let mut rows = Vec::new();
        rows.extend(receipt.executed_rows().iter().map(|row| {
            Self::new(DerivedInvalidationDiagnosticRowInput {
                receipt,
                source_row_digest: row.source_selected_row_digest(),
                outcome: row.outcome(),
                family_identity: Some(row.family_identity()),
                family_digest: Some(row.family_digest()),
                residue_label: None,
                query_receipt_digest: row.query_receipt_digest(),
                legality_receipt_digest: row.legality_receipt_digest(),
                execution_report_digest: Some(row.execution_report_digest()),
                materialization_report_digest: row.materialization_report_digest(),
                required_query_posture: None,
                required_legality_posture: None,
                reason: row.outcome().as_str(),
            })
        }));
        rows.extend(receipt.unaffected_rows().iter().map(|row| {
            Self::new(DerivedInvalidationDiagnosticRowInput {
                receipt,
                source_row_digest: row.source_unaffected_row_digest(),
                outcome: row.outcome(),
                family_identity: Some(row.family_identity()),
                family_digest: Some(row.family_digest()),
                residue_label: None,
                query_receipt_digest: None,
                legality_receipt_digest: None,
                execution_report_digest: None,
                materialization_report_digest: None,
                required_query_posture: None,
                required_legality_posture: None,
                reason: "declared_consumed_facts_do_not_intersect_touched_closure",
            })
        }));
        rows.extend(receipt.denied_rows().iter().map(|row| {
            Self::new(DerivedInvalidationDiagnosticRowInput {
                receipt,
                source_row_digest: row.source_denial_digest(),
                outcome: row.outcome(),
                family_identity: Some(row.family_identity()),
                family_digest: Some(row.family_digest()),
                residue_label: None,
                query_receipt_digest: None,
                legality_receipt_digest: None,
                execution_report_digest: None,
                materialization_report_digest: None,
                required_query_posture: row.required_query_posture(),
                required_legality_posture: row.required_legality_posture(),
                reason: row.denial_kind().as_str(),
            })
        }));
        rows.extend(receipt.residue_rows().iter().map(|row| {
            Self::new(DerivedInvalidationDiagnosticRowInput {
                receipt,
                source_row_digest: row.source_residue_row_digest(),
                outcome: row.outcome(),
                family_identity: None,
                family_digest: None,
                residue_label: Some(row.residue_label()),
                query_receipt_digest: None,
                legality_receipt_digest: None,
                execution_report_digest: None,
                materialization_report_digest: None,
                required_query_posture: None,
                required_legality_posture: None,
                reason: "certification_bootstrap_residue_capped",
            })
        }));
        rows
    }

    fn new(input: DerivedInvalidationDiagnosticRowInput<'_>) -> Self {
        let row_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-diagnostic-row:v1".to_string(),
            format!("selected-plan:{}", input.receipt.selected_plan_digest()),
            format!(
                "execution-receipt:{}",
                input.receipt.execution_receipt_digest()
            ),
            format!("touched-closure:{}", input.receipt.touched_closure_digest()),
            format!("query-support:{}", input.receipt.query_support_digest()),
            format!(
                "legality-support:{}",
                input.receipt.legality_support_digest()
            ),
            format!("source-row:{}", input.source_row_digest),
            format!("outcome:{}", input.outcome.as_str()),
            format!(
                "family:{}",
                input
                    .family_identity
                    .map(DerivedTopologyProductFamilyIdentity::as_str)
                    .unwrap_or("not-applicable")
            ),
            format!(
                "family-digest:{}",
                input.family_digest.unwrap_or("not-applicable")
            ),
            format!(
                "residue:{}",
                input.residue_label.unwrap_or("not-applicable")
            ),
            format!(
                "query-receipt:{}",
                input.query_receipt_digest.unwrap_or("not-applicable")
            ),
            format!(
                "legality-receipt:{}",
                input.legality_receipt_digest.unwrap_or("not-applicable")
            ),
            format!(
                "execution-report:{}",
                input.execution_report_digest.unwrap_or("not-applicable")
            ),
            format!(
                "materialization-report:{}",
                input
                    .materialization_report_digest
                    .unwrap_or("not-applicable")
            ),
            format!(
                "required-query:{}",
                input
                    .required_query_posture
                    .map(DerivedTopologyQueryReceiptPosture::as_str)
                    .unwrap_or("not-applicable")
            ),
            format!(
                "required-legality:{}",
                input
                    .required_legality_posture
                    .map(DerivedTopologyLegalityReceiptPosture::as_str)
                    .unwrap_or("not-applicable")
            ),
            format!("reason:{}", input.reason),
        ]);
        Self {
            selected_plan_digest: input.receipt.selected_plan_digest().to_string(),
            execution_receipt_digest: input.receipt.execution_receipt_digest().to_string(),
            touched_closure_digest: input.receipt.touched_closure_digest().to_string(),
            query_support_digest: input.receipt.query_support_digest().to_string(),
            legality_support_digest: input.receipt.legality_support_digest().to_string(),
            source_row_digest: input.source_row_digest.to_string(),
            outcome: input.outcome,
            family_identity: input.family_identity,
            family_digest: input.family_digest.map(str::to_string),
            residue_label: input.residue_label.map(str::to_string),
            query_receipt_digest: input.query_receipt_digest.map(str::to_string),
            legality_receipt_digest: input.legality_receipt_digest.map(str::to_string),
            execution_report_digest: input.execution_report_digest.map(str::to_string),
            materialization_report_digest: input.materialization_report_digest.map(str::to_string),
            required_query_posture: input.required_query_posture,
            required_legality_posture: input.required_legality_posture,
            reason: input.reason.to_string(),
            row_digest,
        }
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn execution_receipt_digest(&self) -> &str {
        &self.execution_receipt_digest
    }

    pub fn touched_closure_digest(&self) -> &str {
        &self.touched_closure_digest
    }

    pub fn query_support_digest(&self) -> &str {
        &self.query_support_digest
    }

    pub fn legality_support_digest(&self) -> &str {
        &self.legality_support_digest
    }

    pub fn source_row_digest(&self) -> &str {
        &self.source_row_digest
    }

    pub const fn outcome(&self) -> DerivedInvalidationExecutionOutcome {
        self.outcome
    }

    pub const fn family_identity(&self) -> Option<DerivedTopologyProductFamilyIdentity> {
        self.family_identity
    }

    pub fn family_digest(&self) -> Option<&str> {
        self.family_digest.as_deref()
    }

    pub fn residue_label(&self) -> Option<&str> {
        self.residue_label.as_deref()
    }

    pub fn query_receipt_digest(&self) -> Option<&str> {
        self.query_receipt_digest.as_deref()
    }

    pub fn legality_receipt_digest(&self) -> Option<&str> {
        self.legality_receipt_digest.as_deref()
    }

    pub fn execution_report_digest(&self) -> Option<&str> {
        self.execution_report_digest.as_deref()
    }

    pub fn materialization_report_digest(&self) -> Option<&str> {
        self.materialization_report_digest.as_deref()
    }

    pub const fn required_query_posture(&self) -> Option<DerivedTopologyQueryReceiptPosture> {
        self.required_query_posture
    }

    pub const fn required_legality_posture(&self) -> Option<DerivedTopologyLegalityReceiptPosture> {
        self.required_legality_posture
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

struct DerivedInvalidationDiagnosticRowInput<'a> {
    receipt: &'a DerivedInvalidationExecutionReceipt,
    source_row_digest: &'a str,
    outcome: DerivedInvalidationExecutionOutcome,
    family_identity: Option<DerivedTopologyProductFamilyIdentity>,
    family_digest: Option<&'a str>,
    residue_label: Option<&'a str>,
    query_receipt_digest: Option<&'a str>,
    legality_receipt_digest: Option<&'a str>,
    execution_report_digest: Option<&'a str>,
    materialization_report_digest: Option<&'a str>,
    required_query_posture: Option<DerivedTopologyQueryReceiptPosture>,
    required_legality_posture: Option<DerivedTopologyLegalityReceiptPosture>,
    reason: &'a str,
}
