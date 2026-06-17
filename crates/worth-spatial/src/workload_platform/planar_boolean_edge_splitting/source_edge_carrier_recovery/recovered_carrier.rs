use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanLoopRole, PlanarBooleanSegmentCarrier,
};

use super::identity::recovered_carrier_identity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitSourceEdgeCarrier {
    recovered_carrier_identity: String,
    operand_side: PlanarBooleanCommonPlaneOperandSide,
    source_face_identity: String,
    source_loop_identity: String,
    source_edge_identity: String,
    loop_role: PlanarBooleanLoopRole,
    carrier_identity: String,
    start_source_endpoint_identity: String,
    start_projected_endpoint_fact_identity: String,
    end_source_endpoint_identity: String,
    end_projected_endpoint_fact_identity: String,
    local_frame_identity: String,
    projection_stage_identity: String,
    precision_basis_identity: String,
}

impl PlanarBooleanSplitSourceEdgeCarrier {
    pub(crate) fn from_segment_carrier(
        scope_admission_identity: &str,
        event_ledger_identity: &str,
        carrier: &PlanarBooleanSegmentCarrier,
    ) -> Self {
        let recovered = Self {
            recovered_carrier_identity: String::new(),
            operand_side: carrier.operand_side(),
            source_face_identity: carrier.source_face_identity().to_string(),
            source_loop_identity: carrier.source_loop_identity().to_string(),
            source_edge_identity: carrier.source_edge_identity().to_string(),
            loop_role: carrier.loop_role(),
            carrier_identity: carrier.carrier_identity().to_string(),
            start_source_endpoint_identity: carrier.start().source_endpoint_identity().to_string(),
            start_projected_endpoint_fact_identity: carrier
                .start()
                .projected_endpoint_fact_identity()
                .to_string(),
            end_source_endpoint_identity: carrier.end().source_endpoint_identity().to_string(),
            end_projected_endpoint_fact_identity: carrier
                .end()
                .projected_endpoint_fact_identity()
                .to_string(),
            local_frame_identity: carrier.local_frame_identity().to_string(),
            projection_stage_identity: carrier.projection_stage_identity().to_string(),
            precision_basis_identity: carrier.precision_basis_identity().to_string(),
        };
        Self {
            recovered_carrier_identity: recovered_carrier_identity(
                scope_admission_identity,
                event_ledger_identity,
                &recovered,
            ),
            ..recovered
        }
    }

    pub fn recovered_carrier_identity(&self) -> &str {
        &self.recovered_carrier_identity
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

    pub fn start_source_endpoint_identity(&self) -> &str {
        &self.start_source_endpoint_identity
    }

    pub fn start_projected_endpoint_fact_identity(&self) -> &str {
        &self.start_projected_endpoint_fact_identity
    }

    pub fn end_source_endpoint_identity(&self) -> &str {
        &self.end_source_endpoint_identity
    }

    pub fn end_projected_endpoint_fact_identity(&self) -> &str {
        &self.end_projected_endpoint_fact_identity
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
}
