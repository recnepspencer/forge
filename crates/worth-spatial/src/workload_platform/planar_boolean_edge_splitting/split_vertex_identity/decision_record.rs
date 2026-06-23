#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanSplitVertexCoalescenceReason {
    DuplicatePointCutReports,
    IntervalEndpointAndPointCut,
    RedundantIntervalEndpoints,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanSplitVertexCoalescenceDecision {
    decision_identity: String,
    split_vertex_identity: String,
    source_edge_identity: String,
    carrier_identity: String,
    parameter_bits: u64,
    reason: PlanarBooleanSplitVertexCoalescenceReason,
    input_identities: Vec<String>,
    point_cut_identities: Vec<String>,
    interval_subdivision_identities: Vec<String>,
    event_group_identities: Vec<String>,
}

impl PlanarBooleanSplitVertexCoalescenceDecision {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        decision_identity: String,
        split_vertex_identity: String,
        source_edge_identity: String,
        carrier_identity: String,
        parameter_bits: u64,
        reason: PlanarBooleanSplitVertexCoalescenceReason,
        input_identities: Vec<String>,
        point_cut_identities: Vec<String>,
        interval_subdivision_identities: Vec<String>,
        event_group_identities: Vec<String>,
    ) -> Self {
        Self {
            decision_identity,
            split_vertex_identity,
            source_edge_identity,
            carrier_identity,
            parameter_bits,
            reason,
            input_identities,
            point_cut_identities,
            interval_subdivision_identities,
            event_group_identities,
        }
    }

    pub fn decision_identity(&self) -> &str {
        &self.decision_identity
    }
    pub fn split_vertex_identity(&self) -> &str {
        &self.split_vertex_identity
    }
    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }
    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }
    pub fn parameter_bits(&self) -> u64 {
        self.parameter_bits
    }
    pub fn reason(&self) -> PlanarBooleanSplitVertexCoalescenceReason {
        self.reason
    }
    pub fn input_identities(&self) -> &[String] {
        &self.input_identities
    }
    pub fn point_cut_identities(&self) -> &[String] {
        &self.point_cut_identities
    }
    pub fn interval_subdivision_identities(&self) -> &[String] {
        &self.interval_subdivision_identities
    }
    pub fn event_group_identities(&self) -> &[String] {
        &self.event_group_identities
    }
}
