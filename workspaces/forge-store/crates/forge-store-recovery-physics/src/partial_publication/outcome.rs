use super::{
    AmbiguousPublicationReport, NoUndoPartialPublicationClassification,
    NonAuthoritativePublicationDenial, PartialPublicationCounterSnapshot, TornPublicationDenial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnacknowledgedPublicationOutcome {
    NoWalAppendObserved,
    WalAppendedButNotDurable,
    DurableWalReplayable,
    AcknowledgedBeforePageFlush,
    CheckpointCutoverAmbiguous,
    RejectedNonAuthoritativePromotion,
    TornPublicationRejected,
    RejectedNoUndoHazard,
    NoUndoPostureSatisfied,
    RollbackImageProtected,
    UndoCapableRecoveryDeferred,
    Ambiguous,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveredOrRejectedPartialPublication {
    NoRecoveredWork {
        counters: PartialPublicationCounterSnapshot,
    },
    ReplayableUnacknowledgedWal {
        counters: PartialPublicationCounterSnapshot,
    },
    AcknowledgedWorkAwaitingPageFlush {
        counters: PartialPublicationCounterSnapshot,
    },
    RejectedTornPublication {
        denial: TornPublicationDenial,
        counters: PartialPublicationCounterSnapshot,
    },
    RejectedNonAuthoritativePromotion {
        denial: NonAuthoritativePublicationDenial,
        counters: PartialPublicationCounterSnapshot,
    },
    RejectedNoUndoHazard {
        classification: NoUndoPartialPublicationClassification,
        counters: PartialPublicationCounterSnapshot,
    },
    NoUndoPostureAccepted {
        classification: NoUndoPartialPublicationClassification,
        counters: PartialPublicationCounterSnapshot,
    },
    UndoCapableRecoveryDeferred {
        classification: NoUndoPartialPublicationClassification,
        counters: PartialPublicationCounterSnapshot,
    },
    Ambiguous {
        report: AmbiguousPublicationReport,
        counters: PartialPublicationCounterSnapshot,
    },
}

impl RecoveredOrRejectedPartialPublication {
    pub const fn is_replayable_without_promoting_acknowledgment(&self) -> bool {
        matches!(self, Self::ReplayableUnacknowledgedWal { .. })
    }

    pub const fn counters(&self) -> PartialPublicationCounterSnapshot {
        match self {
            Self::NoRecoveredWork { counters }
            | Self::ReplayableUnacknowledgedWal { counters }
            | Self::AcknowledgedWorkAwaitingPageFlush { counters }
            | Self::RejectedTornPublication { counters, .. }
            | Self::RejectedNonAuthoritativePromotion { counters, .. }
            | Self::RejectedNoUndoHazard { counters, .. }
            | Self::NoUndoPostureAccepted { counters, .. }
            | Self::UndoCapableRecoveryDeferred { counters, .. }
            | Self::Ambiguous { counters, .. } => *counters,
        }
    }
}
