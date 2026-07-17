impl super::UiAdmittedAllocationInvalidationTargetSet {
    pub fn graph_node_identities(&self) -> &[crate::graph::UiGraphNodeIdentity] {
        &self.touched_graph_node_identities
    }

    pub fn neighborhood_count(&self) -> usize {
        1 + self.widened.len()
    }

    pub(crate) fn widened(&self) -> &[super::UiAdmittedAllocationInvalidationTarget] {
        &self.widened
    }

    pub(crate) fn primary(&self) -> &super::UiAdmittedAllocationInvalidationTarget {
        &self.primary
    }

    pub(crate) fn with_graph_index_probes(mut self, probes: u16) -> Self {
        self.primary.graph_membership_probes = probes;
        for target in &mut self.widened {
            target.graph_membership_probes = probes;
        }
        self
    }
}
