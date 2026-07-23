mod dispatcher;
mod epoch;
mod framework_turn;
pub(in crate::runtime) use framework_turn::UiAllocationTransactionAuthority;
pub(crate) mod gateway;
mod ingress;
mod lifecycle;
mod mailbox;
mod output;
mod retry_ledger;
mod runtime_lifecycle;
mod source_registry;

pub(crate) use dispatcher::{
    UiAllocationFrameDispatcher, UiAllocationFrameEpochAssignment,
    UiAllocationFrameSubmissionTransition,
};
pub use epoch::UiAllocationFrameEpoch;
pub(in crate::runtime) use framework_turn::UiPreparedFrameReplacementCommit;
#[cfg(test)]
pub use framework_turn::WorthUiPreviewPaintFollowOn;
pub(crate) use framework_turn::{
    UiAllocationFrameFrameworkScheduler, UiPendingAllocationFrameHandoff,
};
pub use framework_turn::{
    UiFrameworkTransitionPlanningCounters, WorthUiFrameworkTurn, WorthUiFrameworkTurnCompletion,
    WorthUiFrameworkTurnExecution, WorthUiInteractionTurnSource, WorthUiQueryProjectionTurnSource,
};
pub(crate) use gateway::UiAllocationFrameGatewayState;
pub use gateway::{
    UiAllocationFrameGatewayOutcome, UiAllocationFrameQueryWarningPosture,
    UiAllocationFrameSourceFact, UiAllocationFrameSourceFactPosture,
    WorthUiQueryFrameIngressCounters, WorthUiQueryFrameIngressDenial,
    WorthUiQueryFrameIngressOutcome,
};
#[cfg(test)]
pub(crate) use gateway::{
    WorthUiDurableResizeSubmission, WorthUiHostMeasurementSubmission, WorthUiInteractionSubmission,
};
pub(crate) use ingress::UiAllocationFrameSourceLease;
pub use ingress::{
    UiAdmittedAllocationSourceOrder, UiAdmittedAllocationStreamIngress,
    UiAllocationFrameIngressDescriptor, UiAllocationFrameIngressIdentity,
    UiAllocationFrameIngressKey, UiAllocationFrameIngressSequence,
    UiAllocationFrameSourceGeneration, UiAllocationFrameSourceIdentity,
    UiAllocationFrameSourceLane, UiAllocationFrameSourceLeaseIdentity,
    UiAllocationFrameSubmissionDenial,
};
pub use lifecycle::{UiAllocationFrameDispatcherState, UiAllocationFramePauseReason};
pub use mailbox::{UiAllocationFrameIngressView, UiAllocationFrameMailboxStoragePosture};
pub(crate) use mailbox::{UiAllocationFrameMailbox, UiAllocationFrameMailboxDrain};
pub(in crate::runtime::allocation_frame_dispatch) use output::UiAllocationFrameSubmissionAssignmentBatch;
pub(crate) use output::{UiAdmittedAllocationStreamFrame, UiAllocationFrameTransitionOutcome};
pub use output::{
    UiAllocationFrameDispatchDenial, UiAllocationFrameDispatcherCounters,
    UiAllocationFrameDuplicateWitness, UiAllocationFrameSubmissionAssignment,
    UiAllocationFrameSubmissionOutcome,
};
pub(crate) use output::{
    UiAllocationFrameQueueDisposition, UiAllocationFrameReplacementTransition,
};
pub(crate) use retry_ledger::UiAllocationFrameRetryState;
#[cfg(test)]
pub use retry_ledger::{
    UiAllocationFrameSourceRetirementDenial, UiAllocationFrameSourceRetirementOutcome,
};
pub use source_registry::UiAllocationFrameSourceAdmissionDenial;
pub(crate) use source_registry::UiAllocationFrameSourceRegistry;
