use super::{
    ForgeQueryGraphReadMaterializationCheckpoint, ForgeQueryGraphReadMaterializationProgress,
    ForgeQueryGraphReadMaterializationRequest,
};
use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadMaterializationReceipt {
    digest: String,
    job_digest: String,
    request_digest: String,
    admission_digest: String,
    materialization_digest: String,
    final_progress_digest: String,
    final_checkpoint_digest: String,
    emitted_rows: usize,
    touched_edges: usize,
    max_resident_bytes_observed: usize,
    checkpoint_count: usize,
}

impl ForgeQueryGraphReadMaterializationReceipt {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn job_digest(&self) -> &str {
        &self.job_digest
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn admission_digest(&self) -> &str {
        &self.admission_digest
    }

    pub fn materialization_digest(&self) -> &str {
        &self.materialization_digest
    }

    pub fn final_progress_digest(&self) -> &str {
        &self.final_progress_digest
    }

    pub fn final_checkpoint_digest(&self) -> &str {
        &self.final_checkpoint_digest
    }

    pub fn emitted_rows(&self) -> usize {
        self.emitted_rows
    }

    pub fn touched_edges(&self) -> usize {
        self.touched_edges
    }

    pub fn max_resident_bytes_observed(&self) -> usize {
        self.max_resident_bytes_observed
    }

    pub fn checkpoint_count(&self) -> usize {
        self.checkpoint_count
    }

    pub(crate) fn from_completed_job(
        job_digest: impl Into<String>,
        request: &ForgeQueryGraphReadMaterializationRequest,
        progress: &ForgeQueryGraphReadMaterializationProgress,
        checkpoint: &ForgeQueryGraphReadMaterializationCheckpoint,
        snapshot_identity: &str,
    ) -> Self {
        let job_digest = job_digest.into();
        let request_digest = request.digest().to_string();
        let admission_digest = request.admission_digest().to_string();
        let final_progress_digest = progress.digest().to_string();
        let final_checkpoint_digest = checkpoint.digest().to_string();
        let emitted_rows = progress.emitted_rows();
        let touched_edges = progress.touched_edges();
        let max_resident_bytes_observed = progress.allocated_bytes();
        let checkpoint_count = progress.checkpoint_count();
        let materialization_digest = hash_parts(&[
            "forge_query_graph_read_materialization_output_v1".to_string(),
            format!("request:{request_digest}"),
            format!("snapshot:{snapshot_identity}"),
            format!("progress:{final_progress_digest}"),
            format!("checkpoint:{final_checkpoint_digest}"),
        ]);
        let digest = hash_parts(&[
            "forge_query_graph_read_materialization_receipt_v1".to_string(),
            format!("job:{job_digest}"),
            format!("request:{request_digest}"),
            format!("admission:{admission_digest}"),
            format!("materialization:{materialization_digest}"),
            format!("progress:{final_progress_digest}"),
            format!("checkpoint:{final_checkpoint_digest}"),
            format!("emitted_rows:{emitted_rows}"),
            format!("touched_edges:{touched_edges}"),
            format!("max_resident_bytes_observed:{max_resident_bytes_observed}"),
            format!("checkpoint_count:{checkpoint_count}"),
        ]);
        Self {
            digest,
            job_digest,
            request_digest,
            admission_digest,
            materialization_digest,
            final_progress_digest,
            final_checkpoint_digest,
            emitted_rows,
            touched_edges,
            max_resident_bytes_observed,
            checkpoint_count,
        }
    }
}
