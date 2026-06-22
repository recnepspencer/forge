#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanEdgeSplitCloseoutCandidateRow {
    candidate_identity: String,
    left_source_edge_identity: String,
    right_source_edge_identity: String,
    broad_phase_reason: String,
    envelope_basis_identity: String,
    local_frame_identity: String,
    precision_basis_identity: String,
}

impl PlanarBooleanEdgeSplitCloseoutCandidateRow {
    pub(crate) fn new(
        candidate_identity: String,
        left_source_edge_identity: String,
        right_source_edge_identity: String,
        broad_phase_reason: String,
        envelope_basis_identity: String,
        local_frame_identity: String,
        precision_basis_identity: String,
    ) -> Self {
        Self {
            candidate_identity,
            left_source_edge_identity,
            right_source_edge_identity,
            broad_phase_reason,
            envelope_basis_identity,
            local_frame_identity,
            precision_basis_identity,
        }
    }

    pub fn candidate_identity(&self) -> &str {
        &self.candidate_identity
    }
    pub fn left_source_edge_identity(&self) -> &str {
        &self.left_source_edge_identity
    }
    pub fn right_source_edge_identity(&self) -> &str {
        &self.right_source_edge_identity
    }
    pub fn broad_phase_reason(&self) -> &str {
        &self.broad_phase_reason
    }
    pub fn envelope_basis_identity(&self) -> &str {
        &self.envelope_basis_identity
    }
    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }
    pub fn precision_basis_identity(&self) -> &str {
        &self.precision_basis_identity
    }
}

pub(crate) fn closeout_candidate_manifest_rows(
    product: &crate::workload_platform::planar_boolean_events::PlanarBooleanSegmentCandidateIndexProduct,
) -> Vec<PlanarBooleanEdgeSplitCloseoutCandidateRow> {
    product
        .rows()
        .iter()
        .map(|row| {
            PlanarBooleanEdgeSplitCloseoutCandidateRow::new(
                row.candidate_identity().to_string(),
                row.left_source_edge_identity().to_string(),
                row.right_source_edge_identity().to_string(),
                row.broad_phase_reason().as_str().to_string(),
                row.envelope_basis().envelope_basis_identity(),
                row.local_frame_identity().to_string(),
                row.precision_basis_identity().to_string(),
            )
        })
        .collect()
}
