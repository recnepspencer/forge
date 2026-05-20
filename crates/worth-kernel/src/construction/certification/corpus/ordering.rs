use crate::construction::digest::digest_owned_parts;

use super::replay_siege_report::PrimitiveConstructionCorpusReplaySiegeRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionCorpusAuthoringOrderRow {
    lane_name: String,
    lane_digest: String,
    normalized_matrix_digest: String,
    row_count: usize,
    parity_verified: bool,
    row_digest: String,
}

impl PrimitiveConstructionCorpusAuthoringOrderRow {
    pub fn new(
        lane_name: String,
        lane_digest: String,
        normalized_matrix_digest: String,
        row_count: usize,
        parity_verified: bool,
    ) -> Self {
        let row_digest = digest_owned_parts(&[
            lane_name.clone(),
            lane_digest.clone(),
            normalized_matrix_digest.clone(),
            row_count.to_string(),
            parity_verified.to_string(),
        ]);
        Self {
            lane_name,
            lane_digest,
            normalized_matrix_digest,
            row_count,
            parity_verified,
            row_digest,
        }
    }

    pub fn lane_name(&self) -> &str {
        &self.lane_name
    }

    pub fn lane_digest(&self) -> &str {
        &self.lane_digest
    }

    pub fn normalized_matrix_digest(&self) -> &str {
        &self.normalized_matrix_digest
    }

    pub fn row_count(&self) -> usize {
        self.row_count
    }

    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

pub(super) fn lane_digest(rows: &[PrimitiveConstructionCorpusReplaySiegeRow]) -> String {
    digest_owned_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    )
}
