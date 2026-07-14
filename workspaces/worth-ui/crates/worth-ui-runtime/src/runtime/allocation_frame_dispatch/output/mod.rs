//! Immutable output vocabulary issued by the frame dispatcher.

mod counters;
mod replacement_transition;
mod sealed_frame;
mod submission_outcome;
mod terminal_disposition;

pub use counters::UiAllocationFrameDispatcherCounters;
pub(crate) use replacement_transition::UiAllocationFrameReplacementTransition;
pub(in crate::runtime::allocation_frame_dispatch) use sealed_frame::UiAllocationFrameSubmissionAssignmentBatch;
pub(crate) use sealed_frame::{
    UiAdmittedAllocationStreamFrame, UiAllocationFrameTransitionOutcome,
};
pub use sealed_frame::{
    UiAllocationFrameDispatchDenial, UiAllocationFrameDuplicateWitness,
    UiAllocationFrameSubmissionAssignment,
};
pub use submission_outcome::UiAllocationFrameSubmissionOutcome;
pub(crate) use terminal_disposition::UiAllocationFrameQueueDisposition;
