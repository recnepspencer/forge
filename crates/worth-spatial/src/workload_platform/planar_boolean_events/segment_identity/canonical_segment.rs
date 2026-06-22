use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_events::endpoint_normalization::PlanarBooleanNormalizedEndpoint;
use crate::workload_platform::planar_boolean_events::endpoint_normalization::PlanarBooleanNormalizedEndpointPair;
use crate::workload_platform::planar_boolean_events::segment_carriers::{
    PlanarBooleanLoopRole, PlanarBooleanSegmentCarrier,
};

use super::identity::canonical_segment_identity;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanCanonicalSegment {
    operand_side: PlanarBooleanCommonPlaneOperandSide,
    source_face_identity: String,
    source_loop_identity: String,
    source_edge_identity: String,
    loop_role: PlanarBooleanLoopRole,
    carrier_identity: String,
    normalized_endpoints: PlanarBooleanNormalizedEndpointPair,
    local_frame_identity: String,
    projection_stage_identity: String,
    precision_basis_identity: String,
    canonical_segment_identity: String,
}

impl PlanarBooleanCanonicalSegment {
    pub(crate) fn from_carrier(
        carrier: &PlanarBooleanSegmentCarrier,
        normalized_endpoints: PlanarBooleanNormalizedEndpointPair,
    ) -> Self {
        let segment = Self {
            operand_side: carrier.operand_side(),
            source_face_identity: carrier.source_face_identity().to_string(),
            source_loop_identity: carrier.source_loop_identity().to_string(),
            source_edge_identity: carrier.source_edge_identity().to_string(),
            loop_role: carrier.loop_role(),
            carrier_identity: carrier.carrier_identity().to_string(),
            normalized_endpoints,
            local_frame_identity: carrier.local_frame_identity().to_string(),
            projection_stage_identity: carrier.projection_stage_identity().to_string(),
            precision_basis_identity: carrier.precision_basis_identity().to_string(),
            canonical_segment_identity: String::new(),
        };
        Self {
            canonical_segment_identity: canonical_segment_identity(&segment),
            ..segment
        }
    }

    pub fn operand_side(&self) -> PlanarBooleanCommonPlaneOperandSide {
        self.operand_side
    }

    pub fn source_face_identity(&self) -> &str {
        &self.source_face_identity
    }

    pub fn source_loop_identity(&self) -> &str {
        &self.source_loop_identity
    }

    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }

    pub fn loop_role(&self) -> PlanarBooleanLoopRole {
        self.loop_role
    }

    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }

    pub fn normalized_endpoints(&self) -> &PlanarBooleanNormalizedEndpointPair {
        &self.normalized_endpoints
    }

    pub fn source_ordered_start_endpoint(&self) -> &PlanarBooleanNormalizedEndpoint {
        if self.orientation_was_reversed() {
            self.normalized_endpoints.high()
        } else {
            self.normalized_endpoints.low()
        }
    }

    pub fn source_ordered_end_endpoint(&self) -> &PlanarBooleanNormalizedEndpoint {
        if self.orientation_was_reversed() {
            self.normalized_endpoints.low()
        } else {
            self.normalized_endpoints.high()
        }
    }

    pub fn orientation_was_reversed(&self) -> bool {
        self.normalized_endpoints.orientation_was_reversed()
    }

    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }

    pub fn projection_stage_identity(&self) -> &str {
        &self.projection_stage_identity
    }

    pub fn precision_basis_identity(&self) -> &str {
        &self.precision_basis_identity
    }

    pub fn canonical_segment_identity(&self) -> &str {
        &self.canonical_segment_identity
    }
}
