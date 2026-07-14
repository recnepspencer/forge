use super::contract::{AccessShapeContract, ExpectedCounterClass};
use super::detail::AccessShapeDetail;
use super::lane::AccessLaneClassification;

pub(crate) const fn point_lookup_declaration() -> AccessShapeContract {
    AccessShapeContract::exact_read_declaration(
        AccessShapeDetail::PointLookup,
        AccessLaneClassification::Foreground,
        ExpectedCounterClass::PointLookup,
    )
}
