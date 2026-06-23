#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanFragmentConsumptionProofRow {
    closed_walk_candidate_identity: String,
    fragment_identities: Vec<String>,
    split_vertex_identities: Vec<String>,
    continuation_identities: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanFragmentConsumptionProof {
    fragment_consumption_proof_identity: String,
    request_identity: String,
    continuation_index_identity: String,
    rows: Vec<PlanarBooleanFragmentConsumptionProofRow>,
}

impl PlanarBooleanFragmentConsumptionProofRow {
    pub(crate) fn new(
        closed_walk_candidate_identity: String,
        fragment_identities: Vec<String>,
        split_vertex_identities: Vec<String>,
        continuation_identities: Vec<String>,
    ) -> Self {
        Self {
            closed_walk_candidate_identity,
            fragment_identities,
            split_vertex_identities,
            continuation_identities,
        }
    }

    pub fn closed_walk_candidate_identity(&self) -> &str {
        &self.closed_walk_candidate_identity
    }

    pub fn fragment_identities(&self) -> &[String] {
        &self.fragment_identities
    }

    pub fn split_vertex_identities(&self) -> &[String] {
        &self.split_vertex_identities
    }

    pub fn continuation_identities(&self) -> &[String] {
        &self.continuation_identities
    }
}

impl PlanarBooleanFragmentConsumptionProof {
    pub(crate) fn new(
        fragment_consumption_proof_identity: String,
        request_identity: String,
        continuation_index_identity: String,
        rows: Vec<PlanarBooleanFragmentConsumptionProofRow>,
    ) -> Self {
        Self {
            fragment_consumption_proof_identity,
            request_identity,
            continuation_index_identity,
            rows,
        }
    }

    pub fn fragment_consumption_proof_identity(&self) -> &str {
        &self.fragment_consumption_proof_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn continuation_index_identity(&self) -> &str {
        &self.continuation_index_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanFragmentConsumptionProofRow] {
        &self.rows
    }

    pub fn proof_for_candidate_identity(
        &self,
        closed_walk_candidate_identity: &str,
    ) -> Option<&PlanarBooleanFragmentConsumptionProofRow> {
        self.rows
            .iter()
            .find(|row| row.closed_walk_candidate_identity() == closed_walk_candidate_identity)
    }
}
