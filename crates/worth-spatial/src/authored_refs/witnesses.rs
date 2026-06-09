use crate::authored_refs::{SpatialAxis, SpatialFrameRef};
use worth_geom::ParameterSpacePoint;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpatialCarrierKind {
    Curve,
    Surface,
    Feature,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpatialCarrierPointRole {
    Point,
    Origin,
    Anchor,
    Junction,
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
        parameter: ParameterSpacePoint,
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
        Self::curve_parameter_point(
            carrier,
            ParameterSpacePoint::try_new([parameter, 0.0])
                .expect("curve-point witness requires finite parameter-space coordinates"),
        )
    }

    pub fn curve_parameter_point(
        carrier: impl Into<String>,
        parameter: ParameterSpacePoint,
    ) -> Self {
        Self::ParameterSpacePoint {
            carrier_kind: SpatialCarrierKind::Curve,
            carrier: carrier.into(),
            parameter,
        }
    }

    pub fn surface_point(carrier: impl Into<String>, u: f64, v: f64) -> Self {
        Self::surface_parameter_point(
            carrier,
            ParameterSpacePoint::try_new([u, v])
                .expect("surface-point witness requires finite parameter-space coordinates"),
        )
    }

    pub fn surface_parameter_point(
        carrier: impl Into<String>,
        parameter: ParameterSpacePoint,
    ) -> Self {
        Self::ParameterSpacePoint {
            carrier_kind: SpatialCarrierKind::Surface,
            carrier: carrier.into(),
            parameter,
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
        parameter: ParameterSpacePoint,
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
        Self::curve_direction_at_parameter(
            carrier,
            ParameterSpacePoint::try_new([parameter, 0.0])
                .expect("curve-direction witness requires finite parameter-space coordinates"),
            SpatialCarrierDirectionRole::Tangent,
        )
    }

    pub fn curve_direction_at_parameter(
        carrier: impl Into<String>,
        parameter: ParameterSpacePoint,
        role: SpatialCarrierDirectionRole,
    ) -> Self {
        Self::ParameterSpaceDirection {
            carrier_kind: SpatialCarrierKind::Curve,
            carrier: carrier.into(),
            parameter,
            role,
        }
    }

    pub fn surface_normal(carrier: impl Into<String>, u: f64, v: f64) -> Self {
        Self::surface_direction_at_parameter(
            carrier,
            ParameterSpacePoint::try_new([u, v])
                .expect("surface-direction witness requires finite parameter-space coordinates"),
            SpatialCarrierDirectionRole::Normal,
        )
    }

    pub fn surface_direction_at_parameter(
        carrier: impl Into<String>,
        parameter: ParameterSpacePoint,
        role: SpatialCarrierDirectionRole,
    ) -> Self {
        Self::ParameterSpaceDirection {
            carrier_kind: SpatialCarrierKind::Surface,
            carrier: carrier.into(),
            parameter,
            role,
        }
    }

    pub fn surface_tangent_u(carrier: impl Into<String>, u: f64, v: f64) -> Self {
        Self::surface_direction_at_parameter(
            carrier,
            ParameterSpacePoint::try_new([u, v])
                .expect("surface-direction witness requires finite parameter-space coordinates"),
            SpatialCarrierDirectionRole::TangentU,
        )
    }

    pub fn surface_tangent_v(carrier: impl Into<String>, u: f64, v: f64) -> Self {
        Self::surface_direction_at_parameter(
            carrier,
            ParameterSpacePoint::try_new([u, v])
                .expect("surface-direction witness requires finite parameter-space coordinates"),
            SpatialCarrierDirectionRole::TangentV,
        )
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
