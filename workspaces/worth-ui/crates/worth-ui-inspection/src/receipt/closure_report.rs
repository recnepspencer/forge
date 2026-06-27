use crate::UiInspectionScopeSupportRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiInspectionClosureReport {
    rows: Box<[UiInspectionScopeSupportRow]>,
}

impl UiInspectionClosureReport {
    pub(crate) fn new(rows: &[UiInspectionScopeSupportRow]) -> Self {
        Self {
            rows: rows.to_vec().into_boxed_slice(),
        }
    }

    pub fn rows(&self) -> &[UiInspectionScopeSupportRow] {
        &self.rows
    }
}
