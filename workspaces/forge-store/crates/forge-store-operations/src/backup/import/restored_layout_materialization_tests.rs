use forge_proof::TransitionOutcome;
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_layout_indexes::declarations::{
    layout_declarations, AdmittedPhysicalArtifactFamily,
};
use forge_store_layout_indexes::materialization::{
    LayoutMaterializationSourceKind, MaterializationDenial,
};
use forge_store_offline_verifier::OfflineCustodyCapsuleObservation;
use forge_store_recovery_physics::{
    CheckpointManifestMaterialization, CheckpointPageImageMaterialization,
    PersistedRecoveryArtifactMaterialization, RecoveryOfflineVerifier, RecoveryProfileId,
    WalRedoFrameMaterialization,
};
use forge_store_security::{
    admit_store_security_scope, StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreRawSecurityScopeDeclaration, StoreSecurityScopeAdmissionExpectation,
    StoreSecurityScopeAdmissionRequest, StoreTenantScope, StoreTrustBoundaryCrossing,
};
use std::collections::BTreeSet;

use super::admit_restored_layout_materialization;
use crate::backup::export::{backup_capsule_authenticity, current_authority, readmission_trigger};
use crate::{
    BackupExportCustodyDeclaration, BackupExportCustodyMode, BackupImportCustodyReadmission,
    RestoredLayoutMaterializationView,
};

#[test]
fn verified_restore_content_and_current_custody_issue_exact_materialization() {
    let authority = current_authority("restore.materialization.current");
    let family = admitted_page_family(&authority);
    let physical_family = family.declaration().family();
    let custody = readmitted_custody(&authority);
    let catalog = forge_store_test_support::harness::layout::admitted_layout_bootstrap_catalog();

    let first = admit_restored_layout_materialization(
        physical_family,
        family,
        &catalog,
        &reopened_artifact("restore-a"),
        &custody,
    )
    .into_materialized()
    .expect("verified restored content should materialize");
    let second = admit_restored_layout_materialization(
        physical_family,
        family,
        &catalog,
        &reopened_artifact("restore-b"),
        &custody,
    )
    .into_materialized()
    .expect("second verified restored content should materialize");

    assert!(first.coverage().is_exact());
    let LayoutMaterializationSourceKind::RestoredArtifact(first_identity) = first.source().kind()
    else {
        panic!("restore must retain restored-artifact source identity")
    };
    let LayoutMaterializationSourceKind::RestoredArtifact(second_identity) = second.source().kind()
    else {
        panic!("restore must retain restored-artifact source identity")
    };
    assert_ne!(first_identity, second_identity);
}

#[test]
fn restore_rejects_target_family_admitted_by_another_store_authority() {
    let custody_authority = current_authority("restore.materialization.custody-store");
    let target_authority = current_authority("restore.materialization.target-store");
    let family = admitted_page_family(&target_authority);
    let catalog = forge_store_test_support::harness::layout::admitted_layout_bootstrap_catalog();

    let denial = admit_restored_layout_materialization(
        family.declaration().family(),
        family,
        &catalog,
        &reopened_artifact("restore-cross-store"),
        &readmitted_custody(&custody_authority),
    );

    assert!(matches!(
        denial.view(),
        RestoredLayoutMaterializationView::MaterializationDenied(
            MaterializationDenial::RestoreCurrentStoreAuthorityRequired
        )
    ));
}

#[test]
fn restore_declares_exactly_the_cases_ordinary_admission_emits() {
    let authority = current_authority("restore.materialization.case-coverage");
    let other_authority = current_authority("restore.materialization.case-coverage.other");
    let family = admitted_page_family(&authority);
    let physical_family = family.declaration().family();
    let catalog = forge_store_test_support::harness::layout::admitted_layout_bootstrap_catalog();
    let reopened = reopened_artifact("restore-case-coverage");

    let materialized = admit_restored_layout_materialization(
        physical_family,
        family,
        &catalog,
        &reopened,
        &readmitted_custody(&authority),
    );
    let custody_denied = admit_restored_layout_materialization(
        physical_family,
        family,
        &catalog,
        &reopened,
        &outbound_custody(&authority),
    );
    let materialization_denied = admit_restored_layout_materialization(
        physical_family,
        family,
        &catalog,
        &reopened,
        &readmitted_custody(&other_authority),
    );

    assert_eq!(
        [materialized, custody_denied, materialization_denied]
            .into_iter()
            .map(|outcome| outcome.case_id())
            .collect::<BTreeSet<_>>(),
        crate::restored_layout_materialization_cases().collect::<BTreeSet<_>>()
    );
}

fn admitted_page_family(
    authority: &forge_store_authority::StoreCurrentAuthorityWitness,
) -> AdmittedPhysicalArtifactFamily {
    let request = StoreSecurityScopeAdmissionRequest::platform_page_envelope(
        authority,
        StoreKeyVersionPosture::Current,
        StoreCustodyPosture::InternalStoreCustody,
    );
    let scope = match admit_store_security_scope(request) {
        TransitionOutcome::Success(scope) => scope,
        outcome => panic!("target page security scope should admit: {outcome:?}"),
    };
    let declaration = layout_declarations()
        .declaration(DurableArtifactFamilyId::PhysicalPage)
        .unwrap();
    layout_declarations()
        .admit_physical_artifact_family(declaration, scope.witnesses())
        .unwrap()
}

fn readmitted_custody(
    authority: &forge_store_authority::StoreCurrentAuthorityWitness,
) -> crate::BackupExportCustodyAdmission {
    let expectation = StoreSecurityScopeAdmissionExpectation::new(
        StoreKeyScope::BackupExportEnvelope,
        StoreTenantScope::ImportReadmissionBoundary,
        backup_capsule_authenticity(),
        StoreCustodyPosture::Readmitted,
    );
    let raw = StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
        authority.physical_witness(),
        StoreKeyScope::BackupExportEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::ImportReadmissionBoundary,
        Some(backup_capsule_authenticity()),
        Some(StoreCustodyPosture::ImportedUnreadmitted),
    );
    let trigger = readmission_trigger(
        StoreTrustBoundaryCrossing::OfflineExportImport,
        raw,
        authority,
        expectation,
    );
    let observation = OfflineCustodyCapsuleObservation::from_deserialized_capsule(raw, trigger)
        .expect("offline custody observation should remain non-authoritative");
    BackupImportCustodyReadmission::new(observation)
        .readmit_with_current_authority(authority)
        .expect("current custody should readmit")
}

fn outbound_custody(
    authority: &forge_store_authority::StoreCurrentAuthorityWitness,
) -> crate::BackupExportCustodyAdmission {
    BackupExportCustodyDeclaration::native(
        authority,
        BackupExportCustodyMode::Backup,
        StoreKeyVersionPosture::Current,
    )
    .unwrap()
    .admit_with_current_authority(authority)
    .unwrap()
}

fn reopened_artifact(
    seed: &str,
) -> forge_store_recovery_physics::ReopenedRecoveryArtifactAdmission {
    let recovery_profile = RecoveryProfileId::strict_offline_recovery_artifacts();
    let artifacts = PersistedRecoveryArtifactMaterialization::new(
        seed,
        "posix",
        recovery_profile.clone(),
        CheckpointManifestMaterialization::new(
            &format!("checkpoint-{seed}"),
            &format!("root-{seed}"),
            19,
            "checkpoint",
            1,
            4096,
            1,
            4096,
            1,
        ),
        WalRedoFrameMaterialization::new(
            &format!("wal-{seed}"),
            20,
            1,
            &format!("sha256:op-{seed}"),
            &format!("sha256:idem-{seed}"),
        ),
        CheckpointPageImageMaterialization::new(
            &format!("page-{seed}"),
            1,
            7,
            19,
            &format!("sha256:page-{seed}"),
        ),
    )
    .materialize()
    .unwrap();
    let report = RecoveryOfflineVerifier::for_profile(seed, "posix", recovery_profile)
        .verify_persisted_artifacts(&artifacts)
        .unwrap();
    forge_store_recovery_physics::ReopenedRecoveryArtifactAdmission::admit(report, &artifacts)
        .unwrap()
}
