use super::contract::{S8AccessShapeContract, S8ExpectedCounterClass};
use super::denial::S8AccessShapeUnsupportedDenial;
use super::detail::{S8AccessShapeDetail, S8BatchPointBasis, S8SortedBatchBasis};
use super::lane::S8AccessLaneClassification;
use crate::materialization::S8LayoutCoverageWitness;

pub(crate) fn point_lookup(
    coverage: S8LayoutCoverageWitness,
) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
    Ok(S8AccessShapeContract::exact_read(
        S8AccessShapeDetail::PointLookup,
        S8AccessLaneClassification::Foreground,
        S8ExpectedCounterClass::PointLookup,
        coverage
            .require_exact()
            .map_err(S8AccessShapeUnsupportedDenial::MaterializationDenied)?,
    ))
}

pub(crate) fn batch_point_lookup(
    coverage: S8LayoutCoverageWitness,
    basis: S8BatchPointBasis,
) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
    Ok(S8AccessShapeContract::exact_read(
        S8AccessShapeDetail::BatchPointLookup(basis),
        S8AccessLaneClassification::Foreground,
        S8ExpectedCounterClass::BatchPointLookup,
        coverage
            .require_exact()
            .map_err(S8AccessShapeUnsupportedDenial::MaterializationDenied)?,
    ))
}

pub(crate) fn sorted_batch_lookup(
    coverage: S8LayoutCoverageWitness,
    basis: S8SortedBatchBasis,
) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
    Ok(S8AccessShapeContract::exact_read(
        S8AccessShapeDetail::SortedBatchLookup(basis),
        S8AccessLaneClassification::Foreground,
        S8ExpectedCounterClass::SortedBatchLookup,
        coverage
            .require_exact()
            .map_err(S8AccessShapeUnsupportedDenial::MaterializationDenied)?,
    ))
}
