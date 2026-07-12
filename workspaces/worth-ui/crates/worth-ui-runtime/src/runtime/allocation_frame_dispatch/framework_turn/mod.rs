mod allocation_transaction;
pub(in crate::runtime) use allocation_transaction::UiAllocationTransactionAuthority;
mod completion;
mod execution;
mod outcome;
mod owner;
#[cfg(test)]
mod owner_test_support;
mod pending_durable_transaction;
mod pending_handoff;
mod scheduler;
mod source_capabilities;

pub use completion::{
    WorthUiFrameworkTurnCompletion, WorthUiPreviewPaintFollowOn,
    WorthUiResolvedPreviewPaintCompletion,
};
pub(crate) use completion::{WorthUiPendingDurableResize, WorthUiPendingPreviewPaint};
pub use execution::WorthUiFrameworkTurnExecution;
pub(crate) use outcome::UiAllocationFrameTurnOutcome;
pub use owner::{
    WorthUiDurableResizeTurnSource, WorthUiFrameworkTurn, WorthUiHostMeasurementTurnSource,
    WorthUiInteractionTurnSource, WorthUiQueryProjectionTurnSource, WorthUiResizePreviewTurnSource,
    WorthUiScrollOffsetTurnSource,
};
pub(super) use pending_durable_transaction::UiPendingDurableResizeCommitPort;
pub(crate) use pending_handoff::UiPendingAllocationFrameHandoff;
pub(crate) use scheduler::UiAllocationFrameFrameworkScheduler;
pub(in crate::runtime::allocation_frame_dispatch) use scheduler::UiAllocationFrameIngressMailbox;
pub(in crate::runtime) use scheduler::UiPreparedFrameReplacementCommit;

#[cfg(test)]
mod tests;

/// Move-only proof that a receipt transition is being driven by the canonical
/// framework-turn owner. Receipt and invalidation authorities may consume this
/// proof, but no sibling runtime module can issue one.
#[derive(Debug)]
pub(super) struct UiAllocationFrameDispatchAuthority(());

impl UiAllocationFrameDispatchAuthority {
    fn issue() -> Self {
        Self(())
    }
}
