use super::{
    ForgeQueryGraphReadMaterializationCheckpoint, ForgeQueryGraphReadMaterializationJobState,
    ForgeQueryGraphReadMaterializationProgress,
};
use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadMaterializationCancellationReceipt {
    digest: String,
    job_digest: String,
    request_digest: String,
    progress_digest: String,
    last_checkpoint_digest: String,
    released_frontier_pages: usize,
    released_allocated_bytes: usize,
    cancellation_poll_count: usize,
    cancellation_scope: String,
    final_job_state: ForgeQueryGraphReadMaterializationJobState,
}

impl ForgeQueryGraphReadMaterializationCancellationReceipt {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn job_digest(&self) -> &str {
        &self.job_digest
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn progress_digest(&self) -> &str {
        &self.progress_digest
    }

    pub fn last_checkpoint_digest(&self) -> &str {
        &self.last_checkpoint_digest
    }

    pub fn released_frontier_pages(&self) -> usize {
        self.released_frontier_pages
    }

    pub fn released_allocated_bytes(&self) -> usize {
        self.released_allocated_bytes
    }

    pub fn cancellation_poll_count(&self) -> usize {
        self.cancellation_poll_count
    }

    pub fn cancellation_scope(&self) -> &str {
        &self.cancellation_scope
    }

    pub fn final_job_state(&self) -> &ForgeQueryGraphReadMaterializationJobState {
        &self.final_job_state
    }

    pub(crate) fn from_job_progress(
        job_digest: impl Into<String>,
        cancellation_scope: impl Into<String>,
        progress: &ForgeQueryGraphReadMaterializationProgress,
        last_checkpoint: &ForgeQueryGraphReadMaterializationCheckpoint,
        final_job_state: ForgeQueryGraphReadMaterializationJobState,
    ) -> Self {
        let job_digest = job_digest.into();
        let cancellation_scope = cancellation_scope.into();
        let request_digest = progress.request_digest().to_string();
        let progress_digest = progress.digest().to_string();
        let last_checkpoint_digest = last_checkpoint.digest().to_string();
        let released_frontier_pages = progress.frontier_pages();
        let released_allocated_bytes = progress.allocated_bytes();
        let cancellation_poll_count = progress.cancellation_poll_count();
        let digest = hash_parts(&[
            "forge_query_graph_read_materialization_cancellation_receipt_v1".to_string(),
            format!("job:{job_digest}"),
            format!("request:{request_digest}"),
            format!("progress:{progress_digest}"),
            format!("last_checkpoint:{last_checkpoint_digest}"),
            format!("released_frontier_pages:{released_frontier_pages}"),
            format!("released_allocated_bytes:{released_allocated_bytes}"),
            format!("cancellation_poll_count:{cancellation_poll_count}"),
            format!("cancellation_scope:{cancellation_scope}"),
            format!("final_job_state:{}", final_job_state.as_str()),
        ]);
        Self {
            digest,
            job_digest,
            request_digest,
            progress_digest,
            last_checkpoint_digest,
            released_frontier_pages,
            released_allocated_bytes,
            cancellation_poll_count,
            cancellation_scope,
            final_job_state,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadMaterializationRecoveryHandle {
    digest: String,
    job_digest: String,
    request_digest: String,
    last_checkpoint_digest: String,
    progress_digest: String,
    recovery_reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadMaterializationResourceLimitReceipt {
    digest: String,
    job_digest: String,
    request_digest: String,
    progress_digest: String,
    last_checkpoint_digest: String,
    estimated_resident_bytes: usize,
    max_resident_bytes: usize,
    final_job_state: ForgeQueryGraphReadMaterializationJobState,
}

impl ForgeQueryGraphReadMaterializationResourceLimitReceipt {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn job_digest(&self) -> &str {
        &self.job_digest
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn progress_digest(&self) -> &str {
        &self.progress_digest
    }

    pub fn last_checkpoint_digest(&self) -> &str {
        &self.last_checkpoint_digest
    }

    pub fn estimated_resident_bytes(&self) -> usize {
        self.estimated_resident_bytes
    }

    pub fn max_resident_bytes(&self) -> usize {
        self.max_resident_bytes
    }

    pub fn final_job_state(&self) -> &ForgeQueryGraphReadMaterializationJobState {
        &self.final_job_state
    }

    pub(crate) fn from_limit_breach(
        job_digest: impl Into<String>,
        progress: &ForgeQueryGraphReadMaterializationProgress,
        last_checkpoint: &ForgeQueryGraphReadMaterializationCheckpoint,
        estimated_resident_bytes: usize,
        final_job_state: ForgeQueryGraphReadMaterializationJobState,
    ) -> Self {
        let job_digest = job_digest.into();
        let request_digest = progress.request_digest().to_string();
        let progress_digest = progress.digest().to_string();
        let last_checkpoint_digest = last_checkpoint.digest().to_string();
        let max_resident_bytes = progress.admitted_limits().max_resident_bytes();
        let digest = hash_parts(&[
            "forge_query_graph_read_materialization_resource_limit_receipt_v1".to_string(),
            format!("job:{job_digest}"),
            format!("request:{request_digest}"),
            format!("progress:{progress_digest}"),
            format!("last_checkpoint:{last_checkpoint_digest}"),
            format!("estimated_resident_bytes:{estimated_resident_bytes}"),
            format!("max_resident_bytes:{max_resident_bytes}"),
            format!("final_job_state:{}", final_job_state.as_str()),
        ]);
        Self {
            digest,
            job_digest,
            request_digest,
            progress_digest,
            last_checkpoint_digest,
            estimated_resident_bytes,
            max_resident_bytes,
            final_job_state,
        }
    }
}

impl ForgeQueryGraphReadMaterializationRecoveryHandle {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn job_digest(&self) -> &str {
        &self.job_digest
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn last_checkpoint_digest(&self) -> &str {
        &self.last_checkpoint_digest
    }

    pub fn progress_digest(&self) -> &str {
        &self.progress_digest
    }

    pub fn recovery_reason(&self) -> &str {
        &self.recovery_reason
    }

    pub(crate) fn from_checkpoint(
        job_digest: impl Into<String>,
        checkpoint: &ForgeQueryGraphReadMaterializationCheckpoint,
        progress: &ForgeQueryGraphReadMaterializationProgress,
        recovery_reason: impl Into<String>,
    ) -> Self {
        let job_digest = job_digest.into();
        let request_digest = checkpoint.request_digest().to_string();
        let last_checkpoint_digest = checkpoint.digest().to_string();
        let progress_digest = progress.digest().to_string();
        let recovery_reason = recovery_reason.into();
        let digest = hash_parts(&[
            "forge_query_graph_read_materialization_recovery_handle_v1".to_string(),
            format!("job:{job_digest}"),
            format!("request:{request_digest}"),
            format!("checkpoint:{last_checkpoint_digest}"),
            format!("progress:{progress_digest}"),
            format!("reason:{recovery_reason}"),
        ]);
        Self {
            digest,
            job_digest,
            request_digest,
            last_checkpoint_digest,
            progress_digest,
            recovery_reason,
        }
    }
}
