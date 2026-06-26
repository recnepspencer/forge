#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct WorthUiArtifactEquivalenceMetrics {
    modules_compared: usize,
    nodes_compared: usize,
    semantic_payloads_compared: usize,
    broad_scans: usize,
}

impl WorthUiArtifactEquivalenceMetrics {
    pub(crate) fn record_module_compared(&mut self) {
        self.modules_compared += 1;
    }

    pub(crate) fn record_node_compared(&mut self) {
        self.nodes_compared += 1;
    }

    pub(crate) fn record_semantic_payload_compared(&mut self) {
        self.semantic_payloads_compared += 1;
    }

    pub(crate) fn modules_compared(&self) -> usize {
        self.modules_compared
    }

    pub(crate) fn nodes_compared(&self) -> usize {
        self.nodes_compared
    }

    pub(crate) fn semantic_payloads_compared(&self) -> usize {
        self.semantic_payloads_compared
    }

    pub(crate) fn broad_scans(&self) -> usize {
        self.broad_scans
    }
}
