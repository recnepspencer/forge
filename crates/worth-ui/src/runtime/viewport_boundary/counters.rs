#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiViewportBoundaryCounters {
    boundary_count: usize,
    descendant_count: usize,
    clipped_descendant_count: usize,
    selected_graph_obligation_count: usize,
    source_reparse_count: usize,
    renderer_parse_count: usize,
    artifact_scan_count: usize,
}

impl WorthUiViewportBoundaryCounters {
    pub(super) fn new(
        boundary_count: usize,
        descendant_count: usize,
        clipped_descendant_count: usize,
        selected_graph_obligation_count: usize,
    ) -> Self {
        Self {
            boundary_count,
            descendant_count,
            clipped_descendant_count,
            selected_graph_obligation_count,
            source_reparse_count: 0,
            renderer_parse_count: 0,
            artifact_scan_count: 0,
        }
    }

    pub fn boundary_count(self) -> usize {
        self.boundary_count
    }

    pub fn descendant_count(self) -> usize {
        self.descendant_count
    }

    pub fn clipped_descendant_count(self) -> usize {
        self.clipped_descendant_count
    }

    pub fn selected_graph_obligation_count(self) -> usize {
        self.selected_graph_obligation_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn renderer_parse_count(self) -> usize {
        self.renderer_parse_count
    }

    pub fn artifact_scan_count(self) -> usize {
        self.artifact_scan_count
    }
}
