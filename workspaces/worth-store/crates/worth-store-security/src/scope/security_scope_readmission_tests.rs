use worth_proof::TransitionOutcome;

use crate::scope::security_scope_test_support::{
    current_authority, deserialized_declaration, platform_authenticity_requirement,
    platform_deserialized_declaration, raw_request, readmit_platform,
};
use crate::{
    admit_store_security_scope, readmit_deserialized_security_scope_declaration,
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreKeyVersionPosture, StoreRawSecurityScopeDeclaration,
    StoreSecurityScopeAdmissionDenial, StoreSecurityScopeAdmissionExpectation,
    StoreSecurityScopeAdmissionStale, StoreTenantScope,
};

#[test]
fn deserialized_scope_requires_readmission_before_admission() {
    let authority = current_authority("store.s51.security.readmission", "page-0008");
    let deserialized = platform_deserialized_declaration(&authority);

    assert!(matches!(
        admit_store_security_scope(raw_request(&authority, deserialized)),
        TransitionOutcome::Denied(
            StoreSecurityScopeAdmissionDenial::DeserializedSecurityScopeRequiresReadmission
        )
    ));

    let readmitted = readmit_platform(&authority, deserialized).unwrap();
    let readmitted_request = raw_request(&authority, readmitted);
    let expected_progression = readmitted_request.basis().proof_progression_identity();
    let admitted = match admit_store_security_scope(readmitted_request) {
        TransitionOutcome::Success(admitted) => admitted,
        outcome => panic!("readmitted declaration should admit: {outcome:?}"),
    };

    assert_eq!(
        admitted.receipt().proof_progression_identity(),
        expected_progression
    );
}

#[test]
fn readmission_denies_wrong_key_tenant_authenticity_and_custody_expectations() {
    let authority = current_authority("store.s51.security.readmission.denials", "page-0009");
    let deserialized = platform_deserialized_declaration(&authority);

    assert_readmission_denial(
        &authority,
        deserialized,
        StoreSecurityScopeAdmissionExpectation::new(
            StoreKeyScope::BackupExportEnvelope,
            StoreTenantScope::TenantPhysicalBoundary,
            platform_authenticity_requirement(),
            StoreCustodyPosture::InternalStoreCustody,
        ),
        StoreSecurityScopeAdmissionDenial::WrongKeyScope,
    );
    assert_readmission_denial(
        &authority,
        deserialized,
        StoreSecurityScopeAdmissionExpectation::new(
            StoreKeyScope::PageEnvelope,
            StoreTenantScope::ImportReadmissionBoundary,
            platform_authenticity_requirement(),
            StoreCustodyPosture::InternalStoreCustody,
        ),
        StoreSecurityScopeAdmissionDenial::WrongTenantScope,
    );
    assert_readmission_denial(
        &authority,
        deserialized,
        StoreSecurityScopeAdmissionExpectation::new(
            StoreKeyScope::PageEnvelope,
            StoreTenantScope::TenantPhysicalBoundary,
            StoreAuthenticityRequirement::required(
                StoreAuthenticityRequirementClass::AuthenticatedManifest,
            ),
            StoreCustodyPosture::InternalStoreCustody,
        ),
        StoreSecurityScopeAdmissionDenial::UnsupportedAuthenticityRequirement,
    );
    assert_readmission_denial(
        &authority,
        deserialized,
        StoreSecurityScopeAdmissionExpectation::new(
            StoreKeyScope::PageEnvelope,
            StoreTenantScope::TenantPhysicalBoundary,
            platform_authenticity_requirement(),
            StoreCustodyPosture::ExportPrepared,
        ),
        StoreSecurityScopeAdmissionDenial::WrongCustodyPosture,
    );
}

#[test]
fn readmission_preserves_stale_key_version_for_typed_admission_outcome() {
    let authority = current_authority("store.s51.security.readmission.stale", "page-0011");
    let stale = StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
        authority.physical_witness(),
        StoreKeyScope::PageEnvelope,
        StoreKeyVersionPosture::Stale,
        StoreTenantScope::TenantPhysicalBoundary,
        Some(platform_authenticity_requirement()),
        Some(StoreCustodyPosture::InternalStoreCustody),
    );

    let readmitted = readmit_platform(&authority, stale).unwrap();
    assert!(matches!(
        admit_store_security_scope(raw_request(&authority, readmitted)),
        TransitionOutcome::Stale(StoreSecurityScopeAdmissionStale::StaleKeyVersionPosture(
            StoreKeyVersionPosture::Stale
        ))
    ));
}

#[test]
fn readmission_denies_missing_authenticity_missing_custody_and_replayed_evidence() {
    let authority = current_authority("store.s51.security.readmission.missing", "page-0010");

    let missing_authenticity = deserialized_declaration(
        &authority,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        None,
        Some(StoreCustodyPosture::InternalStoreCustody),
    );
    assert!(matches!(
        readmit_platform(&authority, missing_authenticity),
        Err(StoreSecurityScopeAdmissionDenial::MissingAuthenticityRequirement)
    ));

    let missing_custody = deserialized_declaration(
        &authority,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        Some(platform_authenticity_requirement()),
        None,
    );
    assert!(matches!(
        readmit_platform(&authority, missing_custody),
        Err(StoreSecurityScopeAdmissionDenial::MissingCustodyPosture)
    ));

    let replayed = StoreRawSecurityScopeDeclaration::replayed_admission_evidence(
        authority.physical_witness(),
        StoreKeyScope::PageEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::TenantPhysicalBoundary,
        Some(platform_authenticity_requirement()),
        Some(StoreCustodyPosture::InternalStoreCustody),
    );
    assert!(matches!(
        readmit_platform(&authority, replayed),
        Err(StoreSecurityScopeAdmissionDenial::ReplayedAdmissionEvidence)
    ));
}

fn assert_readmission_denial(
    authority: &worth_store_authority::StoreCurrentAuthorityWitness,
    declaration: StoreRawSecurityScopeDeclaration,
    expectation: StoreSecurityScopeAdmissionExpectation,
    expected_denial: StoreSecurityScopeAdmissionDenial,
) {
    assert_eq!(
        readmit_deserialized_security_scope_declaration(authority, declaration, expectation),
        Err(expected_denial)
    );
}
