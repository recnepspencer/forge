pub(crate) mod dirty_publication;

mod dirty_counters;
mod dirty_state;

#[cfg(test)]
pub(crate) mod dirty_state_test_support;
#[cfg(test)]
mod dirty_state_tests;

pub use dirty_counters::DirtyPageCounterSnapshot;
pub use dirty_publication::{DirtyPublicationPlan, DirtyPublicationReceipt};
pub use dirty_state::{
    DirtyPageAccessOrigin, DirtyPageIdentity, DirtyPageState, DirtyShutdownPosture,
    DirtyShutdownReport,
};
