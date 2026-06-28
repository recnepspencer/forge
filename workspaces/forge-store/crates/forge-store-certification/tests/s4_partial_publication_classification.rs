#[path = "s4_partial_publication_classification/crash_edge_observations.rs"]
mod crash_edge_observations;
#[path = "s4_partial_publication_classification/durable_page_mutation_observations.rs"]
mod durable_page_mutation_observations;
#[path = "s4_partial_publication_classification/non_authoritative_observations.rs"]
mod non_authoritative_observations;

use forge_store_recovery_physics::{
    BackendResidueKind, LogSequenceNumber, PartialPublicationClassification,
    PartialPublicationEvidence, PartialPublicationObservationSet, PartialPublicationPersistedBytes,
    RecoveredOrRejectedPartialPublication, RollbackImageRequiredPosture, TornPublicationDenial,
    UnacknowledgedPublicationOutcome,
};

use crash_edge_observations::*;
use durable_page_mutation_observations::*;
use non_authoritative_observations::*;

#[test]
fn identical_partial_publication_bytes_classify_identically() {
    let persisted_bytes = persisted_before_durability_bytes(20, 21);
    let first = PartialPublicationClassification::classify_observations(
        PartialPublicationObservationSet::new().with_persisted_bytes(
            PartialPublicationPersistedBytes::from_bytes(persisted_bytes.clone()),
        ),
    );
    let second = PartialPublicationClassification::classify_observations(
        PartialPublicationObservationSet::new().with_persisted_bytes(
            PartialPublicationPersistedBytes::from_bytes(persisted_bytes),
        ),
    );

    assert_eq!(first.outcome(), second.outcome());
    assert_eq!(first.counters(), second.counters());
    assert_eq!(
        first.classification_digest(),
        second.classification_digest()
    );
    assert_eq!(
        first.outcome(),
        UnacknowledgedPublicationOutcome::WalAppendedButNotDurable
    );
    assert_eq!(first.counters().observed_crash_edges(), 1);
}

#[test]
fn residue_live_ack_memory_and_logs_cannot_promote_unacknowledged_truth() {
    let residue = PartialPublicationClassification::classify_observations(
        PartialPublicationObservationSet::new()
            .with_backend_residue(BackendResidueKind::StalePageImage, "stale-page-residue"),
    );
    let live_ack = PartialPublicationClassification::classify_observations(
        PartialPublicationObservationSet::new().with_live_ack_memory("heap-ack-memory"),
    );
    let log_only = PartialPublicationClassification::classify_observations(
        PartialPublicationObservationSet::new().with_log_only("operator-log"),
    );

    for classification in [&residue, &live_ack, &log_only] {
        assert_eq!(
            classification.outcome(),
            UnacknowledgedPublicationOutcome::RejectedNonAuthoritativePromotion
        );
        assert!(matches!(
            classification.recovered_or_rejected(),
            RecoveredOrRejectedPartialPublication::RejectedNonAuthoritativePromotion { .. }
        ));
    }
    assert_eq!(residue.counters().rejected_residue_promotions(), 1);
    assert_eq!(live_ack.counters().rejected_live_ack_promotions(), 1);
    assert_eq!(log_only.counters().rejected_log_only_promotions(), 1);
}

#[test]
fn non_authoritative_observations_do_not_outrank_replayable_wal() {
    let classification = PartialPublicationClassification::classify_observations(
        replayable_wal_with_non_authoritative_observations(20, 21),
    );

    assert_eq!(
        classification.outcome(),
        UnacknowledgedPublicationOutcome::DurableWalReplayable
    );
    assert_eq!(classification.counters().replayable_unacknowledged_wal(), 1);
    assert_eq!(classification.counters().rejected_residue_promotions(), 0);
    assert_eq!(classification.counters().rejected_live_ack_promotions(), 0);
    assert_eq!(classification.counters().rejected_log_only_promotions(), 0);
    assert!(matches!(
        classification.recovered_or_rejected(),
        RecoveredOrRejectedPartialPublication::ReplayableUnacknowledgedWal { .. }
    ));
}

#[test]
fn torn_publication_and_insufficient_evidence_do_not_share_outcome() {
    let torn = PartialPublicationClassification::classify(
        PartialPublicationEvidence::from_torn_publication(TornPublicationDenial::new(
            Some(LogSequenceNumber::new(21)),
            "phase8-torn-publication",
        )),
    );
    let ambiguous = PartialPublicationClassification::classify(
        PartialPublicationEvidence::insufficient_persisted_evidence("phase8-ambiguous"),
    );

    assert_eq!(
        torn.outcome(),
        UnacknowledgedPublicationOutcome::TornPublicationRejected
    );
    assert_eq!(
        ambiguous.outcome(),
        UnacknowledgedPublicationOutcome::Ambiguous
    );
    assert_ne!(torn.outcome(), ambiguous.outcome());
    assert_eq!(torn.counters().torn_publication_denials(), 1);
    assert_eq!(torn.counters().ambiguous_outcomes(), 0);
    assert_eq!(ambiguous.counters().ambiguous_outcomes(), 1);
    assert!(matches!(
        torn.recovered_or_rejected(),
        RecoveredOrRejectedPartialPublication::RejectedTornPublication { .. }
    ));
    assert!(matches!(
        ambiguous.recovered_or_rejected(),
        RecoveredOrRejectedPartialPublication::Ambiguous { .. }
    ));
}

#[test]
fn missing_rollback_image_is_typed_no_undo_denial() {
    let classification = PartialPublicationClassification::classify_observations(
        missing_rollback_image_observations(),
    );

    assert_eq!(
        classification.outcome(),
        UnacknowledgedPublicationOutcome::RejectedNoUndoHazard
    );
    assert_eq!(classification.counters().no_undo_denials(), 1);
    match classification.recovered_or_rejected() {
        RecoveredOrRejectedPartialPublication::RejectedNoUndoHazard { classification, .. } => {
            assert_eq!(
                classification.posture(),
                RollbackImageRequiredPosture::RequiredButMissing
            );
        }
        other => panic!("expected no-undo hazard rejection, got {other:?}"),
    }
}

#[test]
fn rollback_image_posture_is_not_rejected_as_no_undo_hazard() {
    let classification = PartialPublicationClassification::classify_observations(
        rollback_image_protected_observations(),
    );

    assert_eq!(
        classification.outcome(),
        UnacknowledgedPublicationOutcome::RollbackImageProtected
    );
    assert_eq!(classification.counters().no_undo_denials(), 0);
    assert_eq!(classification.counters().no_undo_postures(), 1);
    match classification.recovered_or_rejected() {
        RecoveredOrRejectedPartialPublication::NoUndoPostureAccepted { classification, .. } => {
            assert_eq!(
                classification.posture(),
                RollbackImageRequiredPosture::ProtectedByRollbackImage
            );
        }
        other => panic!("expected rollback-protected posture, got {other:?}"),
    }
}

#[test]
fn admitted_redo_only_posture_is_not_rejected_as_no_undo_hazard() {
    let classification =
        PartialPublicationClassification::classify_observations(admitted_redo_only_observations());

    assert_eq!(
        classification.outcome(),
        UnacknowledgedPublicationOutcome::NoUndoPostureSatisfied
    );
    assert_eq!(classification.counters().no_undo_denials(), 0);
    assert_eq!(classification.counters().no_undo_postures(), 1);
    match classification.recovered_or_rejected() {
        RecoveredOrRejectedPartialPublication::NoUndoPostureAccepted { classification, .. } => {
            assert_eq!(
                classification.posture(),
                RollbackImageRequiredPosture::NotRequiredForAdmittedRedoOnlyMutation
            );
        }
        other => panic!("expected admitted redo-only posture, got {other:?}"),
    }
}

#[test]
fn required_crash_edges_classify_as_distinct_outcomes() {
    let before_wal = PartialPublicationClassification::classify(
        PartialPublicationEvidence::from_persisted_crash_edge(before_wal_append_edge()),
    );
    let before_durability = PartialPublicationClassification::classify(
        PartialPublicationEvidence::from_persisted_crash_edge(
            after_wal_append_before_durability_edge(20, 21),
        ),
    );
    let before_ack = PartialPublicationClassification::classify(
        PartialPublicationEvidence::from_persisted_crash_edge(after_durability_before_ack_edge(
            20, 21,
        )),
    );
    let after_ack = PartialPublicationClassification::classify(
        PartialPublicationEvidence::from_persisted_crash_edge(after_ack_before_page_flush_edge(
            20, 21,
        )),
    );
    let checkpoint_cutover = PartialPublicationClassification::classify(
        PartialPublicationEvidence::from_persisted_crash_edge(during_checkpoint_cutover_edge()),
    );

    let outcomes = [
        before_wal.outcome(),
        before_durability.outcome(),
        before_ack.outcome(),
        after_ack.outcome(),
        checkpoint_cutover.outcome(),
    ];
    assert_eq!(
        outcomes,
        [
            UnacknowledgedPublicationOutcome::NoWalAppendObserved,
            UnacknowledgedPublicationOutcome::WalAppendedButNotDurable,
            UnacknowledgedPublicationOutcome::DurableWalReplayable,
            UnacknowledgedPublicationOutcome::AcknowledgedBeforePageFlush,
            UnacknowledgedPublicationOutcome::CheckpointCutoverAmbiguous,
        ]
    );
    assert_eq!(checkpoint_cutover.counters().ambiguous_outcomes(), 1);
    assert_edge_recovered_as_no_work(
        &before_wal,
        UnacknowledgedPublicationOutcome::NoWalAppendObserved,
    );
    assert_edge_recovered_as_no_work(
        &before_durability,
        UnacknowledgedPublicationOutcome::WalAppendedButNotDurable,
    );
    assert!(matches!(
        before_ack.recovered_or_rejected(),
        RecoveredOrRejectedPartialPublication::ReplayableUnacknowledgedWal { .. }
    ));
    assert!(matches!(
        after_ack.recovered_or_rejected(),
        RecoveredOrRejectedPartialPublication::AcknowledgedWorkAwaitingPageFlush { .. }
    ));
    assert!(matches!(
        checkpoint_cutover.recovered_or_rejected(),
        RecoveredOrRejectedPartialPublication::Ambiguous { .. }
    ));
}

fn assert_edge_recovered_as_no_work(
    classification: &PartialPublicationClassification,
    expected_outcome: UnacknowledgedPublicationOutcome,
) {
    assert_eq!(classification.outcome(), expected_outcome);
    assert!(matches!(
        classification.recovered_or_rejected(),
        RecoveredOrRejectedPartialPublication::NoRecoveredWork { .. }
    ));
    assert_eq!(classification.counters().observed_crash_edges(), 1);
}
