use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

use super::execution_result_row::WorthQueryGraphObligationExecutionResultRow;
use super::reduction::WorthQueryGraphObligationReduction;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationExecutionResultEnvelope {
    rows: Vec<WorthQueryGraphObligationExecutionResultRow>,
    envelope_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphObligationExecutionResultEnvelope {
    pub fn new(mut rows: Vec<WorthQueryGraphObligationExecutionResultRow>) -> Self {
        rows.sort_by(|left, right| left.row_digest().cmp(right.row_digest()));
        let row_digests = rows
            .iter()
            .map(WorthQueryGraphObligationExecutionResultRow::row_evidence_digest)
            .collect::<Vec<_>>();
        let envelope_digest = worth_query_evidence_identity(
            WorthQueryEvidenceScope::GraphObligationExecutionResultEnvelope,
        )
        .field_usize(WorthQueryEvidenceTag::new("rows"), rows.len())
        .field_evidence_identity_sequence(WorthQueryEvidenceTag::new("row"), row_digests)
        .seal();
        Self {
            rows,
            envelope_digest,
        }
    }

    pub fn rows(&self) -> &[WorthQueryGraphObligationExecutionResultRow] {
        &self.rows
    }

    pub fn envelope_digest(&self) -> &str {
        self.envelope_digest.as_str()
    }

    pub fn reduce(&self) -> WorthQueryGraphObligationReduction {
        WorthQueryGraphObligationReduction::from_rows(self.rows.clone())
    }
}
