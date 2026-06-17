use crate::workload_platform::planar_boolean_events::PlanarBooleanSourceIntervalSense;

use super::boundary_role::PlanarBooleanOverlapChainBoundaryRole;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanOverlapEdgeChainMember {
    member_identity: String,
    fragment_identity: String,
    interval_subdivision_identity: String,
    source_edge_identity: String,
    carrier_identity: String,
    fragment_parameter_range: [f64; 2],
    source_interval_identity: String,
    source_parameter_range: [f64; 2],
    source_sense: PlanarBooleanSourceIntervalSense,
    normalized_interval_identity: String,
    normalized_parameter_range: [f64; 2],
    boundary_role: PlanarBooleanOverlapChainBoundaryRole,
    local_frame_identity: String,
    precision_basis_identity: String,
    event_group_identities: Vec<String>,
    provenance_identities: Vec<String>,
}

impl PlanarBooleanOverlapEdgeChainMember {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        member_identity: String,
        fragment_identity: String,
        interval_subdivision_identity: String,
        source_edge_identity: String,
        carrier_identity: String,
        fragment_parameter_range: [f64; 2],
        source_interval_identity: String,
        source_parameter_range: [f64; 2],
        source_sense: PlanarBooleanSourceIntervalSense,
        normalized_interval_identity: String,
        normalized_parameter_range: [f64; 2],
        boundary_role: PlanarBooleanOverlapChainBoundaryRole,
        local_frame_identity: String,
        precision_basis_identity: String,
        event_group_identities: Vec<String>,
        provenance_identities: Vec<String>,
    ) -> Self {
        Self {
            member_identity,
            fragment_identity,
            interval_subdivision_identity,
            source_edge_identity,
            carrier_identity,
            fragment_parameter_range,
            source_interval_identity,
            source_parameter_range,
            source_sense,
            normalized_interval_identity,
            normalized_parameter_range,
            boundary_role,
            local_frame_identity,
            precision_basis_identity,
            event_group_identities,
            provenance_identities,
        }
    }

    pub fn member_identity(&self) -> &str {
        &self.member_identity
    }
    pub fn fragment_identity(&self) -> &str {
        &self.fragment_identity
    }
    pub fn interval_subdivision_identity(&self) -> &str {
        &self.interval_subdivision_identity
    }
    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }
    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }
    pub fn fragment_parameter_range(&self) -> [f64; 2] {
        self.fragment_parameter_range
    }
    pub fn source_interval_identity(&self) -> &str {
        &self.source_interval_identity
    }
    pub fn source_parameter_range(&self) -> [f64; 2] {
        self.source_parameter_range
    }
    pub fn source_sense(&self) -> PlanarBooleanSourceIntervalSense {
        self.source_sense
    }
    pub fn normalized_interval_identity(&self) -> &str {
        &self.normalized_interval_identity
    }
    pub fn normalized_parameter_range(&self) -> [f64; 2] {
        self.normalized_parameter_range
    }
    pub fn boundary_role(&self) -> PlanarBooleanOverlapChainBoundaryRole {
        self.boundary_role
    }
    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }
    pub fn precision_basis_identity(&self) -> &str {
        &self.precision_basis_identity
    }
    pub fn event_group_identities(&self) -> &[String] {
        &self.event_group_identities
    }
    pub fn provenance_identities(&self) -> &[String] {
        &self.provenance_identities
    }
}
