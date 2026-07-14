use super::{
    WorthQueryGraphReadMaterializationCancellationReceipt,
    WorthQueryGraphReadMaterializationCheckpoint, WorthQueryGraphReadMaterializationProgress,
    WorthQueryGraphReadMaterializationReceipt, WorthQueryGraphReadMaterializationRecoveryHandle,
    WorthQueryGraphReadMaterializationRequest,
    WorthQueryGraphReadMaterializationResourceLimitReceipt,
};
use crate::identity::hash_parts;

use super::WorthQueryGraphReadMaterializedArtifact;
use super::{
    WorthQueryGraphReadMaterializationAdmittedLimits, WorthQueryGraphReadMaterializationCounters,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphReadMaterializationJobState {
    Running,
    Cancelled,
    Completed,
    Indeterminate,
}

impl WorthQueryGraphReadMaterializationJobState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Cancelled => "cancelled",
            Self::Completed => "completed",
            Self::Indeterminate => "indeterminate",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadMaterializationJob {
    digest: String,
    request: WorthQueryGraphReadMaterializationRequest,
    snapshot_identity: String,
    progress: WorthQueryGraphReadMaterializationProgress,
    target_progress: WorthQueryGraphReadMaterializationProgress,
    checkpoint: WorthQueryGraphReadMaterializationCheckpoint,
    checkpoints: Vec<WorthQueryGraphReadMaterializationCheckpoint>,
    state: WorthQueryGraphReadMaterializationJobState,
}

impl WorthQueryGraphReadMaterializationJob {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn request(&self) -> &WorthQueryGraphReadMaterializationRequest {
        &self.request
    }

    pub fn snapshot_identity(&self) -> &str {
        &self.snapshot_identity
    }

    pub fn progress(&self) -> &WorthQueryGraphReadMaterializationProgress {
        &self.progress
    }

    pub fn checkpoint(&self) -> &WorthQueryGraphReadMaterializationCheckpoint {
        &self.checkpoint
    }

    pub fn checkpoints(&self) -> &[WorthQueryGraphReadMaterializationCheckpoint] {
        &self.checkpoints
    }

    pub fn state(&self) -> &WorthQueryGraphReadMaterializationJobState {
        &self.state
    }

    pub fn advance_to_next_checkpoint(&mut self) -> WorthQueryGraphReadMaterializationCheckpoint {
        let next_sequence =
            (self.checkpoint.sequence() + 1).min(self.target_progress.checkpoint_count());
        self.progress = checkpoint_progress(
            self.request.digest(),
            self.target_progress.admitted_limits().clone(),
            &self.target_progress,
            next_sequence,
        );
        self.checkpoint = WorthQueryGraphReadMaterializationCheckpoint::from_progress(
            self.request.digest(),
            self.progress.checkpoint_count(),
            self.progress.touched_edges(),
            self.progress.emitted_rows(),
            self.progress.allocated_bytes(),
        );
        self.checkpoints.push(self.checkpoint.clone());
        self.checkpoint.clone()
    }

    pub fn cancel(mut self) -> WorthQueryGraphReadMaterializationCancellationReceipt {
        self.state = WorthQueryGraphReadMaterializationJobState::Cancelled;
        WorthQueryGraphReadMaterializationCancellationReceipt::from_job_progress(
            self.digest,
            self.request.policy().cancellation_scope(),
            &self.progress,
            &self.checkpoint,
            self.state,
        )
    }

    pub fn cancel_after_checkpoint(
        mut self,
    ) -> WorthQueryGraphReadMaterializationCancellationReceipt {
        if self.checkpoint.sequence() == 0 {
            self.advance_to_next_checkpoint();
        }
        self.cancel()
    }

    pub fn complete(mut self) -> WorthQueryGraphReadMaterializationReceipt {
        self.state = WorthQueryGraphReadMaterializationJobState::Completed;
        self.progress = self.target_progress.clone();
        self.checkpoint = WorthQueryGraphReadMaterializationCheckpoint::from_progress(
            self.request.digest(),
            self.progress.checkpoint_count(),
            self.progress.touched_edges(),
            self.progress.emitted_rows(),
            self.progress.allocated_bytes(),
        );
        WorthQueryGraphReadMaterializationReceipt::from_completed_job(
            self.digest,
            &self.request,
            &self.progress,
            &self.checkpoint,
            &self.snapshot_identity,
        )
    }

    pub fn complete_to_artifact(self) -> WorthQueryGraphReadMaterializedArtifact {
        WorthQueryGraphReadMaterializedArtifact::from_receipt(self.complete())
    }

    pub fn recovery_handle_for_indeterminate_stop(
        mut self,
        recovery_reason: impl Into<String>,
    ) -> WorthQueryGraphReadMaterializationRecoveryHandle {
        self.state = WorthQueryGraphReadMaterializationJobState::Indeterminate;
        WorthQueryGraphReadMaterializationRecoveryHandle::from_checkpoint(
            self.digest,
            &self.checkpoint,
            &self.progress,
            recovery_reason,
        )
    }

    pub fn stop_indeterminate_after_checkpoint(
        mut self,
        recovery_reason: impl Into<String>,
    ) -> WorthQueryGraphReadMaterializationRecoveryHandle {
        if self.checkpoint.sequence() == 0 {
            self.advance_to_next_checkpoint();
        }
        self.recovery_handle_for_indeterminate_stop(recovery_reason)
    }

    pub fn stop_for_resource_limit(
        mut self,
    ) -> Option<WorthQueryGraphReadMaterializationResourceLimitReceipt> {
        if self.request.estimated_resident_bytes()
            <= self.target_progress.admitted_limits().max_resident_bytes()
            && self.request.estimated_touched_edges()
                <= self.target_progress.admitted_limits().max_touched_edges()
        {
            return None;
        }
        if self.checkpoint.sequence() == 0 {
            self.advance_to_next_checkpoint();
        }
        self.state = WorthQueryGraphReadMaterializationJobState::Indeterminate;
        Some(
            WorthQueryGraphReadMaterializationResourceLimitReceipt::from_limit_breach(
                self.digest,
                &self.progress,
                &self.checkpoint,
                self.request.estimated_resident_bytes(),
                self.state,
            ),
        )
    }

    pub(crate) fn running(
        request: WorthQueryGraphReadMaterializationRequest,
        snapshot_identity: impl Into<String>,
        initial_progress: WorthQueryGraphReadMaterializationProgress,
        target_progress: WorthQueryGraphReadMaterializationProgress,
        initial_checkpoint: WorthQueryGraphReadMaterializationCheckpoint,
    ) -> Self {
        let snapshot_identity = snapshot_identity.into();
        let state = WorthQueryGraphReadMaterializationJobState::Running;
        let digest = hash_parts(&[
            "worth_query_graph_read_materialization_job_v1".to_string(),
            format!("request:{}", request.digest()),
            format!("snapshot:{snapshot_identity}"),
            format!("progress:{}", initial_progress.digest()),
            format!("target_progress:{}", target_progress.digest()),
            format!("checkpoint:{}", initial_checkpoint.digest()),
            format!("state:{}", state.as_str()),
        ]);
        Self {
            digest,
            request,
            snapshot_identity,
            progress: initial_progress,
            target_progress,
            checkpoint: initial_checkpoint.clone(),
            checkpoints: vec![initial_checkpoint],
            state,
        }
    }
}

fn checkpoint_progress(
    request_digest: &str,
    admitted_limits: WorthQueryGraphReadMaterializationAdmittedLimits,
    target_progress: &WorthQueryGraphReadMaterializationProgress,
    checkpoint_sequence: usize,
) -> WorthQueryGraphReadMaterializationProgress {
    let target_checkpoint_count = target_progress.checkpoint_count().max(1);
    let bounded_sequence = checkpoint_sequence.min(target_checkpoint_count);
    let counters = WorthQueryGraphReadMaterializationCounters::new(
        scaled_counter(
            target_progress.touched_edges(),
            bounded_sequence,
            target_checkpoint_count,
        ),
        scaled_counter(
            target_progress.frontier_pages(),
            bounded_sequence,
            target_checkpoint_count,
        ),
        scaled_counter(
            target_progress.allocated_bytes(),
            bounded_sequence,
            target_checkpoint_count,
        ),
        scaled_counter(
            target_progress.emitted_rows(),
            bounded_sequence,
            target_checkpoint_count,
        ),
        bounded_sequence,
        bounded_sequence,
    );
    WorthQueryGraphReadMaterializationProgress::from_request_parts(
        request_digest,
        admitted_limits,
        counters,
    )
}

fn scaled_counter(total: usize, sequence: usize, checkpoint_count: usize) -> usize {
    total.saturating_mul(sequence).div_ceil(checkpoint_count)
}
