use crate::workload_platform::planar_boolean_events::PlanarBooleanSourceIntervalSense;

use super::row::{
    PlanarBooleanFragmentContinuationEndpointRole, PlanarBooleanFragmentContinuationRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanContinuationOrderingBasis {
    basis_identity: String,
    request_identity: String,
    continuation_index_identity: String,
    ordered_continuation_identities: Vec<String>,
}

impl PlanarBooleanContinuationOrderingBasis {
    pub(crate) fn new(
        basis_identity: String,
        request_identity: String,
        continuation_index_identity: String,
        ordered_continuation_identities: Vec<String>,
    ) -> Self {
        Self {
            basis_identity,
            request_identity,
            continuation_index_identity,
            ordered_continuation_identities,
        }
    }

    pub fn basis_identity(&self) -> &str {
        &self.basis_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn continuation_index_identity(&self) -> &str {
        &self.continuation_index_identity
    }

    pub fn ordered_continuation_identities(&self) -> &[String] {
        &self.ordered_continuation_identities
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PlanarBooleanContinuationOrderingKey<'a> {
    split_vertex_identity: &'a str,
    source_sense: PlanarBooleanSourceIntervalSense,
    endpoint_parameter_bits: u64,
    fragment_start_parameter_bits: u64,
    fragment_end_parameter_bits: u64,
    fragment_endpoint_role: PlanarBooleanFragmentContinuationEndpointRole,
    source_loop_identity: &'a str,
    fragment_identity: &'a str,
    source_edge_identity: &'a str,
    carrier_identity: &'a str,
    continuation_identity: &'a str,
}

impl<'a> PlanarBooleanContinuationOrderingKey<'a> {
    pub fn split_vertex_identity(&self) -> &'a str {
        self.split_vertex_identity
    }

    pub fn source_sense(&self) -> PlanarBooleanSourceIntervalSense {
        self.source_sense
    }

    pub fn endpoint_parameter_bits(&self) -> u64 {
        self.endpoint_parameter_bits
    }

    pub fn fragment_parameter_range_bits(&self) -> [u64; 2] {
        [
            self.fragment_start_parameter_bits,
            self.fragment_end_parameter_bits,
        ]
    }

    pub fn fragment_endpoint_role(&self) -> PlanarBooleanFragmentContinuationEndpointRole {
        self.fragment_endpoint_role
    }

    pub fn source_loop_identity(&self) -> &'a str {
        self.source_loop_identity
    }

    pub fn fragment_identity(&self) -> &'a str {
        self.fragment_identity
    }

    pub fn source_edge_identity(&self) -> &'a str {
        self.source_edge_identity
    }

    pub fn carrier_identity(&self) -> &'a str {
        self.carrier_identity
    }

    pub fn continuation_identity(&self) -> &'a str {
        self.continuation_identity
    }
}

pub(crate) fn canonicalize_continuation_rows(
    rows: &mut [PlanarBooleanFragmentContinuationRow],
) -> Vec<String> {
    rows.sort_by(|left, right| continuation_order_key(left).cmp(&continuation_order_key(right)));
    rows.iter()
        .map(|row| row.continuation_identity().to_string())
        .collect()
}

pub(crate) fn continuation_order_key(
    row: &PlanarBooleanFragmentContinuationRow,
) -> PlanarBooleanContinuationOrderingKey<'_> {
    PlanarBooleanContinuationOrderingKey {
        split_vertex_identity: row.split_vertex_identity(),
        source_sense: row.source_sense(),
        endpoint_parameter_bits: row.endpoint_parameter_bits(),
        fragment_start_parameter_bits: row.fragment_parameter_range_bits()[0],
        fragment_end_parameter_bits: row.fragment_parameter_range_bits()[1],
        fragment_endpoint_role: row.fragment_endpoint_role(),
        source_loop_identity: row.source_loop_identity(),
        fragment_identity: row.fragment_identity(),
        source_edge_identity: row.source_edge_identity(),
        carrier_identity: row.carrier_identity(),
        continuation_identity: row.continuation_identity(),
    }
}
