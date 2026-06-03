use serde_json::json;

use super::runtime_payload::LoweredSpatialRuntimePayload;
use super::runtime_targets::RuntimeDirectionWitnessTarget;
use crate::spatial_intent::lowering::anchors::{
    LoweredDirectionAnchorOrigin, LoweredPointAnchorOrigin, LoweringAnchorDenial,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LoweredSpatialIntentFamily {
    Move,
    Offset,
    Rotate,
    Reorient,
    LiesOn,
    PointsToward,
    AnchorMatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LoweredSpatialNumericPosture {
    Direct,
    Normalized,
    FallbackDerived,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LoweredSpatialTargetBindingPosture {
    PointWitness,
    DirectionWitness,
    FrameTarget,
    AnchorTarget,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LoweredSpatialAftermathFamily {
    PlacementMutation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RuntimeAnchorSemantic {
    ShapeOriginPoint,
    ExternalReferencePoint,
    FeatureOwnedPoint,
    GeometricTagPoint,
    ShapeAxisDirection,
    FrameAxisDirection,
    FeatureAxisDirection,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpatialLoweringDenial {
    Ambiguous,
    Unsupported,
    Undefined,
    Degenerate,
    Coincident,
    NonPointLike,
    NonDirectionLike,
    WitnessFailure(crate::spatial_intent::resolution::SpatialWitnessFailureClass),
    TagFailure(crate::spatial_intent::refs::SpatialGeometricTagFailureClass),
    InvalidReferenceFrame(crate::spatial_intent::resolution::SpatialFrameError),
    InvalidExistingPlacement,
}

impl From<LoweringAnchorDenial> for SpatialLoweringDenial {
    fn from(value: LoweringAnchorDenial) -> Self {
        match value {
            LoweringAnchorDenial::Ambiguous => Self::Ambiguous,
            LoweringAnchorDenial::Unsupported => Self::Unsupported,
            LoweringAnchorDenial::Undefined => Self::Undefined,
            LoweringAnchorDenial::Degenerate => Self::Degenerate,
            LoweringAnchorDenial::Coincident => Self::Coincident,
            LoweringAnchorDenial::NonPointLike => Self::NonPointLike,
            LoweringAnchorDenial::NonDirectionLike => Self::NonDirectionLike,
            LoweringAnchorDenial::WitnessFailure(value) => Self::WitnessFailure(value),
            LoweringAnchorDenial::TagFailure(value) => Self::TagFailure(value),
            LoweringAnchorDenial::InvalidReferenceFrame(value) => {
                Self::InvalidReferenceFrame(value)
            }
            LoweringAnchorDenial::InvalidExistingPlacement => Self::InvalidExistingPlacement,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct LoweredSpatialRuntimeDeclaration {
    family: LoweredSpatialIntentFamily,
    subject_anchor: Option<RuntimeAnchorSemantic>,
    target_anchor: Option<RuntimeAnchorSemantic>,
    numeric_posture: LoweredSpatialNumericPosture,
    target_binding: LoweredSpatialTargetBindingPosture,
    aftermath: LoweredSpatialAftermathFamily,
    payload: LoweredSpatialRuntimePayload,
}

impl LoweredSpatialRuntimeDeclaration {
    pub(crate) fn new(
        family: LoweredSpatialIntentFamily,
        subject_anchor: Option<RuntimeAnchorSemantic>,
        target_anchor: Option<RuntimeAnchorSemantic>,
        numeric_posture: LoweredSpatialNumericPosture,
        target_binding: LoweredSpatialTargetBindingPosture,
        payload: LoweredSpatialRuntimePayload,
    ) -> Self {
        Self {
            family,
            subject_anchor,
            target_anchor,
            numeric_posture,
            target_binding,
            aftermath: LoweredSpatialAftermathFamily::PlacementMutation,
            payload,
        }
    }

    #[cfg(test)]
    pub(crate) fn family(&self) -> LoweredSpatialIntentFamily {
        self.family
    }

    #[cfg(test)]
    pub(crate) fn numeric_posture(&self) -> LoweredSpatialNumericPosture {
        self.numeric_posture
    }

    pub fn to_query_intent_declaration(&self) -> forge_query::facade::ForgeQueryIntentDeclaration {
        forge_query::facade::ForgeQueryIntentDeclaration::strategy_commit(
            format!("worth.spatial.lowered.{}", self.family.as_str()),
            "worth.spatial.lowering.runtime_handoff",
            "1.0",
            "worth.spatial.lowered_runtime_declaration.v1",
            json!({
                "family": self.family.as_str(),
                "subject_anchor": self.subject_anchor.map(RuntimeAnchorSemantic::as_str),
                "target_anchor": self.target_anchor.map(RuntimeAnchorSemantic::as_str),
                "numeric_posture": self.numeric_posture.as_str(),
                "target_binding": self.target_binding.as_str(),
                "aftermath": self.aftermath.as_str(),
                "payload": self.payload.to_json(),
            }),
        )
    }

    #[cfg(test)]
    pub(crate) fn to_query_runtime_request(
        &self,
    ) -> Result<
        forge_query::facade::ForgeQueryRawIntentAdmissionRequest,
        forge_query::facade::ForgeQueryIntentViolationDecision,
    > {
        forge_query::facade::ForgeQueryRawIntentAdmissionRequest::authoritative_runtime_entrypoint(
            self.to_query_intent_declaration(),
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct LoweredSpatialIntent {
    runtime_declaration: LoweredSpatialRuntimeDeclaration,
    operation: LoweredSpatialOperation,
}

impl LoweredSpatialIntent {
    pub(crate) fn new(
        runtime_declaration: LoweredSpatialRuntimeDeclaration,
        operation: LoweredSpatialOperation,
    ) -> Self {
        Self {
            runtime_declaration,
            operation,
        }
    }

    #[cfg(test)]
    pub(crate) fn family(&self) -> LoweredSpatialIntentFamily {
        self.runtime_declaration.family()
    }

    #[cfg(test)]
    pub(crate) fn numeric_posture(&self) -> LoweredSpatialNumericPosture {
        self.runtime_declaration.numeric_posture()
    }

    pub(crate) fn to_query_intent_declaration(
        &self,
    ) -> forge_query::facade::ForgeQueryIntentDeclaration {
        self.runtime_declaration.to_query_intent_declaration()
    }

    #[cfg(test)]
    pub(crate) fn to_query_runtime_request(
        &self,
    ) -> Result<
        forge_query::facade::ForgeQueryRawIntentAdmissionRequest,
        forge_query::facade::ForgeQueryIntentViolationDecision,
    > {
        self.runtime_declaration.to_query_runtime_request()
    }

    #[cfg(test)]
    pub(crate) fn admit_query_runtime_intent(
        &self,
    ) -> Result<
        forge_query::facade::ForgeQueryIntentAdmissionDecision,
        forge_query::facade::ForgeQueryIntentViolationDecision,
    > {
        self.to_query_runtime_request()
            .map(forge_query::facade::admit_runtime_intent_request)
    }

    pub(crate) fn operation(&self) -> &LoweredSpatialOperation {
        &self.operation
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum LoweredSpatialOperation {
    Move {
        anchor_world_point: [f64; 3],
        target_world_point: [f64; 3],
    },
    Offset {
        offset: [f64; 3],
    },
    RotateFacingOnly {
        source_facing: [f64; 3],
        axis: [f64; 3],
        angle_radians: f64,
    },
    RotateAroundPivot {
        source_facing: [f64; 3],
        pivot_world_point: [f64; 3],
        axis: [f64; 3],
        angle_radians: f64,
    },
    ReorientPointLike {
        source_anchor_world_point: [f64; 3],
        target_direction: RuntimeDirectionWitnessTarget,
    },
    ReorientDirectional {
        source_world_direction: [f64; 3],
        target_world_direction: [f64; 3],
    },
    LiesOnShapeOrigin {
        target_frame: crate::spatial_intent::resolution::AdmittedSpatialFrameRef,
    },
    LiesOnProjected {
        target_frame: crate::spatial_intent::resolution::AdmittedSpatialFrameRef,
        anchor_world_point: [f64; 3],
    },
    PointsToward {
        anchor_world_point: [f64; 3],
        target_world_point: [f64; 3],
    },
    AnchorMatch {
        anchor_world_point: [f64; 3],
        target_world_point: [f64; 3],
    },
}

impl LoweredSpatialIntentFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Move => "move",
            Self::Offset => "offset",
            Self::Rotate => "rotate",
            Self::Reorient => "reorient",
            Self::LiesOn => "lies_on",
            Self::PointsToward => "points_toward",
            Self::AnchorMatch => "anchor_match",
        }
    }
}

impl LoweredSpatialNumericPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::Normalized => "normalized",
            Self::FallbackDerived => "fallback_derived",
        }
    }
}

impl LoweredSpatialTargetBindingPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PointWitness => "point_witness",
            Self::DirectionWitness => "direction_witness",
            Self::FrameTarget => "frame_target",
            Self::AnchorTarget => "anchor_target",
        }
    }
}

impl LoweredSpatialAftermathFamily {
    pub fn as_str(self) -> &'static str {
        "placement_mutation"
    }
}

impl RuntimeAnchorSemantic {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ShapeOriginPoint => "shape_origin_point",
            Self::ExternalReferencePoint => "external_reference_point",
            Self::FeatureOwnedPoint => "feature_owned_point",
            Self::GeometricTagPoint => "geometric_tag_point",
            Self::ShapeAxisDirection => "shape_axis_direction",
            Self::FrameAxisDirection => "frame_axis_direction",
            Self::FeatureAxisDirection => "feature_axis_direction",
        }
    }
}

impl From<LoweredPointAnchorOrigin> for RuntimeAnchorSemantic {
    fn from(value: LoweredPointAnchorOrigin) -> Self {
        match value {
            LoweredPointAnchorOrigin::ShapeOrigin => Self::ShapeOriginPoint,
            LoweredPointAnchorOrigin::ExternalReference => Self::ExternalReferencePoint,
            LoweredPointAnchorOrigin::FeatureOwned => Self::FeatureOwnedPoint,
            LoweredPointAnchorOrigin::GeometricTag => Self::GeometricTagPoint,
        }
    }
}

impl From<LoweredDirectionAnchorOrigin> for RuntimeAnchorSemantic {
    fn from(value: LoweredDirectionAnchorOrigin) -> Self {
        match value {
            LoweredDirectionAnchorOrigin::ShapeAxis => Self::ShapeAxisDirection,
            LoweredDirectionAnchorOrigin::FrameAxis => Self::FrameAxisDirection,
            LoweredDirectionAnchorOrigin::FeatureAxis => Self::FeatureAxisDirection,
        }
    }
}
