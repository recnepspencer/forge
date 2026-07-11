use crate::corruption::{layout_corruption, S8LayoutCorruptionInput, S8LayoutCorruptionView};
use crate::materialization::{S8LayoutCoverageWitness, S8LayoutMaterializationState};
use crate::{layout_declarations, LayoutCorruptionClassification, S8PhysicalCoverageBasis};
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_physical_format::PhysicalEpoch;
use forge_store_recovery_physics::{
    RecoveryOfflineVerifier, RecoveryProfileId, S4CheckpointManifestMaterialization,
    S4CheckpointPageImageMaterialization, S4PersistedRecoveryArtifactMaterialization,
    S4WalRedoFrameMaterialization,
};

pub(super) fn family() -> crate::PhysicalArtifactFamily {
    layout_declarations().seed_family().family()
}

fn absent_coverage() -> S8LayoutCoverageWitness {
    S8LayoutCoverageWitness::exact_through(
        S8LayoutMaterializationState::absent(family()),
        S8PhysicalCoverageBasis::root_epoch(PhysicalEpoch::from_raw(7).unwrap()).watermark(),
    )
    .expect("absent coverage should admit")
}

pub(super) fn offline_admission(
    seed: &str,
) -> forge_store_recovery_physics::ReopenedRecoveryArtifactAdmission {
    let recovery_profile = RecoveryProfileId::strict_s4();
    let artifacts = S4PersistedRecoveryArtifactMaterialization::new(
        seed,
        "posix",
        recovery_profile.clone(),
        S4CheckpointManifestMaterialization::new(
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
        S4WalRedoFrameMaterialization::new(
            &format!("wal-{seed}"),
            20,
            1,
            &format!("sha256:op-{seed}"),
            &format!("sha256:idem-{seed}"),
        ),
        S4CheckpointPageImageMaterialization::new(
            &format!("page-{seed}"),
            1,
            7,
            19,
            &format!("sha256:page-{seed}"),
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
fn materialization_states_keep_not_found_stale_and_quarantine_distinct() {
    assert!(matches!(
        layout_corruption().classify(S8LayoutCorruptionInput::Materialization(absent_coverage())).view(),
        S8LayoutCorruptionView::NotFound(actual_family) if *actual_family == family()
    ));

    let stale = crate::facade::access_planning()
        .stale_root_epoch_coverage(
            layout_declarations().seed_family(),
            PhysicalEpoch::from_raw(9).unwrap(),
        )
        .unwrap();
    assert!(matches!(
        layout_corruption().classify(S8LayoutCorruptionInput::Materialization(stale)).view(),
        S8LayoutCorruptionView::StaleBinding(_)
    ));

    let quarantined = crate::facade::access_planning()
        .quarantined_wal_lsn_coverage(
            layout_declarations().seed_family(),
            forge_store_recovery_physics::LogSequenceNumber::new(21),
            forge_store_recovery_physics::LogSequenceNumber::new(24),
            forge_store_recovery_physics::CheckpointCoveredLsnRange::new(
                forge_store_recovery_physics::LogSequenceNumber::new(22),
                forge_store_recovery_physics::LogSequenceNumber::new(23),
            )
            .unwrap(),
        )
        .unwrap();
    assert!(matches!(
        layout_corruption().classify(S8LayoutCorruptionInput::Materialization(quarantined)).view(),
        S8LayoutCorruptionView::Quarantined(_)
    ));
}

#[test]
fn rebuild_classification_keeps_derived_and_authoritative_corruption_distinct() {
    assert!(matches!(
        layout_corruption().classify(S8LayoutCorruptionInput::RebuildClassification(
            LayoutCorruptionClassification::DerivedProjectionRebuildToParity
        )).view(),
        S8LayoutCorruptionView::RebuildRequired(_)
    ));

    assert!(matches!(
        layout_corruption().classify(S8LayoutCorruptionInput::RebuildClassification(
            LayoutCorruptionClassification::AuthoritativeSourceQuarantineRequired {
                family: family()
            }
        )).view(),
        S8LayoutCorruptionView::Quarantined(_)
    ));
}

#[test]
fn terminal_and_offline_inputs_require_store_readmission_on_distinct_lanes() {
    let offline = layout_corruption().classify(S8LayoutCorruptionInput::OfflineEvidence {
        family: family(),
        admission: offline_admission("offline-required"),
    });
    let terminal = layout_corruption().classify(S8LayoutCorruptionInput::TerminalImport {
        witness: super::readmission_tests::import_witness(family(), "terminal-import"),
    });

    assert!(matches!(
        offline.view(),
        S8LayoutCorruptionView::OfflineReadmissionRequired(requirement)
            if requirement.family() == family()
    ));
    assert!(matches!(
        terminal.view(),
        S8LayoutCorruptionView::ImportReadmissionRequired(requirement)
            if requirement.family() == family()
    ));
}

pub(super) fn other_family() -> crate::PhysicalArtifactFamily {
    layout_declarations()
        .declaration(DurableArtifactFamilyId::PublicationSnapshotImage)
        .expect("publication snapshot image family should be declared")
        .family()
}
