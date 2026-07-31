mod ack_precondition;
mod ack_receipt;
mod append_plan;
mod append_receipt;
#[cfg(feature = "certification-test-authority")]
mod certification_probe;
mod crash_posture;
mod crash_record;
mod denial;
mod durability_observation;
#[cfg(all(test, feature = "certification-test-authority"))]
mod tests;

pub use ack_precondition::AcknowledgmentPrecondition;
pub use ack_receipt::{DurableAckBasis, DurableAckReceipt};
pub use append_plan::{WalAppendDurabilityScope, WalAppendPlan, WalAppendProgress};
pub(crate) use append_receipt::{WalAppendByteObservation, WalDurabilityFailure};
pub use append_receipt::{WalAppendReceipt, WalFrameDigest};
#[cfg(feature = "certification-test-authority")]
pub use certification_probe::{
    certify_wal_durability_mechanism, certify_wal_durability_mechanism_with_boundary_control,
    CertifiedWalDurabilityMechanismObservation, WalDurabilityMechanismProbeError,
};
pub use crash_posture::{WalDurabilityCrashBasis, WalDurabilityCrashPosture};
pub use crash_record::{ReopenedWalDurabilityCrashRecord, WalDurabilityCrashRecord};
pub use denial::{IllegalAcknowledgmentDenial, IllegalAcknowledgmentDenialKind};
pub use durability_observation::{WalDurabilityObservation, WalDurabilityObservationSequence};
