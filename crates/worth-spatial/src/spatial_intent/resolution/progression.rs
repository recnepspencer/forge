use forge_proof::{Artifact, DenialTransitionOutcome, PhaseMarker, TransitionOutcome};

use crate::spatial_intent::refs::{
    SpatialDirectionWitnessRef, SpatialFrameRef, SpatialPointWitnessRef, SpatialWitnessCatalog,
};
use crate::spatial_intent::resolution::{
    admit_spatial_frame, AdmittedSpatialFrameRef, ResolvedSpatialDirectionWitness,
    ResolvedSpatialPointWitness, SpatialWitnessFailureClass,
};

use super::resolution::{
    resolve_admitted_spatial_direction_witness_request,
    resolve_admitted_spatial_point_witness_request,
};
use super::witness_support::{finite_point, normalize_direction};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RequestedSpatialPointWitnessPhase;
impl PhaseMarker for RequestedSpatialPointWitnessPhase {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AdmittedSpatialPointWitnessRequestPhase;
impl PhaseMarker for AdmittedSpatialPointWitnessRequestPhase {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ResolvedSpatialPointWitnessPhase;
impl PhaseMarker for ResolvedSpatialPointWitnessPhase {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RequestedSpatialDirectionWitnessPhase;
impl PhaseMarker for RequestedSpatialDirectionWitnessPhase {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AdmittedSpatialDirectionWitnessRequestPhase;
impl PhaseMarker for AdmittedSpatialDirectionWitnessRequestPhase {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ResolvedSpatialDirectionWitnessPhase;
impl PhaseMarker for ResolvedSpatialDirectionWitnessPhase {}

pub(crate) type RequestedSpatialPointWitnessArtifact =
    Artifact<RequestedSpatialPointWitnessPhase, SpatialPointWitnessRef>;
pub(crate) type AdmittedSpatialPointWitnessRequestArtifact =
    Artifact<AdmittedSpatialPointWitnessRequestPhase, AdmittedSpatialPointWitnessRequest>;
pub(crate) type ResolvedSpatialPointWitnessArtifact =
    Artifact<ResolvedSpatialPointWitnessPhase, ResolvedSpatialPointWitness>;
pub(crate) type RequestedSpatialDirectionWitnessArtifact =
    Artifact<RequestedSpatialDirectionWitnessPhase, SpatialDirectionWitnessRef>;
pub(crate) type AdmittedSpatialDirectionWitnessRequestArtifact =
    Artifact<AdmittedSpatialDirectionWitnessRequestPhase, AdmittedSpatialDirectionWitnessRequest>;
pub(crate) type ResolvedSpatialDirectionWitnessArtifact =
    Artifact<ResolvedSpatialDirectionWitnessPhase, ResolvedSpatialDirectionWitness>;

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

pub(crate) fn request_spatial_point_witness(
    requested: SpatialPointWitnessRef,
) -> RequestedSpatialPointWitnessArtifact {
    Artifact::new(requested)
}

pub(crate) fn admit_requested_spatial_point_witness(
    requested: RequestedSpatialPointWitnessArtifact,
) -> DenialTransitionOutcome<AdmittedSpatialPointWitnessRequestArtifact, SpatialWitnessFailureClass>
{
    let admitted = match requested.payload() {
        SpatialPointWitnessRef::WorldPoint(point) => {
            let point = match finite_point(*point) {
                Ok(point) => point,
                Err(denial) => return TransitionOutcome::denied(denial),
            };
            AdmittedSpatialPointWitnessRequest::WorldPoint {
                requested: requested.payload().clone(),
                point,
            }
        }
        SpatialPointWitnessRef::FrameOrigin(frame) => {
            let frame = match admitted_frame(frame.clone()) {
                Ok(frame) => frame,
                Err(denial) => return TransitionOutcome::denied(denial),
            };
            AdmittedSpatialPointWitnessRequest::FrameOrigin {
                requested: requested.payload().clone(),
                frame,
            }
        }
        SpatialPointWitnessRef::CarrierPoint {
            carrier_kind,
            carrier,
        } => AdmittedSpatialPointWitnessRequest::CarrierPoint {
            requested: requested.payload().clone(),
            carrier_kind: *carrier_kind,
            carrier: carrier.clone(),
        },
        SpatialPointWitnessRef::ParameterSpacePoint {
            carrier_kind,
            carrier,
            parameter,
        } => AdmittedSpatialPointWitnessRequest::ParameterSpacePoint {
            requested: requested.payload().clone(),
            carrier_kind: *carrier_kind,
            carrier: carrier.clone(),
            parameter: *parameter,
        },
        SpatialPointWitnessRef::FeatureOwnedPoint { feature, role } => {
            AdmittedSpatialPointWitnessRequest::FeatureOwnedPoint {
                requested: requested.payload().clone(),
                feature: feature.clone(),
                role: *role,
            }
        }
    };

    TransitionOutcome::success(Artifact::new(admitted))
}

pub(crate) fn resolve_admitted_spatial_point_witness<C: SpatialWitnessCatalog>(
    admitted: AdmittedSpatialPointWitnessRequestArtifact,
    catalog: &C,
) -> DenialTransitionOutcome<ResolvedSpatialPointWitnessArtifact, SpatialWitnessFailureClass> {
    match resolve_admitted_spatial_point_witness_request(admitted.payload().clone(), catalog) {
        Ok(resolved) => TransitionOutcome::success(Artifact::new(resolved)),
        Err(denial) => TransitionOutcome::denied(denial),
    }
}

pub(crate) fn request_spatial_direction_witness(
    requested: SpatialDirectionWitnessRef,
) -> RequestedSpatialDirectionWitnessArtifact {
    Artifact::new(requested)
}

pub(crate) fn admit_requested_spatial_direction_witness(
    requested: RequestedSpatialDirectionWitnessArtifact,
) -> DenialTransitionOutcome<
    AdmittedSpatialDirectionWitnessRequestArtifact,
    SpatialWitnessFailureClass,
> {
    let admitted = match requested.payload() {
        SpatialDirectionWitnessRef::WorldDirection(direction) => {
            let direction = match normalize_direction(*direction) {
                Ok(direction) => direction,
                Err(denial) => return TransitionOutcome::denied(denial),
            };
            AdmittedSpatialDirectionWitnessRequest::WorldDirection {
                requested: requested.payload().clone(),
                direction,
            }
        }
        SpatialDirectionWitnessRef::FrameAxis { frame, axis } => {
            let frame = match admitted_frame(frame.clone()) {
                Ok(frame) => frame,
                Err(denial) => return TransitionOutcome::denied(denial),
            };
            AdmittedSpatialDirectionWitnessRequest::FrameAxis {
                requested: requested.payload().clone(),
                frame,
                axis: *axis,
            }
        }
        SpatialDirectionWitnessRef::FramePerpendicularAxis { frame, axis } => {
            let frame = match admitted_frame(frame.clone()) {
                Ok(frame) => frame,
                Err(denial) => return TransitionOutcome::denied(denial),
            };
            AdmittedSpatialDirectionWitnessRequest::FramePerpendicularAxis {
                requested: requested.payload().clone(),
                frame,
                axis: *axis,
            }
        }
        SpatialDirectionWitnessRef::CarrierDirection {
            carrier_kind,
            carrier,
        } => AdmittedSpatialDirectionWitnessRequest::CarrierDirection {
            requested: requested.payload().clone(),
            carrier_kind: *carrier_kind,
            carrier: carrier.clone(),
        },
        SpatialDirectionWitnessRef::ParameterSpaceDirection {
            carrier_kind,
            carrier,
            parameter,
            role,
        } => AdmittedSpatialDirectionWitnessRequest::ParameterSpaceDirection {
            requested: requested.payload().clone(),
            carrier_kind: *carrier_kind,
            carrier: carrier.clone(),
            parameter: *parameter,
            role: *role,
        },
        SpatialDirectionWitnessRef::FeatureOwnedDirection { feature, role } => {
            AdmittedSpatialDirectionWitnessRequest::FeatureOwnedDirection {
                requested: requested.payload().clone(),
                feature: feature.clone(),
                role: *role,
            }
        }
    };

    TransitionOutcome::success(Artifact::new(admitted))
}

pub(crate) fn resolve_admitted_spatial_direction_witness<C: SpatialWitnessCatalog>(
    admitted: AdmittedSpatialDirectionWitnessRequestArtifact,
    catalog: &C,
) -> DenialTransitionOutcome<ResolvedSpatialDirectionWitnessArtifact, SpatialWitnessFailureClass> {
    match resolve_admitted_spatial_direction_witness_request(admitted.payload().clone(), catalog) {
        Ok(resolved) => TransitionOutcome::success(Artifact::new(resolved)),
        Err(denial) => TransitionOutcome::denied(denial),
    }
}

fn admitted_frame(
    frame: SpatialFrameRef,
) -> Result<AdmittedSpatialFrameRef, SpatialWitnessFailureClass> {
    admit_spatial_frame(frame).map_err(|_| SpatialWitnessFailureClass::Degenerate)
}

#[cfg(test)]
#[path = "progression_tests.rs"]
mod progression_tests;
