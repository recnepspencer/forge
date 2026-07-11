mod declaration;
pub mod damage_map;
mod denial;
mod inspection_envelope;
mod payload;
mod admission;

pub use admission::IntegrityHandoffAdmission;
pub use declaration::IntegrityHandoffDeclaration;
pub use denial::{IntegrityHandoffDenial, IntegrityHandoffDenialKind};
pub use inspection_envelope::{BoundedInspectionEnvelopeEvidence, ChecksumAlgorithmScopeBasis};
pub use payload::{
    IntegrityHandoffCounters, IntegrityHandoffPayload, RawBytesExcludedFromRecoveryHandoff,
};
