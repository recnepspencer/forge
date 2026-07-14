mod outcome;
mod planning_basis;
mod value;

pub(crate) use outcome::UiResizePreviewOutcome;
pub use outcome::{
    UiDragResizeCounters, UiDurableResizeCommitDenialReport, UiDurableResizeCommitOutcome,
};
pub use planning_basis::UiResizeAllocationPlanningBasis;
pub use value::{UiDurableResizeCommitIntent, UiResizeLogicalExtent, UiResizePreviewSample};

#[cfg(test)]
mod isolation_tests;
#[cfg(test)]
mod tests;
