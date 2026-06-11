#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorkloadEvidenceCounters {
    rows: usize,
}

impl WorkloadEvidenceCounters {
    pub(crate) fn new(rows: usize) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }
}
