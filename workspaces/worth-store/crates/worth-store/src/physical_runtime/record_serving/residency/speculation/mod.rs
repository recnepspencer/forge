mod failure;
mod intent;
mod outcome;
#[cfg(feature = "certification-test-authority")]
mod work_submission;

pub use failure::PhysicalSpeculativeReadFailure;
pub use intent::{PhysicalPrefetchIntent, PhysicalReadAheadIntent, PhysicalReadAheadIntentDenial};
pub use outcome::{
    PhysicalPrefetchOutcome, PhysicalReadAheadBatch, PhysicalReadAheadFrameOutcome,
    PhysicalReadAheadOutcome, PhysicalSpeculativeReadDrop,
};
