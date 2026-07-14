use super::contract::{AccessShapeContract, ExpectedCounterClass};
#[cfg(test)]
use super::detail::GroupedPrefixBasis;
use super::detail::{AccessShapeDetail, PrefixBasis};
use super::lane::AccessLaneClassification;

pub(crate) const fn prefix_lookup_declaration() -> AccessShapeContract {
    AccessShapeContract::exact_read_declaration(
        AccessShapeDetail::PrefixLookup(PrefixBasis::CanonicalPrefixBounds),
        AccessLaneClassification::Foreground,
        ExpectedCounterClass::PrefixLookup,
    )
}

#[cfg(test)]
pub(crate) const fn grouped_prefix_lookup_declaration(
    basis: GroupedPrefixBasis,
) -> AccessShapeContract {
    AccessShapeContract::exact_read_declaration(
        AccessShapeDetail::GroupedPrefixLookup(basis),
        AccessLaneClassification::Foreground,
        ExpectedCounterClass::GroupedPrefixLookup,
    )
}
