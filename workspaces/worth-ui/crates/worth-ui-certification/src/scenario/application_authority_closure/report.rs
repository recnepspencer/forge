#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApplicationAuthorityClosureReport {
    generation_changed_once: bool,
    file_rust_converged: bool,
    graph_node_count: usize,
    query_binding_count: usize,
    host_session_preserved: bool,
    planning_policy_family_count: u8,
    planning_classification_count: u8,
}

impl ApplicationAuthorityClosureReport {
    pub(super) fn new(
        generation_changed_once: bool,
        file_rust_converged: bool,
        graph_node_count: usize,
        query_binding_count: usize,
        host_session_preserved: bool,
        planning_policy_family_count: u8,
        planning_classification_count: u8,
    ) -> Self {
        Self {
            generation_changed_once,
            file_rust_converged,
            graph_node_count,
            query_binding_count,
            host_session_preserved,
            planning_policy_family_count,
            planning_classification_count,
        }
    }

    pub fn generation_changed_once(&self) -> bool {
        self.generation_changed_once
    }

    pub fn file_rust_converged(&self) -> bool {
        self.file_rust_converged
    }

    pub fn graph_node_count(&self) -> usize {
        self.graph_node_count
    }

    pub fn query_binding_count(&self) -> usize {
        self.query_binding_count
    }

    pub fn host_session_preserved(&self) -> bool {
        self.host_session_preserved
    }

    pub fn planning_policy_family_count(&self) -> u8 {
        self.planning_policy_family_count
    }

    pub fn planning_classification_count(&self) -> u8 {
        self.planning_classification_count
    }
}
