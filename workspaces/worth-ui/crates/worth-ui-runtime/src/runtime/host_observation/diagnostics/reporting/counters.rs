#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiRuntimeDiagnosticCounters {
    source_row_count: usize,
    emitted_row_count: usize,
    phase_reference_count: usize,
    query_link_count: usize,
    support_section_count: usize,
    rich_materialization_count: usize,
}

impl WorthUiRuntimeDiagnosticCounters {
    pub(crate) fn record_source_row(&mut self) {
        self.source_row_count += 1;
    }

    pub(crate) fn record_emitted_row(&mut self) {
        self.emitted_row_count += 1;
    }

    pub(crate) fn record_phase_reference(&mut self) {
        self.phase_reference_count += 1;
    }

    pub(crate) fn record_support_section(&mut self) {
        self.support_section_count += 1;
    }

    pub(crate) fn record_rich_materialization(&mut self) {
        self.rich_materialization_count += 1;
    }

    pub fn source_row_count(self) -> usize {
        self.source_row_count
    }

    pub fn emitted_row_count(self) -> usize {
        self.emitted_row_count
    }

    pub fn phase_reference_count(self) -> usize {
        self.phase_reference_count
    }

    pub fn query_link_count(self) -> usize {
        self.query_link_count
    }

    pub fn support_section_count(self) -> usize {
        self.support_section_count
    }

    pub fn rich_materialization_count(self) -> usize {
        self.rich_materialization_count
    }
}
