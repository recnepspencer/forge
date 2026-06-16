use super::{WorthUiRuntimeFactId, WorthUiRuntimeFactSet};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorthUiProjectionDependencySet {
    facts: WorthUiRuntimeFactSet,
}

impl WorthUiProjectionDependencySet {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn depends_on(mut self, fact: WorthUiRuntimeFactId) -> Self {
        self.facts.insert(fact);
        self
    }

    pub fn merge(mut self, other: &Self) -> Self {
        self.facts.extend(other.facts().cloned());
        self
    }

    pub fn intersects(&self, changed_facts: &WorthUiRuntimeFactSet) -> bool {
        self.facts.intersects(changed_facts)
    }

    pub fn facts(&self) -> impl Iterator<Item = &WorthUiRuntimeFactId> {
        self.facts.facts()
    }

    pub fn len(&self) -> usize {
        self.facts.len()
    }
}
