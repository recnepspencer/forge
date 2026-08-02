use super::{
    AmbiguousPublicationReport, NonAuthoritativePublicationDenial,
    PartialPublicationCounterSnapshot, TornPublicationDenial, UnacknowledgedDurableWal,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnacknowledgedPublicationOutcome {
    NoWalAppendObserved,
    WalAppendedButNotDurable,
    DurableWalReplayable,
    CheckpointCutoverAmbiguous,
    RejectedNonAuthoritativePromotion,
    TornPublicationRejected,
    Ambiguous,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveredOrRejectedPartialPublication {
    NoRecoveredWork {
        counters: PartialPublicationCounterSnapshot,
    },
    ReplayableUnacknowledgedWal {
        durable_wal: UnacknowledgedDurableWal,
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
    Ambiguous {
        report: AmbiguousPublicationReport,
        counters: PartialPublicationCounterSnapshot,
    },
}

impl RecoveredOrRejectedPartialPublication {
    pub const fn is_replayable_without_promoting_acknowledgment(&self) -> bool {
        matches!(self, Self::ReplayableUnacknowledgedWal { .. })
    }

    pub const fn replayable_durable_wal(&self) -> Option<&UnacknowledgedDurableWal> {
        match self {
            Self::ReplayableUnacknowledgedWal { durable_wal, .. } => Some(durable_wal),
            _ => None,
        }
    }

    pub const fn counters(&self) -> PartialPublicationCounterSnapshot {
        match self {
            Self::NoRecoveredWork { counters }
            | Self::ReplayableUnacknowledgedWal { counters, .. }
            | Self::RejectedTornPublication { counters, .. }
            | Self::RejectedNonAuthoritativePromotion { counters, .. }
            | Self::Ambiguous { counters, .. } => *counters,
        }
    }
}
