#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiGraphGenerationRelation {
    Equivalent,
    DirectSuccessor,
    DirectPredecessor,
    Unrelated,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct UiGraphGeneration {
    value: u64,
    predecessor: Option<u64>,
}

impl UiGraphGeneration {
    pub(crate) const fn initial() -> Self {
        Self {
            value: 1,
            predecessor: None,
        }
    }

    pub(crate) const fn successor_of(previous: Self) -> Self {
        Self {
            value: previous.value + 1,
            predecessor: Some(previous.value),
        }
    }

    pub fn as_u64(self) -> u64 {
        self.value
    }

    pub fn relation_to(self, other: Self) -> UiGraphGenerationRelation {
        if self == other {
            UiGraphGenerationRelation::Equivalent
        } else if self.predecessor == Some(other.value) {
            UiGraphGenerationRelation::DirectSuccessor
        } else if other.predecessor == Some(self.value) {
            UiGraphGenerationRelation::DirectPredecessor
        } else {
            UiGraphGenerationRelation::Unrelated
        }
    }
}
