use serde_json::{json, Value};

use super::runtime_targets::{RuntimeDirectionWitnessTarget, RuntimeFrameTarget};

#[derive(Clone, Debug, PartialEq)]
pub enum LoweredSpatialRuntimePayload {
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
        source_origin: [f64; 3],
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
        target_frame: RuntimeFrameTarget,
    },
    LiesOnProjected {
        target_frame: RuntimeFrameTarget,
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

impl LoweredSpatialRuntimePayload {
    pub fn to_json(&self) -> Value {
        match self {
            Self::Move {
                anchor_world_point,
                target_world_point,
            } => json!({
                "kind": "move",
                "anchor_world_point": anchor_world_point,
                "target_world_point": target_world_point,
            }),
            Self::Offset { offset } => json!({
                "kind": "offset",
                "offset": offset,
            }),
            Self::RotateFacingOnly {
                source_facing,
                axis,
                angle_radians,
            } => json!({
                "kind": "rotate_facing_only",
                "source_facing": source_facing,
                "axis": axis,
                "angle_radians": angle_radians,
            }),
            Self::RotateAroundPivot {
                source_origin,
                source_facing,
                pivot_world_point,
                axis,
                angle_radians,
            } => json!({
                "kind": "rotate_around_pivot",
                "source_origin": source_origin,
                "source_facing": source_facing,
                "pivot_world_point": pivot_world_point,
                "axis": axis,
                "angle_radians": angle_radians,
            }),
            Self::ReorientPointLike {
                source_anchor_world_point,
                target_direction,
            } => json!({
                "kind": "reorient_point_like",
                "source_anchor_world_point": source_anchor_world_point,
                "target_direction": target_direction.to_json(),
            }),
            Self::ReorientDirectional {
                source_world_direction,
                target_world_direction,
            } => json!({
                "kind": "reorient_directional",
                "source_world_direction": source_world_direction,
                "target_world_direction": target_world_direction,
            }),
            Self::LiesOnShapeOrigin { target_frame } => json!({
                "kind": "lies_on_shape_origin",
                "target_frame": target_frame.to_json(),
            }),
            Self::LiesOnProjected {
                target_frame,
                anchor_world_point,
            } => json!({
                "kind": "lies_on_projected",
                "target_frame": target_frame.to_json(),
                "anchor_world_point": anchor_world_point,
            }),
            Self::PointsToward {
                anchor_world_point,
                target_world_point,
            } => json!({
                "kind": "points_toward",
                "anchor_world_point": anchor_world_point,
                "target_world_point": target_world_point,
            }),
            Self::AnchorMatch {
                anchor_world_point,
                target_world_point,
            } => json!({
                "kind": "anchor_match",
                "anchor_world_point": anchor_world_point,
                "target_world_point": target_world_point,
            }),
        }
    }
}
