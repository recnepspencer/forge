use crate::runtime::WorthUiVisibleRange;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiVirtualizedDataLane {
    RowList,
    CellGrid,
}

impl WorthUiVirtualizedDataLane {
    pub(crate) fn from_visible_range(range: WorthUiVisibleRange) -> Self {
        if range.column_count() == 1 {
            Self::RowList
        } else {
            Self::CellGrid
        }
    }
}
