#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct WorthUiArtifactAssemblyMetrics {
    modules_assembled: usize,
    nodes_assembled: usize,
    modules_with_reordered_nodes: usize,
    re_resolved_capability_count: usize,
    rechecked_legality_count: usize,
}

impl WorthUiArtifactAssemblyMetrics {
    pub(crate) fn record_module_assembled(&mut self) {
        self.modules_assembled += 1;
    }

    pub(crate) fn record_node_assembled(&mut self) {
        self.nodes_assembled += 1;
    }

    pub(crate) fn record_module_with_reordered_nodes(&mut self) {
        self.modules_with_reordered_nodes += 1;
    }

    pub(crate) fn modules_assembled(&self) -> usize {
        self.modules_assembled
    }

    pub(crate) fn nodes_assembled(&self) -> usize {
        self.nodes_assembled
    }

    pub(crate) fn modules_with_reordered_nodes(&self) -> usize {
        self.modules_with_reordered_nodes
    }

    pub(crate) fn re_resolved_capability_count(&self) -> usize {
        self.re_resolved_capability_count
    }

    pub(crate) fn rechecked_legality_count(&self) -> usize {
        self.rechecked_legality_count
    }
}
