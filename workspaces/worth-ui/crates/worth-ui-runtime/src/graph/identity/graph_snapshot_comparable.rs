use crate::graph::{UiGraphGeneration, UiGraphGenerationRelation};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiGraphWorldDifferenceKind {
    SameWorldEquivalent,
    SameWorldSuccessor,
    SameWorldUnrelatedGeneration,
    SameDeclarationDifferentWorld,
    DifferentDeclarationAuthority,
    NotComparable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphSnapshotComparable {
    kind: UiGraphWorldDifferenceKind,
    generation_relation: UiGraphGenerationRelation,
    current_generation: UiGraphGeneration,
    compared_generation: UiGraphGeneration,
}

impl UiGraphSnapshotComparable {
    pub(crate) const fn new(
        kind: UiGraphWorldDifferenceKind,
        generation_relation: UiGraphGenerationRelation,
        current_generation: UiGraphGeneration,
        compared_generation: UiGraphGeneration,
    ) -> Self {
        Self {
            kind,
            generation_relation,
            current_generation,
            compared_generation,
        }
    }

    pub fn kind(self) -> UiGraphWorldDifferenceKind {
        self.kind
    }

    pub fn generation_relation(self) -> UiGraphGenerationRelation {
        self.generation_relation
    }

    pub fn current_generation(self) -> UiGraphGeneration {
        self.current_generation
    }

    pub fn compared_generation(self) -> UiGraphGeneration {
        self.compared_generation
    }
}
