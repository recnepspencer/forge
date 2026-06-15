use crate::runtime::{WorthUiVisibleRangeDenial, WorthUiVisibleRangeDenialReason};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiVisibleRange {
    start_row: u32,
    row_count: u32,
    start_column: u32,
    column_count: u32,
}

impl WorthUiVisibleRange {
    pub fn rows(start_row: u32, row_count: u32) -> Result<Self, WorthUiVisibleRangeDenial> {
        reject_empty(row_count, WorthUiVisibleRangeDenialReason::EmptyRowRange)?;
        reject_overflow(start_row, row_count)?;
        Ok(Self {
            start_row,
            row_count,
            start_column: 0,
            column_count: 1,
        })
    }

    pub fn grid(
        start_row: u32,
        row_count: u32,
        start_column: u32,
        column_count: u32,
    ) -> Result<Self, WorthUiVisibleRangeDenial> {
        Self::rows(start_row, row_count)?.with_columns(start_column, column_count)
    }

    pub fn with_columns(
        mut self,
        start_column: u32,
        column_count: u32,
    ) -> Result<Self, WorthUiVisibleRangeDenial> {
        reject_empty(
            column_count,
            WorthUiVisibleRangeDenialReason::EmptyColumnRange,
        )?;
        reject_overflow(start_column, column_count)?;
        self.start_column = start_column;
        self.column_count = column_count;
        Ok(self)
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

    pub fn end_row_exclusive(self) -> u32 {
        self.start_row + self.row_count
    }

    pub fn end_column_exclusive(self) -> u32 {
        self.start_column + self.column_count
    }
}

fn reject_empty(
    count: u32,
    reason: WorthUiVisibleRangeDenialReason,
) -> Result<(), WorthUiVisibleRangeDenial> {
    if count == 0 {
        Err(WorthUiVisibleRangeDenial::new(reason))
    } else {
        Ok(())
    }
}

fn reject_overflow(start: u32, count: u32) -> Result<(), WorthUiVisibleRangeDenial> {
    start.checked_add(count).map(|_| ()).ok_or_else(|| {
        WorthUiVisibleRangeDenial::new(WorthUiVisibleRangeDenialReason::RangeOverflow)
    })
}
