use crate::workload_platform::planar_boolean_events::segment_carriers::PlanarBooleanSegmentCarrier;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanCanonicalSegmentSetDenialKind {
    CollapsedProjectedSegment,
    NonFiniteEndpointCoordinate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanCanonicalSegmentSetDenial {
    kind: PlanarBooleanCanonicalSegmentSetDenialKind,
    carrier_identity: String,
    local_frame_identity: String,
    projection_stage_identity: String,
    precision_basis_identity: String,
    human_reason: String,
}

impl PlanarBooleanCanonicalSegmentSetDenial {
    pub(crate) fn from_carrier(
        kind: PlanarBooleanCanonicalSegmentSetDenialKind,
        carrier: &PlanarBooleanSegmentCarrier,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            carrier_identity: carrier.carrier_identity().to_string(),
            local_frame_identity: carrier.local_frame_identity().to_string(),
            projection_stage_identity: carrier.projection_stage_identity().to_string(),
            precision_basis_identity: carrier.precision_basis_identity().to_string(),
            human_reason: human_reason.into(),
        }
    }

    pub fn kind(&self) -> PlanarBooleanCanonicalSegmentSetDenialKind {
        self.kind
    }

    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
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

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
