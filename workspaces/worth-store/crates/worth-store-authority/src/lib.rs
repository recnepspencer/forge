#![forbid(unsafe_code)]

mod aspect_native_authority;
mod authority_readmission;
mod canonical_authority_record;
mod current_authority;
mod current_authority_identity;
mod derived_authority_evidence;
mod external_authority_token;
mod retained_authority_evidence;

pub use aspect_native_authority::{
    admit_aspect_native_authority_record, AspectNativeAuthorityRecord,
};
pub use authority_readmission::{
    deny_lower_authority_source_as_current_authority,
    deny_lower_authority_source_readmission_as_current_authority,
    deny_unsupported_authority_source_as_current_authority,
    deny_unsupported_authority_source_readmission_as_current_authority,
    readmit_external_store_authority_token, readmit_retained_store_authority_evidence,
    StoreAuthorityReadmissionDenial, StoreAuthorityReadmissionOutcome, StoreLowerAuthoritySource,
};
pub use canonical_authority_record::CanonicalAuthorityRecord;
pub use current_authority::{
    require_current_physical_authority, require_current_store_authority,
    StoreCurrentAuthorityWitness, StoreCurrentPhysicalAuthorityWitness,
};
pub use current_authority_identity::StoreCurrentAuthorityIdentity;
pub use derived_authority_evidence::{
    report_derived_store_authority_evidence, StoreDerivedAuthorityEvidence,
    StoreDerivedAuthorityEvidenceRole,
};
pub use external_authority_token::{
    StoreAuthorityFilename, StoreExternalAuthorityToken, StoreExternalAuthorityTokenFreshness,
};
pub use retained_authority_evidence::{
    compare_retained_store_authority_evidence, report_retained_store_authority_evidence,
    StoreRetainedAuthorityEvidence, StoreRetainedAuthorityEvidenceComparison,
};
