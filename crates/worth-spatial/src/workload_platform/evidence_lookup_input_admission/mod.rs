mod admission;
mod admitted_input;
mod counters;
mod current_query_receipt;
mod error;
mod product_separation;
mod query_support;
mod request;
mod stage_receipt_identity;
mod topology_support;

#[cfg(test)]
mod tests;

pub use admission::admit_evidence_lookup_input;
pub use admitted_input::EvidenceLookupAdmittedInput;
pub use counters::EvidenceLookupInputAdmissionCounters;
pub(crate) use current_query_receipt::current_projection_consumption_receipt;
#[cfg(test)]
pub(crate) use current_query_receipt::real_projection_consumption_receipt;
pub use error::{EvidenceLookupInputAdmissionError, EvidenceLookupInputAdmissionErrorKind};
pub use product_separation::EvidenceLookupProductSeparationProof;
pub use query_support::{
    EvidenceLookupQueryAdmissionEvidenceSet, EvidenceLookupQueryAdmissionSupport,
    EvidenceLookupQuerySupportState,
};
pub use request::EvidenceLookupInputAdmissionRequest;
pub use stage_receipt_identity::EvidenceLookupStageReceiptAdmission;
pub use topology_support::{
    EvidenceLookupTopologyAdmissionSupport, EvidenceLookupTopologySupportState,
};
