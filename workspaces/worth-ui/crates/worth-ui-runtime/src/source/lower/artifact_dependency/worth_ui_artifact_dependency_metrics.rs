#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct WorthUiArtifactDependencyMetrics {
    nodes_indexed: usize,
    dependency_edges_recorded: usize,
    subtree_digests_recorded: usize,
    runtime_hooks_recorded: usize,
}

impl WorthUiArtifactDependencyMetrics {
    pub(crate) fn record_node_indexed(&mut self) {
        self.nodes_indexed += 1;
    }

    pub(crate) fn record_dependency_edge(&mut self) {
        self.dependency_edges_recorded += 1;
    }

    pub(crate) fn record_subtree_digest(&mut self) {
        self.subtree_digests_recorded += 1;
    }

    pub(crate) fn record_runtime_hook(&mut self) {
        self.runtime_hooks_recorded += 1;
    }

    #[cfg(test)]
    pub(crate) fn nodes_indexed(self) -> usize {
        self.nodes_indexed
    }

    #[cfg(test)]
    pub(crate) fn dependency_edges_recorded(self) -> usize {
        self.dependency_edges_recorded
    }

    #[cfg(test)]
    pub(crate) fn subtree_digests_recorded(self) -> usize {
        self.subtree_digests_recorded
    }

    #[cfg(test)]
    pub(crate) fn runtime_hooks_recorded(self) -> usize {
        self.runtime_hooks_recorded
    }
}
