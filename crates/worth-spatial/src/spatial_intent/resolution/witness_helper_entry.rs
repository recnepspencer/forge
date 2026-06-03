use crate::spatial_intent::refs::{
    EmptySpatialWitnessCatalog, SpatialDirectionWitnessRef, SpatialPointWitnessRef,
    SpatialWitnessCatalog,
};

use super::admitted_witness_requests::{
    admit_spatial_direction_witness_request, admit_spatial_point_witness_request,
};
use super::resolution::{
    resolve_admitted_spatial_direction_witness_request,
    resolve_admitted_spatial_point_witness_request, ResolvedSpatialDirectionWitness,
    ResolvedSpatialPointWitness,
};
use super::witness_support::SpatialWitnessFailureClass;

pub fn resolve_spatial_point_witness(
    requested: SpatialPointWitnessRef,
) -> Result<ResolvedSpatialPointWitness, SpatialWitnessFailureClass> {
    resolve_spatial_point_witness_with_catalog(requested, &EmptySpatialWitnessCatalog)
}

pub fn resolve_spatial_point_witness_with_catalog(
    requested: SpatialPointWitnessRef,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<ResolvedSpatialPointWitness, SpatialWitnessFailureClass> {
    let admitted = admit_spatial_point_witness_request(requested)?;
    resolve_admitted_spatial_point_witness_request(admitted, catalog)
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
    let admitted = admit_spatial_direction_witness_request(requested)?;
    resolve_admitted_spatial_direction_witness_request(admitted, catalog)
}
