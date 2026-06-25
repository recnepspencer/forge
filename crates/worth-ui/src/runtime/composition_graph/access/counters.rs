#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiCompositionGraphAccessCounters {
    planned_index_family_count: usize,
    index_build_node_count: usize,
    index_build_edge_count: usize,
    index_build_policy_count: usize,
    request_child_row_count: usize,
    request_ancestor_row_count: usize,
    request_participation_row_count: usize,
    request_affected_consumer_row_count: usize,
    materialized_row_count: usize,
    caller_owned_recursive_walk_count: usize,
    caller_owned_scan_count: usize,
    source_reparse_count: usize,
    renderer_parse_count: usize,
}

impl WorthUiCompositionGraphAccessCounters {
    pub(super) fn planned(
        planned_index_family_count: usize,
        index_build_node_count: usize,
        index_build_edge_count: usize,
        index_build_policy_count: usize,
        request_child_row_count: usize,
        request_ancestor_row_count: usize,
        request_participation_row_count: usize,
        request_affected_consumer_row_count: usize,
    ) -> Self {
        let materialized_row_count = request_child_row_count
            + request_ancestor_row_count
            + request_participation_row_count
            + request_affected_consumer_row_count;
        Self {
            planned_index_family_count,
            index_build_node_count,
            index_build_edge_count,
            index_build_policy_count,
            request_child_row_count,
            request_ancestor_row_count,
            request_participation_row_count,
            request_affected_consumer_row_count,
            materialized_row_count,
            caller_owned_recursive_walk_count: 0,
            caller_owned_scan_count: 0,
            source_reparse_count: 0,
            renderer_parse_count: 0,
        }
    }

    pub fn planned_index_family_count(self) -> usize {
        self.planned_index_family_count
    }

    pub fn index_build_node_count(self) -> usize {
        self.index_build_node_count
    }

    pub fn index_build_edge_count(self) -> usize {
        self.index_build_edge_count
    }

    pub fn index_build_policy_count(self) -> usize {
        self.index_build_policy_count
    }

    pub fn request_child_row_count(self) -> usize {
        self.request_child_row_count
    }

    pub fn request_ancestor_row_count(self) -> usize {
        self.request_ancestor_row_count
    }

    pub fn request_participation_row_count(self) -> usize {
        self.request_participation_row_count
    }

    pub fn request_affected_consumer_row_count(self) -> usize {
        self.request_affected_consumer_row_count
    }

    pub fn materialized_row_count(self) -> usize {
        self.materialized_row_count
    }

    pub fn child_lookup_count(self) -> usize {
        self.request_child_row_count
    }

    pub fn ancestor_lookup_count(self) -> usize {
        self.request_ancestor_row_count
    }

    pub fn participation_filter_count(self) -> usize {
        self.request_participation_row_count
    }

    pub fn affected_consumer_lookup_count(self) -> usize {
        self.request_affected_consumer_row_count
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
