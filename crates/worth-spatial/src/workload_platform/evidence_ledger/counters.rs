#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorkloadEvidenceCounters {
    rows: usize,
    boolean_rows: usize,
}

impl WorkloadEvidenceCounters {
    pub(crate) fn new(rows: usize, boolean_rows: usize) -> Self {
        Self { rows, boolean_rows }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn boolean_rows(&self) -> usize {
        self.boolean_rows
    }
}
