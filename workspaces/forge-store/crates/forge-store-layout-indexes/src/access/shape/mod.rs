mod append;
mod contract;
mod degraded;
mod denial;
mod detail;
mod kind;
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
mod scan;
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
pub use detail::FullDeclaredScanBasis;
pub use detail::{AccessShapeDetail, PrefixBasis, RangeBasis};
#[cfg(test)]
pub use detail::{
    BoundedScanBasis, CoalescedPageReadBasis, DegradedExactScanBasis, GroupedPrefixBasis,
    MaintenanceReadBasis, ManifestGraphWalkBasis, MultiRangeBasis, MutationAccessBasis,
    StreamingContinuationBasis, StreamingReadBasis,
};
pub use kind::AccessShape;
pub use lane::AccessLaneClassification;
pub use registry::access_shapes;
pub use scan::{
    full_declared_scan_cases, FullDeclaredScanCaseId, FullDeclaredScanOutcome, FullDeclaredScanView,
};
