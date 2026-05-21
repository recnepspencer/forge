use crate::spatial_intent::refs::{
    EmptySpatialWitnessCatalog, SpatialAxis, SpatialCarrierDirectionRole, SpatialCarrierPointRole,
    SpatialCatalogParameterAdmission, SpatialDirectionWitnessRef, SpatialFrameRef,
    SpatialPointWitnessRef, SpatialWitnessCatalog,
};
use crate::spatial_intent::resolution::{
    admit_spatial_frame, SpatialWitnessFailureClass, SpatialWitnessResolutionClass,
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

pub fn resolve_spatial_point_witness(
    requested: SpatialPointWitnessRef,
) -> Result<ResolvedSpatialPointWitness, SpatialWitnessFailureClass> {
    resolve_spatial_point_witness_with_catalog(requested, &EmptySpatialWitnessCatalog)
}

pub fn resolve_spatial_point_witness_with_catalog(
    requested: SpatialPointWitnessRef,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<ResolvedSpatialPointWitness, SpatialWitnessFailureClass> {
    let (resolved_world_point, resolution_class, parameter_admission) = match &requested {
        SpatialPointWitnessRef::WorldPoint(point) => (
            finite_point(*point)?,
            SpatialWitnessResolutionClass::DirectWorld,
            None,
        ),
        SpatialPointWitnessRef::FrameOrigin(frame) => (
            admitted_frame_origin(frame.clone())?,
            SpatialWitnessResolutionClass::FrameDerived,
            None,
        ),
        SpatialPointWitnessRef::CarrierPoint { .. } => {
            return Err(SpatialWitnessFailureClass::Ambiguous);
        }
        SpatialPointWitnessRef::ParameterSpacePoint {
            carrier_kind,
            carrier,
            parameter,
        } => {
            let resolved =
                catalog.resolve_parameter_space_point(*carrier_kind, carrier, *parameter)?;
            (
                finite_point(resolved.world_point())?,
                resolved.resolution_class().as_witness_resolution_class(),
                resolved.parameter_admission().cloned(),
            )
        }
        SpatialPointWitnessRef::FeatureOwnedPoint { feature, role } => {
            let _role: SpatialCarrierPointRole = *role;
            let resolved = catalog.resolve_feature_owned_point(feature, *role)?;
            (
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

pub fn resolve_spatial_direction_witness(
    requested: SpatialDirectionWitnessRef,
) -> Result<ResolvedSpatialDirectionWitness, SpatialWitnessFailureClass> {
    resolve_spatial_direction_witness_with_catalog(requested, &EmptySpatialWitnessCatalog)
}

pub fn resolve_spatial_direction_witness_with_catalog(
    requested: SpatialDirectionWitnessRef,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<ResolvedSpatialDirectionWitness, SpatialWitnessFailureClass> {
    let (resolved_world_direction, resolution_class, parameter_admission) = match &requested {
        SpatialDirectionWitnessRef::WorldDirection(direction) => (
            normalize_direction(*direction)?,
            SpatialWitnessResolutionClass::DirectWorld,
            None,
        ),
        SpatialDirectionWitnessRef::FrameAxis { frame, axis } => {
            let direction = admitted_frame_axis(frame.clone(), *axis)?;
            (
                normalize_direction(direction)
                    .map_err(|_| SpatialWitnessFailureClass::Degenerate)?,
                SpatialWitnessResolutionClass::FrameDerived,
                None,
            )
        }
        SpatialDirectionWitnessRef::FramePerpendicularAxis { frame, axis } => {
            let parallel = admitted_frame_axis(frame.clone(), *axis)?;
            (
                fallback_perpendicular(parallel)?,
                SpatialWitnessResolutionClass::FallbackDerived,
                None,
            )
        }
        SpatialDirectionWitnessRef::CarrierDirection { .. } => {
            return Err(SpatialWitnessFailureClass::Ambiguous);
        }
        SpatialDirectionWitnessRef::ParameterSpaceDirection {
            carrier_kind,
            carrier,
            parameter,
            role,
        } => {
            let resolved = catalog.resolve_parameter_space_direction(
                *carrier_kind,
                carrier,
                *parameter,
                *role,
            )?;
            (
                normalize_direction(resolved.world_direction())?,
                resolved.resolution_class().as_witness_resolution_class(),
                resolved.parameter_admission().cloned(),
            )
        }
        SpatialDirectionWitnessRef::FeatureOwnedDirection { feature, role } => {
            let _role: SpatialCarrierDirectionRole = *role;
            let resolved = catalog.resolve_feature_owned_direction(feature, *role)?;
            (
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

fn admitted_frame_origin(frame: SpatialFrameRef) -> Result<[f64; 3], SpatialWitnessFailureClass> {
    let admitted =
        admit_spatial_frame(frame).map_err(|_| SpatialWitnessFailureClass::Degenerate)?;
    Ok(admitted.basis().origin())
}

fn admitted_frame_axis(
    frame: SpatialFrameRef,
    axis: SpatialAxis,
) -> Result<[f64; 3], SpatialWitnessFailureClass> {
    let admitted =
        admit_spatial_frame(frame).map_err(|_| SpatialWitnessFailureClass::Degenerate)?;
    Ok(axis_from_basis(admitted.basis(), axis))
}

#[cfg(test)]
#[path = "resolution_tests.rs"]
mod resolution_tests;
