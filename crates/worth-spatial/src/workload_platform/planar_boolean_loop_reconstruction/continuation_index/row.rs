use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole;
use crate::workload_platform::planar_boolean_events::PlanarBooleanSourceIntervalSense;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PlanarBooleanFragmentContinuationEndpointRole {
    Start,
    End,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanFragmentContinuationRow {
    continuation_identity: String,
    neighborhood_identity: String,
    split_vertex_identity: String,
    fragment_identity: String,
    source_loop_identity: String,
    source_face_identity: String,
    source_edge_identity: String,
    carrier_identity: String,
    source_loop_carrier_identity: String,
    fragment_endpoint_role: PlanarBooleanFragmentContinuationEndpointRole,
    source_sense: PlanarBooleanSourceIntervalSense,
    endpoint_parameter_bits: u64,
    fragment_parameter_range_bits: [u64; 2],
    local_frame_identity: String,
    precision_basis_identity: String,
    event_group_identities: Vec<String>,
    boundary_roles: Vec<PlanarBooleanOverlapChainBoundaryRole>,
}

impl PlanarBooleanFragmentContinuationRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        continuation_identity: String,
        neighborhood_identity: String,
        split_vertex_identity: String,
        fragment_identity: String,
        source_loop_identity: String,
        source_face_identity: String,
        source_edge_identity: String,
        carrier_identity: String,
        source_loop_carrier_identity: String,
        fragment_endpoint_role: PlanarBooleanFragmentContinuationEndpointRole,
        source_sense: PlanarBooleanSourceIntervalSense,
        endpoint_parameter_bits: u64,
        fragment_parameter_range_bits: [u64; 2],
        local_frame_identity: String,
        precision_basis_identity: String,
        event_group_identities: Vec<String>,
        boundary_roles: Vec<PlanarBooleanOverlapChainBoundaryRole>,
    ) -> Self {
        Self {
            continuation_identity,
            neighborhood_identity,
            split_vertex_identity,
            fragment_identity,
            source_loop_identity,
            source_face_identity,
            source_edge_identity,
            carrier_identity,
            source_loop_carrier_identity,
            fragment_endpoint_role,
            source_sense,
            endpoint_parameter_bits,
            fragment_parameter_range_bits,
            local_frame_identity,
            precision_basis_identity,
            event_group_identities,
            boundary_roles,
        }
    }

    pub fn continuation_identity(&self) -> &str {
        &self.continuation_identity
    }

    pub fn neighborhood_identity(&self) -> &str {
        &self.neighborhood_identity
    }

    pub fn split_vertex_identity(&self) -> &str {
        &self.split_vertex_identity
    }

    pub fn fragment_identity(&self) -> &str {
        &self.fragment_identity
    }

    pub fn source_loop_identity(&self) -> &str {
        &self.source_loop_identity
    }

    pub fn source_face_identity(&self) -> &str {
        &self.source_face_identity
    }

    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }

    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }

    pub fn source_loop_carrier_identity(&self) -> &str {
        &self.source_loop_carrier_identity
    }

    pub fn fragment_endpoint_role(&self) -> PlanarBooleanFragmentContinuationEndpointRole {
        self.fragment_endpoint_role
    }

    pub fn source_sense(&self) -> PlanarBooleanSourceIntervalSense {
        self.source_sense
    }

    pub fn endpoint_parameter_bits(&self) -> u64 {
        self.endpoint_parameter_bits
    }

    pub fn fragment_parameter_range_bits(&self) -> [u64; 2] {
        self.fragment_parameter_range_bits
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

    pub fn boundary_roles(&self) -> &[PlanarBooleanOverlapChainBoundaryRole] {
        &self.boundary_roles
    }
}
