use crate::runtime::{WorthQueryRuntimeError, WorthQueryWorkspace};

use super::{
    WorthQueryManagedLiveCheckpointReceipt, WorthQueryManagedLiveContinuation,
    WorthQueryManagedLiveResumeReceipt,
};
use crate::ordinary::live::WorthQueryManagedLiveHandle;

#[derive(Debug)]
#[must_use = "checkpoint may stop and successful continuations retain resource ownership"]
pub enum WorthQueryManagedLiveCheckpointOutcome {
    Checkpointed(WorthQueryManagedLiveCheckpointCompletion),
    Stopped(WorthQueryManagedLiveCheckpointStop),
}

#[derive(Debug)]
pub struct WorthQueryManagedLiveCheckpointCompletion {
    continuation: WorthQueryManagedLiveContinuation,
}

impl WorthQueryManagedLiveCheckpointCompletion {
    pub fn checkpoint(&self) -> &WorthQueryManagedLiveCheckpointReceipt {
        self.continuation.checkpoint()
    }

    pub fn into_continuation(self) -> WorthQueryManagedLiveContinuation {
        self.continuation
    }

    pub(super) fn new(continuation: WorthQueryManagedLiveContinuation) -> Self {
        Self { continuation }
    }
}

#[derive(Debug)]
pub struct WorthQueryManagedLiveCheckpointStop {
    handle: WorthQueryManagedLiveHandle,
    error: WorthQueryRuntimeError,
}

impl WorthQueryManagedLiveCheckpointStop {
    pub fn error(&self) -> &WorthQueryRuntimeError {
        &self.error
    }

    pub fn into_handle(self) -> WorthQueryManagedLiveHandle {
        self.handle
    }

    pub(super) fn new(handle: WorthQueryManagedLiveHandle, error: WorthQueryRuntimeError) -> Self {
        Self { handle, error }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryManagedLiveResumeStopKind {
    ForeignWorkspace,
    MissingResource,
    ContinuationIdentityMismatch,
    StaleBasis,
    AuthorityRebindRequired,
    PreviewIsolation,
    RuntimeStateUnavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryManagedLiveResumeNextAction {
    UseOwningWorkspace,
    ReopenDeclaration,
    SupplyFreshBasis,
    RebindAuthority,
    ReturnToAuthoritativeLane,
    AwaitRuntimeRecovery,
}

#[derive(Debug)]
#[must_use = "resume may stop and stopped outcomes retain the continuation for recovery"]
pub enum WorthQueryManagedLiveResumeOutcome {
    Resumed(WorthQueryManagedLiveResumeCompletion),
    Stopped(WorthQueryManagedLiveResumeStop),
}

#[derive(Debug)]
pub struct WorthQueryManagedLiveResumeCompletion {
    handle: WorthQueryManagedLiveHandle,
    receipt: WorthQueryManagedLiveResumeReceipt,
}

impl WorthQueryManagedLiveResumeCompletion {
    pub fn handle(&self) -> &WorthQueryManagedLiveHandle {
        &self.handle
    }

    pub fn receipt(&self) -> &WorthQueryManagedLiveResumeReceipt {
        &self.receipt
    }

    pub fn into_handle(self) -> WorthQueryManagedLiveHandle {
        self.handle
    }

    pub(super) fn new(
        handle: WorthQueryManagedLiveHandle,
        receipt: WorthQueryManagedLiveResumeReceipt,
    ) -> Self {
        Self { handle, receipt }
    }
}

#[derive(Debug)]
pub struct WorthQueryManagedLiveResumeStop {
    continuation: WorthQueryManagedLiveContinuation,
    kind: WorthQueryManagedLiveResumeStopKind,
    runtime_error: Option<WorthQueryRuntimeError>,
}

impl WorthQueryManagedLiveResumeStop {
    pub fn kind(&self) -> WorthQueryManagedLiveResumeStopKind {
        self.kind
    }

    pub fn next_action(&self) -> WorthQueryManagedLiveResumeNextAction {
        match self.kind {
            WorthQueryManagedLiveResumeStopKind::ForeignWorkspace => {
                WorthQueryManagedLiveResumeNextAction::UseOwningWorkspace
            }
            WorthQueryManagedLiveResumeStopKind::MissingResource
            | WorthQueryManagedLiveResumeStopKind::ContinuationIdentityMismatch => {
                WorthQueryManagedLiveResumeNextAction::ReopenDeclaration
            }
            WorthQueryManagedLiveResumeStopKind::StaleBasis => {
                WorthQueryManagedLiveResumeNextAction::SupplyFreshBasis
            }
            WorthQueryManagedLiveResumeStopKind::AuthorityRebindRequired => {
                WorthQueryManagedLiveResumeNextAction::RebindAuthority
            }
            WorthQueryManagedLiveResumeStopKind::PreviewIsolation => {
                WorthQueryManagedLiveResumeNextAction::ReturnToAuthoritativeLane
            }
            WorthQueryManagedLiveResumeStopKind::RuntimeStateUnavailable => {
                WorthQueryManagedLiveResumeNextAction::AwaitRuntimeRecovery
            }
        }
    }

    pub fn runtime_error(&self) -> Option<&WorthQueryRuntimeError> {
        self.runtime_error.as_ref()
    }

    pub fn into_continuation(self) -> WorthQueryManagedLiveContinuation {
        self.continuation
    }

    pub fn close(
        self,
        workspace: &mut WorthQueryWorkspace,
    ) -> crate::ordinary::live::WorthQueryManagedLiveCloseOutcome {
        let (view, capability, projection_binding) = self.continuation.into_resource_parts();
        WorthQueryManagedLiveHandle::new(view, capability, projection_binding).close(workspace)
    }

    pub(super) fn new(
        continuation: WorthQueryManagedLiveContinuation,
        kind: WorthQueryManagedLiveResumeStopKind,
        runtime_error: Option<WorthQueryRuntimeError>,
    ) -> Self {
        Self {
            continuation,
            kind,
            runtime_error,
        }
    }
}
