use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanPointSplitPosture;

use super::boundary_position::PlanarBooleanSplitBoundaryPosition;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanEndpointContactDecision {
    decision_identity: String,
    normalized_cut_identity: String,
    duplicate_report_identity: String,
    source_edge_identity: String,
    carrier_identity: String,
    boundary_position: PlanarBooleanSplitBoundaryPosition,
    posture: PlanarBooleanPointSplitPosture,
    source_endpoint_identity: String,
    projected_endpoint_fact_identity: String,
    provenance_entry_identities: Vec<String>,
    event_group_identities: Vec<String>,
    shared_endpoint_source_identities: Vec<String>,
    shared_endpoint_projection_fact_digests: Vec<String>,
}

impl PlanarBooleanEndpointContactDecision {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        decision_identity: String,
        normalized_cut_identity: String,
        duplicate_report_identity: String,
        source_edge_identity: String,
        carrier_identity: String,
        boundary_position: PlanarBooleanSplitBoundaryPosition,
        posture: PlanarBooleanPointSplitPosture,
        source_endpoint_identity: String,
        projected_endpoint_fact_identity: String,
        provenance_entry_identities: Vec<String>,
        event_group_identities: Vec<String>,
        shared_endpoint_source_identities: Vec<String>,
        shared_endpoint_projection_fact_digests: Vec<String>,
    ) -> Self {
        Self {
            decision_identity,
            normalized_cut_identity,
            duplicate_report_identity,
            source_edge_identity,
            carrier_identity,
            boundary_position,
            posture,
            source_endpoint_identity,
            projected_endpoint_fact_identity,
            provenance_entry_identities,
            event_group_identities,
            shared_endpoint_source_identities,
            shared_endpoint_projection_fact_digests,
        }
    }

    pub fn decision_identity(&self) -> &str {
        &self.decision_identity
    }
    pub fn normalized_cut_identity(&self) -> &str {
        &self.normalized_cut_identity
    }
    pub fn duplicate_report_identity(&self) -> &str {
        &self.duplicate_report_identity
    }
    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }
    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }
    pub fn boundary_position_name(&self) -> &str {
        self.boundary_position.as_str()
    }
    pub fn posture(&self) -> PlanarBooleanPointSplitPosture {
        self.posture
    }
    pub fn source_endpoint_identity(&self) -> &str {
        &self.source_endpoint_identity
    }
    pub fn projected_endpoint_fact_identity(&self) -> &str {
        &self.projected_endpoint_fact_identity
    }
    pub fn provenance_entry_identities(&self) -> &[String] {
        &self.provenance_entry_identities
    }
    pub fn event_group_identities(&self) -> &[String] {
        &self.event_group_identities
    }
    pub fn shared_endpoint_source_identities(&self) -> &[String] {
        &self.shared_endpoint_source_identities
    }
    pub fn shared_endpoint_projection_fact_digests(&self) -> &[String] {
        &self.shared_endpoint_projection_fact_digests
    }
}
