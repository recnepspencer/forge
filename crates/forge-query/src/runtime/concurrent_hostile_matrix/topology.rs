#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryConcurrentHostileMatrixTopology {
    reader_thread_count: usize,
    submitter_thread_count: usize,
    submission_round_count: usize,
}

impl ForgeQueryConcurrentHostileMatrixTopology {
    pub fn new(
        reader_thread_count: usize,
        submitter_thread_count: usize,
        submission_round_count: usize,
    ) -> Self {
        Self {
            reader_thread_count,
            submitter_thread_count,
            submission_round_count,
        }
    }

    pub fn reader_thread_count(self) -> usize {
        self.reader_thread_count
    }

    pub fn submitter_thread_count(self) -> usize {
        self.submitter_thread_count
    }

    pub fn submission_round_count(self) -> usize {
        self.submission_round_count
    }

    pub fn satisfies_phase_sixteen_minimums(self) -> bool {
        self.reader_thread_count >= 3
            && self.submitter_thread_count >= 2
            && self.submission_round_count > 0
    }
}
