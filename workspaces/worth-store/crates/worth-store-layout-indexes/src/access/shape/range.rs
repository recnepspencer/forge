use super::contract::{AccessShapeContract, ExpectedCounterClass};
#[cfg(test)]
use super::detail::MultiRangeBasis;
use super::detail::{AccessShapeDetail, RangeBasis};
use super::lane::AccessLaneClassification;

pub(crate) const fn range_lookup_declaration() -> AccessShapeContract {
    AccessShapeContract::exact_read_declaration(
        AccessShapeDetail::RangeLookup(RangeBasis::CanonicalRangeBounds),
        AccessLaneClassification::Foreground,
        ExpectedCounterClass::RangeLookup,
    )
}

#[cfg(test)]
pub(crate) const fn multi_range_lookup_declaration(basis: MultiRangeBasis) -> AccessShapeContract {
    AccessShapeContract::exact_read_declaration(
        AccessShapeDetail::MultiRangeLookup(basis),
        AccessLaneClassification::Foreground,
        ExpectedCounterClass::MultiRangeLookup,
    )
}
