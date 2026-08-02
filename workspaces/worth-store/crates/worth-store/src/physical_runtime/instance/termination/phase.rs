#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalStoreClosePhase {
    CheckpointDrained,
    AdmissionStopped,
    SafeCancellationComplete,
    DispatchSettlementComplete,
    SignalDisposed,
    ResidencyClosed,
    MediaReleased,
}

pub(super) const CLOSE_PHASES: [PhysicalStoreClosePhase; 7] = [
    PhysicalStoreClosePhase::CheckpointDrained,
    PhysicalStoreClosePhase::AdmissionStopped,
    PhysicalStoreClosePhase::SafeCancellationComplete,
    PhysicalStoreClosePhase::DispatchSettlementComplete,
    PhysicalStoreClosePhase::SignalDisposed,
    PhysicalStoreClosePhase::ResidencyClosed,
    PhysicalStoreClosePhase::MediaReleased,
];

impl PhysicalStoreClosePhase {
    pub(super) const fn sequence_number(self) -> u8 {
        match self {
            Self::CheckpointDrained => 1,
            Self::AdmissionStopped => 2,
            Self::SafeCancellationComplete => 3,
            Self::DispatchSettlementComplete => 4,
            Self::SignalDisposed => 5,
            Self::ResidencyClosed => 6,
            Self::MediaReleased => 7,
        }
    }
}
