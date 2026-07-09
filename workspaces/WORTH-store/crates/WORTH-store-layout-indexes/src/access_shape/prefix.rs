use super::contract::{S8AccessShapeContract, S8ExpectedCounterClass};
use super::denial::S8AccessShapeUnsupportedDenial;
use super::detail::{S8AccessShapeDetail, S8GroupedPrefixBasis, S8PrefixBasis};
use super::lane::S8AccessLaneClassification;
use crate::materialization::S8LayoutCoverageWitness;

pub(crate) fn prefix_lookup(
    coverage: S8LayoutCoverageWitness,
) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
    let completeness = coverage
        .require_exact_prefix_completeness()
        .map_err(S8AccessShapeUnsupportedDenial::MaterializationDenied)?;
    Ok(S8AccessShapeContract::exact_read(
        S8AccessShapeDetail::PrefixLookup(S8PrefixBasis::CanonicalPrefixBounds),
        S8AccessLaneClassification::Foreground,
        S8ExpectedCounterClass::PrefixLookup,
        completeness.coverage(),
    ))
}

pub(crate) fn grouped_prefix_lookup(
    coverage: S8LayoutCoverageWitness,
    basis: S8GroupedPrefixBasis,
) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
    let completeness = coverage
        .require_exact_prefix_completeness()
        .map_err(S8AccessShapeUnsupportedDenial::MaterializationDenied)?;
    Ok(S8AccessShapeContract::exact_read(
        S8AccessShapeDetail::GroupedPrefixLookup(basis),
        S8AccessLaneClassification::Foreground,
        S8ExpectedCounterClass::GroupedPrefixLookup,
        completeness.coverage(),
    ))
}
