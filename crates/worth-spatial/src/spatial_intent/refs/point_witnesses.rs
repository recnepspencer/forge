use crate::spatial_intent::refs::{SpatialCarrierKind, SpatialFrameRef};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpatialCarrierPointRole {
    Point,
    Origin,
    Anchor,
    Junction,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SpatialPointWitnessRef {
    WorldPoint([f64; 3]),
    FrameOrigin(SpatialFrameRef),
    CarrierPoint {
        carrier_kind: SpatialCarrierKind,
        carrier: String,
    },
    ParameterSpacePoint {
        carrier_kind: SpatialCarrierKind,
        carrier: String,
        parameter: [f64; 2],
    },
    FeatureOwnedPoint {
        feature: String,
        role: SpatialCarrierPointRole,
    },
}

impl SpatialPointWitnessRef {
    pub fn world_point(point: [f64; 3]) -> Self {
        Self::WorldPoint(point)
    }

    pub fn frame_origin(frame: SpatialFrameRef) -> Self {
        Self::FrameOrigin(frame)
    }

    pub fn ambiguous_curve_point(carrier: impl Into<String>) -> Self {
        Self::CarrierPoint {
            carrier_kind: SpatialCarrierKind::Curve,
            carrier: carrier.into(),
        }
    }

    pub fn ambiguous_surface_point(carrier: impl Into<String>) -> Self {
        Self::CarrierPoint {
            carrier_kind: SpatialCarrierKind::Surface,
            carrier: carrier.into(),
        }
    }

    pub fn curve_point(carrier: impl Into<String>, parameter: f64) -> Self {
        Self::ParameterSpacePoint {
            carrier_kind: SpatialCarrierKind::Curve,
            carrier: carrier.into(),
            parameter: [parameter, 0.0],
        }
    }

    pub fn surface_point(carrier: impl Into<String>, u: f64, v: f64) -> Self {
        Self::ParameterSpacePoint {
            carrier_kind: SpatialCarrierKind::Surface,
            carrier: carrier.into(),
            parameter: [u, v],
        }
    }

    pub fn feature_origin(feature: impl Into<String>) -> Self {
        Self::FeatureOwnedPoint {
            feature: feature.into(),
            role: SpatialCarrierPointRole::Origin,
        }
    }

    pub fn feature_anchor(feature: impl Into<String>) -> Self {
        Self::FeatureOwnedPoint {
            feature: feature.into(),
            role: SpatialCarrierPointRole::Anchor,
        }
    }
}
