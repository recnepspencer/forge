#[path = "batch_surface/caught_up.rs"]
mod caught_up;
#[path = "batch_surface/control.rs"]
mod control;
#[path = "batch_surface/identity.rs"]
mod identity;
#[path = "batch_surface/narrow.rs"]
mod narrow;
#[path = "batch_surface/result.rs"]
mod result;
#[path = "batch_surface/widened.rs"]
mod widened;

pub use caught_up::CaughtUpContinuationBatch;
pub use control::ControlLaneBatchReceipt;
pub use identity::ContinuationBatchId;
pub use narrow::AdmittedNarrowBatchReceipt;
pub use result::ContinuationBatchResult;
pub use widened::BroadenedBatchReceipt;
