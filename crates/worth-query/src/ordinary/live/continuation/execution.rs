use crate::ordinary::live::WorthQueryManagedLiveHandle;
use crate::ordinary_outcome::{
    WorthQueryOrdinaryRuntimeBasisPostureKind, WorthQueryOrdinaryRuntimePostureKind,
};
use crate::runtime::{WorthQueryAuthorityLane, WorthQueryWorkspace};

use super::{
    WorthQueryManagedLiveCheckpointCompletion, WorthQueryManagedLiveCheckpointOutcome,
    WorthQueryManagedLiveCheckpointReceipt, WorthQueryManagedLiveCheckpointStop,
    WorthQueryManagedLiveContinuation, WorthQueryManagedLiveResumeCompletion,
    WorthQueryManagedLiveResumeOutcome, WorthQueryManagedLiveResumeReceipt,
    WorthQueryManagedLiveResumeStop, WorthQueryManagedLiveResumeStopKind,
};

impl WorthQueryManagedLiveHandle {
    pub fn checkpoint(
        self,
        workspace: &mut WorthQueryWorkspace,
    ) -> WorthQueryManagedLiveCheckpointOutcome {
        let observation = match self.observe(workspace) {
            Ok(observation) => observation,
            Err(error) => {
                return WorthQueryManagedLiveCheckpointOutcome::Stopped(
                    WorthQueryManagedLiveCheckpointStop::new(self, error),
                );
            }
        };
        let checkpoint = WorthQueryManagedLiveCheckpointReceipt::new(
            observation.resource_name(),
            observation.installation_identity(),
            observation.basis_binding_identity(),
            observation.last_delivery_sequence(),
        );
        let (view, capability) = self.into_resource_parts();
        WorthQueryManagedLiveCheckpointOutcome::Checkpointed(
            WorthQueryManagedLiveCheckpointCompletion::new(WorthQueryManagedLiveContinuation::new(
                view, capability, checkpoint,
            )),
        )
    }
}

impl WorthQueryManagedLiveContinuation {
    pub fn resume(self, workspace: &mut WorthQueryWorkspace) -> WorthQueryManagedLiveResumeOutcome {
        if let Err(error) =
            workspace.admit_managed_live_capability(self.workspace_capability(), self.view().name())
        {
            return stopped(
                self,
                WorthQueryManagedLiveResumeStopKind::ForeignWorkspace,
                Some(error),
            );
        }
        let observation =
            match workspace.observe_managed_live_view(self.view(), self.workspace_capability()) {
                Ok(observation) => observation,
                Err(error) => {
                    return stopped(
                        self,
                        WorthQueryManagedLiveResumeStopKind::MissingResource,
                        Some(error),
                    );
                }
            };
        if observation.authority_lane() != WorthQueryAuthorityLane::AuthoritativeTruth {
            return stopped(
                self,
                WorthQueryManagedLiveResumeStopKind::PreviewIsolation,
                None,
            );
        }
        if observation.installation_identity() != self.checkpoint().installation_identity()
            || observation.basis_binding_identity() != self.checkpoint().basis_binding_identity()
        {
            return stopped(
                self,
                WorthQueryManagedLiveResumeStopKind::ContinuationIdentityMismatch,
                None,
            );
        }
        if observation.runtime_posture().basis_posture()
            != WorthQueryOrdinaryRuntimeBasisPostureKind::Stable
        {
            return stopped(self, WorthQueryManagedLiveResumeStopKind::StaleBasis, None);
        }
        if observation.runtime_posture().remask_posture().is_some() {
            return stopped(
                self,
                WorthQueryManagedLiveResumeStopKind::AuthorityRebindRequired,
                None,
            );
        }
        if !runtime_posture_can_resume(observation.runtime_posture().kind()) {
            return stopped(
                self,
                WorthQueryManagedLiveResumeStopKind::RuntimeStateUnavailable,
                None,
            );
        }

        let receipt = WorthQueryManagedLiveResumeReceipt::new(
            self.checkpoint(),
            observation.last_delivery_sequence(),
        );
        let (view, capability) = self.into_resource_parts();
        WorthQueryManagedLiveResumeOutcome::Resumed(WorthQueryManagedLiveResumeCompletion::new(
            WorthQueryManagedLiveHandle::new(view, capability),
            receipt,
        ))
    }
}

fn runtime_posture_can_resume(kind: WorthQueryOrdinaryRuntimePostureKind) -> bool {
    matches!(
        kind,
        WorthQueryOrdinaryRuntimePostureKind::Current
            | WorthQueryOrdinaryRuntimePostureKind::Pending
            | WorthQueryOrdinaryRuntimePostureKind::Retried
            | WorthQueryOrdinaryRuntimePostureKind::Revalidating
    )
}

fn stopped(
    continuation: WorthQueryManagedLiveContinuation,
    kind: WorthQueryManagedLiveResumeStopKind,
    runtime_error: Option<crate::runtime::WorthQueryRuntimeError>,
) -> WorthQueryManagedLiveResumeOutcome {
    WorthQueryManagedLiveResumeOutcome::Stopped(WorthQueryManagedLiveResumeStop::new(
        continuation,
        kind,
        runtime_error,
    ))
}
