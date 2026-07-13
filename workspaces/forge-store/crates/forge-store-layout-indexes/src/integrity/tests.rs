use crate::integrity::{layout_corruption, LayoutCorruptionInput, LayoutCorruptionView};
use crate::materialization::{LayoutCoverageWitness, LayoutMaterializationState};
use crate::{layout_declarations, LayoutCorruptionClassification};
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_physical_format::PhysicalEpoch;
use forge_store_recovery_physics::{
    CheckpointManifestMaterialization, CheckpointPageImageMaterialization,
    PersistedRecoveryArtifactMaterialization, RecoveryOfflineVerifier, RecoveryProfileId,
    WalRedoFrameMaterialization,
};

pub(super) fn family() -> crate::PhysicalArtifactFamily {
    layout_declarations().seed_family().family()
}

fn absent_coverage() -> LayoutCoverageWitness {
    crate::materialization::test_support::materialization_observations()
        .exact_root_epoch_coverage(
            LayoutMaterializationState::absent(family()),
            PhysicalEpoch::from_raw(7).unwrap(),
        )
        .expect("absent coverage should admit")
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
        layout_corruption().classify(LayoutCorruptionInput::Materialization(absent_coverage())).view(),
        LayoutCorruptionView::NotFound(actual_family) if *actual_family == family()
    ));

    let stale = crate::facade::access_planning()
        .stale_root_epoch_coverage(
            layout_declarations().seed_family(),
            PhysicalEpoch::from_raw(9).unwrap(),
        )
        .unwrap();
    assert!(matches!(
        layout_corruption()
            .classify(LayoutCorruptionInput::Materialization(stale))
            .view(),
        LayoutCorruptionView::StaleBinding(_)
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
        layout_corruption()
            .classify(LayoutCorruptionInput::Materialization(quarantined))
            .view(),
        LayoutCorruptionView::Quarantined(_)
    ));
}

#[test]
fn rebuild_classification_keeps_derived_and_authoritative_corruption_distinct() {
    assert!(matches!(
        layout_corruption()
            .classify(LayoutCorruptionInput::RebuildClassification(
                LayoutCorruptionClassification::DerivedProjectionRebuildToParity
            ))
            .view(),
        LayoutCorruptionView::RebuildRequired(_)
    ));

    assert!(matches!(
        layout_corruption()
            .classify(LayoutCorruptionInput::RebuildClassification(
                LayoutCorruptionClassification::AuthoritativeSourceQuarantineRequired {
                    family: family()
                }
            ))
            .view(),
        LayoutCorruptionView::Quarantined(_)
    ));
}

#[test]
fn terminal_and_offline_inputs_require_store_readmission_on_distinct_lanes() {
    let offline = layout_corruption().classify(LayoutCorruptionInput::OfflineEvidence {
        family: family(),
        admission: offline_admission("offline-required"),
    });
    let terminal = layout_corruption().classify(LayoutCorruptionInput::TerminalImport {
        witness: super::readmission_tests::import_witness(family(), "terminal-import"),
    });

    assert!(matches!(
        offline.view(),
        LayoutCorruptionView::OfflineReadmissionRequired(requirement)
            if requirement.family() == family()
    ));
    assert!(matches!(
        terminal.view(),
        LayoutCorruptionView::ImportReadmissionRequired(requirement)
            if requirement.family() == family()
    ));
}

#[test]
fn corruption_classification_declares_exactly_the_cases_owner_operations_emit() {
    use std::collections::BTreeSet;

    let coverage = |state| {
        crate::materialization::test_support::materialization_observations()
            .exact_root_epoch_coverage(state, PhysicalEpoch::from_raw(7).unwrap())
            .unwrap()
    };
    let stale = crate::facade::access_planning()
        .stale_root_epoch_coverage(
            layout_declarations().seed_family(),
            PhysicalEpoch::from_raw(9).unwrap(),
        )
        .unwrap();
    let quarantine_record =
        super::readmission_test_support::authoritative_quarantine_record("classification-matrix");
    let quarantined =
        layout_corruption().classify(LayoutCorruptionInput::AuthoritativeQuarantine {
            family: family(),
            record: quarantine_record.clone(),
        });
    let quarantine_required = layout_corruption()
        .require_record_backed_recovery_readmission(
            layout_corruption().classify(LayoutCorruptionInput::AuthoritativeQuarantine {
                family: family(),
                record: quarantine_record,
            }),
            &super::readmission_test_support::current_authority(
                "store.new.corruption",
                "classification-matrix",
            ),
        )
        .unwrap();

    let observed = [
        layout_corruption()
            .classify(LayoutCorruptionInput::Materialization(coverage(
                LayoutMaterializationState::exact(family()),
            )))
            .case_id(),
        layout_corruption()
            .classify(LayoutCorruptionInput::Materialization(absent_coverage()))
            .case_id(),
        layout_corruption()
            .classify(LayoutCorruptionInput::Materialization(coverage(
                LayoutMaterializationState::declared_only(family()),
            )))
            .case_id(),
        layout_corruption()
            .classify(LayoutCorruptionInput::Materialization(stale))
            .case_id(),
        layout_corruption()
            .classify(LayoutCorruptionInput::RebuildClassification(
                LayoutCorruptionClassification::DerivedProjectionRebuildToParity,
            ))
            .case_id(),
        quarantined.case_id(),
        quarantine_required.case_id(),
        layout_corruption()
            .classify(LayoutCorruptionInput::OfflineEvidence {
                family: family(),
                admission: offline_admission("classification-matrix"),
            })
            .case_id(),
        layout_corruption()
            .classify(LayoutCorruptionInput::TerminalImport {
                witness: super::readmission_tests::import_witness(
                    family(),
                    "classification-matrix",
                ),
            })
            .case_id(),
        layout_corruption()
            .classify(LayoutCorruptionInput::MigrationRequired { family: family() })
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
