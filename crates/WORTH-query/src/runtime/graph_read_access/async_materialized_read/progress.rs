use super::{WorthQueryGraphReadMaterializationCounters, WorthQueryGraphReadMaterializationPolicy};
use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadMaterializationAdmittedLimits {
    digest: String,
    max_resident_bytes: usize,
    max_touched_edges: usize,
}

impl WorthQueryGraphReadMaterializationAdmittedLimits {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn max_resident_bytes(&self) -> usize {
        self.max_resident_bytes
    }

    pub fn max_touched_edges(&self) -> usize {
        self.max_touched_edges
    }

    pub(crate) fn from_policy(policy: &WorthQueryGraphReadMaterializationPolicy) -> Self {
        let max_resident_bytes = policy.max_resident_bytes();
        let max_touched_edges = policy.max_touched_edges();
        let digest = hash_parts(&[
            "worth_query_graph_read_materialization_admitted_limits_v1".to_string(),
            format!("max_resident_bytes:{max_resident_bytes}"),
            format!("max_touched_edges:{max_touched_edges}"),
            format!("policy:{}", policy.digest()),
        ]);
        Self {
            digest,
            max_resident_bytes,
            max_touched_edges,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadMaterializationProgress {
    digest: String,
    request_digest: String,
    admitted_limits: WorthQueryGraphReadMaterializationAdmittedLimits,
    counters: WorthQueryGraphReadMaterializationCounters,
}

impl WorthQueryGraphReadMaterializationProgress {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn admitted_limits(&self) -> &WorthQueryGraphReadMaterializationAdmittedLimits {
        &self.admitted_limits
    }

    pub fn counters(&self) -> &WorthQueryGraphReadMaterializationCounters {
        &self.counters
    }

    pub fn touched_edges(&self) -> usize {
        self.counters.touched_edges()
    }

    pub fn frontier_pages(&self) -> usize {
        self.counters.frontier_pages()
    }

    pub fn allocated_bytes(&self) -> usize {
        self.counters.allocated_bytes()
    }

    pub fn emitted_rows(&self) -> usize {
        self.counters.emitted_rows()
    }

    pub fn checkpoint_count(&self) -> usize {
        self.counters.checkpoint_count()
    }

    pub fn cancellation_poll_count(&self) -> usize {
        self.counters.cancellation_poll_count()
    }

    pub(crate) fn from_request_parts(
        request_digest: impl Into<String>,
        admitted_limits: WorthQueryGraphReadMaterializationAdmittedLimits,
        counters: WorthQueryGraphReadMaterializationCounters,
    ) -> Self {
        let request_digest = request_digest.into();
        let digest = hash_parts(&[
            "worth_query_graph_read_materialization_progress_v1".to_string(),
            format!("request:{request_digest}"),
            format!("limits:{}", admitted_limits.digest()),
            format!("counters:{}", counters.digest()),
        ]);
        Self {
            digest,
            request_digest,
            admitted_limits,
            counters,
        }
    }
}
