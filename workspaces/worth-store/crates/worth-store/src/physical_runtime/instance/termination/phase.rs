#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalStoreClosePhase {
    AdmissionStopped,
    SafeCancellationComplete,
    DispatchSettlementComplete,
    SignalDisposed,
    ResidencyClosed,
    MediaReleased,
}

pub(super) const CLOSE_PHASES: [PhysicalStoreClosePhase; 6] = [
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
            Self::AdmissionStopped => 1,
            Self::SafeCancellationComplete => 2,
            Self::DispatchSettlementComplete => 3,
            Self::SignalDisposed => 4,
            Self::ResidencyClosed => 5,
            Self::MediaReleased => 6,
        }
    }
}
