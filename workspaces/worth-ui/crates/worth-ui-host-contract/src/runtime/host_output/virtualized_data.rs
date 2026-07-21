#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiVirtualizedDataHostOutput {
    start_row: u32,
    row_count: u32,
    start_column: u32,
    column_count: u32,
    evidence_identity_digest: u64,
}

impl WorthUiVirtualizedDataHostOutput {
    pub fn new(
        start_row: u32,
        row_count: u32,
        start_column: u32,
        column_count: u32,
        evidence_identity_digest: u64,
    ) -> Self {
        Self {
            start_row,
            row_count,
            start_column,
            column_count,
            evidence_identity_digest,
        }
    }

    pub fn start_row(self) -> u32 {
        self.start_row
    }

    pub fn row_count(self) -> u32 {
        self.row_count
    }

    pub fn start_column(self) -> u32 {
        self.start_column
    }

    pub fn column_count(self) -> u32 {
        self.column_count
    }

    pub fn evidence_identity_digest(self) -> u64 {
        self.evidence_identity_digest
    }

    pub fn meaning_digest(self) -> u64 {
        u64::from(self.start_row)
            ^ u64::from(self.row_count).rotate_left(13)
            ^ u64::from(self.start_column).rotate_left(29)
            ^ u64::from(self.column_count).rotate_left(43)
            ^ self.evidence_identity_digest.rotate_left(53)
    }
}
