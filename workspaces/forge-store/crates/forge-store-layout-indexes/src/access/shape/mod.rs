mod append;
mod contract;
mod degraded;
mod denial;
mod detail;
mod lane;
mod point;
mod prefix;
#[cfg(test)]
mod quarantine;
mod range;
mod rebuild;
mod registry;
#[cfg(test)]
mod repair;
#[cfg(test)]
mod scan;
mod shape;
#[cfg(test)]
mod streaming;
#[cfg(test)]
pub(crate) mod tests;
#[cfg(test)]
mod verifier;

pub use contract::{
    AccessAuthorityPosture, AccessShapeContract, AccessStaleDisposition, ExpectedCounterClass,
};
pub use degraded::DegradedExactScanRequest;
pub use denial::AccessShapeUnsupportedDenial;
pub use detail::{AccessShapeDetail, PrefixBasis, RangeBasis};
#[cfg(test)]
pub use detail::{
    BoundedScanBasis, CoalescedPageReadBasis, DegradedExactScanBasis, FullDeclaredScanBasis,
    GroupedPrefixBasis, MaintenanceReadBasis, ManifestGraphWalkBasis, MultiRangeBasis,
    MutationAccessBasis, StreamingContinuationBasis, StreamingReadBasis,
};
pub use lane::AccessLaneClassification;
pub use registry::access_shapes;
pub use shape::AccessShape;
