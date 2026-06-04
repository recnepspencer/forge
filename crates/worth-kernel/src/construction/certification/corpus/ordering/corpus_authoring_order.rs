use crate::construction::digest::digest_owned_parts;

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

    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

pub(crate) fn lane_digest(row_digests: impl IntoIterator<Item = String>) -> String {
    digest_owned_parts(&row_digests.into_iter().collect::<Vec<_>>())
}

pub(crate) fn normalized_matrix_digest(
    row_pairs: impl IntoIterator<Item = (String, String)>,
) -> String {
    let mut parts = row_pairs
        .into_iter()
        .map(|(scenario_id, row_digest)| format!("{scenario_id}:{row_digest}"))
        .collect::<Vec<_>>();
    parts.sort();
    digest_owned_parts(&parts)
}
