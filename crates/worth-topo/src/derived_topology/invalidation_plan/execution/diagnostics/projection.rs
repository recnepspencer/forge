use serde::Serialize;

use super::DerivedInvalidationDiagnosticRow;
use crate::derived_topology::invalidation_plan::execution::DerivedInvalidationExecutionReceipt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationDiagnosticProjection {
    selected_plan_digest: String,
    execution_receipt_digest: String,
    rows: Vec<DerivedInvalidationDiagnosticRow>,
    diagnostic_projection_digest: String,
}

impl DerivedInvalidationDiagnosticProjection {
    pub fn from_execution_receipt(receipt: &DerivedInvalidationExecutionReceipt) -> Self {
        let rows = DerivedInvalidationDiagnosticRow::from_execution_receipt(receipt);
        let diagnostic_projection_digest = diagnostic_projection_digest(receipt, &rows);
        Self {
            selected_plan_digest: receipt.selected_plan_digest().to_string(),
            execution_receipt_digest: receipt.execution_receipt_digest().to_string(),
            rows,
            diagnostic_projection_digest,
        }
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn execution_receipt_digest(&self) -> &str {
        &self.execution_receipt_digest
    }

    pub fn rows(&self) -> &[DerivedInvalidationDiagnosticRow] {
        &self.rows
    }

    pub fn diagnostic_projection_digest(&self) -> &str {
        &self.diagnostic_projection_digest
    }
}

fn diagnostic_projection_digest(
    receipt: &DerivedInvalidationExecutionReceipt,
    rows: &[DerivedInvalidationDiagnosticRow],
) -> String {
    let mut parts = vec![
        "worth-topo:derived-invalidation-diagnostic-projection:v1".to_string(),
        format!("selected-plan:{}", receipt.selected_plan_digest()),
        format!("execution-receipt:{}", receipt.execution_receipt_digest()),
    ];
    parts.extend(
        rows.iter()
            .map(|row| format!("diagnostic-row:{}", row.row_digest())),
    );
    super::super::super::catalog::catalog_digest(parts)
}
