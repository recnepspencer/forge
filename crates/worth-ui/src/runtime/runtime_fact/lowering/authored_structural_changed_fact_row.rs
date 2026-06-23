use crate::runtime::{
    WorthUiRuntimeFactFamily, WorthUiRuntimeFactSet, WorthUiTouchedAuthoredSemanticSliceRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAuthoredStructuralChangedFactRow {
    semantic_row: WorthUiTouchedAuthoredSemanticSliceRow,
    changed_facts: WorthUiRuntimeFactSet,
    changed_fact_families: Vec<WorthUiRuntimeFactFamily>,
}

impl WorthUiAuthoredStructuralChangedFactRow {
    pub(crate) fn new(
        semantic_row: WorthUiTouchedAuthoredSemanticSliceRow,
        changed_facts: WorthUiRuntimeFactSet,
    ) -> Self {
        let mut changed_fact_families = changed_facts
            .facts()
            .map(|fact| fact.family())
            .collect::<Vec<_>>();
        changed_fact_families.dedup();
        Self {
            semantic_row,
            changed_facts,
            changed_fact_families,
        }
    }

    pub fn semantic_row(&self) -> &WorthUiTouchedAuthoredSemanticSliceRow {
        &self.semantic_row
    }

    pub fn changed_facts(&self) -> &WorthUiRuntimeFactSet {
        &self.changed_facts
    }

    pub fn changed_fact_count(&self) -> usize {
        self.changed_facts.len()
    }

    pub fn changed_fact_families(&self) -> &[WorthUiRuntimeFactFamily] {
        &self.changed_fact_families
    }
}
