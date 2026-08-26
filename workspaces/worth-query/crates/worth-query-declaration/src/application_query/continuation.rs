use super::{
    ApplicationQueryCardinality, ApplicationQueryMarkerIdentity, ApplicationQueryResultRelationRef,
    ApplicationQueryResultTraversal, ApplicationQueryResultTraversalDirection, ManyResults,
};
use crate::portable_identity::{WorthQueryPortableType, WorthQueryPortableTypeIdentity};

/// Identity-bearing declaration of the one result collection advanced by an
/// application-query continuation.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationQueryContinuationTarget {
    query_type: WorthQueryPortableTypeIdentity,
    slot_type: WorthQueryPortableTypeIdentity,
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
        Query: ApplicationQueryMarkerIdentity,
        Slot: WorthQueryPortableType,
    {
        Self {
            query_type: relation.slot_key().query_identity(),
            slot_type: relation.slot_key().slot_identity(),
            relation: relation.relation(),
            parent_entity: relation.parent(),
            child_entity: relation.child(),
            direction: relation.direction(),
        }
    }

    pub const fn query_type(&self) -> &'static str {
        self.query_type.as_str()
    }

    pub const fn query_identity(&self) -> WorthQueryPortableTypeIdentity {
        self.query_type
    }

    pub const fn slot_type(&self) -> &'static str {
        self.slot_type.as_str()
    }

    pub const fn slot_identity(&self) -> WorthQueryPortableTypeIdentity {
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
