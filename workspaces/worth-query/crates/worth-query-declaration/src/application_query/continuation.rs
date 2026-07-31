use super::{
    ApplicationQueryCardinality, ApplicationQueryResultRelationRef,
    ApplicationQueryResultTraversal, ApplicationQueryResultTraversalDirection, ManyResults,
};

/// Identity-bearing declaration of the one result collection advanced by an
/// application-query continuation.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationQueryContinuationTarget {
    query_type: &'static str,
    slot_type: &'static str,
    relation: &'static str,
    parent_entity: &'static str,
    child_entity: &'static str,
    direction: ApplicationQueryResultTraversalDirection,
}

impl ApplicationQueryContinuationTarget {
    pub(super) fn from_many_relation<Query, Slot, Schema, Relation, From, To, Direction>(
        relation: ApplicationQueryResultRelationRef<
            Query,
            Slot,
            Schema,
            Relation,
            From,
            To,
            Direction,
            ManyResults,
        >,
    ) -> Self
    where
        Direction: ApplicationQueryResultTraversal,
    {
        Self {
            query_type: relation.query_type(),
            slot_type: relation.slot_type(),
            relation: relation.relation(),
            parent_entity: relation.parent(),
            child_entity: relation.child(),
            direction: relation.direction(),
        }
    }

    pub const fn query_type(&self) -> &'static str {
        self.query_type
    }

    pub const fn slot_type(&self) -> &'static str {
        self.slot_type
    }

    pub const fn relation(&self) -> &'static str {
        self.relation
    }

    pub const fn parent_entity(&self) -> &'static str {
        self.parent_entity
    }

    pub const fn child_entity(&self) -> &'static str {
        self.child_entity
    }

    pub const fn direction(&self) -> ApplicationQueryResultTraversalDirection {
        self.direction
    }

    pub const fn cardinality(&self) -> ApplicationQueryCardinality {
        ApplicationQueryCardinality::Many
    }
}
