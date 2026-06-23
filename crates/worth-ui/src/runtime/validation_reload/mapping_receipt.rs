use crate::runtime::{
    WorthUiAuthoredDeltaSummary, WorthUiAuthoredStructuralChangedFactRow, WorthUiRuntimeFactSet,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiValidationChangedFactMappingReceipt {
    authored_delta_summary: WorthUiAuthoredDeltaSummary,
    rows: Vec<WorthUiAuthoredStructuralChangedFactRow>,
    changed_facts: WorthUiRuntimeFactSet,
}

impl WorthUiValidationChangedFactMappingReceipt {
    pub(crate) fn new(
        authored_delta_summary: WorthUiAuthoredDeltaSummary,
        rows: Vec<WorthUiAuthoredStructuralChangedFactRow>,
        changed_facts: WorthUiRuntimeFactSet,
    ) -> Self {
        Self {
            authored_delta_summary,
            rows,
            changed_facts,
        }
    }

    pub fn authored_delta_summary(&self) -> &WorthUiAuthoredDeltaSummary {
        &self.authored_delta_summary
    }

    pub fn changed_facts(&self) -> &WorthUiRuntimeFactSet {
        &self.changed_facts
    }

    pub fn rows(&self) -> &[WorthUiAuthoredStructuralChangedFactRow] {
        &self.rows
    }
}
