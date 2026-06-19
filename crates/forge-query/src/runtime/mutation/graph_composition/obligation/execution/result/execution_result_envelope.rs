use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::execution_result_row::ForgeQueryGraphObligationExecutionResultRow;
use super::reduction::ForgeQueryGraphObligationReduction;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationExecutionResultEnvelope {
    rows: Vec<ForgeQueryGraphObligationExecutionResultRow>,
    envelope_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphObligationExecutionResultEnvelope {
    pub fn new(mut rows: Vec<ForgeQueryGraphObligationExecutionResultRow>) -> Self {
        rows.sort_by(|left, right| left.row_digest().cmp(right.row_digest()));
        let row_digests = rows
            .iter()
            .map(ForgeQueryGraphObligationExecutionResultRow::row_evidence_digest)
            .collect::<Vec<_>>();
        let envelope_digest = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::GraphObligationExecutionResultEnvelope,
        )
        .field_usize(ForgeQueryEvidenceTag::new("rows"), rows.len())
        .field_evidence_identity_sequence(ForgeQueryEvidenceTag::new("row"), row_digests)
        .seal();
        Self {
            rows,
            envelope_digest,
        }
    }

    pub fn rows(&self) -> &[ForgeQueryGraphObligationExecutionResultRow] {
        &self.rows
    }

    pub fn envelope_digest(&self) -> &str {
        self.envelope_digest.as_str()
    }

    pub fn reduce(&self) -> ForgeQueryGraphObligationReduction {
        ForgeQueryGraphObligationReduction::from_rows(self.rows.clone())
    }
}
