use crate::spatial_intent::refs::SpatialAxis;
use worth_math::{canonical_perpendicular_unit_vector, FinitePoint3, UnitVector3};

use super::SpatialFrameBasis;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpatialWitnessResolutionClass {
    DirectWorld,
    FrameDerived,
    CarrierDerived,
    FallbackDerived,
    Exhausted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpatialWitnessFailureClass {
    NonFinite,
    Ambiguous,
    Undefined,
    Unsupported,
    Degenerate,
    Coincident,
    Exhausted,
}

pub fn finite_point(point: [f64; 3]) -> Result<[f64; 3], SpatialWitnessFailureClass> {
    FinitePoint3::try_new(point)
        .map(FinitePoint3::as_array)
        .map_err(|_| SpatialWitnessFailureClass::NonFinite)
}

pub fn normalize_direction(vector: [f64; 3]) -> Result<[f64; 3], SpatialWitnessFailureClass> {
    if vector.iter().any(|value| !value.is_finite()) {
        return Err(SpatialWitnessFailureClass::NonFinite);
    }
    UnitVector3::try_new(vector)
        .map(UnitVector3::as_array)
        .map_err(|_| SpatialWitnessFailureClass::Undefined)
}

pub fn axis_from_basis(basis: SpatialFrameBasis, axis: SpatialAxis) -> [f64; 3] {
    match axis {
        SpatialAxis::U => basis.u_axis(),
        SpatialAxis::V => basis.v_axis(),
        SpatialAxis::W => basis.w_axis(),
    }
}

pub fn fallback_perpendicular(parallel: [f64; 3]) -> Result<[f64; 3], SpatialWitnessFailureClass> {
    if parallel.iter().any(|value| !value.is_finite()) {
        return Err(SpatialWitnessFailureClass::NonFinite);
    }
    let unit = UnitVector3::try_new(parallel).map_err(|_| SpatialWitnessFailureClass::Exhausted)?;
    Ok(canonical_perpendicular_unit_vector(unit).as_array())
}
