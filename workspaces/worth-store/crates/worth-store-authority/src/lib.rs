#![forbid(unsafe_code)]

mod aspect_native_authority;
mod authority_readmission;
mod backup_restore_admission;
mod canonical_authority_record;
mod current_authority;
mod current_authority_identity;
mod derived_authority_evidence;
mod external_authority_token;
mod fencing_authority;
mod recovery_authority_admission_policy;
mod recovery_authority_posture;
mod recovery_cutover;
mod recovery_fence_release;
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
pub use backup_restore_admission::{
    BackupRestoreAdmissionAuthority, BackupRestoreAdmissionDenial, BackupRestoreAdmissionPolicy,
    BackupRestoreAdmissionReceipt, BackupRestoreAdmissionRequest,
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
pub use fencing_authority::{
    ControlStoreFencingAuthority, ControlStoreFencingPort, ControlStoreFencingProviderDenial,
    ControlStoreGeneration, ControlStoreSelectionCoordinates, SelectedControlStoreGeneration,
};
pub use recovery_authority_admission_policy::{
    RecoveryAuthorityAdmissionPolicy, RecoveryAuthorityAdmissionPolicyDenial,
    RecoveryAuthorityAdmissionPolicyKind,
};
pub use recovery_authority_posture::{
    RecoveryAuthorityAdmissionPosture, RecoveryAuthorityRegionPosture,
};
pub use recovery_cutover::{
    CurrentAuthorityReadmissionReceipt, RecoveryAuthorityReadmissionDenial,
    RecoveryCutoverAuthorityOwner, RecoveryWriteFenceDenial, RecoveryWriteFenceDisposition,
    RecoveryWriteFencePlan, RecoveryWriteFencePort, RecoveryWriteFenceProviderReceipt,
    RecoveryWriteFenceReceipt, RecoveryWriteFenceRecoveryRequest,
    RecoveryWriteFenceReleaseProviderReceipt, RecoveryWriteFenceReleaseReceipt,
    RecoveryWriteFenceReleaseRequest, RecoveryWriteFenceRequest,
};
pub use retained_authority_evidence::{
    compare_retained_store_authority_evidence, report_retained_store_authority_evidence,
    StoreRetainedAuthorityEvidence, StoreRetainedAuthorityEvidenceComparison,
};
