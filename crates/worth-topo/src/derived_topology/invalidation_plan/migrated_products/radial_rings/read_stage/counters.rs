use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct RadialRingReadStageCounters {
    touched_anchor_count: usize,
    half_edge_lookup_count: usize,
    radial_relation_lookup_count: usize,
    selected_radial_root_count: usize,
    selected_source_row_count: usize,
    available_source_row_count: usize,
    touched_neighborhood_breadth_count: usize,
    unrelated_source_breadth_count: usize,
    whole_view_fallback_count: usize,
}

impl RadialRingReadStageCounters {
    pub const fn new(
        touched_anchor_count: usize,
        half_edge_lookup_count: usize,
        radial_relation_lookup_count: usize,
        selected_radial_root_count: usize,
        selected_source_row_count: usize,
        available_source_row_count: usize,
        touched_neighborhood_breadth_count: usize,
        unrelated_source_breadth_count: usize,
        whole_view_fallback_count: usize,
    ) -> Self {
        Self {
            touched_anchor_count,
            half_edge_lookup_count,
            radial_relation_lookup_count,
            selected_radial_root_count,
            selected_source_row_count,
            available_source_row_count,
            touched_neighborhood_breadth_count,
            unrelated_source_breadth_count,
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
            selected_source_row_count,
            available_source_row_count,
            selected_source_row_count,
            available_source_row_count.saturating_sub(selected_source_row_count),
            0,
        )
    }

    pub const fn touched_anchor_count(&self) -> usize {
        self.touched_anchor_count
    }

    pub const fn half_edge_lookup_count(&self) -> usize {
        self.half_edge_lookup_count
    }

    pub const fn radial_relation_lookup_count(&self) -> usize {
        self.radial_relation_lookup_count
    }

    pub const fn selected_radial_root_count(&self) -> usize {
        self.selected_radial_root_count
    }

    pub const fn selected_source_row_count(&self) -> usize {
        self.selected_source_row_count
    }

    pub const fn available_source_row_count(&self) -> usize {
        self.available_source_row_count
    }

    pub const fn touched_neighborhood_breadth_count(&self) -> usize {
        self.touched_neighborhood_breadth_count
    }

    pub const fn unrelated_source_breadth_count(&self) -> usize {
        self.unrelated_source_breadth_count
    }

    pub const fn whole_view_fallback_count(&self) -> usize {
        self.whole_view_fallback_count
    }
}
