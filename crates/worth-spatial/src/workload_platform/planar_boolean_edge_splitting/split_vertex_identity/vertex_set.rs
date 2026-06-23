use super::counters::PlanarBooleanSplitVertexIdentityCounters;
use super::decision_record::PlanarBooleanSplitVertexCoalescenceDecision;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanSplitVertexIdentityRow {
    split_vertex_identity: String,
    source_edge_identity: String,
    carrier_identity: String,
    normalized_parameter: f64,
    normalized_parameter_bits: u64,
    local_frame_identity: String,
    precision_basis_identity: String,
    point_cut_identities: Vec<String>,
    parameter_fact_identities: Vec<String>,
    interval_subdivision_identities: Vec<String>,
    normalized_interval_identities: Vec<String>,
    coordinate_fact_identities: Vec<String>,
    coalescence_provenance: Vec<String>,
    event_group_identities: Vec<String>,
}

impl PlanarBooleanSplitVertexIdentityRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        split_vertex_identity: String,
        source_edge_identity: String,
        carrier_identity: String,
        normalized_parameter: f64,
        normalized_parameter_bits: u64,
        local_frame_identity: String,
        precision_basis_identity: String,
        point_cut_identities: Vec<String>,
        parameter_fact_identities: Vec<String>,
        interval_subdivision_identities: Vec<String>,
        normalized_interval_identities: Vec<String>,
        coordinate_fact_identities: Vec<String>,
        coalescence_provenance: Vec<String>,
        event_group_identities: Vec<String>,
    ) -> Self {
        Self {
            split_vertex_identity,
            source_edge_identity,
            carrier_identity,
            normalized_parameter,
            normalized_parameter_bits,
            local_frame_identity,
            precision_basis_identity,
            point_cut_identities,
            parameter_fact_identities,
            interval_subdivision_identities,
            normalized_interval_identities,
            coordinate_fact_identities,
            coalescence_provenance,
            event_group_identities,
        }
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
    pub fn normalized_parameter(&self) -> f64 {
        self.normalized_parameter
    }
    pub fn normalized_parameter_bits(&self) -> u64 {
        self.normalized_parameter_bits
    }
    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }
    pub fn precision_basis_identity(&self) -> &str {
        &self.precision_basis_identity
    }
    pub fn point_cut_identities(&self) -> &[String] {
        &self.point_cut_identities
    }
    pub fn parameter_fact_identities(&self) -> &[String] {
        &self.parameter_fact_identities
    }
    pub fn interval_subdivision_identities(&self) -> &[String] {
        &self.interval_subdivision_identities
    }
    pub fn normalized_interval_identities(&self) -> &[String] {
        &self.normalized_interval_identities
    }
    pub fn coordinate_fact_identities(&self) -> &[String] {
        &self.coordinate_fact_identities
    }
    pub fn coalescence_provenance(&self) -> &[String] {
        &self.coalescence_provenance
    }
    pub fn event_group_identities(&self) -> &[String] {
        &self.event_group_identities
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanSplitVertexIdentitySchedule {
    schedule_identity: String,
    interval_subdivision_schedule_identity: String,
    source_edge_identity: String,
    carrier_identity: String,
    vertices: Vec<PlanarBooleanSplitVertexIdentityRow>,
    coalescence_decisions: Vec<PlanarBooleanSplitVertexCoalescenceDecision>,
}

impl PlanarBooleanSplitVertexIdentitySchedule {
    pub(crate) fn new(
        schedule_identity: String,
        interval_subdivision_schedule_identity: String,
        source_edge_identity: String,
        carrier_identity: String,
        vertices: Vec<PlanarBooleanSplitVertexIdentityRow>,
        coalescence_decisions: Vec<PlanarBooleanSplitVertexCoalescenceDecision>,
    ) -> Self {
        Self {
            schedule_identity,
            interval_subdivision_schedule_identity,
            source_edge_identity,
            carrier_identity,
            vertices,
            coalescence_decisions,
        }
    }

    pub fn schedule_identity(&self) -> &str {
        &self.schedule_identity
    }
    pub fn interval_subdivision_schedule_identity(&self) -> &str {
        &self.interval_subdivision_schedule_identity
    }
    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }
    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }
    pub fn vertices(&self) -> &[PlanarBooleanSplitVertexIdentityRow] {
        &self.vertices
    }
    pub fn coalescence_decisions(&self) -> &[PlanarBooleanSplitVertexCoalescenceDecision] {
        &self.coalescence_decisions
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanSplitVertexIdentitySet {
    split_vertex_identity_set_identity: String,
    interval_subdivision_schedule_set_identity: String,
    schedules: Vec<PlanarBooleanSplitVertexIdentitySchedule>,
    counters: PlanarBooleanSplitVertexIdentityCounters,
}

impl PlanarBooleanSplitVertexIdentitySet {
    pub(crate) fn new(
        split_vertex_identity_set_identity: String,
        interval_subdivision_schedule_set_identity: String,
        schedules: Vec<PlanarBooleanSplitVertexIdentitySchedule>,
        counters: PlanarBooleanSplitVertexIdentityCounters,
    ) -> Self {
        Self {
            split_vertex_identity_set_identity,
            interval_subdivision_schedule_set_identity,
            schedules,
            counters,
        }
    }

    pub fn split_vertex_identity_set_identity(&self) -> &str {
        &self.split_vertex_identity_set_identity
    }
    pub fn interval_subdivision_schedule_set_identity(&self) -> &str {
        &self.interval_subdivision_schedule_set_identity
    }
    pub fn schedules(&self) -> &[PlanarBooleanSplitVertexIdentitySchedule] {
        &self.schedules
    }
    pub fn counters(&self) -> PlanarBooleanSplitVertexIdentityCounters {
        self.counters
    }
    pub fn vertices(&self) -> impl Iterator<Item = &PlanarBooleanSplitVertexIdentityRow> {
        self.schedules
            .iter()
            .flat_map(|schedule| schedule.vertices())
    }
    pub fn coalescence_decisions(
        &self,
    ) -> impl Iterator<Item = &PlanarBooleanSplitVertexCoalescenceDecision> {
        self.schedules
            .iter()
            .flat_map(|schedule| schedule.coalescence_decisions())
    }
}
