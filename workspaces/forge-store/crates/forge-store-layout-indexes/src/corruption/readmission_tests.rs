pub(super) use super::readmission_test_support::import_witness;
use super::readmission_test_support::{
    authoritative_quarantine_record, current_authority, offline_witness, quarantine_witness,
    record_backed_witness,
};
use super::tests::{family, offline_admission, other_family};
use crate::corruption::{
    layout_corruption, S8CorruptionDenial, S8LayoutCorruptionInput, S8LayoutCorruptionOutcome,
    S8LayoutReadmissionSource,
};

#[test]
fn offline_readmission_resumes_foreground_authority_with_family_bound_store_witness() {
    let required = layout_corruption().classify(S8LayoutCorruptionInput::OfflineEvidence {
        family: family(),
        admission: offline_admission("offline-success"),
    });
    let outcome = layout_corruption().readmit_with(
        required,
        crate::corruption::S8NativeReadmissionInput::RecoveryWitness {
            witness: offline_witness(family(), "offline-success"),
        },
    );

    assert!(matches!(
        outcome,
        crate::corruption::S8LayoutReadmissionOutcome::Readmitted(witness)
            if witness.family() == family()
                && witness.source() == S8LayoutReadmissionSource::OfflineRecoveryEvidence
    ));
}

#[test]
fn quarantine_readmission_resumes_foreground_authority_with_family_bound_store_witness() {
    let quarantine_record = authoritative_quarantine_record("quarantine-success");
    let required = layout_corruption()
        .require_record_backed_recovery_readmission(
            layout_corruption().classify(S8LayoutCorruptionInput::AuthoritativeQuarantine {
                family: family(),
                record: quarantine_record.clone(),
            }),
            &current_authority("store.s8.corruption", "quarantine-success"),
        )
        .expect("record-backed quarantine should derive readmission requirement");
    let outcome = layout_corruption().readmit_with(
        required,
        crate::corruption::S8NativeReadmissionInput::RecoveryWitness {
            witness: record_backed_witness(family(), &quarantine_record, "quarantine-success"),
        },
    );

    assert!(matches!(
        outcome,
        crate::corruption::S8LayoutReadmissionOutcome::Readmitted(witness)
            if witness.family() == family()
                && witness.source() == S8LayoutReadmissionSource::QuarantineRecovery
    ));
}

#[test]
fn quarantine_readmission_rejects_witness_for_different_family_or_artifact_identity() {
    let quarantine_record = authoritative_quarantine_record("quarantine-required-a");
    let required = layout_corruption()
        .require_record_backed_recovery_readmission(
            layout_corruption().classify(S8LayoutCorruptionInput::AuthoritativeQuarantine {
                family: family(),
                record: quarantine_record.clone(),
            }),
            &current_authority("store.s8.corruption", "quarantine-required-a"),
        )
        .expect("record-backed quarantine should derive readmission requirement");

    let wrong_family = layout_corruption().readmit_with(
        required.clone(),
        crate::corruption::S8NativeReadmissionInput::RecoveryWitness {
            witness: record_backed_witness(
                other_family(),
                &quarantine_record,
                "quarantine-required-a",
            ),
        },
    );

    let wrong_identity = layout_corruption().readmit_with(
        required,
        crate::corruption::S8NativeReadmissionInput::RecoveryWitness {
            witness: quarantine_witness(family(), "quarantine-required-b"),
        },
    );

    assert!(matches!(
        wrong_family,
        crate::corruption::S8LayoutReadmissionOutcome::Denied(
            S8CorruptionDenial::FamilyBoundReadmissionWitnessRequired {
                family: actual_family,
                source: S8LayoutReadmissionSource::QuarantineRecovery,
            }
        ) if actual_family == family()
    ));
    assert!(matches!(
        wrong_identity,
        crate::corruption::S8LayoutReadmissionOutcome::Denied(
            S8CorruptionDenial::FamilyBoundReadmissionWitnessRequired {
                family: actual_family,
                source: S8LayoutReadmissionSource::QuarantineRecovery,
            }
        ) if actual_family == family()
    ));
}

#[test]
fn quarantine_readmission_requirement_refuses_placeholder_quarantine_outcomes() {
    let rebuild_required =
        layout_corruption().classify(S8LayoutCorruptionInput::RebuildClassification(
            crate::LayoutCorruptionClassification::AuthoritativeSourceQuarantineRequired {
                family: family(),
            },
        ));
    let materialization_required =
        layout_corruption().classify(S8LayoutCorruptionInput::Materialization(
            crate::facade::access_planning()
                .quarantined_wal_lsn_coverage(
                    crate::layout_declarations().seed_family(),
                    forge_store_recovery_physics::LogSequenceNumber::new(21),
                    forge_store_recovery_physics::LogSequenceNumber::new(24),
                    forge_store_recovery_physics::CheckpointCoveredLsnRange::new(
                        forge_store_recovery_physics::LogSequenceNumber::new(22),
                        forge_store_recovery_physics::LogSequenceNumber::new(23),
                    )
                    .unwrap(),
                )
                .expect("quarantined coverage should admit"),
        ));

    let rebuild_denied = layout_corruption().require_record_backed_recovery_readmission(
        rebuild_required,
        &current_authority("store.s8.corruption", "placeholder-rebuild"),
    );
    let materialization_denied = layout_corruption().require_record_backed_recovery_readmission(
        materialization_required,
        &current_authority("store.s8.corruption", "placeholder-materialization"),
    );

    assert!(matches!(
        rebuild_denied,
        Err(S8CorruptionDenial::QuarantineRecordBackedReadmissionEvidenceRequired {
            family: actual_family,
        }) if actual_family == family()
    ));
    assert!(matches!(
        materialization_denied,
        Err(S8CorruptionDenial::QuarantineRecordBackedReadmissionEvidenceRequired {
            family: actual_family,
        }) if actual_family == family()
    ));
}

#[test]
fn offline_readmission_rejects_witness_for_different_family_or_artifact_identity() {
    let required = layout_corruption().classify(S8LayoutCorruptionInput::OfflineEvidence {
        family: family(),
        admission: offline_admission("offline-required-a"),
    });

    let wrong_family = layout_corruption().readmit_with(
        required.clone(),
        crate::corruption::S8NativeReadmissionInput::RecoveryWitness {
            witness: offline_witness(other_family(), "offline-required-a"),
        },
    );

    let wrong_identity = layout_corruption().readmit_with(
        required,
        crate::corruption::S8NativeReadmissionInput::RecoveryWitness {
            witness: offline_witness(family(), "offline-required-b"),
        },
    );

    assert!(matches!(
        wrong_family,
        crate::corruption::S8LayoutReadmissionOutcome::Denied(
            S8CorruptionDenial::FamilyBoundReadmissionWitnessRequired {
                family: actual_family,
                source: S8LayoutReadmissionSource::OfflineRecoveryEvidence,
            }
        ) if actual_family == family()
    ));
    assert!(matches!(
        wrong_identity,
        crate::corruption::S8LayoutReadmissionOutcome::Denied(
            S8CorruptionDenial::FamilyBoundReadmissionWitnessRequired {
                family: actual_family,
                source: S8LayoutReadmissionSource::OfflineRecoveryEvidence,
            }
        ) if actual_family == family()
    ));
}

#[test]
fn terminal_import_readmission_resumes_foreground_authority_with_family_bound_store_witness() {
    let required = layout_corruption().classify(S8LayoutCorruptionInput::TerminalImport {
        witness: import_witness(family(), "terminal-import-success"),
    });
    let outcome = layout_corruption().readmit_with(
        required,
        crate::corruption::S8NativeReadmissionInput::RecoveryWitness {
            witness: import_witness(family(), "terminal-import-success"),
        },
    );

    assert!(matches!(
        outcome,
        crate::corruption::S8LayoutReadmissionOutcome::Readmitted(witness)
            if witness.family() == family()
                && witness.source() == S8LayoutReadmissionSource::TerminalImport
    ));
}

#[test]
fn terminal_import_does_not_accept_offline_recovery_witness_as_readmission_authority() {
    let required = layout_corruption().classify(S8LayoutCorruptionInput::TerminalImport {
        witness: import_witness(family(), "terminal-import"),
    });
    let outcome = layout_corruption().readmit_with(
        required,
        crate::corruption::S8NativeReadmissionInput::RecoveryWitness {
            witness: offline_witness(family(), "offline-terminal-mismatch"),
        },
    );

    assert!(matches!(
        outcome,
        crate::corruption::S8LayoutReadmissionOutcome::Denied(
            S8CorruptionDenial::FamilyBoundReadmissionWitnessRequired {
                family: actual_family,
                source: S8LayoutReadmissionSource::TerminalImport,
            }
        ) if actual_family == family()
    ));
}

#[test]
fn terminal_import_keeps_receipt_identity_in_required_outcome() {
    let witness = import_witness(family(), "terminal-import-identity");
    let expected_identity = witness.identity().clone();
    let required =
        layout_corruption().classify(S8LayoutCorruptionInput::TerminalImport { witness });

    assert!(matches!(
        required,
        S8LayoutCorruptionOutcome::TerminalImportReadmissionRequired { family: actual_family, identity }
            if actual_family == family()
                && identity == expected_identity
    ));
}

#[test]
fn terminal_import_readmission_rejects_witness_for_different_family_or_artifact_identity() {
    let required = layout_corruption().classify(S8LayoutCorruptionInput::TerminalImport {
        witness: import_witness(family(), "terminal-required-a"),
    });

    let wrong_family = layout_corruption().readmit_with(
        required.clone(),
        crate::corruption::S8NativeReadmissionInput::RecoveryWitness {
            witness: import_witness(other_family(), "terminal-required-a"),
        },
    );

    let wrong_identity = layout_corruption().readmit_with(
        required,
        crate::corruption::S8NativeReadmissionInput::RecoveryWitness {
            witness: import_witness(family(), "terminal-required-b"),
        },
    );

    assert!(matches!(
        wrong_family,
        crate::corruption::S8LayoutReadmissionOutcome::Denied(
            S8CorruptionDenial::FamilyBoundReadmissionWitnessRequired {
                family: actual_family,
                source: S8LayoutReadmissionSource::TerminalImport,
            }
        ) if actual_family == family()
    ));
    assert!(matches!(
        wrong_identity,
        crate::corruption::S8LayoutReadmissionOutcome::Denied(
            S8CorruptionDenial::FamilyBoundReadmissionWitnessRequired {
                family: actual_family,
                source: S8LayoutReadmissionSource::TerminalImport,
            }
        ) if actual_family == family()
    ));
}
