use worth_store_physical_backend::{BackendDurabilityProfile, BackendDurabilitySupport};

use super::{IllegalAcknowledgmentDenial, WalAppendReceipt, WalDurabilityFailure};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcknowledgmentPrecondition<P: BackendDurabilityProfile> {
    receipt: WalAppendReceipt<P>,
}

impl<P: BackendDurabilityProfile> AcknowledgmentPrecondition<P> {
    pub fn from_append_receipt(
        receipt: WalAppendReceipt<P>,
    ) -> Result<Self, IllegalAcknowledgmentDenial> {
        reject_unsupported_profile::<P>()?;
        reject_failed_append(&receipt)?;
        reject_incomplete_barriers::<P>(&receipt)?;
        Ok(Self { receipt })
    }

    pub(crate) fn into_receipt(self) -> WalAppendReceipt<P> {
        self.receipt
    }

    pub fn append_receipt(&self) -> &WalAppendReceipt<P> {
        &self.receipt
    }
}

fn reject_unsupported_profile<P: BackendDurabilityProfile>(
) -> Result<(), IllegalAcknowledgmentDenial> {
    match P::SUPPORT {
        BackendDurabilitySupport::Certified => Ok(()),
        BackendDurabilitySupport::UnsupportedDurabilityCapability => {
            Err(IllegalAcknowledgmentDenial::unsupported_profile(P::ID))
        }
        BackendDurabilitySupport::AdversarialLostFlush => {
            Err(IllegalAcknowledgmentDenial::lost_flush(P::ID))
        }
    }
}

fn reject_failed_append<P: BackendDurabilityProfile>(
    receipt: &WalAppendReceipt<P>,
) -> Result<(), IllegalAcknowledgmentDenial> {
    if receipt.observed_bytes() != receipt.expected_bytes() {
        return Err(IllegalAcknowledgmentDenial::short_write(
            P::ID,
            receipt.segment_id(),
            receipt.generation(),
            receipt.lsn_range(),
            receipt.expected_bytes(),
            receipt.observed_bytes(),
        ));
    }
    match receipt.failure() {
        Some(WalDurabilityFailure::BarrierFailed(barrier)) => {
            Err(IllegalAcknowledgmentDenial::barrier_failed(P::ID, barrier))
        }
        Some(WalDurabilityFailure::DelayedFlush(barrier)) => {
            Err(IllegalAcknowledgmentDenial::delayed_flush(P::ID, barrier))
        }
        Some(WalDurabilityFailure::LostFlush) => {
            Err(IllegalAcknowledgmentDenial::lost_flush(P::ID))
        }
        None => Ok(()),
    }
}

fn reject_incomplete_barriers<P: BackendDurabilityProfile>(
    receipt: &WalAppendReceipt<P>,
) -> Result<(), IllegalAcknowledgmentDenial> {
    let required = P::REQUIRED_BARRIERS;
    let completed = receipt.completed_barriers();
    if !completed.satisfies(required) {
        let missing = completed
            .first_missing_from(required)
            .expect("unsatisfied barrier set must name missing barrier");
        return Err(IllegalAcknowledgmentDenial::missing_barrier(
            P::ID,
            required,
            completed,
            missing,
        ));
    }
    Ok(())
}
