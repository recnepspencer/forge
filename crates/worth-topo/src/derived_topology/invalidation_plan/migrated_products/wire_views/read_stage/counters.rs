use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WireViewReadStageCounters {
    touched_wire_count: usize,
    touched_half_edge_lookup_count: usize,
    selected_wire_root_count: usize,
    selected_source_row_count: usize,
    available_source_row_count: usize,
    touched_terminal_vertex_count: usize,
    touched_branch_vertex_count: usize,
    unrelated_wire_breadth_count: usize,
    whole_view_fallback_count: usize,
}

impl WireViewReadStageCounters {
    pub const fn new(
        touched_wire_count: usize,
        touched_half_edge_lookup_count: usize,
        selected_wire_root_count: usize,
        selected_source_row_count: usize,
        available_source_row_count: usize,
        touched_terminal_vertex_count: usize,
        touched_branch_vertex_count: usize,
        unrelated_wire_breadth_count: usize,
        whole_view_fallback_count: usize,
    ) -> Self {
        Self {
            touched_wire_count,
            touched_half_edge_lookup_count,
            selected_wire_root_count,
            selected_source_row_count,
            available_source_row_count,
            touched_terminal_vertex_count,
            touched_branch_vertex_count,
            unrelated_wire_breadth_count,
            whole_view_fallback_count,
        }
    }

    pub const fn for_selected_rows(
        selected_source_row_count: usize,
        available_source_row_count: usize,
    ) -> Self {
        Self::new(
            selected_source_row_count,
            selected_source_row_count,
            selected_source_row_count,
            selected_source_row_count,
            available_source_row_count,
            selected_source_row_count,
            0,
            available_source_row_count.saturating_sub(selected_source_row_count),
            0,
        )
    }

    pub const fn touched_wire_count(&self) -> usize {
        self.touched_wire_count
    }

    pub const fn touched_half_edge_lookup_count(&self) -> usize {
        self.touched_half_edge_lookup_count
    }

    pub const fn selected_wire_root_count(&self) -> usize {
        self.selected_wire_root_count
    }

    pub const fn selected_source_row_count(&self) -> usize {
        self.selected_source_row_count
    }

    pub const fn available_source_row_count(&self) -> usize {
        self.available_source_row_count
    }

    pub const fn touched_terminal_vertex_count(&self) -> usize {
        self.touched_terminal_vertex_count
    }

    pub const fn touched_branch_vertex_count(&self) -> usize {
        self.touched_branch_vertex_count
    }

    pub const fn unrelated_wire_breadth_count(&self) -> usize {
        self.unrelated_wire_breadth_count
    }

    pub const fn whole_view_fallback_count(&self) -> usize {
        self.whole_view_fallback_count
    }
}
