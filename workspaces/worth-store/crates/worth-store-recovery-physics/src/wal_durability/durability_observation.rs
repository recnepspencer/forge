use worth_store_physical_backend::{
    BackendDurabilityProfile, WalDurabilityBarrier, WalDurabilityBarrierReceipt,
};

use super::{
    IllegalAcknowledgmentDenial, WalAppendDurabilityScope, WalAppendProgress, WalAppendReceipt,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WalDurabilityObservation<P: BackendDurabilityProfile> {
    Completed(WalDurabilityBarrierReceipt<P, WalAppendDurabilityScope>),
    BarrierFailed(WalDurabilityBarrier),
    DelayedFlush(WalDurabilityBarrier),
    LostFlush,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalDurabilityObservationSequence<P: BackendDurabilityProfile> {
    progress: WalAppendProgress<P>,
}

impl<P: BackendDurabilityProfile> WalDurabilityObservationSequence<P> {
    pub const fn new(progress: WalAppendProgress<P>) -> Self {
        Self { progress }
    }

    pub fn observe(
        self,
        observation: WalDurabilityObservation<P>,
    ) -> Result<Self, IllegalAcknowledgmentDenial> {
        match observation {
            WalDurabilityObservation::Completed(receipt) => self.completed(receipt),
            WalDurabilityObservation::BarrierFailed(barrier) => Ok(self.barrier_failed(barrier)),
            WalDurabilityObservation::DelayedFlush(barrier) => Ok(self.delayed_flush(barrier)),
            WalDurabilityObservation::LostFlush => Ok(self.lost_flush()),
        }
    }

    pub fn completed(
        self,
        receipt: WalDurabilityBarrierReceipt<P, WalAppendDurabilityScope>,
    ) -> Result<Self, IllegalAcknowledgmentDenial> {
        Ok(Self {
            progress: self.progress.complete_barrier(receipt)?,
        })
    }

    pub fn barrier_failed(self, barrier: WalDurabilityBarrier) -> Self {
        Self {
            progress: self.progress.fail_barrier(barrier),
        }
    }

    pub fn delayed_flush(self, barrier: WalDurabilityBarrier) -> Self {
        Self {
            progress: self.progress.delay_flush(barrier),
        }
    }

    pub fn lost_flush(self) -> Self {
        Self {
            progress: self.progress.lose_flush(),
        }
    }

    pub fn finish(self) -> Result<WalAppendReceipt<P>, IllegalAcknowledgmentDenial> {
        self.progress.finish()
    }
}
