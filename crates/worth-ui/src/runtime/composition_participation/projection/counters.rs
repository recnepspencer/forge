use super::receipt::WorthUiCompositionParticipationCounters;

impl WorthUiCompositionParticipationCounters {
    pub fn accessibility_node_count(self) -> usize {
        self.accessibility_node_count
    }

    pub fn focus_node_count(self) -> usize {
        self.focus_node_count
    }

    pub fn focus_scope_count(self) -> usize {
        self.focus_scope_count
    }

    pub fn association_count(self) -> usize {
        self.association_count
    }

    pub fn relationship_count(self) -> usize {
        self.relationship_count
    }

    pub fn selected_graph_obligation_count(self) -> usize {
        self.selected_graph_obligation_count
    }

    pub fn graph_child_row_count(self) -> usize {
        self.graph_child_row_count
    }

    pub fn caller_owned_recursive_walk_count(self) -> usize {
        self.caller_owned_recursive_walk_count
    }

    pub fn caller_owned_scan_count(self) -> usize {
        self.caller_owned_scan_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn renderer_parse_count(self) -> usize {
        self.renderer_parse_count
    }
}
