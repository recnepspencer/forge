use super::assembly::{classification, classification_with_before_wal_operation_digest};
use super::PartialPublicationClassification;
use crate::partial_publication::{
    AmbiguousPublicationReport, PartialPublicationCounterSnapshot, PartialPublicationCrashEdge,
    RecoveredOrRejectedPartialPublication, UnacknowledgedPublicationOutcome,
};

pub(super) fn classify_persisted_crash_edge(
    edge: &PartialPublicationCrashEdge,
    digest: &str,
) -> PartialPublicationClassification {
    let counters = PartialPublicationCounterSnapshot::default().with_observed_crash_edge();
    match edge {
        PartialPublicationCrashEdge::BeforeWalAppend { operation_digest } => {
            classification_with_before_wal_operation_digest(
                UnacknowledgedPublicationOutcome::NoWalAppendObserved,
                RecoveredOrRejectedPartialPublication::NoRecoveredWork { counters },
                counters,
                digest,
                operation_digest.clone(),
            )
        }
        PartialPublicationCrashEdge::AfterWalAppendBeforeDurability { .. } => classification(
            UnacknowledgedPublicationOutcome::WalAppendedButNotDurable,
            RecoveredOrRejectedPartialPublication::NoRecoveredWork { counters },
            counters,
            digest,
        ),
        PartialPublicationCrashEdge::AfterDurabilityBeforeAck { durable_wal } => {
            let counters = counters.with_replayable_unacknowledged_wal();
            classification(
                UnacknowledgedPublicationOutcome::DurableWalReplayable,
                RecoveredOrRejectedPartialPublication::ReplayableUnacknowledgedWal {
                    durable_wal: durable_wal.clone(),
                    counters,
                },
                counters,
                digest,
            )
        }
        PartialPublicationCrashEdge::AfterAckBeforePageFlush { .. } => classification(
            UnacknowledgedPublicationOutcome::AcknowledgedBeforePageFlush,
            RecoveredOrRejectedPartialPublication::AcknowledgedWorkAwaitingPageFlush { counters },
            counters,
            digest,
        ),
        PartialPublicationCrashEdge::DuringCheckpointCutover { checkpoint_digest } => {
            let counters = counters.with_ambiguous_outcome();
            let report = AmbiguousPublicationReport::insufficient_persisted_evidence(
                checkpoint_digest.clone(),
            );
            classification(
                UnacknowledgedPublicationOutcome::CheckpointCutoverAmbiguous,
                RecoveredOrRejectedPartialPublication::Ambiguous { report, counters },
                counters,
                digest,
            )
        }
    }
}
