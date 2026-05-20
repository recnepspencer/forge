use crate::spatial_intent::refs::{SpatialAxis, SpatialFrameRef};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpatialCarrierKind {
    Curve,
    Surface,
    Feature,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpatialCarrierDirectionRole {
    Tangent,
    Normal,
    TangentU,
    TangentV,
    Axis,
    Spine,
    Rail,
    JunctionFrame,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SpatialDirectionWitnessRef {
    WorldDirection([f64; 3]),
    FrameAxis {
        frame: SpatialFrameRef,
        axis: SpatialAxis,
    },
    FramePerpendicularAxis {
        frame: SpatialFrameRef,
        axis: SpatialAxis,
    },
    CarrierDirection {
        carrier_kind: SpatialCarrierKind,
        carrier: String,
    },
    ParameterSpaceDirection {
        carrier_kind: SpatialCarrierKind,
        carrier: String,
        parameter: [f64; 2],
        role: SpatialCarrierDirectionRole,
    },
    FeatureOwnedDirection {
        feature: String,
        role: SpatialCarrierDirectionRole,
    },
}

impl SpatialDirectionWitnessRef {
    pub fn world_direction(direction: [f64; 3]) -> Self {
        Self::WorldDirection(direction)
    }

    pub fn frame_axis(frame: SpatialFrameRef, axis: SpatialAxis) -> Self {
        Self::FrameAxis { frame, axis }
    }

    pub fn frame_perpendicular_axis(frame: SpatialFrameRef, axis: SpatialAxis) -> Self {
        Self::FramePerpendicularAxis { frame, axis }
    }

    pub fn ambiguous_curve(carrier: impl Into<String>) -> Self {
        Self::CarrierDirection {
            carrier_kind: SpatialCarrierKind::Curve,
            carrier: carrier.into(),
        }
    }

    pub fn ambiguous_surface(carrier: impl Into<String>) -> Self {
        Self::CarrierDirection {
            carrier_kind: SpatialCarrierKind::Surface,
            carrier: carrier.into(),
        }
    }

    pub fn curve_tangent(carrier: impl Into<String>, parameter: f64) -> Self {
        Self::ParameterSpaceDirection {
            carrier_kind: SpatialCarrierKind::Curve,
            carrier: carrier.into(),
            parameter: [parameter, 0.0],
            role: SpatialCarrierDirectionRole::Tangent,
        }
    }

    pub fn surface_normal(carrier: impl Into<String>, u: f64, v: f64) -> Self {
        Self::ParameterSpaceDirection {
            carrier_kind: SpatialCarrierKind::Surface,
            carrier: carrier.into(),
            parameter: [u, v],
            role: SpatialCarrierDirectionRole::Normal,
        }
    }

    pub fn surface_tangent_u(carrier: impl Into<String>, u: f64, v: f64) -> Self {
        Self::ParameterSpaceDirection {
            carrier_kind: SpatialCarrierKind::Surface,
            carrier: carrier.into(),
            parameter: [u, v],
            role: SpatialCarrierDirectionRole::TangentU,
        }
    }

    pub fn surface_tangent_v(carrier: impl Into<String>, u: f64, v: f64) -> Self {
        Self::ParameterSpaceDirection {
            carrier_kind: SpatialCarrierKind::Surface,
            carrier: carrier.into(),
            parameter: [u, v],
            role: SpatialCarrierDirectionRole::TangentV,
        }
    }

    pub fn feature_axis(feature: impl Into<String>) -> Self {
        Self::FeatureOwnedDirection {
            feature: feature.into(),
            role: SpatialCarrierDirectionRole::Axis,
        }
    }

    pub fn feature_spine(feature: impl Into<String>) -> Self {
        Self::FeatureOwnedDirection {
            feature: feature.into(),
            role: SpatialCarrierDirectionRole::Spine,
        }
    }

    pub fn feature_rail(feature: impl Into<String>) -> Self {
        Self::FeatureOwnedDirection {
            feature: feature.into(),
            role: SpatialCarrierDirectionRole::Rail,
        }
    }

    pub fn feature_junction_frame(feature: impl Into<String>) -> Self {
        Self::FeatureOwnedDirection {
            feature: feature.into(),
            role: SpatialCarrierDirectionRole::JunctionFrame,
        }
    }
}
