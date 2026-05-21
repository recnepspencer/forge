mod bundle_verified;
mod registry;
mod truth;

pub(crate) use bundle_verified::{verify_bundle, PrimitiveConstructionCompoundParityReportBundle};
pub use bundle_verified::{
    PrimitiveConstructionCompoundParityReport,
    PrimitiveConstructionCompoundParityVerificationFailure,
    PrimitiveConstructionCompoundParityVerificationMismatch,
};
pub(crate) use registry::{compound_parity_registry, exhaustion_witness_kind_for};
pub use truth::PrimitiveConstructionCompoundParityCanonicalTruth;
