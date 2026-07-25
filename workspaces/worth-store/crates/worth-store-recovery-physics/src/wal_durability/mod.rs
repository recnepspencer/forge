mod ack_precondition;
mod ack_receipt;
mod append_plan;
mod append_receipt;
mod crash_posture;
mod crash_record;
mod denial;
mod durability_observation;
mod executed_append;
#[cfg(all(test, feature = "certification-test-authority"))]
mod tests;

pub use ack_precondition::AcknowledgmentPrecondition;
pub use ack_receipt::{DurableAckBasis, DurableAckReceipt};
pub use append_plan::{WalAppendDurabilityScope, WalAppendPlan, WalAppendProgress};
pub(crate) use append_receipt::{WalAppendByteObservation, WalDurabilityFailure};
pub use append_receipt::{WalAppendReceipt, WalFrameDigest};
pub use crash_posture::{WalDurabilityCrashBasis, WalDurabilityCrashPosture};
pub use crash_record::{ReopenedWalDurabilityCrashRecord, WalDurabilityCrashRecord};
pub use denial::{IllegalAcknowledgmentDenial, IllegalAcknowledgmentDenialKind};
pub use durability_observation::{WalDurabilityObservation, WalDurabilityObservationSequence};
#[cfg(feature = "certification-test-authority")]
pub use executed_append::execute_wal_durability_with_boundary_control;
pub use executed_append::{
    execute_wal_durability, ExecutedWalDurabilityOutcome, WalDurabilityExecutionError,
};
