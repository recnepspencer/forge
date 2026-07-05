#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanOverlapArrangementGraphCounters {
    lowered_neighborhood_count: usize,
    emitted_boundary_segment_count: usize,
    emitted_boundary_component_count: usize,
    emitted_graph_row_count: usize,
    emitted_cell_count: usize,
    denied_neighborhood_count: usize,
}

impl PlanarBooleanOverlapArrangementGraphCounters {
    pub(crate) fn lowered_neighborhood(&mut self) {
        self.lowered_neighborhood_count += 1;
    }

    pub(crate) fn emitted_boundary_segment(&mut self) {
        self.emitted_boundary_segment_count += 1;
    }

    pub(crate) fn emitted_boundary_component(&mut self) {
        self.emitted_boundary_component_count += 1;
    }

    pub(crate) fn emitted_graph_row(&mut self) {
        self.emitted_graph_row_count += 1;
    }

    pub(crate) fn emitted_cell(&mut self) {
        self.emitted_cell_count += 1;
    }

    pub(crate) fn denied_neighborhood(&mut self) {
        self.denied_neighborhood_count += 1;
    }

    pub fn lowered_neighborhood_count(&self) -> usize {
        self.lowered_neighborhood_count
    }

    pub fn emitted_graph_row_count(&self) -> usize {
        self.emitted_graph_row_count
    }

    pub fn emitted_boundary_segment_count(&self) -> usize {
        self.emitted_boundary_segment_count
    }

    pub fn emitted_boundary_component_count(&self) -> usize {
        self.emitted_boundary_component_count
    }

    pub fn emitted_cell_count(&self) -> usize {
        self.emitted_cell_count
    }

    pub fn denied_neighborhood_count(&self) -> usize {
        self.denied_neighborhood_count
    }
}
