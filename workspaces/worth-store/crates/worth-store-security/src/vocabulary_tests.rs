use worth_store_aspect_native::StorePhysicalBoundaryWitness;
use worth_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};

use crate::{
    classify_app_org_id_as_security_scope_source,
    classify_foundational_evidence_as_security_scope_source,
    classify_iam_role_as_security_scope_source,
    classify_identity_provider_claim_as_security_scope_source,
    classify_kms_key_id_as_security_scope_source,
    classify_operator_identity_as_security_scope_source,
    classify_proof_progression_as_security_scope_source,
    classify_raw_string_as_security_scope_source, classify_semantic_id_as_security_scope_source,
    classify_store_current_authority_as_security_scope_source,
    classify_terminal_json_label_as_security_scope_source, reject_non_store_security_scope_source,
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreKeyVersionPosture, StoreLegacySecurityPosture,
    StoreSecurityEvidenceVocabulary, StoreSecurityReadinessVocabularyTerm,
    StoreSecurityRequirementVocabulary, StoreSecurityResultVocabulary,
    StoreSecurityScopeDenialKind, StoreSecurityScopeIdentity, StoreSecurityWitnessVocabularyTerm,
    StoreTenantScope,
};

#[test]
fn every_lower_authority_source_has_a_specific_security_scope_denial() {
    let cases = [
        (
            classify_raw_string_as_security_scope_source(),
            StoreSecurityScopeDenialKind::RawStringIsNotSecurityScope,
        ),
        (
            classify_semantic_id_as_security_scope_source(),
            StoreSecurityScopeDenialKind::SemanticIdIsNotSecurityScope,
        ),
        (
            classify_terminal_json_label_as_security_scope_source(),
            StoreSecurityScopeDenialKind::TerminalJsonLabelIsNotSecurityScope,
        ),
        (
            classify_identity_provider_claim_as_security_scope_source(),
            StoreSecurityScopeDenialKind::JwtSubjectIsNotTenantScope,
        ),
        (
            classify_app_org_id_as_security_scope_source(),
            StoreSecurityScopeDenialKind::ApplicationOrgIdIsNotTenantScope,
        ),
        (
            classify_kms_key_id_as_security_scope_source(),
            StoreSecurityScopeDenialKind::KmsKeyIdIsNotKeyScope,
        ),
        (
            classify_iam_role_as_security_scope_source(),
            StoreSecurityScopeDenialKind::IamRoleIsNotCustodyPosture,
        ),
        (
            classify_operator_identity_as_security_scope_source(),
            StoreSecurityScopeDenialKind::OperatorIdentityIsNotRepairAuthority,
        ),
        (
            classify_foundational_evidence_as_security_scope_source(),
            StoreSecurityScopeDenialKind::FoundationalEvidenceIsNotStoreSecurityAuthority,
        ),
        (
            classify_proof_progression_as_security_scope_source(),
            StoreSecurityScopeDenialKind::ProofProgressionIsNotStoreSecurityAuthority,
        ),
        (
            classify_store_current_authority_as_security_scope_source(),
            StoreSecurityScopeDenialKind::StoreCurrentAuthorityWitnessIsNotSecurityScopeAuthority,
        ),
    ];

    for (source, expected_kind) in cases {
        let denial = reject_non_store_security_scope_source(source);
        assert_eq!(denial.source(), source);
        assert_eq!(denial.kind(), expected_kind);
    }
}

#[test]
fn phase_required_vocabulary_families_are_distinct_and_exercised() {
    assert_ne!(StoreKeyScope::TenantEnvelope, StoreKeyScope::PageEnvelope);
    assert!(StoreKeyVersionPosture::Current.is_admissible_for_platform_lane());
    assert!(!StoreKeyVersionPosture::Unsupported.is_admissible_for_platform_lane());
    assert_ne!(
        StoreKeyVersionPosture::Unsupported,
        StoreKeyVersionPosture::Unavailable
    );
    assert_ne!(
        StoreKeyVersionPosture::Unavailable,
        StoreKeyVersionPosture::Stale
    );
    assert!(StoreTenantScope::TenantPhysicalBoundary.is_store_physical_blast_radius());
    assert!(!StoreTenantScope::TenantPhysicalBoundary.is_identity_provider_claim());
    assert!(StoreCustodyPosture::InternalStoreCustody.is_store_custody_vocabulary());
    assert!(!StoreCustodyPosture::InternalStoreCustody.is_iam_role());
    assert_ne!(
        StoreCustodyPosture::CustodyUnsupported,
        StoreCustodyPosture::CustodyUnavailable
    );
    assert!(StoreLegacySecurityPosture::LegacyUnscoped.requires_readmission_when_unscoped());
    assert!(StoreLegacySecurityPosture::ReadmissionRequired.requires_readmission_when_unscoped());
    assert!(StoreLegacySecurityPosture::SecurityMetadataUnavailable
        .requires_readmission_when_unscoped());
    assert!(
        StoreLegacySecurityPosture::UnsupportedLegacyArtifact.requires_readmission_when_unscoped()
    );
    assert!(!StoreLegacySecurityPosture::NativeScoped.requires_readmission_when_unscoped());

    let requirement = StoreSecurityRequirementVocabulary::AuthenticityRequirementDeclared;
    let result = StoreSecurityResultVocabulary::AuthenticityObservedResult;
    let witness_term = StoreSecurityWitnessVocabularyTerm::KeyScopeWitness;
    let evidence = StoreSecurityEvidenceVocabulary::PublishableBoundaryEvidence;
    let readiness_term = StoreSecurityReadinessVocabularyTerm::SecurityFoundationReadiness;

    assert_eq!(
        requirement,
        StoreSecurityRequirementVocabulary::AuthenticityRequirementDeclared
    );
    assert_eq!(
        result,
        StoreSecurityResultVocabulary::AuthenticityObservedResult
    );
    assert_eq!(
        witness_term,
        StoreSecurityWitnessVocabularyTerm::KeyScopeWitness
    );
    assert_eq!(
        evidence,
        StoreSecurityEvidenceVocabulary::PublishableBoundaryEvidence
    );
    assert_eq!(
        readiness_term,
        StoreSecurityReadinessVocabularyTerm::SecurityFoundationReadiness
    );
}

#[test]
fn authenticity_requirement_cannot_be_required_not_required() {
    let not_required = StoreAuthenticityRequirement::not_required();
    let required = StoreAuthenticityRequirement::required(
        StoreAuthenticityRequirementClass::AuthenticatedWalRecord,
    );

    assert_eq!(not_required.class(), None);
    assert_eq!(
        required.class(),
        Some(StoreAuthenticityRequirementClass::AuthenticatedWalRecord)
    );
    assert!(!not_required.requires_admission_before_result());
    assert!(required.requires_admission_before_result());
}

#[test]
fn identical_physical_witnesses_are_not_security_equivalent_across_scope_drift() {
    let physical_witness = physical_witness();
    let requirement = StoreAuthenticityRequirement::required(
        StoreAuthenticityRequirementClass::AuthenticatedFrame,
    );
    let tenant_boundary = StoreSecurityScopeIdentity::from_physical_security_scope(
        physical_witness,
        StoreKeyScope::PageEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::TenantPhysicalBoundary,
        requirement,
        StoreCustodyPosture::InternalStoreCustody,
    );
    let import_boundary = StoreSecurityScopeIdentity::from_physical_security_scope(
        physical_witness,
        StoreKeyScope::PageEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::ImportReadmissionBoundary,
        requirement,
        StoreCustodyPosture::InternalStoreCustody,
    );
    let backup_key_scope = StoreSecurityScopeIdentity::from_physical_security_scope(
        physical_witness,
        StoreKeyScope::BackupExportEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::TenantPhysicalBoundary,
        requirement,
        StoreCustodyPosture::InternalStoreCustody,
    );

    assert_eq!(tenant_boundary.physical_witness(), physical_witness);
    assert_ne!(tenant_boundary, import_boundary);
    assert_ne!(tenant_boundary, backup_key_scope);
}

fn physical_witness() -> StorePhysicalBoundaryWitness {
    StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .unwrap(),
    )
    .unwrap()
}
