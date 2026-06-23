use crate::workload_platform::planar_boolean_events::PlanarBooleanSourceIntervalSense;

use super::endpoint_ref::PlanarBooleanSplitEdgeFragmentEndpointRef;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanSplitEdgeFragment {
    fragment_identity: String,
    source_edge_identity: String,
    carrier_identity: String,
    start_endpoint: PlanarBooleanSplitEdgeFragmentEndpointRef,
    end_endpoint: PlanarBooleanSplitEdgeFragmentEndpointRef,
    parameter_range: [f64; 2],
    parameter_range_bits: [u64; 2],
    local_frame_identity: String,
    precision_basis_identity: String,
    source_senses: Vec<PlanarBooleanSourceIntervalSense>,
    point_cut_identities: Vec<String>,
    interval_subdivision_identities: Vec<String>,
    normalized_interval_identities: Vec<String>,
    event_group_identities: Vec<String>,
    cause_provenance_identities: Vec<String>,
}

impl PlanarBooleanSplitEdgeFragment {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        fragment_identity: String,
        source_edge_identity: String,
        carrier_identity: String,
        start_endpoint: PlanarBooleanSplitEdgeFragmentEndpointRef,
        end_endpoint: PlanarBooleanSplitEdgeFragmentEndpointRef,
        parameter_range: [f64; 2],
        parameter_range_bits: [u64; 2],
        local_frame_identity: String,
        precision_basis_identity: String,
        source_senses: Vec<PlanarBooleanSourceIntervalSense>,
        point_cut_identities: Vec<String>,
        interval_subdivision_identities: Vec<String>,
        normalized_interval_identities: Vec<String>,
        event_group_identities: Vec<String>,
        cause_provenance_identities: Vec<String>,
    ) -> Self {
        Self {
            fragment_identity,
            source_edge_identity,
            carrier_identity,
            start_endpoint,
            end_endpoint,
            parameter_range,
            parameter_range_bits,
            local_frame_identity,
            precision_basis_identity,
            source_senses,
            point_cut_identities,
            interval_subdivision_identities,
            normalized_interval_identities,
            event_group_identities,
            cause_provenance_identities,
        }
    }

    pub fn fragment_identity(&self) -> &str {
        &self.fragment_identity
    }
    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }
    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }
    pub fn start_endpoint(&self) -> &PlanarBooleanSplitEdgeFragmentEndpointRef {
        &self.start_endpoint
    }
    pub fn end_endpoint(&self) -> &PlanarBooleanSplitEdgeFragmentEndpointRef {
        &self.end_endpoint
    }
    pub fn parameter_range(&self) -> [f64; 2] {
        self.parameter_range
    }
    pub fn parameter_range_bits(&self) -> [u64; 2] {
        self.parameter_range_bits
    }
    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }
    pub fn precision_basis_identity(&self) -> &str {
        &self.precision_basis_identity
    }
    pub fn source_senses(&self) -> &[PlanarBooleanSourceIntervalSense] {
        &self.source_senses
    }
    pub fn point_cut_identities(&self) -> &[String] {
        &self.point_cut_identities
    }
    pub fn interval_subdivision_identities(&self) -> &[String] {
        &self.interval_subdivision_identities
    }
    pub fn normalized_interval_identities(&self) -> &[String] {
        &self.normalized_interval_identities
    }
    pub fn event_group_identities(&self) -> &[String] {
        &self.event_group_identities
    }
    pub fn cause_provenance_identities(&self) -> &[String] {
        &self.cause_provenance_identities
    }
}
