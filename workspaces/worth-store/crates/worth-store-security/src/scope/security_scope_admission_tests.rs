use worth_proof::TransitionOutcome;

use crate::scope::security_scope_test_support::{
    current_authority, platform_authenticity_requirement, platform_request, raw_request,
    request_with_custody,
};
use crate::{
    admission_counter_snapshot, admit_store_security_scope, StoreAuthenticityRequirement,
    StoreAuthenticityRequirementClass, StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreRawSecurityScopeDeclaration, StoreSecurityScopeAdmissionDenial,
    StoreSecurityScopeAdmissionExpectation, StoreSecurityScopeAdmissionRebindRequired,
    StoreSecurityScopeAdmissionRequest, StoreSecurityScopeAdmissionStale, StoreTenantScope,
};

#[test]
fn sealed_store_admission_issues_current_security_scope_witnesses_and_receipt() {
    let authority = current_authority("store.s51.security.current", "page-0001");
    let request = platform_request(&authority, StoreKeyVersionPosture::Current);
    let expected_progression = request.basis().proof_progression_identity();

    let admitted = match admit_store_security_scope(request) {
        TransitionOutcome::Success(admitted) => admitted,
        outcome => panic!("current platform scope should admit: {outcome:?}"),
    };

    let identity = admitted.identity();
    let witnesses = admitted.witnesses();
    let receipt = admitted.receipt();
    let counters = admission_counter_snapshot(&admitted);

    assert_eq!(identity.physical_witness(), authority.physical_witness());
    assert_eq!(
        witnesses.key_scope().key_scope(),
        StoreKeyScope::PageEnvelope
    );
    assert_eq!(
        witnesses.tenant_scope().tenant_scope(),
        StoreTenantScope::TenantPhysicalBoundary
    );
    assert_eq!(
        witnesses.authenticity_scope().requirement(),
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        )
    );
    assert_eq!(
        witnesses.custody_scope().custody_posture(),
        StoreCustodyPosture::InternalStoreCustody
    );
    assert_eq!(counters.requests(), 1);
    assert_eq!(counters.current_authority_checks(), 1);
    assert_eq!(counters.physical_binding_checks(), 1);
    assert_eq!(counters.key_scope_checks(), 1);
    assert_eq!(counters.key_version_checks(), 1);
    assert_eq!(counters.tenant_scope_checks(), 1);
    assert_eq!(counters.authenticity_requirement_checks(), 1);
    assert_eq!(counters.custody_posture_checks(), 1);
    assert_eq!(counters.witnesses_issued(), 4);
    assert_eq!(counters.denials(), 0);
    assert_eq!(receipt.identity(), witnesses.key_scope().identity());
    assert_eq!(receipt.proof_progression_identity(), expected_progression);
    assert_eq!(
        receipt.receipt_id().proof_progression_fingerprint(),
        expected_progression.progression_fingerprint()
    );
}

#[test]
fn explicit_admission_expectation_is_not_hidden_by_new_constructor() {
    let authority = current_authority("store.s51.security.explicit", "page-0002");
    let wrong_key_expectation = StoreSecurityScopeAdmissionExpectation::new(
        StoreKeyScope::BackupExportEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        platform_authenticity_requirement(),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let request = StoreSecurityScopeAdmissionRequest::new(
        &authority,
        StoreKeyScope::PageEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::TenantPhysicalBoundary,
        platform_authenticity_requirement(),
        StoreCustodyPosture::InternalStoreCustody,
        wrong_key_expectation,
    );

    assert!(matches!(
        admit_store_security_scope(request),
        TransitionOutcome::Denied(StoreSecurityScopeAdmissionDenial::WrongKeyScope)
    ));
}

#[test]
fn proof_progression_identity_changes_when_admission_basis_changes() {
    let authority = current_authority("store.s51.security.progression", "page-0003");
    let native_request = platform_request(&authority, StoreKeyVersionPosture::Current);
    let raw_deserialized =
        crate::scope::security_scope_test_support::platform_deserialized_declaration(&authority);
    let readmitted =
        crate::scope::security_scope_test_support::readmit_platform(&authority, raw_deserialized)
            .unwrap();
    let readmitted_request = raw_request(&authority, readmitted);

    assert_ne!(
        native_request
            .basis()
            .proof_progression_identity()
            .progression_fingerprint(),
        readmitted_request
            .basis()
            .proof_progression_identity()
            .progression_fingerprint()
    );
}

#[test]
fn key_version_non_success_topology_stays_distinct() {
    let authority = current_authority("store.s51.security.key_version", "page-0004");

    assert!(matches!(
        admit_store_security_scope(platform_request(&authority, StoreKeyVersionPosture::Stale)),
        TransitionOutcome::Stale(StoreSecurityScopeAdmissionStale::StaleKeyVersionPosture(
            StoreKeyVersionPosture::Stale
        ))
    ));
    assert!(matches!(
        admit_store_security_scope(platform_request(
            &authority,
            StoreKeyVersionPosture::RebindRequired
        )),
        TransitionOutcome::RebindRequired(
            StoreSecurityScopeAdmissionRebindRequired::KeyVersionRebindRequired(
                StoreKeyVersionPosture::RebindRequired
            )
        )
    ));
    assert!(matches!(
        admit_store_security_scope(platform_request(
            &authority,
            StoreKeyVersionPosture::Unsupported
        )),
        TransitionOutcome::Denied(StoreSecurityScopeAdmissionDenial::UnsupportedKeyVersionPosture)
    ));
    assert!(matches!(
        admit_store_security_scope(platform_request(
            &authority,
            StoreKeyVersionPosture::Unavailable
        )),
        TransitionOutcome::Denied(StoreSecurityScopeAdmissionDenial::UnavailableKeyVersionPosture)
    ));
}

#[test]
fn custody_posture_non_success_topology_stays_distinct() {
    let authority = current_authority("store.s51.security.custody", "page-0005");

    assert!(matches!(
        admit_store_security_scope(request_with_custody(
            &authority,
            StoreCustodyPosture::ImportedUnreadmitted
        )),
        TransitionOutcome::Denied(
            StoreSecurityScopeAdmissionDenial::ImportedCustodyRequiresReadmission
        )
    ));
    assert!(matches!(
        admit_store_security_scope(request_with_custody(
            &authority,
            StoreCustodyPosture::ExportedOutOfCustody
        )),
        TransitionOutcome::Denied(
            StoreSecurityScopeAdmissionDenial::ExportedCustodyRequiresReadmission
        )
    ));
    assert!(matches!(
        admit_store_security_scope(request_with_custody(
            &authority,
            StoreCustodyPosture::CustodyUnavailable
        )),
        TransitionOutcome::Denied(StoreSecurityScopeAdmissionDenial::UnavailableCustodyPosture)
    ));
    assert!(matches!(
        admit_store_security_scope(request_with_custody(
            &authority,
            StoreCustodyPosture::CustodyUnsupported
        )),
        TransitionOutcome::Denied(StoreSecurityScopeAdmissionDenial::UnsupportedCustodyPosture)
    ));
}

#[test]
fn wrong_scope_and_replayed_evidence_are_typed_denials() {
    let authority = current_authority("store.s51.security.wrong_scope", "page-0006");
    let wrong_tenant = StoreRawSecurityScopeDeclaration::native(
        authority.physical_witness(),
        StoreKeyScope::PageEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::ImportReadmissionBoundary,
        platform_authenticity_requirement(),
        StoreCustodyPosture::InternalStoreCustody,
    );
    assert!(matches!(
        admit_store_security_scope(raw_request(&authority, wrong_tenant)),
        TransitionOutcome::Denied(StoreSecurityScopeAdmissionDenial::WrongTenantScope)
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
        admit_store_security_scope(raw_request(&authority, replayed)),
        TransitionOutcome::Denied(StoreSecurityScopeAdmissionDenial::ReplayedAdmissionEvidence)
    ));
}

#[test]
fn unsupported_authenticity_and_wrong_key_are_distinct_denials() {
    let authority = current_authority("store.s51.security.unsupported", "page-0007");
    let wrong_key = StoreRawSecurityScopeDeclaration::native(
        authority.physical_witness(),
        StoreKeyScope::BackupExportEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::TenantPhysicalBoundary,
        platform_authenticity_requirement(),
        StoreCustodyPosture::InternalStoreCustody,
    );
    assert!(matches!(
        admit_store_security_scope(raw_request(&authority, wrong_key)),
        TransitionOutcome::Denied(StoreSecurityScopeAdmissionDenial::WrongKeyScope)
    ));

    let unsupported_authenticity = StoreRawSecurityScopeDeclaration::native(
        authority.physical_witness(),
        StoreKeyScope::PageEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::not_required(),
        StoreCustodyPosture::InternalStoreCustody,
    );
    assert!(matches!(
        admit_store_security_scope(raw_request(&authority, unsupported_authenticity)),
        TransitionOutcome::Denied(
            StoreSecurityScopeAdmissionDenial::UnsupportedAuthenticityRequirement
        )
    ));
}
