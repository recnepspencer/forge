#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorkloadEvidenceStageIndexCounters {
    row_count: usize,
    indexed_stage_count: usize,
    duplicate_stage_count: usize,
    manual_row_count: usize,
    unadmitted_row_count: usize,
    boolean_row_count: usize,
    counterless_boolean_row_count: usize,
}

impl WorkloadEvidenceStageIndexCounters {
    pub(crate) fn new(input: WorkloadEvidenceStageIndexCounterInput) -> Self {
        Self {
            row_count: input.row_count,
            indexed_stage_count: input.indexed_stage_count,
            duplicate_stage_count: input.duplicate_stage_count,
            manual_row_count: input.manual_row_count,
            unadmitted_row_count: input.unadmitted_row_count,
            boolean_row_count: input.boolean_row_count,
            counterless_boolean_row_count: input.counterless_boolean_row_count,
        }
    }

    pub fn row_count(self) -> usize {
        self.row_count
    }

    pub fn indexed_stage_count(self) -> usize {
        self.indexed_stage_count
    }

    pub fn duplicate_stage_count(self) -> usize {
        self.duplicate_stage_count
    }

    pub fn manual_row_count(self) -> usize {
        self.manual_row_count
    }

    pub fn unadmitted_row_count(self) -> usize {
        self.unadmitted_row_count
    }

    pub fn boolean_row_count(self) -> usize {
        self.boolean_row_count
    }

    pub fn counterless_boolean_row_count(self) -> usize {
        self.counterless_boolean_row_count
    }
}

pub(crate) struct WorkloadEvidenceStageIndexCounterInput {
    pub row_count: usize,
    pub indexed_stage_count: usize,
    pub duplicate_stage_count: usize,
    pub manual_row_count: usize,
    pub unadmitted_row_count: usize,
    pub boolean_row_count: usize,
    pub counterless_boolean_row_count: usize,
}
