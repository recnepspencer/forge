use crate::runtime::{WorthQueryIntentExecutionProvenance, WorthQueryRuntimeError};

use super::super::WorthQueryIntentDecisionTraceEnvelope;
use super::{
    WorthQueryDerivedMaterializationReceipt, WorthQueryRetainedFieldPath,
    WorthQueryRetainedMaterializedRow,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryDerivedMaterializationResult {
    rows: Vec<WorthQueryRetainedMaterializedRow>,
    receipt: WorthQueryDerivedMaterializationReceipt,
}

impl WorthQueryDerivedMaterializationResult {
    pub fn retained_rows(&self) -> &[WorthQueryRetainedMaterializedRow] {
        &self.rows
    }

    pub fn single_retained_row(
        &self,
    ) -> Result<&WorthQueryRetainedMaterializedRow, WorthQueryRuntimeError> {
        match self.rows.as_slice() {
            [] => Err(WorthQueryRuntimeError::RetainedRowDecode {
                view_name: self.receipt.view_name().to_string(),
                stage: "derived-materialization",
                message: "expected one retained row, found none".to_string(),
            }),
            [row] => Ok(row),
            rows => Err(WorthQueryRuntimeError::RetainedRowDecode {
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
        field_path: &WorthQueryRetainedFieldPath,
    ) -> Result<Option<worth_foundational::facade::AspectValue>, String> {
        let Some(row) = self.rows.get(row_index) else {
            return Ok(None);
        };
        Ok(row.scalar_value_at(field_path).cloned())
    }

    pub fn retained_native_value_at_path(
        &self,
        row_index: usize,
        field_path: &WorthQueryRetainedFieldPath,
    ) -> Option<super::WorthQueryRetainedValueView<'_>> {
        self.rows.get(row_index)?.native_value_at(field_path)
    }

    pub fn receipt(&self) -> &WorthQueryDerivedMaterializationReceipt {
        &self.receipt
    }

    pub(in crate::runtime) fn from_retained_rows(
        rows: Vec<WorthQueryRetainedMaterializedRow>,
        receipt: WorthQueryDerivedMaterializationReceipt,
    ) -> Self {
        Self { rows, receipt }
    }

    #[cfg(test)]
    pub(crate) fn test_only_retained_rows(
        rows: Vec<WorthQueryRetainedMaterializedRow>,
        receipt: WorthQueryDerivedMaterializationReceipt,
    ) -> Self {
        Self::from_retained_rows(rows, receipt)
    }

    pub(in crate::runtime) fn attach_intent_admission_evidence(
        &mut self,
        decision_trace_envelope: WorthQueryIntentDecisionTraceEnvelope,
        execution_provenance: WorthQueryIntentExecutionProvenance,
    ) {
        self.receipt.decision_trace_envelope = Some(decision_trace_envelope);
        self.receipt.execution_provenance = Some(execution_provenance);
    }
}
