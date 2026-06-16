use std::collections::BTreeSet;

use super::{WorthUiRuntimeFactFamily, WorthUiRuntimeFactId};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorthUiRuntimeFactSet {
    facts: BTreeSet<WorthUiRuntimeFactId>,
}

impl WorthUiRuntimeFactSet {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn single(fact: WorthUiRuntimeFactId) -> Self {
        let mut facts = BTreeSet::new();
        facts.insert(fact);
        Self { facts }
    }

    pub fn insert(&mut self, fact: WorthUiRuntimeFactId) {
        self.facts.insert(fact);
    }

    pub fn extend(&mut self, facts: impl IntoIterator<Item = WorthUiRuntimeFactId>) {
        self.facts.extend(facts);
    }

    pub fn contains(&self, fact: &WorthUiRuntimeFactId) -> bool {
        self.facts.contains(fact)
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.facts.iter().any(|fact| other.contains(fact))
    }

    pub fn contains_family(&self, family: WorthUiRuntimeFactFamily) -> bool {
        self.facts.iter().any(|fact| fact.family() == family)
    }

    pub fn len(&self) -> usize {
        self.facts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.facts.is_empty()
    }

    pub fn facts(&self) -> impl Iterator<Item = &WorthUiRuntimeFactId> {
        self.facts.iter()
    }
}
