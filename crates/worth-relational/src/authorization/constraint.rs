use worth_foundational::facade::{AspectFieldLocator, AspectValue};

use crate::identity::data::{EntityId, KindId};

use super::RelationalAuthorizationTraversal;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelationalAuthorizationFieldComparison {
    Equal,
    AtMost,
    AtLeast,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationalAuthorizationPredicate {
    traversal_ordinal: usize,
    entity_kind: KindId,
    field: AspectFieldLocator,
    comparison: RelationalAuthorizationFieldComparison,
    expected: AspectValue,
}

impl RelationalAuthorizationPredicate {
    pub fn new(
        traversal_ordinal: usize,
        entity_kind: KindId,
        field: AspectFieldLocator,
        expected: AspectValue,
    ) -> Self {
        Self::compare(
            traversal_ordinal,
            entity_kind,
            field,
            RelationalAuthorizationFieldComparison::Equal,
            expected,
        )
    }

    pub fn compare(
        traversal_ordinal: usize,
        entity_kind: KindId,
        field: AspectFieldLocator,
        comparison: RelationalAuthorizationFieldComparison,
        expected: AspectValue,
    ) -> Self {
        Self {
            traversal_ordinal,
            entity_kind,
            field,
            comparison,
            expected,
        }
    }

    pub const fn traversal_ordinal(&self) -> usize {
        self.traversal_ordinal
    }

    pub const fn entity_kind(&self) -> KindId {
        self.entity_kind
    }

    pub fn field(&self) -> &AspectFieldLocator {
        &self.field
    }

    pub const fn comparison(&self) -> RelationalAuthorizationFieldComparison {
        self.comparison
    }

    pub fn expected(&self) -> &AspectValue {
        &self.expected
    }

    pub(crate) fn matches(&self, observed: &AspectValue) -> bool {
        if observed.value_family() != self.expected.value_family() {
            return false;
        }
        match self.comparison {
            RelationalAuthorizationFieldComparison::Equal => observed == &self.expected,
            RelationalAuthorizationFieldComparison::AtMost => observed <= &self.expected,
            RelationalAuthorizationFieldComparison::AtLeast => observed >= &self.expected,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RelationalAuthorizationEntityAnchor {
    traversal_ordinal: usize,
    entity_kind: KindId,
    entity: EntityId,
}

impl RelationalAuthorizationEntityAnchor {
    pub const fn new(traversal_ordinal: usize, entity_kind: KindId, entity: EntityId) -> Self {
        Self {
            traversal_ordinal,
            entity_kind,
            entity,
        }
    }

    pub const fn traversal_ordinal(&self) -> usize {
        self.traversal_ordinal
    }

    pub const fn entity_kind(&self) -> KindId {
        self.entity_kind
    }

    pub const fn entity(&self) -> EntityId {
        self.entity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationalAuthorizationRelatedEntityConstraint {
    traversal_ordinal: usize,
    traversal: RelationalAuthorizationTraversal,
    entity: EntityId,
}

impl RelationalAuthorizationRelatedEntityConstraint {
    pub fn new(
        traversal_ordinal: usize,
        traversal: RelationalAuthorizationTraversal,
        entity: EntityId,
    ) -> Self {
        Self {
            traversal_ordinal,
            traversal,
            entity,
        }
    }

    pub const fn traversal_ordinal(&self) -> usize {
        self.traversal_ordinal
    }

    pub const fn traversal(&self) -> &RelationalAuthorizationTraversal {
        &self.traversal
    }

    pub const fn entity(&self) -> EntityId {
        self.entity
    }
}
