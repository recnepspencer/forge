use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;

use super::endpoint_facts::PlanarBooleanSegmentCarrierEndpointFacts;
use super::identity::segment_carrier_identity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanLoopRole {
    OuterBoundary,
}

impl PlanarBooleanLoopRole {
    pub fn query_key(self) -> &'static str {
        match self {
            Self::OuterBoundary => "outer_boundary",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanSegmentCarrier {
    operand_side: PlanarBooleanCommonPlaneOperandSide,
    source_face_identity: String,
    source_loop_identity: String,
    source_edge_identity: String,
    loop_role: PlanarBooleanLoopRole,
    start: PlanarBooleanSegmentCarrierEndpointFacts,
    end: PlanarBooleanSegmentCarrierEndpointFacts,
    local_frame_identity: String,
    projection_stage_identity: String,
    precision_basis_identity: String,
    carrier_identity: String,
}

impl PlanarBooleanSegmentCarrier {
    pub(crate) fn new(input: PlanarBooleanSegmentCarrierInput) -> Self {
        let carrier = Self {
            operand_side: input.operand_side,
            source_face_identity: input.source_face_identity,
            source_loop_identity: input.source_loop_identity,
            source_edge_identity: input.source_edge_identity,
            loop_role: input.loop_role,
            start: input.start,
            end: input.end,
            local_frame_identity: input.local_frame_identity,
            projection_stage_identity: input.projection_stage_identity,
            precision_basis_identity: input.precision_basis_identity,
            carrier_identity: String::new(),
        };
        Self {
            carrier_identity: segment_carrier_identity(&carrier),
            ..carrier
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

    pub fn start(&self) -> &PlanarBooleanSegmentCarrierEndpointFacts {
        &self.start
    }

    pub fn end(&self) -> &PlanarBooleanSegmentCarrierEndpointFacts {
        &self.end
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

    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }

    #[cfg(test)]
    pub(crate) fn for_canonical_segment_test(
        start: PlanarBooleanSegmentCarrierEndpointFacts,
        end: PlanarBooleanSegmentCarrierEndpointFacts,
    ) -> Self {
        Self::for_canonical_segment_test_on_side(
            PlanarBooleanCommonPlaneOperandSide::Left,
            start,
            end,
        )
    }

    #[cfg(test)]
    pub(crate) fn for_canonical_segment_test_on_side(
        operand_side: PlanarBooleanCommonPlaneOperandSide,
        start: PlanarBooleanSegmentCarrierEndpointFacts,
        end: PlanarBooleanSegmentCarrierEndpointFacts,
    ) -> Self {
        Self::for_canonical_segment_test_on_side_with_source_edge(
            operand_side,
            "test edge",
            start,
            end,
        )
    }

    #[cfg(test)]
    pub(crate) fn for_canonical_segment_test_on_side_with_source_edge(
        operand_side: PlanarBooleanCommonPlaneOperandSide,
        source_edge_identity: impl Into<String>,
        start: PlanarBooleanSegmentCarrierEndpointFacts,
        end: PlanarBooleanSegmentCarrierEndpointFacts,
    ) -> Self {
        Self::new(PlanarBooleanSegmentCarrierInput {
            operand_side,
            source_face_identity: "test face".to_string(),
            source_loop_identity: "test loop".to_string(),
            source_edge_identity: source_edge_identity.into(),
            loop_role: PlanarBooleanLoopRole::OuterBoundary,
            start,
            end,
            local_frame_identity: "test local frame".to_string(),
            projection_stage_identity: "test projection stage".to_string(),
            precision_basis_identity: "test precision basis".to_string(),
        })
    }
}

pub(crate) struct PlanarBooleanSegmentCarrierInput {
    pub(crate) operand_side: PlanarBooleanCommonPlaneOperandSide,
    pub(crate) source_face_identity: String,
    pub(crate) source_loop_identity: String,
    pub(crate) source_edge_identity: String,
    pub(crate) loop_role: PlanarBooleanLoopRole,
    pub(crate) start: PlanarBooleanSegmentCarrierEndpointFacts,
    pub(crate) end: PlanarBooleanSegmentCarrierEndpointFacts,
    pub(crate) local_frame_identity: String,
    pub(crate) projection_stage_identity: String,
    pub(crate) precision_basis_identity: String,
}
