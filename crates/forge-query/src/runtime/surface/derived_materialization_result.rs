use serde::de::DeserializeOwned;

use crate::runtime::retained_rows::decode_single_retained_row;
use crate::runtime::{ForgeQueryIntentExecutionProvenance, ForgeQueryRuntimeError};

use super::super::ForgeQueryIntentDecisionTraceEnvelope;
use super::ForgeQueryDerivedMaterializationReceipt;

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryDerivedMaterializationResult {
    rows: Vec<serde_json::Value>,
    receipt: ForgeQueryDerivedMaterializationReceipt,
}

impl ForgeQueryDerivedMaterializationResult {
    pub fn rows(&self) -> &[serde_json::Value] {
        &self.rows
    }

    pub fn decode_single_row<T>(&self) -> Result<T, ForgeQueryRuntimeError>
    where
        T: DeserializeOwned,
    {
        decode_single_retained_row(
            &self.rows,
            self.receipt.view_name(),
            "derived-materialization",
        )
    }

    pub fn receipt(&self) -> &ForgeQueryDerivedMaterializationReceipt {
        &self.receipt
    }

    pub(in crate::runtime) fn new(
        rows: Vec<serde_json::Value>,
        receipt: ForgeQueryDerivedMaterializationReceipt,
    ) -> Self {
        Self { rows, receipt }
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
