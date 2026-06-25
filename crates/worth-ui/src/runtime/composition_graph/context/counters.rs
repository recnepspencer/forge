#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiCompositionContextCounters {
    node_context_count: usize,
    local_context_count: usize,
    override_count: usize,
    affected_consumer_count: usize,
    selected_graph_obligation_count: usize,
    graph_access_count: usize,
    source_reparse_count: usize,
    renderer_parse_count: usize,
}

impl WorthUiCompositionContextCounters {
    pub(crate) fn new(
        node_context_count: usize,
        local_context_count: usize,
        override_count: usize,
        affected_consumer_count: usize,
        selected_graph_obligation_count: usize,
        graph_access_count: usize,
    ) -> Self {
        Self {
            node_context_count,
            local_context_count,
            override_count,
            affected_consumer_count,
            selected_graph_obligation_count,
            graph_access_count,
            source_reparse_count: 0,
            renderer_parse_count: 0,
        }
    }

    pub fn node_context_count(self) -> usize {
        self.node_context_count
    }

    pub fn local_context_count(self) -> usize {
        self.local_context_count
    }

    pub fn override_count(self) -> usize {
        self.override_count
    }

    pub fn affected_consumer_count(self) -> usize {
        self.affected_consumer_count
    }

    pub fn selected_graph_obligation_count(self) -> usize {
        self.selected_graph_obligation_count
    }

    pub fn graph_access_count(self) -> usize {
        self.graph_access_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn renderer_parse_count(self) -> usize {
        self.renderer_parse_count
    }
}
