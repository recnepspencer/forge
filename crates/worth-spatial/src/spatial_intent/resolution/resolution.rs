use crate::spatial_intent::refs::{
    SpatialCarrierDirectionRole, SpatialCarrierPointRole, SpatialCatalogParameterAdmission,
    SpatialDirectionWitnessRef, SpatialPointWitnessRef, SpatialWitnessCatalog,
};
use crate::spatial_intent::resolution::{
    SpatialWitnessFailureClass, SpatialWitnessResolutionClass,
};

use super::admitted_witness_requests::{
    AdmittedSpatialDirectionWitnessRequest, AdmittedSpatialPointWitnessRequest,
};
use super::witness_support::{
    axis_from_basis, fallback_perpendicular, finite_point, normalize_direction,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedSpatialPointWitness {
    requested: SpatialPointWitnessRef,
    resolved_world_point: [f64; 3],
    resolution_class: SpatialWitnessResolutionClass,
    parameter_admission: Option<SpatialCatalogParameterAdmission>,
}

impl ResolvedSpatialPointWitness {
    pub fn requested(&self) -> &SpatialPointWitnessRef {
        &self.requested
    }

    pub fn resolved_world_point(&self) -> [f64; 3] {
        self.resolved_world_point
    }

    pub fn resolution_class(&self) -> SpatialWitnessResolutionClass {
        self.resolution_class
    }

    pub fn parameter_admission(&self) -> Option<&SpatialCatalogParameterAdmission> {
        self.parameter_admission.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedSpatialDirectionWitness {
    requested: SpatialDirectionWitnessRef,
    resolved_world_direction: [f64; 3],
    resolution_class: SpatialWitnessResolutionClass,
    parameter_admission: Option<SpatialCatalogParameterAdmission>,
}

impl ResolvedSpatialDirectionWitness {
    pub fn requested(&self) -> &SpatialDirectionWitnessRef {
        &self.requested
    }

    pub fn resolved_world_direction(&self) -> [f64; 3] {
        self.resolved_world_direction
    }

    pub fn resolution_class(&self) -> SpatialWitnessResolutionClass {
        self.resolution_class
    }

    pub fn parameter_admission(&self) -> Option<&SpatialCatalogParameterAdmission> {
        self.parameter_admission.as_ref()
    }
}

pub(crate) fn resolve_admitted_spatial_point_witness_request(
    admitted: AdmittedSpatialPointWitnessRequest,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<ResolvedSpatialPointWitness, SpatialWitnessFailureClass> {
    let (requested, resolved_world_point, resolution_class, parameter_admission) = match admitted {
        AdmittedSpatialPointWitnessRequest::WorldPoint { requested, point } => (
            requested,
            point,
            SpatialWitnessResolutionClass::DirectWorld,
            None,
        ),
        AdmittedSpatialPointWitnessRequest::FrameOrigin { requested, frame } => (
            requested,
            frame.basis().origin(),
            SpatialWitnessResolutionClass::FrameDerived,
            None,
        ),
        AdmittedSpatialPointWitnessRequest::CarrierPoint { .. } => {
            return Err(SpatialWitnessFailureClass::Ambiguous);
        }
        AdmittedSpatialPointWitnessRequest::ParameterSpacePoint {
            requested,
            carrier_kind,
            carrier,
            parameter,
        } => {
            let resolved =
                catalog.resolve_parameter_space_point(carrier_kind, &carrier, parameter)?;
            (
                requested,
                finite_point(resolved.world_point())?,
                resolved.resolution_class().as_witness_resolution_class(),
                resolved.parameter_admission().cloned(),
            )
        }
        AdmittedSpatialPointWitnessRequest::FeatureOwnedPoint {
            requested,
            feature,
            role,
        } => {
            let _role: SpatialCarrierPointRole = role;
            let resolved = catalog.resolve_feature_owned_point(&feature, role)?;
            (
                requested,
                finite_point(resolved.world_point())?,
                resolved.resolution_class().as_witness_resolution_class(),
                resolved.parameter_admission().cloned(),
            )
        }
    };

    Ok(ResolvedSpatialPointWitness {
        requested,
        resolved_world_point,
        resolution_class,
        parameter_admission,
    })
}

pub(crate) fn resolve_admitted_spatial_direction_witness_request(
    admitted: AdmittedSpatialDirectionWitnessRequest,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<ResolvedSpatialDirectionWitness, SpatialWitnessFailureClass> {
    let (requested, resolved_world_direction, resolution_class, parameter_admission) =
        match admitted {
            AdmittedSpatialDirectionWitnessRequest::WorldDirection {
                requested,
                direction,
            } => (
                requested,
                direction,
                SpatialWitnessResolutionClass::DirectWorld,
                None,
            ),
            AdmittedSpatialDirectionWitnessRequest::FrameAxis {
                requested,
                frame,
                axis,
            } => (
                requested,
                normalize_direction(axis_from_basis(frame.basis(), axis))
                    .map_err(|_| SpatialWitnessFailureClass::Degenerate)?,
                SpatialWitnessResolutionClass::FrameDerived,
                None,
            ),
            AdmittedSpatialDirectionWitnessRequest::FramePerpendicularAxis {
                requested,
                frame,
                axis,
            } => (
                requested,
                fallback_perpendicular(axis_from_basis(frame.basis(), axis))?,
                SpatialWitnessResolutionClass::FallbackDerived,
                None,
            ),
            AdmittedSpatialDirectionWitnessRequest::CarrierDirection { .. } => {
                return Err(SpatialWitnessFailureClass::Ambiguous);
            }
            AdmittedSpatialDirectionWitnessRequest::ParameterSpaceDirection {
                requested,
                carrier_kind,
                carrier,
                parameter,
                role,
            } => {
                let resolved = catalog.resolve_parameter_space_direction(
                    carrier_kind,
                    &carrier,
                    parameter,
                    role,
                )?;
                (
                    requested,
                    normalize_direction(resolved.world_direction())?,
                    resolved.resolution_class().as_witness_resolution_class(),
                    resolved.parameter_admission().cloned(),
                )
            }
            AdmittedSpatialDirectionWitnessRequest::FeatureOwnedDirection {
                requested,
                feature,
                role,
            } => {
                let _role: SpatialCarrierDirectionRole = role;
                let resolved = catalog.resolve_feature_owned_direction(&feature, role)?;
                (
                    requested,
                    normalize_direction(resolved.world_direction())?,
                    resolved.resolution_class().as_witness_resolution_class(),
                    resolved.parameter_admission().cloned(),
                )
            }
        };

    Ok(ResolvedSpatialDirectionWitness {
        requested,
        resolved_world_direction,
        resolution_class,
        parameter_admission,
    })
}

#[cfg(test)]
#[path = "resolution_tests.rs"]
mod resolution_tests;
