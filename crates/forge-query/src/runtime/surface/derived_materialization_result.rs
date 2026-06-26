use crate::runtime::{ForgeQueryIntentExecutionProvenance, ForgeQueryRuntimeError};

use super::super::ForgeQueryIntentDecisionTraceEnvelope;
use super::{
    ForgeQueryDerivedMaterializationReceipt, ForgeQueryRetainedFieldPath,
    ForgeQueryRetainedMaterializedRow,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryDerivedMaterializationResult {
    rows: Vec<ForgeQueryRetainedMaterializedRow>,
    receipt: ForgeQueryDerivedMaterializationReceipt,
}

impl ForgeQueryDerivedMaterializationResult {
    pub fn retained_rows(&self) -> &[ForgeQueryRetainedMaterializedRow] {
        &self.rows
    }

    pub fn single_retained_row(
        &self,
    ) -> Result<&ForgeQueryRetainedMaterializedRow, ForgeQueryRuntimeError> {
        match self.rows.as_slice() {
            [] => Err(ForgeQueryRuntimeError::RetainedRowDecode {
                view_name: self.receipt.view_name().to_string(),
                stage: "derived-materialization",
                message: "expected one retained row, found none".to_string(),
            }),
            [row] => Ok(row),
            rows => Err(ForgeQueryRuntimeError::RetainedRowDecode {
                view_name: self.receipt.view_name().to_string(),
                stage: "derived-materialization",
                message: format!("expected one retained row, found {}", rows.len()),
            }),
        }
    }

    pub(crate) fn retained_row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn retained_scalar_value_at_path(
        &self,
        row_index: usize,
        field_path: &ForgeQueryRetainedFieldPath,
    ) -> Result<Option<forge_foundational::facade::AspectValue>, String> {
        let Some(row) = self.rows.get(row_index) else {
            return Ok(None);
        };
        Ok(row.field_value_at(field_path).cloned())
    }

    pub fn receipt(&self) -> &ForgeQueryDerivedMaterializationReceipt {
        &self.receipt
    }

    pub(in crate::runtime) fn from_retained_rows(
        rows: Vec<ForgeQueryRetainedMaterializedRow>,
        receipt: ForgeQueryDerivedMaterializationReceipt,
    ) -> Self {
        Self { rows, receipt }
    }

    #[cfg(test)]
    pub(crate) fn test_only_retained_rows(
        rows: Vec<ForgeQueryRetainedMaterializedRow>,
        receipt: ForgeQueryDerivedMaterializationReceipt,
    ) -> Self {
        Self::from_retained_rows(rows, receipt)
    }

    pub(in crate::runtime) fn attach_intent_admission_evidence(
        &mut self,
        decision_trace_envelope: ForgeQueryIntentDecisionTraceEnvelope,
        execution_provenance: ForgeQueryIntentExecutionProvenance,
    ) {
        self.receipt.decision_trace_envelope = Some(decision_trace_envelope);
        self.receipt.execution_provenance = Some(execution_provenance);
    }
}
