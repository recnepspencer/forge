use crate::spatial_intent::refs::{
    SpatialDirectionWitnessRef, SpatialFrameRef, SpatialPointWitnessRef,
};
use crate::spatial_intent::resolution::{
    admit_spatial_frame, AdmittedSpatialFrameRef, SpatialWitnessFailureClass,
};

use super::witness_support::{finite_point, normalize_direction};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum AdmittedSpatialPointWitnessRequest {
    WorldPoint {
        requested: SpatialPointWitnessRef,
        point: [f64; 3],
    },
    FrameOrigin {
        requested: SpatialPointWitnessRef,
        frame: AdmittedSpatialFrameRef,
    },
    CarrierPoint {
        requested: SpatialPointWitnessRef,
        carrier_kind: crate::spatial_intent::refs::SpatialCarrierKind,
        carrier: String,
    },
    ParameterSpacePoint {
        requested: SpatialPointWitnessRef,
        carrier_kind: crate::spatial_intent::refs::SpatialCarrierKind,
        carrier: String,
        parameter: worth_geom::ParameterSpacePoint,
    },
    FeatureOwnedPoint {
        requested: SpatialPointWitnessRef,
        feature: String,
        role: crate::spatial_intent::refs::SpatialCarrierPointRole,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum AdmittedSpatialDirectionWitnessRequest {
    WorldDirection {
        requested: SpatialDirectionWitnessRef,
        direction: [f64; 3],
    },
    FrameAxis {
        requested: SpatialDirectionWitnessRef,
        frame: AdmittedSpatialFrameRef,
        axis: crate::spatial_intent::refs::SpatialAxis,
    },
    FramePerpendicularAxis {
        requested: SpatialDirectionWitnessRef,
        frame: AdmittedSpatialFrameRef,
        axis: crate::spatial_intent::refs::SpatialAxis,
    },
    CarrierDirection {
        requested: SpatialDirectionWitnessRef,
        carrier_kind: crate::spatial_intent::refs::SpatialCarrierKind,
        carrier: String,
    },
    ParameterSpaceDirection {
        requested: SpatialDirectionWitnessRef,
        carrier_kind: crate::spatial_intent::refs::SpatialCarrierKind,
        carrier: String,
        parameter: worth_geom::ParameterSpacePoint,
        role: crate::spatial_intent::refs::SpatialCarrierDirectionRole,
    },
    FeatureOwnedDirection {
        requested: SpatialDirectionWitnessRef,
        feature: String,
        role: crate::spatial_intent::refs::SpatialCarrierDirectionRole,
    },
}

pub(crate) fn admit_spatial_point_witness_request(
    requested: SpatialPointWitnessRef,
) -> Result<AdmittedSpatialPointWitnessRequest, SpatialWitnessFailureClass> {
    match requested {
        SpatialPointWitnessRef::WorldPoint(point) => {
            Ok(AdmittedSpatialPointWitnessRequest::WorldPoint {
                requested: SpatialPointWitnessRef::WorldPoint(point),
                point: finite_point(point)?,
            })
        }
        SpatialPointWitnessRef::FrameOrigin(frame) => {
            Ok(AdmittedSpatialPointWitnessRequest::FrameOrigin {
                requested: SpatialPointWitnessRef::FrameOrigin(frame.clone()),
                frame: admitted_frame(frame)?,
            })
        }
        SpatialPointWitnessRef::CarrierPoint {
            carrier_kind,
            carrier,
        } => Ok(AdmittedSpatialPointWitnessRequest::CarrierPoint {
            requested: SpatialPointWitnessRef::CarrierPoint {
                carrier_kind,
                carrier: carrier.clone(),
            },
            carrier_kind,
            carrier,
        }),
        SpatialPointWitnessRef::ParameterSpacePoint {
            carrier_kind,
            carrier,
            parameter,
        } => Ok(AdmittedSpatialPointWitnessRequest::ParameterSpacePoint {
            requested: SpatialPointWitnessRef::ParameterSpacePoint {
                carrier_kind,
                carrier: carrier.clone(),
                parameter,
            },
            carrier_kind,
            carrier,
            parameter,
        }),
        SpatialPointWitnessRef::FeatureOwnedPoint { feature, role } => {
            Ok(AdmittedSpatialPointWitnessRequest::FeatureOwnedPoint {
                requested: SpatialPointWitnessRef::FeatureOwnedPoint {
                    feature: feature.clone(),
                    role,
                },
                feature,
                role,
            })
        }
    }
}

pub(crate) fn admit_spatial_direction_witness_request(
    requested: SpatialDirectionWitnessRef,
) -> Result<AdmittedSpatialDirectionWitnessRequest, SpatialWitnessFailureClass> {
    match requested {
        SpatialDirectionWitnessRef::WorldDirection(direction) => {
            Ok(AdmittedSpatialDirectionWitnessRequest::WorldDirection {
                requested: SpatialDirectionWitnessRef::WorldDirection(direction),
                direction: normalize_direction(direction)?,
            })
        }
        SpatialDirectionWitnessRef::FrameAxis { frame, axis } => {
            Ok(AdmittedSpatialDirectionWitnessRequest::FrameAxis {
                requested: SpatialDirectionWitnessRef::FrameAxis {
                    frame: frame.clone(),
                    axis,
                },
                frame: admitted_frame(frame)?,
                axis,
            })
        }
        SpatialDirectionWitnessRef::FramePerpendicularAxis { frame, axis } => Ok(
            AdmittedSpatialDirectionWitnessRequest::FramePerpendicularAxis {
                requested: SpatialDirectionWitnessRef::FramePerpendicularAxis {
                    frame: frame.clone(),
                    axis,
                },
                frame: admitted_frame(frame)?,
                axis,
            },
        ),
        SpatialDirectionWitnessRef::CarrierDirection {
            carrier_kind,
            carrier,
        } => Ok(AdmittedSpatialDirectionWitnessRequest::CarrierDirection {
            requested: SpatialDirectionWitnessRef::CarrierDirection {
                carrier_kind,
                carrier: carrier.clone(),
            },
            carrier_kind,
            carrier,
        }),
        SpatialDirectionWitnessRef::ParameterSpaceDirection {
            carrier_kind,
            carrier,
            parameter,
            role,
        } => Ok(
            AdmittedSpatialDirectionWitnessRequest::ParameterSpaceDirection {
                requested: SpatialDirectionWitnessRef::ParameterSpaceDirection {
                    carrier_kind,
                    carrier: carrier.clone(),
                    parameter,
                    role,
                },
                carrier_kind,
                carrier,
                parameter,
                role,
            },
        ),
        SpatialDirectionWitnessRef::FeatureOwnedDirection { feature, role } => Ok(
            AdmittedSpatialDirectionWitnessRequest::FeatureOwnedDirection {
                requested: SpatialDirectionWitnessRef::FeatureOwnedDirection {
                    feature: feature.clone(),
                    role,
                },
                feature,
                role,
            },
        ),
    }
}

fn admitted_frame(
    frame: SpatialFrameRef,
) -> Result<AdmittedSpatialFrameRef, SpatialWitnessFailureClass> {
    admit_spatial_frame(frame).map_err(|_| SpatialWitnessFailureClass::Degenerate)
}

#[cfg(test)]
#[path = "admitted_witness_requests_tests.rs"]
mod admitted_witness_requests_tests;
