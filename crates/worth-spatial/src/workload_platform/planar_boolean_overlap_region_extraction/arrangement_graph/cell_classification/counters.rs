#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanOverlapCellClassificationCounters {
    classified_cell_count: usize,
    emitted_containment_row_count: usize,
    emitted_winding_row_count: usize,
    denied_input_count: usize,
}

impl PlanarBooleanOverlapCellClassificationCounters {
    pub(crate) fn classified_cell(&mut self) {
        self.classified_cell_count += 1;
    }

    pub(crate) fn emitted_containment_row(&mut self) {
        self.emitted_containment_row_count += 1;
    }

    pub(crate) fn emitted_winding_row(&mut self) {
        self.emitted_winding_row_count += 1;
    }

    pub(crate) fn denied_input(&mut self) {
        self.denied_input_count += 1;
    }

    pub fn classified_cell_count(&self) -> usize {
        self.classified_cell_count
    }

    pub fn emitted_containment_row_count(&self) -> usize {
        self.emitted_containment_row_count
    }

    pub fn emitted_winding_row_count(&self) -> usize {
        self.emitted_winding_row_count
    }

    pub fn denied_input_count(&self) -> usize {
        self.denied_input_count
    }
}
