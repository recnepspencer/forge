use crate::runtime::{
    WorthUiIdentityMatchCounters, WorthUiIdentityMatchEdge, WorthUiIdentityMatchNode,
    WorthUiMovedNodeIdentity, WorthUiRepeatedTemplateIdentity,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiIdentityMatchGraph {
    active_nodes: Vec<WorthUiIdentityMatchNode>,
    candidate_nodes: Vec<WorthUiIdentityMatchNode>,
    matches: Vec<WorthUiIdentityMatchEdge>,
    repeated_template_identities: Vec<WorthUiRepeatedTemplateIdentity>,
    moved_node_identities: Vec<WorthUiMovedNodeIdentity>,
    counters: WorthUiIdentityMatchCounters,
}

impl WorthUiIdentityMatchGraph {
    pub(crate) fn new(
        active_nodes: Vec<WorthUiIdentityMatchNode>,
        candidate_nodes: Vec<WorthUiIdentityMatchNode>,
        matches: Vec<WorthUiIdentityMatchEdge>,
        repeated_template_identities: Vec<WorthUiRepeatedTemplateIdentity>,
        moved_node_identities: Vec<WorthUiMovedNodeIdentity>,
        counters: WorthUiIdentityMatchCounters,
    ) -> Self {
        Self {
            active_nodes,
            candidate_nodes,
            matches,
            repeated_template_identities,
            moved_node_identities,
            counters,
        }
    }

    #[cfg(test)]
    pub fn active_node_count(&self) -> usize {
        self.active_nodes.len()
    }

    #[cfg(test)]
    pub fn candidate_node_count(&self) -> usize {
        self.candidate_nodes.len()
    }

    #[cfg(test)]
    pub fn match_count(&self) -> usize {
        self.matches.len()
    }

    #[cfg(test)]
    pub fn moved_node_count(&self) -> usize {
        self.moved_node_identities.len()
    }

    #[cfg(test)]
    pub fn repeated_template_identity_count(&self) -> usize {
        self.repeated_template_identities.len()
    }

    pub fn is_unambiguous(&self) -> bool {
        self.counters.duplicate_active_identity_count() == 0
            && self.counters.duplicate_candidate_identity_count() == 0
            && self.counters.identity_kind_mismatch_count() == 0
    }

    pub fn counters(&self) -> WorthUiIdentityMatchCounters {
        self.counters
    }

    pub fn active_nodes(&self) -> &[WorthUiIdentityMatchNode] {
        &self.active_nodes
    }

    pub fn candidate_nodes(&self) -> &[WorthUiIdentityMatchNode] {
        &self.candidate_nodes
    }

    pub fn matches(&self) -> &[WorthUiIdentityMatchEdge] {
        &self.matches
    }

    #[cfg(test)]
    pub fn moved_node_identities(&self) -> &[WorthUiMovedNodeIdentity] {
        &self.moved_node_identities
    }
}
