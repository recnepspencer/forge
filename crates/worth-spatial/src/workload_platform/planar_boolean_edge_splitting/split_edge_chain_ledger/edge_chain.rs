#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitEdgeChain {
    chain_identity: String,
    source_edge_identity: String,
    carrier_identity: String,
    endpoint_boundary_schedule_identity: String,
    interval_subdivision_schedule_identity: String,
    split_vertex_schedule_identity: String,
    split_fragment_schedule_identity: String,
    fragment_identities: Vec<String>,
    split_vertex_identities: Vec<String>,
    overlap_chain_identities: Vec<String>,
    persistent_name_row_identities: Vec<String>,
    decision_identities: Vec<String>,
    validation_fragment_coverage_identities: Vec<String>,
    validation_overlap_coverage_identities: Vec<String>,
}

impl PlanarBooleanSplitEdgeChain {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        chain_identity: String,
        source_edge_identity: String,
        carrier_identity: String,
        endpoint_boundary_schedule_identity: String,
        interval_subdivision_schedule_identity: String,
        split_vertex_schedule_identity: String,
        split_fragment_schedule_identity: String,
        fragment_identities: Vec<String>,
        split_vertex_identities: Vec<String>,
        overlap_chain_identities: Vec<String>,
        persistent_name_row_identities: Vec<String>,
        decision_identities: Vec<String>,
        validation_fragment_coverage_identities: Vec<String>,
        validation_overlap_coverage_identities: Vec<String>,
    ) -> Self {
        Self {
            chain_identity,
            source_edge_identity,
            carrier_identity,
            endpoint_boundary_schedule_identity,
            interval_subdivision_schedule_identity,
            split_vertex_schedule_identity,
            split_fragment_schedule_identity,
            fragment_identities,
            split_vertex_identities,
            overlap_chain_identities,
            persistent_name_row_identities,
            decision_identities,
            validation_fragment_coverage_identities,
            validation_overlap_coverage_identities,
        }
    }

    pub fn chain_identity(&self) -> &str {
        &self.chain_identity
    }
    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }
    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }
    pub fn endpoint_boundary_schedule_identity(&self) -> &str {
        &self.endpoint_boundary_schedule_identity
    }
    pub fn interval_subdivision_schedule_identity(&self) -> &str {
        &self.interval_subdivision_schedule_identity
    }
    pub fn split_vertex_schedule_identity(&self) -> &str {
        &self.split_vertex_schedule_identity
    }
    pub fn split_fragment_schedule_identity(&self) -> &str {
        &self.split_fragment_schedule_identity
    }
    pub fn fragment_identities(&self) -> &[String] {
        &self.fragment_identities
    }
    pub fn split_vertex_identities(&self) -> &[String] {
        &self.split_vertex_identities
    }
    pub fn overlap_chain_identities(&self) -> &[String] {
        &self.overlap_chain_identities
    }
    pub fn persistent_name_row_identities(&self) -> &[String] {
        &self.persistent_name_row_identities
    }
    pub fn decision_identities(&self) -> &[String] {
        &self.decision_identities
    }
    pub fn validation_fragment_coverage_identities(&self) -> &[String] {
        &self.validation_fragment_coverage_identities
    }
    pub fn validation_overlap_coverage_identities(&self) -> &[String] {
        &self.validation_overlap_coverage_identities
    }
}
