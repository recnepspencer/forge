use std::collections::BTreeSet;

use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

use super::execution_result_row::WorthQueryGraphObligationExecutionResultRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationReduction {
    rows: Vec<WorthQueryGraphObligationExecutionResultRow>,
    duplicate_rule_observation_count: usize,
    reduction_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphObligationReduction {
    pub(super) fn from_rows(mut rows: Vec<WorthQueryGraphObligationExecutionResultRow>) -> Self {
        rows.sort_by(|left, right| {
            left.input()
                .selected_registration()
                .rule_identity()
                .identity_digest()
                .cmp(
                    right
                        .input()
                        .selected_registration()
                        .rule_identity()
                        .identity_digest(),
                )
                .then_with(|| left.row_digest().cmp(right.row_digest()))
        });
        let duplicate_rule_observation_count = duplicate_rule_observation_count(&rows);
        let row_digests = rows
            .iter()
            .map(WorthQueryGraphObligationExecutionResultRow::row_evidence_digest)
            .collect::<Vec<_>>();
        let reduction_digest =
            worth_query_evidence_identity(WorthQueryEvidenceScope::GraphObligationReduction)
                .field_usize(WorthQueryEvidenceTag::new("rows"), rows.len())
                .field_usize(
                    WorthQueryEvidenceTag::new("duplicate_rule_observations"),
                    duplicate_rule_observation_count,
                )
                .field_evidence_identity_sequence(WorthQueryEvidenceTag::new("row"), row_digests)
                .seal();
        Self {
            rows,
            duplicate_rule_observation_count,
            reduction_digest,
        }
    }

    pub fn rows(&self) -> &[WorthQueryGraphObligationExecutionResultRow] {
        &self.rows
    }

    pub fn duplicate_rule_observation_count(&self) -> usize {
        self.duplicate_rule_observation_count
    }

    pub fn blocking_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.verdict().is_some_and(|verdict| verdict.is_blocking()))
            .count()
    }

    pub fn advisory_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.verdict().is_some_and(|verdict| verdict.is_advisory()))
            .count()
    }

    pub fn allow_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.verdict().is_some_and(|verdict| verdict.is_allow()))
            .count()
    }

    pub fn blocks_if_required(&self) -> bool {
        self.blocking_count() > 0
    }

    pub fn reduction_digest(&self) -> &str {
        self.reduction_digest.as_str()
    }

    pub(crate) fn reduction_evidence_digest(&self) -> &WorthQueryEvidenceIdentity {
        &self.reduction_digest
    }
}

fn duplicate_rule_observation_count(rows: &[WorthQueryGraphObligationExecutionResultRow]) -> usize {
    let mut seen = BTreeSet::new();
    rows.iter()
        .filter(|row| {
            !seen.insert(
                row.input()
                    .selected_registration()
                    .rule_identity()
                    .identity_digest()
                    .to_string(),
            )
        })
        .count()
}
