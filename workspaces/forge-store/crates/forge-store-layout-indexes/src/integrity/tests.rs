use crate::integrity::{layout_corruption, LayoutCorruptionView};
use crate::layout_declarations;
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_recovery_physics::{
    CheckpointManifestMaterialization, CheckpointPageImageMaterialization,
    PersistedRecoveryArtifactMaterialization, RecoveryOfflineVerifier, RecoveryProfileId,
    WalRedoFrameMaterialization,
};

pub(super) fn family() -> crate::PhysicalArtifactFamily {
    layout_declarations().seed_family().family()
}

pub(super) fn admitted_family() -> crate::AdmittedPhysicalArtifactFamily {
    crate::strategy::tests_support::root_manifest_scope().0
}

pub(super) fn admitted_family_for_store(
    store_authority_key: &str,
) -> crate::AdmittedPhysicalArtifactFamily {
    crate::strategy::tests_support::admit_strategy_scope_for_store(
        DurableArtifactFamilyId::PhysicalRootManifest,
        forge_store_security::StoreKeyScope::StoreManagedRoot,
        forge_store_security::StoreTenantScope::StoreInternal,
        forge_store_security::StoreAuthenticityRequirement::not_required(),
        forge_store_security::StoreCustodyPosture::InternalStoreCustody,
        store_authority_key,
    )
    .0
}

pub(super) fn offline_admission(
    seed: &str,
) -> forge_store_recovery_physics::ReopenedRecoveryArtifactAdmission {
    let recovery_profile = RecoveryProfileId::strict_offline_recovery_artifacts();
    let artifacts = PersistedRecoveryArtifactMaterialization::new(
        seed,
        "posix",
        recovery_profile.clone(),
        CheckpointManifestMaterialization::new(
            format!("checkpoint-{seed}"),
            format!("root-{seed}"),
            19,
            "checkpoint",
            1,
            4096,
            1,
            4096,
            1,
        ),
        WalRedoFrameMaterialization::new(
            format!("wal-{seed}"),
            20,
            1,
            format!("sha256:op-{seed}"),
            format!("sha256:idem-{seed}"),
        ),
        CheckpointPageImageMaterialization::new(
            format!("page-{seed}"),
            1,
            7,
            19,
            format!("sha256:page-{seed}"),
        ),
    )
    .materialize()
    .expect("persisted recovery artifacts should materialize");
    let report = RecoveryOfflineVerifier::for_profile(seed, "posix", recovery_profile)
        .verify_persisted_artifacts(&artifacts)
        .expect("offline verifier should admit persisted artifacts");
    forge_store_recovery_physics::ReopenedRecoveryArtifactAdmission::admit(report, &artifacts)
        .expect("reopened admission should succeed")
}

#[test]
fn corruption_assessment_adapts_physical_and_recovery_authority() {
    let record =
        super::readmission_test_support::authoritative_quarantine_record("classification-owner");
    let quarantine = layout_corruption().assess_physical_quarantine(admitted_family(), record);
    assert!(matches!(
        quarantine.view(),
        LayoutCorruptionView::Quarantined(witness) if witness.family() == family()
    ));

    let offline = offline_admission("offline-required");
    let offline = layout_corruption().require_offline_readmission(admitted_family(), &offline);
    assert!(matches!(
        offline.view(),
        LayoutCorruptionView::OfflineReadmissionRequired(requirement)
            if requirement.family() == family()
    ));

    let terminal = layout_corruption().require_import_readmission(
        admitted_family(),
        super::readmission_test_support::import_witness(family(), "terminal-import"),
    );
    assert!(matches!(
        terminal.view(),
        LayoutCorruptionView::ImportReadmissionRequired(requirement)
            if requirement.family() == family()
    ));
}

#[test]
fn corruption_owner_inventory_equals_ordinary_owner_outputs() {
    use std::collections::BTreeSet;

    let record =
        super::readmission_test_support::authoritative_quarantine_record("classification-matrix");
    let quarantine = layout_corruption().assess_physical_quarantine(
        admitted_family_for_store("store.new.strategy"),
        record.clone(),
    );
    let readmission = layout_corruption()
        .require_record_backed_recovery_readmission(
            quarantine,
            &super::readmission_test_support::current_authority(
                "store.new.strategy",
                "classification-matrix",
            ),
            super::readmission_test_support::current_security_scope(
                "store.new.strategy",
                "classification-matrix",
            )
            .witnesses(),
        )
        .unwrap();
    let offline = offline_admission("classification-matrix");

    let observed = [
        layout_corruption()
            .assess_derived_projection(
                crate::LayoutCorruptionClassification::derived_projection_rebuild_to_parity(),
            )
            .case_id(),
        layout_corruption()
            .assess_physical_quarantine(admitted_family(), record)
            .case_id(),
        readmission.case_id(),
        layout_corruption()
            .require_offline_readmission(admitted_family(), &offline)
            .case_id(),
        layout_corruption()
            .require_import_readmission(
                admitted_family(),
                super::readmission_test_support::import_witness(family(), "classification-matrix"),
            )
            .case_id(),
    ];

    assert_eq!(
        crate::integrity::corruption_classification_cases().collect::<BTreeSet<_>>(),
        observed.into_iter().collect()
    );
}

pub(super) fn other_family() -> crate::PhysicalArtifactFamily {
    layout_declarations()
        .declaration(DurableArtifactFamilyId::PublicationSnapshotImage)
        .expect("publication snapshot image family should be declared")
        .family()
}
