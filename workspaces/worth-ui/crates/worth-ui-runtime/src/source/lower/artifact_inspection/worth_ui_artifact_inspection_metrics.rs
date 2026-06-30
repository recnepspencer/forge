#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct WorthUiArtifactInspectionMetrics {
    modules_inspected: usize,
    nodes_inspected: usize,
    capability_references_recorded: usize,
    query_links_recorded: usize,
    broad_registry_scans: usize,
}

impl WorthUiArtifactInspectionMetrics {
    pub(crate) fn record_module_inspected(&mut self) {
        self.modules_inspected += 1;
    }

    pub(crate) fn record_node_inspected(&mut self) {
        self.nodes_inspected += 1;
    }

    pub(crate) fn record_capability_reference(&mut self) {
        self.capability_references_recorded += 1;
    }

    pub(crate) fn record_query_link(&mut self) {
        self.query_links_recorded += 1;
    }

    pub(crate) fn modules_inspected(&self) -> usize {
        self.modules_inspected
    }

    pub(crate) fn nodes_inspected(&self) -> usize {
        self.nodes_inspected
    }

    pub(crate) fn capability_references_recorded(&self) -> usize {
        self.capability_references_recorded
    }

    pub(crate) fn query_links_recorded(&self) -> usize {
        self.query_links_recorded
    }

    pub(crate) fn broad_registry_scans(&self) -> usize {
        self.broad_registry_scans
    }
}
