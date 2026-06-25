use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct VertexDiskReadStageCounters {
    touched_vertex_count: usize,
    touched_half_edge_lookup_count: usize,
    selected_vertex_disk_root_count: usize,
    selected_source_row_count: usize,
    available_source_row_count: usize,
    touched_incident_half_edge_count: usize,
    touched_incident_edge_count: usize,
    unrelated_vertex_disk_breadth_count: usize,
    whole_view_fallback_count: usize,
}

impl VertexDiskReadStageCounters {
    pub const fn new(
        touched_vertex_count: usize,
        touched_half_edge_lookup_count: usize,
        selected_vertex_disk_root_count: usize,
        selected_source_row_count: usize,
        available_source_row_count: usize,
        touched_incident_half_edge_count: usize,
        touched_incident_edge_count: usize,
        unrelated_vertex_disk_breadth_count: usize,
        whole_view_fallback_count: usize,
    ) -> Self {
        Self {
            touched_vertex_count,
            touched_half_edge_lookup_count,
            selected_vertex_disk_root_count,
            selected_source_row_count,
            available_source_row_count,
            touched_incident_half_edge_count,
            touched_incident_edge_count,
            unrelated_vertex_disk_breadth_count,
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
            selected_source_row_count,
            available_source_row_count.saturating_sub(selected_source_row_count),
            0,
        )
    }

    pub const fn touched_vertex_count(&self) -> usize {
        self.touched_vertex_count
    }

    pub const fn touched_half_edge_lookup_count(&self) -> usize {
        self.touched_half_edge_lookup_count
    }

    pub const fn selected_vertex_disk_root_count(&self) -> usize {
        self.selected_vertex_disk_root_count
    }

    pub const fn selected_source_row_count(&self) -> usize {
        self.selected_source_row_count
    }

    pub const fn available_source_row_count(&self) -> usize {
        self.available_source_row_count
    }

    pub const fn touched_incident_half_edge_count(&self) -> usize {
        self.touched_incident_half_edge_count
    }

    pub const fn touched_incident_edge_count(&self) -> usize {
        self.touched_incident_edge_count
    }

    pub const fn unrelated_vertex_disk_breadth_count(&self) -> usize {
        self.unrelated_vertex_disk_breadth_count
    }

    pub const fn whole_view_fallback_count(&self) -> usize {
        self.whole_view_fallback_count
    }
}
