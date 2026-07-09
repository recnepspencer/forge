use crate::security_scope_test_support::{
    boundary_fact, current_authority, platform_authenticity_requirement,
    trust_boundary_readmission_trigger,
};
use crate::{
    readmit_trust_boundary_security_scope_declaration, store_deployment_boundary_fact,
    StoreCustodyPosture, StoreDifferentDeploymentBoundaryEvidence, StoreKeyScope,
    StoreKeyVersionPosture, StoreRawSecurityScopeDeclaration, StoreSecurityScopeAdmissionDenial,
    StoreSecurityScopeAdmissionExpectation, StoreTenantScope, StoreTrustBoundaryCrossing,
    StoreTrustBoundaryEvidenceDenial, StoreTrustBoundaryReadmissionTrigger,
};

#[test]
fn trust_boundary_readmission_transitions_imported_custody_to_readmitted() {
    let authority = current_authority("store.s51.custody.readmit", "capsule-0001");
    let declaration = imported_declaration(&authority);
    let readmitted = readmit_trust_boundary_security_scope_declaration(
        &authority,
        declaration,
        StoreKeyVersionPosture::Current,
        expectation(),
        trigger(
            StoreTrustBoundaryCrossing::OfflineExportImport,
            declaration,
            &authority,
            expectation(),
        ),
    )
    .expect("imported custody should readmit");

    assert_eq!(
        readmitted.custody_posture(),
        Some(StoreCustodyPosture::Readmitted)
    );
    assert_eq!(
        readmitted.provenance(),
        crate::StoreSecurityScopeDeclarationProvenance::StoreReadmitted
    );
}

#[test]
fn trust_boundary_readmission_transitions_exported_custody_to_readmitted() {
    let authority = current_authority("store.s51.custody.exported", "capsule-0001");
    let declaration = exported_declaration(&authority);
    let readmitted = readmit_trust_boundary_security_scope_declaration(
        &authority,
        declaration,
        StoreKeyVersionPosture::Current,
        expectation(),
        trigger(
            StoreTrustBoundaryCrossing::OfflineExportImport,
            declaration,
            &authority,
            expectation(),
        ),
    )
    .expect("exported custody should readmit");

    assert_eq!(
        readmitted.custody_posture(),
        Some(StoreCustodyPosture::Readmitted)
    );
}

#[test]
fn trust_boundary_readmission_rejects_stale_key_wrong_tenant_and_non_import_custody() {
    let authority = current_authority("store.s51.custody.denials", "capsule-0002");
    let declaration = imported_declaration(&authority);

    assert_eq!(
        readmit_trust_boundary_security_scope_declaration(
            &authority,
            stale_declaration(&authority),
            StoreKeyVersionPosture::Current,
            expectation(),
            trigger(
                StoreTrustBoundaryCrossing::BackupRestoreAfterKeyRotation,
                stale_declaration(&authority),
                &authority,
                expectation(),
            ),
        ),
        Err(StoreSecurityScopeAdmissionDenial::DeniedKeyVersionPosture)
    );
    let wrong_tenant_expectation = StoreSecurityScopeAdmissionExpectation::new(
        StoreKeyScope::BackupExportEnvelope,
        StoreTenantScope::BackupRestoreBoundary,
        platform_authenticity_requirement(),
        StoreCustodyPosture::Readmitted,
    );
    assert_eq!(
        readmit_trust_boundary_security_scope_declaration(
            &authority,
            declaration,
            StoreKeyVersionPosture::Current,
            wrong_tenant_expectation,
            trigger(
                StoreTrustBoundaryCrossing::TenantScopeAuthorityChanged,
                declaration,
                &authority,
                wrong_tenant_expectation,
            ),
        ),
        Err(StoreSecurityScopeAdmissionDenial::WrongTenantScope)
    );
    let wrong_custody = StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
        authority.physical_witness(),
        StoreKeyScope::BackupExportEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::ImportReadmissionBoundary,
        Some(platform_authenticity_requirement()),
        Some(StoreCustodyPosture::InternalStoreCustody),
    );
    assert_eq!(
        readmit_trust_boundary_security_scope_declaration(
            &authority,
            wrong_custody,
            StoreKeyVersionPosture::Current,
            expectation(),
            trigger(
                StoreTrustBoundaryCrossing::CustodyDomainChanged,
                wrong_custody,
                &authority,
                expectation(),
            ),
        ),
        Err(StoreSecurityScopeAdmissionDenial::WrongCustodyPosture)
    );
}

#[test]
fn trust_boundary_trigger_cannot_be_reused_for_different_raw_capsule() {
    let authority = current_authority("store.s51.custody.reused-trigger", "capsule-0003");
    let declaration = imported_declaration(&authority);
    let replayed_for_drifted_scope = trigger(
        StoreTrustBoundaryCrossing::OfflineExportImport,
        declaration,
        &authority,
        expectation(),
    );
    let drifted = StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
        authority.physical_witness(),
        StoreKeyScope::BackupExportEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::BackupRestoreBoundary,
        Some(platform_authenticity_requirement()),
        Some(StoreCustodyPosture::ImportedUnreadmitted),
    );

    assert_eq!(
        readmit_trust_boundary_security_scope_declaration(
            &authority,
            drifted,
            StoreKeyVersionPosture::Current,
            expectation(),
            replayed_for_drifted_scope,
        ),
        Err(StoreSecurityScopeAdmissionDenial::ReplayedAdmissionEvidence)
    );
}

#[test]
fn trust_boundary_crossing_evidence_cannot_be_reused_as_another_crossing_kind() {
    let authority = current_authority("store.s51.custody.crossing-kind", "capsule-0004");
    let declaration = imported_declaration(&authority);
    let offline_import = trigger(
        StoreTrustBoundaryCrossing::OfflineExportImport,
        declaration,
        &authority,
        expectation(),
    );
    let backup_restore = trigger(
        StoreTrustBoundaryCrossing::BackupRestoreAfterKeyRotation,
        declaration,
        &authority,
        expectation(),
    );

    assert_eq!(offline_import.evidence(), backup_restore.evidence());
    assert_ne!(
        offline_import.crossing_evidence(),
        backup_restore.crossing_evidence()
    );
    assert_eq!(
        backup_restore.crossing(),
        StoreTrustBoundaryCrossing::BackupRestoreAfterKeyRotation
    );
}

#[test]
fn trust_boundary_category_evidence_rejects_missing_category_delta() {
    let authority = current_authority("store.s51.custody.same-category", "same");
    let fact =
        store_deployment_boundary_fact(boundary_fact("store.trust_boundary.deployment", "same"))
            .expect("deployment category fact should admit");

    assert_eq!(fact.physical_witness(), authority.physical_witness());
    assert_eq!(
        StoreDifferentDeploymentBoundaryEvidence::from_category_facts(fact.clone(), fact),
        Err(StoreTrustBoundaryEvidenceDenial::MissingCategoryBoundaryChange)
    );
}

#[test]
fn trust_boundary_category_fact_rejects_wrong_store_aspect_category() {
    assert_eq!(
        store_deployment_boundary_fact(boundary_fact(
            "store.trust_boundary.backup_restore",
            "restored",
        )),
        Err(StoreTrustBoundaryEvidenceDenial::WrongTrustBoundaryCategory)
    );
}

fn imported_declaration(
    authority: &worth_store_authority::StoreCurrentAuthorityWitness,
) -> StoreRawSecurityScopeDeclaration {
    StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
        authority.physical_witness(),
        StoreKeyScope::BackupExportEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::ImportReadmissionBoundary,
        Some(platform_authenticity_requirement()),
        Some(StoreCustodyPosture::ImportedUnreadmitted),
    )
}

fn exported_declaration(
    authority: &worth_store_authority::StoreCurrentAuthorityWitness,
) -> StoreRawSecurityScopeDeclaration {
    StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
        authority.physical_witness(),
        StoreKeyScope::BackupExportEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::ImportReadmissionBoundary,
        Some(platform_authenticity_requirement()),
        Some(StoreCustodyPosture::ExportedOutOfCustody),
    )
}

fn stale_declaration(
    authority: &worth_store_authority::StoreCurrentAuthorityWitness,
) -> StoreRawSecurityScopeDeclaration {
    StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
        authority.physical_witness(),
        StoreKeyScope::BackupExportEnvelope,
        StoreKeyVersionPosture::Stale,
        StoreTenantScope::ImportReadmissionBoundary,
        Some(platform_authenticity_requirement()),
        Some(StoreCustodyPosture::ImportedUnreadmitted),
    )
}

fn expectation() -> StoreSecurityScopeAdmissionExpectation {
    StoreSecurityScopeAdmissionExpectation::new(
        StoreKeyScope::BackupExportEnvelope,
        StoreTenantScope::ImportReadmissionBoundary,
        platform_authenticity_requirement(),
        StoreCustodyPosture::Readmitted,
    )
}

fn trigger(
    crossing: StoreTrustBoundaryCrossing,
    declaration: StoreRawSecurityScopeDeclaration,
    authority: &worth_store_authority::StoreCurrentAuthorityWitness,
    expectation: StoreSecurityScopeAdmissionExpectation,
) -> StoreTrustBoundaryReadmissionTrigger {
    trust_boundary_readmission_trigger(crossing, declaration, authority, expectation)
}
