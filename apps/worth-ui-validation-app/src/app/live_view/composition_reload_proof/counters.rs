use super::rows::{
    ValidationLiveViewCompositionRebindDecision, ValidationLiveViewCompositionRebindRow,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ValidationLiveViewCompositionReloadCounters {
    compared_child_row_count: usize,
    rebind_row_count: usize,
    preserve_row_count: usize,
    source_reparse_count: usize,
    renderer_parse_count: usize,
}

impl ValidationLiveViewCompositionReloadCounters {
    pub(super) fn from_rows(rows: &[ValidationLiveViewCompositionRebindRow]) -> Self {
        let rebind_row_count = rows
            .iter()
            .filter(|row| row.decision() == ValidationLiveViewCompositionRebindDecision::Rebind)
            .count();
        Self {
            compared_child_row_count: rows.len(),
            rebind_row_count,
            preserve_row_count: rows.len() - rebind_row_count,
            source_reparse_count: 0,
            renderer_parse_count: 0,
        }
    }

    pub fn compared_child_row_count(self) -> usize {
        self.compared_child_row_count
    }

    pub fn rebind_row_count(self) -> usize {
        self.rebind_row_count
    }

    pub fn preserve_row_count(self) -> usize {
        self.preserve_row_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn renderer_parse_count(self) -> usize {
        self.renderer_parse_count
    }
}
