use super::WorthQueryGraphReadCheckpointInterval;
use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadMaterializationPolicy {
    digest: String,
    max_resident_bytes: usize,
    max_touched_edges: usize,
    checkpoint_interval: WorthQueryGraphReadCheckpointInterval,
    cancellation_scope: String,
}

impl WorthQueryGraphReadMaterializationPolicy {
    pub fn bounded() -> Self {
        Self::new(
            64 * 1024,
            16 * 1024,
            WorthQueryGraphReadCheckpointInterval::frontier_pages(4),
            "graph-read-materialization:default-cancellation-scope",
        )
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn max_resident_bytes(&self) -> usize {
        self.max_resident_bytes
    }

    pub fn max_touched_edges(&self) -> usize {
        self.max_touched_edges
    }

    pub fn checkpoint_interval(&self) -> &WorthQueryGraphReadCheckpointInterval {
        &self.checkpoint_interval
    }

    pub fn cancellation_scope(&self) -> &str {
        &self.cancellation_scope
    }

    pub fn with_max_resident_bytes(mut self, max_resident_bytes: usize) -> Self {
        self.max_resident_bytes = max_resident_bytes;
        self.digest = self.recompute_digest();
        self
    }

    pub fn with_max_touched_edges(mut self, max_touched_edges: usize) -> Self {
        self.max_touched_edges = max_touched_edges;
        self.digest = self.recompute_digest();
        self
    }

    pub fn with_checkpoint_interval(
        mut self,
        checkpoint_interval: WorthQueryGraphReadCheckpointInterval,
    ) -> Self {
        self.checkpoint_interval = checkpoint_interval;
        self.digest = self.recompute_digest();
        self
    }

    pub fn with_cancellation_scope(mut self, cancellation_scope: impl Into<String>) -> Self {
        self.cancellation_scope = cancellation_scope.into();
        self.digest = self.recompute_digest();
        self
    }

    fn new(
        max_resident_bytes: usize,
        max_touched_edges: usize,
        checkpoint_interval: WorthQueryGraphReadCheckpointInterval,
        cancellation_scope: impl Into<String>,
    ) -> Self {
        let cancellation_scope = cancellation_scope.into();
        let digest = policy_digest(
            max_resident_bytes,
            max_touched_edges,
            &checkpoint_interval,
            &cancellation_scope,
        );
        Self {
            digest,
            max_resident_bytes,
            max_touched_edges,
            checkpoint_interval,
            cancellation_scope,
        }
    }

    fn recompute_digest(&self) -> String {
        policy_digest(
            self.max_resident_bytes,
            self.max_touched_edges,
            &self.checkpoint_interval,
            &self.cancellation_scope,
        )
    }
}

fn policy_digest(
    max_resident_bytes: usize,
    max_touched_edges: usize,
    checkpoint_interval: &WorthQueryGraphReadCheckpointInterval,
    cancellation_scope: &str,
) -> String {
    hash_parts(&[
        "worth_query_graph_read_materialization_policy_v1".to_string(),
        format!("max_resident_bytes:{max_resident_bytes}"),
        format!("max_touched_edges:{max_touched_edges}"),
        format!("checkpoint_interval:{}", checkpoint_interval.digest()),
        format!("cancellation_scope:{cancellation_scope}"),
    ])
}
