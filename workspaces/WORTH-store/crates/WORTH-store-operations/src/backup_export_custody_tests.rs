use worth_store_offline_verifier::OfflineCustodyCapsuleObservation;
use worth_store_security::{
    StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture, StoreRawSecurityScopeDeclaration,
    StoreSecurityScopeAdmissionDenial, StoreSecurityScopeAdmissionExpectation, StoreTenantScope,
    StoreTrustBoundaryCrossing, StoreTrustBoundaryReadmissionTrigger,
};

use crate::backup_export_custody_declaration::backup_capsule_authenticity;
use crate::backup_export_custody_test_support::{current_authority, readmission_trigger};
use crate::{
    BackupExportCapsuleEmission, BackupExportCustodyDeclaration, BackupExportCustodyDenial,
    BackupExportCustodyMode, BackupExportCustodyReadiness,
    BackupExportTerminalProjectionPreparation, BackupImportCustodyReadmission,
};

#[test]
fn backup_pitr_export_and_import_declarations_bind_current_custody_scope() {
    let authority = current_authority("phase7.declarations");

    for mode in [
        BackupExportCustodyMode::Backup,
        BackupExportCustodyMode::PointInTimeRecovery,
        BackupExportCustodyMode::Export,
    ] {
        let declaration = BackupExportCustodyDeclaration::native(
            &authority,
            mode,
            StoreKeyVersionPosture::Current,
        )
        .expect("current declaration should build");
        let raw = declaration.raw_declaration();

        assert_eq!(raw.key_scope(), StoreKeyScope::BackupExportEnvelope);
        assert_eq!(raw.key_version_posture(), StoreKeyVersionPosture::Current);
        assert_eq!(
            raw.authenticity_requirement(),
            Some(backup_capsule_authenticity())
        );
        assert_eq!(raw.tenant_scope(), StoreTenantScope::BackupRestoreBoundary);
        assert_eq!(
            raw.custody_posture(),
            Some(StoreCustodyPosture::ExportPrepared)
        );
        assert_eq!(declaration.counters().declaration_inputs(), 1);
        assert_eq!(declaration.counters().key_version_checks(), 1);
    }
}

#[test]
fn non_current_key_version_blocks_declaration_before_emission() {
    let authority = current_authority("phase7.stale-key");
    let denial = BackupExportCustodyDeclaration::native(
        &authority,
        BackupExportCustodyMode::Backup,
        StoreKeyVersionPosture::Stale,
    )
    .expect_err("stale key posture should deny declaration");

    assert!(matches!(
        denial,
        BackupExportCustodyDenial::NonCurrentKeyVersion {
            posture: StoreKeyVersionPosture::Stale,
            ..
        }
    ));
    if let BackupExportCustodyDenial::NonCurrentKeyVersion { counters, .. } = denial {
        assert_eq!(counters.key_version_stale(), 1);
        assert_eq!(counters.denials(), 1);
    }
}

#[test]
fn capsule_and_terminal_projection_emission_require_admitted_backup_readiness() {
    let authority = current_authority("phase7.emission");
    let admission = admitted_backup_declaration(&authority);
    let custody = BackupExportCustodyReadiness::from_admitted_custody(admission)
        .expect("backup readiness should admit");
    let emission = BackupExportCapsuleEmission::prepare(custody);

    assert_eq!(
        emission.security_scope().key_scope(),
        StoreKeyScope::BackupExportEnvelope
    );
    assert_eq!(
        emission.security_scope().tenant_scope(),
        StoreTenantScope::BackupRestoreBoundary
    );
    assert_eq!(
        emission.security_scope().custody_posture(),
        StoreCustodyPosture::ExportPrepared
    );
    assert_eq!(emission.counters().emissions_prepared(), 1);
    assert_eq!(emission.counters().custody_admitted(), 1);

    let admission = admitted_backup_declaration(&authority);
    let custody = BackupExportCustodyReadiness::from_admitted_custody(admission)
        .expect("backup readiness should admit");
    let terminal = BackupExportTerminalProjectionPreparation::prepare(custody);

    assert_eq!(terminal.counters().terminal_projections_prepared(), 1);
    assert_eq!(terminal.security_scope(), emission.security_scope());
}

#[test]
fn unsupported_key_posture_carries_exact_counter_evidence() {
    let authority = current_authority("phase7.unsupported-key");
    let denial = BackupExportCustodyDeclaration::native(
        &authority,
        BackupExportCustodyMode::Export,
        StoreKeyVersionPosture::Unsupported,
    )
    .expect_err("unsupported key posture should deny declaration");

    if let BackupExportCustodyDenial::NonCurrentKeyVersion { counters, .. } = denial {
        assert_eq!(counters.unsupported_secure_posture(), 1);
        assert_eq!(counters.denials(), 1);
    } else {
        panic!("unsupported key posture should use key-version denial");
    }
}

#[test]
fn import_and_restore_crossings_require_explicit_current_readmission() {
    let authority = current_authority("phase7.readmission");

    for crossing in trust_boundary_crossings() {
        let raw = imported_declaration(&authority);
        let trigger = trigger(crossing, raw, &authority);
        let observation = OfflineCustodyCapsuleObservation::from_deserialized_capsule(raw, trigger)
            .expect("offline verifier should only observe raw capsule metadata");
        let readiness = BackupImportCustodyReadmission::new(observation)
            .readmit_with_current_authority(&authority)
            .expect("current authority should readmit imported custody");
        let custody = BackupExportCustodyReadiness::from_admitted_custody(readiness)
            .expect("readmitted import readiness should admit");

        assert_eq!(custody.custody_posture(), StoreCustodyPosture::Readmitted);
        assert_eq!(
            custody.identity().tenant_scope(),
            StoreTenantScope::ImportReadmissionBoundary
        );
    }
}

#[test]
fn exported_out_of_custody_capsule_readmits_through_backup_import_path() {
    let authority = current_authority("phase7.exported-readmission");
    let raw = exported_declaration(&authority);
    let observation = OfflineCustodyCapsuleObservation::from_deserialized_capsule(
        raw,
        trigger(
            StoreTrustBoundaryCrossing::OfflineExportImport,
            raw,
            &authority,
        ),
    )
    .expect("offline verifier should observe exported raw capsule metadata");
    let readiness = BackupImportCustodyReadmission::new(observation)
        .readmit_with_current_authority(&authority)
        .expect("current authority should readmit exported custody");
    let custody = BackupExportCustodyReadiness::from_admitted_custody(readiness)
        .expect("readmitted exported readiness should admit");

    assert_eq!(custody.custody_posture(), StoreCustodyPosture::Readmitted);
    assert_eq!(
        custody.identity().tenant_scope(),
        StoreTenantScope::ImportReadmissionBoundary
    );
    assert_eq!(custody.counters().trust_boundary_crossings(), 1);
    assert_eq!(custody.counters().readmissions(), 1);
    assert_eq!(custody.counters().custody_admitted(), 1);
}

#[test]
fn imported_capsule_scope_drift_denies_before_readmission() {
    let authority = current_authority("phase7.drift");
    let raw = StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
        authority.physical_witness(),
        StoreKeyScope::BackupExportEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::TenantPhysicalBoundary,
        Some(backup_capsule_authenticity()),
        Some(StoreCustodyPosture::ImportedUnreadmitted),
    );
    let observation = OfflineCustodyCapsuleObservation::from_deserialized_capsule(
        raw,
        trigger(
            StoreTrustBoundaryCrossing::OfflineExportImport,
            raw,
            &authority,
        ),
    )
    .unwrap();
    let denial = BackupImportCustodyReadmission::new(observation)
        .readmit_with_current_authority(&authority)
        .expect_err("wrong tenant boundary should deny readmission");

    assert!(matches!(
        denial,
        BackupExportCustodyDenial::TrustBoundaryReadmissionDenied { .. }
    ));
    if let BackupExportCustodyDenial::TrustBoundaryReadmissionDenied { counters, .. } = denial {
        assert_eq!(counters.denials(), 1);
    }
}

#[test]
fn imported_capsule_missing_authenticity_denies_before_readmission() {
    let authority = current_authority("phase7.missing-authenticity");
    let raw = incomplete_imported_declaration(
        &authority,
        None,
        Some(StoreCustodyPosture::ImportedUnreadmitted),
    );
    let trigger_source = imported_declaration(&authority);
    let observation = OfflineCustodyCapsuleObservation::from_deserialized_capsule(
        raw,
        trigger(
            StoreTrustBoundaryCrossing::OfflineExportImport,
            trigger_source,
            &authority,
        ),
    )
    .expect("offline verifier should observe raw capsule metadata");
    let denial = BackupImportCustodyReadmission::new(observation)
        .readmit_with_current_authority(&authority)
        .expect_err("missing authenticity should deny readmission");

    assert_trust_boundary_denial(
        denial,
        StoreSecurityScopeAdmissionDenial::MissingAuthenticityRequirement,
        0,
    );
}

#[test]
fn imported_capsule_missing_custody_denies_before_readmission() {
    let authority = current_authority("phase7.missing-custody");
    let raw =
        incomplete_imported_declaration(&authority, Some(backup_capsule_authenticity()), None);
    let trigger_source = imported_declaration(&authority);
    let observation = OfflineCustodyCapsuleObservation::from_deserialized_capsule(
        raw,
        trigger(
            StoreTrustBoundaryCrossing::OfflineExportImport,
            trigger_source,
            &authority,
        ),
    )
    .expect("offline verifier should observe raw capsule metadata");
    let denial = BackupImportCustodyReadmission::new(observation)
        .readmit_with_current_authority(&authority)
        .expect_err("missing custody should deny readmission");

    assert_trust_boundary_denial(
        denial,
        StoreSecurityScopeAdmissionDenial::MissingCustodyPosture,
        1,
    );
}

#[test]
fn imported_capsule_rejects_reused_trust_boundary_evidence() {
    let authority = current_authority("phase7.reused-trigger");
    let admitted_raw = imported_declaration(&authority);
    let trigger = trigger(
        StoreTrustBoundaryCrossing::OfflineExportImport,
        admitted_raw,
        &authority,
    );
    let drifted_raw = StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
        authority.physical_witness(),
        StoreKeyScope::BackupExportEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::BackupRestoreBoundary,
        Some(backup_capsule_authenticity()),
        Some(StoreCustodyPosture::ImportedUnreadmitted),
    );
    let observation =
        OfflineCustodyCapsuleObservation::from_deserialized_capsule(drifted_raw, trigger).unwrap();
    let denial = BackupImportCustodyReadmission::new(observation)
        .readmit_with_current_authority(&authority)
        .expect_err("portable trigger evidence should not readmit drifted capsule");

    assert!(matches!(
        denial,
        BackupExportCustodyDenial::TrustBoundaryReadmissionDenied {
            source: StoreSecurityScopeAdmissionDenial::ReplayedAdmissionEvidence,
            ..
        }
    ));
}

fn trust_boundary_crossings() -> [StoreTrustBoundaryCrossing; 7] {
    [
        StoreTrustBoundaryCrossing::DifferentDeployment,
        StoreTrustBoundaryCrossing::DifferentStoreInstance,
        StoreTrustBoundaryCrossing::KeyScopeGenerationChanged,
        StoreTrustBoundaryCrossing::TenantScopeAuthorityChanged,
        StoreTrustBoundaryCrossing::CustodyDomainChanged,
        StoreTrustBoundaryCrossing::OfflineExportImport,
        StoreTrustBoundaryCrossing::BackupRestoreAfterKeyRotation,
    ]
}

fn imported_declaration(
    authority: &worth_store_authority::StoreCurrentAuthorityWitness,
) -> StoreRawSecurityScopeDeclaration {
    StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
        authority.physical_witness(),
        StoreKeyScope::BackupExportEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::ImportReadmissionBoundary,
        Some(backup_capsule_authenticity()),
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
        Some(backup_capsule_authenticity()),
        Some(StoreCustodyPosture::ExportedOutOfCustody),
    )
}

fn incomplete_imported_declaration(
    authority: &worth_store_authority::StoreCurrentAuthorityWitness,
    authenticity: Option<worth_store_security::StoreAuthenticityRequirement>,
    custody: Option<StoreCustodyPosture>,
) -> StoreRawSecurityScopeDeclaration {
    StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
        authority.physical_witness(),
        StoreKeyScope::BackupExportEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::ImportReadmissionBoundary,
        authenticity,
        custody,
    )
}

fn import_expectation() -> StoreSecurityScopeAdmissionExpectation {
    StoreSecurityScopeAdmissionExpectation::new(
        StoreKeyScope::BackupExportEnvelope,
        StoreTenantScope::ImportReadmissionBoundary,
        backup_capsule_authenticity(),
        StoreCustodyPosture::Readmitted,
    )
}

fn trigger(
    crossing: StoreTrustBoundaryCrossing,
    raw: StoreRawSecurityScopeDeclaration,
    authority: &worth_store_authority::StoreCurrentAuthorityWitness,
) -> StoreTrustBoundaryReadmissionTrigger {
    readmission_trigger(crossing, raw, authority, import_expectation())
}

fn admitted_backup_declaration(
    authority: &worth_store_authority::StoreCurrentAuthorityWitness,
) -> crate::BackupExportCustodyAdmission {
    BackupExportCustodyDeclaration::native(
        authority,
        BackupExportCustodyMode::Backup,
        StoreKeyVersionPosture::Current,
    )
    .unwrap()
    .admit_with_current_authority(authority)
    .expect("backup declaration should admit")
}

fn assert_trust_boundary_denial(
    denial: BackupExportCustodyDenial,
    expected_source: StoreSecurityScopeAdmissionDenial,
    expected_custody_denials: u64,
) {
    if let BackupExportCustodyDenial::TrustBoundaryReadmissionDenied { source, counters } = denial {
        assert_eq!(source, expected_source);
        assert_eq!(counters.trust_boundary_crossings(), 1);
        assert_eq!(counters.denials(), 1);
        assert_eq!(counters.readmissions(), 0);
        assert_eq!(counters.custody_admitted(), 0);
        assert_eq!(counters.custody_denied(), expected_custody_denials);
    } else {
        panic!("missing capsule metadata should deny in trust-boundary readmission");
    }
}
