use crate::spatial_intent::refs::SpatialFrameRef;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SpatialAxis {
    U,
    V,
    W,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SpatialAnchorRef {
    WorldOrigin,
    ShapeOrigin,
    ShapeAxis(SpatialAxis),
    FrameOrigin(SpatialFrameRef),
    FrameAxis {
        frame: SpatialFrameRef,
        axis: SpatialAxis,
    },
    GeometricTag(String),
    ParameterSpace {
        carrier: String,
        parameter: String,
    },
    FeatureOwned(String),
}

impl SpatialAnchorRef {
    pub fn world_origin() -> Self {
        Self::WorldOrigin
    }

    pub fn shape_origin() -> Self {
        Self::ShapeOrigin
    }

    pub fn shape_axis(axis: SpatialAxis) -> Self {
        Self::ShapeAxis(axis)
    }

    pub fn frame_origin(frame: SpatialFrameRef) -> Self {
        Self::FrameOrigin(frame)
    }

    pub fn frame_axis(frame: SpatialFrameRef, axis: SpatialAxis) -> Self {
        Self::FrameAxis { frame, axis }
    }

    pub fn geometric_tag(tag: impl Into<String>) -> Self {
        Self::GeometricTag(tag.into())
    }

    pub fn parameter_space(carrier: impl Into<String>, parameter: impl Into<String>) -> Self {
        Self::ParameterSpace {
            carrier: carrier.into(),
            parameter: parameter.into(),
        }
    }

    pub fn feature_owned(tag: impl Into<String>) -> Self {
        Self::FeatureOwned(tag.into())
    }
}
