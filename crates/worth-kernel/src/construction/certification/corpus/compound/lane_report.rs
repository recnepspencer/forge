use crate::construction::digest::digest_owned_parts;

use super::rows::PrimitiveConstructionCompoundRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionCompoundAuthoringOrderRow {
    lane_name: String,
    lane_digest: String,
    normalized_matrix_digest: String,
    parity_verified: bool,
    row_digest: String,
}

impl PrimitiveConstructionCompoundAuthoringOrderRow {
    pub fn new(
        lane_name: String,
        lane_digest: String,
        normalized_matrix_digest: String,
        parity_verified: bool,
    ) -> Self {
        let row_digest = digest_owned_parts(&[
            lane_name.clone(),
            lane_digest.clone(),
            normalized_matrix_digest.clone(),
            parity_verified.to_string(),
        ]);
        Self {
            lane_name,
            lane_digest,
            normalized_matrix_digest,
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

    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionCompoundOrderLaneReport {
    lane_name: String,
    rows: Vec<PrimitiveConstructionCompoundRow>,
    lane_digest: String,
    normalized_matrix_digest: String,
    parity_verified: bool,
    row_digest: String,
}

impl PrimitiveConstructionCompoundOrderLaneReport {
    pub fn new(
        lane_name: String,
        rows: Vec<PrimitiveConstructionCompoundRow>,
        lane_digest: String,
        normalized_matrix_digest: String,
        parity_verified: bool,
    ) -> Self {
        let row_digest = digest_owned_parts(&[
            lane_name.clone(),
            lane_digest.clone(),
            normalized_matrix_digest.clone(),
            parity_verified.to_string(),
            digest_owned_parts(
                &rows
                    .iter()
                    .map(|row| row.row_digest().to_string())
                    .collect::<Vec<_>>(),
            ),
        ]);
        Self {
            lane_name,
            rows,
            lane_digest,
            normalized_matrix_digest,
            parity_verified,
            row_digest,
        }
    }

    pub fn lane_name(&self) -> &str {
        &self.lane_name
    }

    pub fn rows(&self) -> &[PrimitiveConstructionCompoundRow] {
        &self.rows
    }

    pub fn row_for(&self, scenario_id: &str) -> Option<&PrimitiveConstructionCompoundRow> {
        self.rows
            .iter()
            .find(|row| row.scenario_id() == scenario_id)
    }

    pub fn lane_digest(&self) -> &str {
        &self.lane_digest
    }

    pub fn normalized_matrix_digest(&self) -> &str {
        &self.normalized_matrix_digest
    }

    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }

    pub fn summary_row(&self) -> PrimitiveConstructionCompoundAuthoringOrderRow {
        PrimitiveConstructionCompoundAuthoringOrderRow::new(
            self.lane_name.clone(),
            self.lane_digest.clone(),
            self.normalized_matrix_digest.clone(),
            self.parity_verified,
        )
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
