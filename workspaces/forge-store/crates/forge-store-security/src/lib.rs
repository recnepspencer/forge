#![forbid(unsafe_code)]

mod authenticity_check;
#[cfg(test)]
mod authenticity_check_tests;
mod authenticity_counters;
mod authenticity_denial;
mod authenticity_physical_identity;
mod authenticity_result;
mod authenticity_vocabulary;
mod authenticity_witness;
mod authority_source;
mod physical_security_metadata;
mod physical_security_metadata_canonical;
#[cfg(test)]
mod physical_security_metadata_tests;
mod raw_security_declarations;
mod repair_blast_radius;
mod scope_vocabulary;
mod security_scope_admission;
mod security_scope_admission_basis;
mod security_scope_admission_denial;
mod security_scope_admission_request;
#[cfg(test)]
mod security_scope_admission_tests;
mod security_scope_counters;
mod security_scope_custody_readmission;
#[cfg(test)]
mod security_scope_custody_readmission_tests;
mod security_scope_denial;
mod security_scope_identity;
mod security_scope_propagation;
#[cfg(test)]
mod security_scope_propagation_tests;
#[cfg(test)]
mod security_scope_readmission_tests;
mod security_scope_receipt;
mod security_scope_roles;
#[cfg(any(test, feature = "certification-test-authority"))]
mod security_scope_test_authority;
#[cfg(test)]
mod security_scope_test_support;
mod security_scope_witnesses;
mod trust_boundary;
mod trust_boundary_category;
mod trust_boundary_observation;
#[cfg(test)]
mod vocabulary_tests;

pub use authenticity_check::{
    StoreAuthenticityCheck, StoreAuthenticityCheckInput, StorePhysicalAuthenticityCheck,
    StoreScopedAuthenticityCheck,
};
pub use authenticity_counters::StoreAuthenticityCheckCounterSnapshot;
pub use authenticity_denial::{StoreAuthenticityCheckDenial, StoreAuthenticityCheckDenialKind};
pub use authenticity_physical_identity::StoreAuthenticityPhysicalIdentity;
pub use authenticity_result::{StoreAuthenticityResult, StoreAuthenticityResultKind};
pub use authenticity_vocabulary::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass,
};
pub use authenticity_witness::{
    admit_store_authenticity_witness_observation, StoreAuthenticityWitnessBinding,
    StoreAuthenticityWitnessInput, StoreAuthenticityWitnessObservationDeclaration,
};
pub use authority_source::{
    classify_app_org_id_as_security_scope_source, classify_audit_record_as_security_scope_source,
    classify_foundational_evidence_as_security_scope_source,
    classify_iam_role_as_security_scope_source,
    classify_identity_provider_claim_as_security_scope_source,
    classify_kms_key_id_as_security_scope_source,
    classify_offline_verifier_evidence_as_security_scope_source,
    classify_operator_identity_as_security_scope_source,
    classify_proof_progression_as_security_scope_source,
    classify_raw_string_as_security_scope_source, classify_semantic_id_as_security_scope_source,
    classify_store_current_authority_as_security_scope_source,
    classify_terminal_json_label_as_security_scope_source, StoreSecurityAuthoritySource,
};
pub use physical_security_metadata::{
    admit_store_physical_security_metadata, StoreAllocationClassSecurityMetadataEnvelope,
    StoreExtentSecurityMetadataEnvelope, StoreFreeSpaceSecurityMetadataEnvelope,
    StorePhysicalSecurityMetadataAdmissionInput, StorePhysicalSecurityMetadataCarrier,
    StorePhysicalSecurityMetadataEnvelope, StoreRawPhysicalSecurityMetadataDeclaration,
    StoreRawPhysicalSecurityMetadataProjection, StoreSegmentPageSecurityMetadataEnvelope,
    StoreSegmentSecurityMetadataEnvelope,
};
pub use physical_security_metadata_canonical::{
    compare_store_physical_security_metadata, StorePhysicalSecurityMetadataCanonicalBasis,
};
pub use raw_security_declarations::{
    evaluate_deserialized_security_scope_readmission,
    readmit_deserialized_security_scope_declaration, StoreApplicationOrgIdClaim, StoreIamRoleClaim,
    StoreJwtSubjectClaim, StoreKmsKeyIdentifier, StoreOperatorIdentityClaim,
    StoreRawSecurityScopeDeclaration, StoreRepairAuditRecordClaim,
    StoreSecurityScopeDeclarationProvenance, StoreSecurityScopeReadmissionEvaluation,
};
pub use repair_blast_radius::{
    reject_repair_authority_source, repair_blast_radius_authenticity,
    repair_blast_radius_expectation, StoreRepairPhysicalRegionAdmissionOutcome,
    StoreRepairPhysicalRegionDeclaration, StoreRepairPhysicalRegionWitness,
};
pub use scope_vocabulary::{
    StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture, StoreLegacySecurityPosture,
    StoreTenantScope,
};
pub use security_scope_admission::{
    admission_counter_snapshot, admit_store_security_scope,
    evaluate_store_security_scope_admission, StoreSecurityScopeAdmissionEvaluation,
    StoreSecurityScopeAdmissionOutcome,
};
pub use security_scope_admission_basis::{
    StoreSecurityScopeAdmissionBasis, StoreSecurityScopeAdmissionExpectation,
};
pub use security_scope_admission_denial::{
    StoreSecurityScopeAdmissionDeferred, StoreSecurityScopeAdmissionDenial,
    StoreSecurityScopeAdmissionFailure, StoreSecurityScopeAdmissionRebindRequired,
    StoreSecurityScopeAdmissionStale,
};
pub use security_scope_admission_request::StoreSecurityScopeAdmissionRequest;
pub use security_scope_counters::StoreSecurityScopeAdmissionCounterSnapshot;
pub use security_scope_custody_readmission::readmit_trust_boundary_security_scope_declaration;
pub use security_scope_denial::{
    reject_non_store_security_scope_source, StoreSecurityScopeDenial, StoreSecurityScopeDenialKind,
};
pub use security_scope_identity::StoreSecurityScopeIdentity;
pub use security_scope_propagation::{
    deny_drifted_store_security_scope, deny_missing_store_security_scope,
    deny_stale_store_security_scope, propagate_store_security_scope,
    StoreSecurityScopePropagationCounters, StoreSecurityScopePropagationDenial,
    StoreSecurityScopePropagationOutcome, StoreSecurityScopePropagationSite,
    StoreSecurityScopePropagationWitness,
};
pub use security_scope_receipt::{
    StoreSecurityScopeAdmissionReceipt, StoreSecurityScopeAdmissionReceiptId,
    StoreSecurityScopeProofProgressionIdentity,
};
pub use security_scope_roles::{
    StoreSecurityEvidenceVocabulary, StoreSecurityReadinessVocabulary,
    StoreSecurityReadinessVocabularyTerm, StoreSecurityRequirementVocabulary,
    StoreSecurityResultVocabulary, StoreSecurityWitnessVocabulary,
    StoreSecurityWitnessVocabularyTerm,
};
#[cfg(any(test, feature = "certification-test-authority"))]
pub use security_scope_test_authority::{
    admitted_store_internal_security_scope_for_s6_test,
    admitted_wrong_s6_io_qos_security_scope_for_test,
};
pub use security_scope_witnesses::{
    StoreAdmittedSecurityScope, StoreCurrentAuthenticityScopeWitness,
    StoreCurrentCustodyScopeWitness, StoreCurrentKeyScopeWitness,
    StoreCurrentKeyVersionScopeWitness, StoreCurrentSecurityScopeWitnessSet,
    StoreCurrentTenantScopeWitness,
};
pub use trust_boundary::{
    StoreBackupRestoreAfterKeyRotationBoundaryEvidence,
    StoreBackupRestoreAfterKeyRotationBoundaryFact, StoreBackupRestoreBoundaryFactInput,
    StoreCustodyDomainBoundaryEvidence, StoreCustodyDomainBoundaryFact,
    StoreCustodyDomainBoundaryFactInput, StoreDeploymentBoundaryFact,
    StoreDifferentDeploymentBoundaryEvidence, StoreDifferentDeploymentBoundaryFact,
    StoreDifferentStoreInstanceBoundaryEvidence, StoreDifferentStoreInstanceBoundaryFact,
    StoreKeyScopeGenerationBoundaryEvidence, StoreKeyScopeGenerationBoundaryFact,
    StoreKeyScopeGenerationBoundaryFactInput, StoreOfflineExportImportBoundaryEvidence,
    StoreOfflineExportImportBoundaryFact, StoreOfflineTransferBoundaryFact,
    StoreStoreInstanceBoundaryFact, StoreTenantScopeAuthorityBoundaryEvidence,
    StoreTenantScopeAuthorityBoundaryFact, StoreTenantScopeAuthorityBoundaryFactInput,
    StoreTrustBoundaryCrossing, StoreTrustBoundaryCrossingEvidence, StoreTrustBoundaryEvidence,
    StoreTrustBoundaryEvidenceDenial, StoreTrustBoundaryReadmissionTrigger,
};
pub use trust_boundary_observation::{
    store_backup_restore_boundary_fact, store_custody_domain_boundary_fact,
    store_deployment_boundary_fact, store_instance_boundary_fact,
    store_key_scope_generation_boundary_fact, store_offline_transfer_boundary_fact,
    store_tenant_scope_authority_boundary_fact,
};
