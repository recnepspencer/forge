use worth_store_contracts::DurableArtifactFamilyId;
use worth_store_offline_verifier::OfflineCustodyCapsuleObservation;
use worth_store_security::{StoreTenantScope, StoreTrustBoundaryCrossing};

use crate::backup::export::{backup_capsule_authenticity, current_authority, readmission_trigger};
use crate::{
    BackupExportCustodyDeclaration, BackupExportCustodyMode, BackupExportCustodyReadiness,
    BackupExportTerminalProjectionPreparation, BackupImportCustodyReadmission,
    BackupLayoutEvidenceReport,
};

#[test]
fn backup_and_restore_layout_reports_preserve_terminal_and_readmission_required_posture() {
    let authority = current_authority("operations.layout-projection");
    let admission = BackupExportCustodyDeclaration::native(
        &authority,
        BackupExportCustodyMode::Backup,
        worth_store_security::StoreKeyVersionPosture::Current,
    )
    .expect("declaration")
    .admit_with_current_authority(&authority)
    .expect("admission");
    let backup_preparation = BackupExportTerminalProjectionPreparation::prepare(
        BackupExportCustodyReadiness::from_admitted_custody(admission).expect("readiness"),
    );
    let backup = BackupLayoutEvidenceReport::from_terminal_projection(&backup_preparation);
    assert_eq!(backup.family_id(), DurableArtifactFamilyId::ExportBundle);
    assert!(backup.cannot_be_foreground_authority());

    let raw = worth_store_security::StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
        authority.physical_witness(),
        worth_store_security::StoreKeyScope::BackupExportEnvelope,
        worth_store_security::StoreKeyVersionPosture::Current,
        StoreTenantScope::ImportReadmissionBoundary,
        Some(backup_capsule_authenticity()),
        Some(worth_store_security::StoreCustodyPosture::ImportedUnreadmitted),
    );
    let observation = OfflineCustodyCapsuleObservation::from_deserialized_capsule(
        raw,
        readmission_trigger(
            StoreTrustBoundaryCrossing::OfflineExportImport,
            raw,
            &authority,
            worth_store_security::StoreSecurityScopeAdmissionExpectation::new(
                worth_store_security::StoreKeyScope::BackupExportEnvelope,
                StoreTenantScope::ImportReadmissionBoundary,
                backup_capsule_authenticity(),
                worth_store_security::StoreCustodyPosture::Readmitted,
            ),
        ),
    )
    .expect("offline observation");
    let restore =
        BackupImportCustodyReadmission::new(observation).project_restore_evidence_layout();
    assert_eq!(restore.family_id(), DurableArtifactFamilyId::ImportBundle);
    assert!(restore.requires_explicit_readmission());

    let rotation_raw =
        worth_store_security::StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
            authority.physical_witness(),
            worth_store_security::StoreKeyScope::BackupExportEnvelope,
            worth_store_security::StoreKeyVersionPosture::Current,
            StoreTenantScope::ImportReadmissionBoundary,
            Some(backup_capsule_authenticity()),
            Some(worth_store_security::StoreCustodyPosture::ImportedUnreadmitted),
        );
    let rotation_observation = OfflineCustodyCapsuleObservation::from_deserialized_capsule(
        rotation_raw,
        readmission_trigger(
            StoreTrustBoundaryCrossing::BackupRestoreAfterKeyRotation,
            rotation_raw,
            &authority,
            worth_store_security::StoreSecurityScopeAdmissionExpectation::new(
                worth_store_security::StoreKeyScope::BackupExportEnvelope,
                StoreTenantScope::ImportReadmissionBoundary,
                backup_capsule_authenticity(),
                worth_store_security::StoreCustodyPosture::Readmitted,
            ),
        ),
    )
    .expect("key-rotation observation");
    let restore_after_key_rotation =
        BackupImportCustodyReadmission::new(rotation_observation).project_restore_evidence_layout();
    assert!(restore_after_key_rotation.requires_explicit_readmission());
    assert_eq!(
        restore_after_key_rotation.readmission_trigger().crossing(),
        StoreTrustBoundaryCrossing::BackupRestoreAfterKeyRotation
    );
}
