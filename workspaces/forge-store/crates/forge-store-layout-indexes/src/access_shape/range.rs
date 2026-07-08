use super::contract::{S8AccessShapeContract, S8ExpectedCounterClass};
use super::denial::S8AccessShapeUnsupportedDenial;
use super::detail::{S8AccessShapeDetail, S8MultiRangeBasis, S8RangeBasis};
use super::lane::S8AccessLaneClassification;
use crate::materialization::S8LayoutCoverageWitness;

pub(crate) fn range_lookup(
    coverage: S8LayoutCoverageWitness,
) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
    let completeness = coverage
        .require_exact_range_completeness()
        .map_err(S8AccessShapeUnsupportedDenial::MaterializationDenied)?;
    Ok(S8AccessShapeContract::exact_read(
        S8AccessShapeDetail::RangeLookup(S8RangeBasis::CanonicalRangeBounds),
        S8AccessLaneClassification::Foreground,
        S8ExpectedCounterClass::RangeLookup,
        completeness.coverage(),
    ))
}

pub(crate) fn multi_range_lookup(
    coverage: S8LayoutCoverageWitness,
    basis: S8MultiRangeBasis,
) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
    let completeness = coverage
        .require_exact_range_completeness()
        .map_err(S8AccessShapeUnsupportedDenial::MaterializationDenied)?;
    Ok(S8AccessShapeContract::exact_read(
        S8AccessShapeDetail::MultiRangeLookup(basis),
        S8AccessLaneClassification::Foreground,
        S8ExpectedCounterClass::MultiRangeLookup,
        completeness.coverage(),
    ))
}
