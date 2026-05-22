use serde_json::{json, Value};

use crate::spatial_intent::refs::{
    SpatialAxis, SpatialCarrierDirectionRole, SpatialCarrierKind, SpatialDirectionWitnessRef,
    SpatialFrameRef,
};
use crate::spatial_intent::resolution::AdmittedSpatialFrameRef;

#[derive(Clone, Debug, PartialEq)]
pub enum RuntimeFrameTarget {
    World,
    ShapeLocal,
    Workplane {
        name: String,
        origin: [f64; 3],
        normal: [f64; 3],
    },
    FeatureLocal {
        name: String,
        origin: [f64; 3],
        normal: [f64; 3],
    },
}

impl RuntimeFrameTarget {
    pub fn from_admitted(frame: &AdmittedSpatialFrameRef) -> Self {
        Self::from_spec(frame.spec())
    }

    pub fn from_spec(frame: &SpatialFrameRef) -> Self {
        match frame {
            SpatialFrameRef::World => Self::World,
            SpatialFrameRef::ShapeLocal => Self::ShapeLocal,
            SpatialFrameRef::Workplane {
                name,
                origin,
                normal,
            } => Self::Workplane {
                name: name.clone(),
                origin: *origin,
                normal: *normal,
            },
            SpatialFrameRef::FeatureLocal {
                name,
                origin,
                normal,
            } => Self::FeatureLocal {
                name: name.clone(),
                origin: *origin,
                normal: *normal,
            },
        }
    }

    pub fn support_code(&self) -> &'static str {
        match self {
            Self::World => "worth.spatial.lowering.frame.world",
            Self::ShapeLocal => "worth.spatial.lowering.frame.shape_local",
            Self::Workplane { .. } => "worth.spatial.lowering.frame.workplane",
            Self::FeatureLocal { .. } => "worth.spatial.lowering.frame.feature_local",
        }
    }

    pub fn to_frame_ref(&self) -> SpatialFrameRef {
        match self {
            Self::World => SpatialFrameRef::world(),
            Self::ShapeLocal => SpatialFrameRef::shape_local(),
            Self::Workplane {
                name,
                origin,
                normal,
            } => SpatialFrameRef::workplane(name.clone(), *origin, *normal),
            Self::FeatureLocal {
                name,
                origin,
                normal,
            } => SpatialFrameRef::feature_local(name.clone(), *origin, *normal),
        }
    }

    pub fn to_json(&self) -> Value {
        match self {
            Self::World => json!({ "kind": "world" }),
            Self::ShapeLocal => json!({ "kind": "shape_local" }),
            Self::Workplane {
                name,
                origin,
                normal,
            } => json!({
                "kind": "workplane",
                "name": name,
                "origin": origin,
                "normal": normal,
            }),
            Self::FeatureLocal {
                name,
                origin,
                normal,
            } => json!({
                "kind": "feature_local",
                "name": name,
                "origin": origin,
                "normal": normal,
            }),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RuntimeDirectionWitnessTarget {
    WorldDirection([f64; 3]),
    FrameAxis {
        frame: RuntimeFrameTarget,
        axis: SpatialAxis,
    },
    FramePerpendicularAxis {
        frame: RuntimeFrameTarget,
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

impl RuntimeDirectionWitnessTarget {
    pub fn from_witness(witness: &SpatialDirectionWitnessRef) -> Self {
        match witness {
            SpatialDirectionWitnessRef::WorldDirection(direction) => {
                Self::WorldDirection(*direction)
            }
            SpatialDirectionWitnessRef::FrameAxis { frame, axis } => Self::FrameAxis {
                frame: RuntimeFrameTarget::from_spec(frame),
                axis: *axis,
            },
            SpatialDirectionWitnessRef::FramePerpendicularAxis { frame, axis } => {
                Self::FramePerpendicularAxis {
                    frame: RuntimeFrameTarget::from_spec(frame),
                    axis: *axis,
                }
            }
            SpatialDirectionWitnessRef::CarrierDirection {
                carrier_kind,
                carrier,
            } => Self::CarrierDirection {
                carrier_kind: *carrier_kind,
                carrier: carrier.clone(),
            },
            SpatialDirectionWitnessRef::ParameterSpaceDirection {
                carrier_kind,
                carrier,
                parameter,
                role,
            } => Self::ParameterSpaceDirection {
                carrier_kind: *carrier_kind,
                carrier: carrier.clone(),
                parameter: parameter.as_array(),
                role: *role,
            },
            SpatialDirectionWitnessRef::FeatureOwnedDirection { feature, role } => {
                Self::FeatureOwnedDirection {
                    feature: feature.clone(),
                    role: *role,
                }
            }
        }
    }

    pub fn support_code(&self) -> &'static str {
        match self {
            Self::WorldDirection(_) => "worth.spatial.lowering.direction_target.world",
            Self::FrameAxis { .. } => "worth.spatial.lowering.direction_target.frame_axis",
            Self::FramePerpendicularAxis { .. } => {
                "worth.spatial.lowering.direction_target.frame_perpendicular_axis"
            }
            Self::CarrierDirection { .. } => {
                "worth.spatial.lowering.direction_target.carrier_direction"
            }
            Self::ParameterSpaceDirection { .. } => {
                "worth.spatial.lowering.direction_target.parameter_space_direction"
            }
            Self::FeatureOwnedDirection { .. } => {
                "worth.spatial.lowering.direction_target.feature_direction"
            }
        }
    }

    pub fn to_witness(&self) -> SpatialDirectionWitnessRef {
        match self {
            Self::WorldDirection(direction) => {
                SpatialDirectionWitnessRef::world_direction(*direction)
            }
            Self::FrameAxis { frame, axis } => {
                SpatialDirectionWitnessRef::frame_axis(frame.to_frame_ref(), *axis)
            }
            Self::FramePerpendicularAxis { frame, axis } => {
                SpatialDirectionWitnessRef::frame_perpendicular_axis(frame.to_frame_ref(), *axis)
            }
            Self::CarrierDirection {
                carrier_kind,
                carrier,
            } => SpatialDirectionWitnessRef::CarrierDirection {
                carrier_kind: *carrier_kind,
                carrier: carrier.clone(),
            },
            Self::ParameterSpaceDirection {
                carrier_kind,
                carrier,
                parameter,
                role,
            } => SpatialDirectionWitnessRef::ParameterSpaceDirection {
                carrier_kind: *carrier_kind,
                carrier: carrier.clone(),
                parameter: worth_geom::ParameterSpacePoint::try_new(*parameter)
                    .expect("runtime payload keeps finite parameter-space coordinates"),
                role: *role,
            },
            Self::FeatureOwnedDirection { feature, role } => {
                SpatialDirectionWitnessRef::FeatureOwnedDirection {
                    feature: feature.clone(),
                    role: *role,
                }
            }
        }
    }

    pub fn to_json(&self) -> Value {
        match self {
            Self::WorldDirection(direction) => {
                json!({ "kind": "world_direction", "direction": direction })
            }
            Self::FrameAxis { frame, axis } => json!({
                "kind": "frame_axis",
                "frame": frame.to_json(),
                "axis": format!("{axis:?}"),
            }),
            Self::FramePerpendicularAxis { frame, axis } => json!({
                "kind": "frame_perpendicular_axis",
                "frame": frame.to_json(),
                "axis": format!("{axis:?}"),
            }),
            Self::CarrierDirection {
                carrier_kind,
                carrier,
            } => json!({
                "kind": "carrier_direction",
                "carrier_kind": format!("{carrier_kind:?}"),
                "carrier": carrier,
            }),
            Self::ParameterSpaceDirection {
                carrier_kind,
                carrier,
                parameter,
                role,
            } => json!({
                "kind": "parameter_space_direction",
                "carrier_kind": format!("{carrier_kind:?}"),
                "carrier": carrier,
                "parameter": parameter,
                "role": format!("{role:?}"),
            }),
            Self::FeatureOwnedDirection { feature, role } => json!({
                "kind": "feature_owned_direction",
                "feature": feature,
                "role": format!("{role:?}"),
            }),
        }
    }
}
