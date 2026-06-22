use super::singularity_counters::HighValenceSingularityCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HighValenceSingularityReceipt {
    singularity_digest: String,
    workload_identity: String,
    center_vertex_identity: String,
    local_rebuild_evidence_digest: String,
    counters: HighValenceSingularityCounters,
}

impl HighValenceSingularityReceipt {
    pub(crate) fn new(
        singularity_digest: String,
        workload_identity: String,
        center_vertex_identity: String,
        local_rebuild_evidence_digest: String,
        counters: HighValenceSingularityCounters,
    ) -> Self {
        Self {
            singularity_digest,
            workload_identity,
            center_vertex_identity,
            local_rebuild_evidence_digest,
            counters,
        }
    }

    pub fn singularity_digest(&self) -> &str {
        &self.singularity_digest
    }

    pub fn workload_identity(&self) -> &str {
        &self.workload_identity
    }

    pub fn center_vertex_identity(&self) -> &str {
        &self.center_vertex_identity
    }

    pub fn local_rebuild_evidence_digest(&self) -> &str {
        &self.local_rebuild_evidence_digest
    }

    pub fn counters(&self) -> HighValenceSingularityCounters {
        self.counters
    }
}
