use crate::runtime::WorthUiRuntimeFactId;

use super::WorthUiPrimitiveChangedFactEvidenceRow;

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveProjectionRebindPlan {
    rebuilt_facts: Vec<WorthUiRuntimeFactId>,
    preserved_facts: Vec<WorthUiRuntimeFactId>,
}

impl WorthUiPrimitiveProjectionRebindPlan {
    pub(crate) fn from_changed_rows<'a>(
        dependency_facts: impl Iterator<Item = &'a WorthUiRuntimeFactId>,
        changed_rows: &[WorthUiPrimitiveChangedFactEvidenceRow],
    ) -> Self {
        let changed_facts = changed_rows
            .iter()
            .flat_map(|row| row.changed_facts())
            .collect::<Vec<_>>();
        let mut rebuilt_facts = Vec::new();
        let mut preserved_facts = Vec::new();
        for fact in dependency_facts {
            if changed_facts.iter().any(|changed| *changed == fact) {
                rebuilt_facts.push(fact.clone());
            } else {
                preserved_facts.push(fact.clone());
            }
        }
        Self {
            rebuilt_facts,
            preserved_facts,
        }
    }

    pub fn rebuilt_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.rebuilt_facts
    }

    pub fn preserved_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.preserved_facts
    }

    pub fn has_rebuilt_facts(&self) -> bool {
        !self.rebuilt_facts.is_empty()
    }
}
