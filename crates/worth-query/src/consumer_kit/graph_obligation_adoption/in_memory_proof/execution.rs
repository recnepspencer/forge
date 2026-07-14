use crate::runtime::{
    WorthQueryGraphObligationExecutionResultEnvelope, WorthQueryGraphObligationExecutionResultRow,
    WorthQueryGraphObligationExecutionStatus, WorthQueryGraphObligationStateLoadCounters,
};

use super::super::kit_digest;
use super::selection::WorthQueryGraphObligationInMemoryProof;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationExecutionProof {
    selection_proof: WorthQueryGraphObligationInMemoryProof,
    rows: Vec<WorthQueryGraphObligationExecutionProofRow>,
    envelope_digest: String,
    proof_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationExecutionProofRow {
    status: WorthQueryGraphObligationExecutionStatus,
    verdict: Option<String>,
    verdict_context: Option<String>,
    counters: WorthQueryGraphObligationStateLoadCounters,
    row_digest: String,
}

impl WorthQueryGraphObligationExecutionProof {
    pub fn from_envelope(
        selection_proof: WorthQueryGraphObligationInMemoryProof,
        envelope: WorthQueryGraphObligationExecutionResultEnvelope,
    ) -> Self {
        let rows = envelope
            .rows()
            .iter()
            .map(WorthQueryGraphObligationExecutionProofRow::from_result_row)
            .collect::<Vec<_>>();
        let row_digests = rows.iter().map(|row| row.row_digest.as_str());
        let proof_digest = kit_digest(
            "execution-proof",
            std::iter::once(selection_proof.proof_digest())
                .chain(std::iter::once(envelope.envelope_digest()))
                .chain(row_digests),
        );
        Self {
            selection_proof,
            rows,
            envelope_digest: envelope.envelope_digest().to_string(),
            proof_digest,
        }
    }

    pub fn selection_proof(&self) -> &WorthQueryGraphObligationInMemoryProof {
        &self.selection_proof
    }

    pub fn selected_obligation_count(&self) -> usize {
        self.selection_proof.selected_obligation_count()
    }

    pub fn rows(&self) -> &[WorthQueryGraphObligationExecutionProofRow] {
        &self.rows
    }

    pub fn execution_statuses(&self) -> Vec<WorthQueryGraphObligationExecutionStatus> {
        self.rows.iter().map(|row| row.status()).collect()
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }

    pub fn proof_digest(&self) -> &str {
        &self.proof_digest
    }

    pub fn has_real_executor_rows(&self) -> bool {
        self.rows.len() == self.selection_proof.selected_obligation_count()
            && self.rows.iter().all(|row| !row.row_digest().is_empty())
    }
}

impl WorthQueryGraphObligationExecutionProofRow {
    fn from_result_row(row: &WorthQueryGraphObligationExecutionResultRow) -> Self {
        Self {
            status: row.status(),
            verdict: row.verdict().map(|verdict| verdict.as_str().to_string()),
            verdict_context: row
                .verdict()
                .and_then(|verdict| verdict.context().map(ToOwned::to_owned)),
            counters: row.state_load_counters().clone(),
            row_digest: row.row_digest().to_string(),
        }
    }

    pub fn status(&self) -> WorthQueryGraphObligationExecutionStatus {
        self.status
    }

    pub fn verdict(&self) -> Option<&str> {
        self.verdict.as_deref()
    }

    pub fn verdict_context(&self) -> Option<&str> {
        self.verdict_context.as_deref()
    }

    pub fn state_load_counters(&self) -> &WorthQueryGraphObligationStateLoadCounters {
        &self.counters
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
