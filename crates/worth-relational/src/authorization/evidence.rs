use worth_foundational::facade::AspectFieldLocator;

use crate::identity::data::{EntityId, KindId, RelationId};
use crate::snapshots::data::SnapshotHandle;

use super::{
    RelationalAuthorizationEffectTarget, RelationalAuthorizationPathEffect,
    RelationalAuthorizationTraversalDirection,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RelationalAuthorizationPlanIdentity(pub(crate) [u8; 32]);

impl RelationalAuthorizationPlanIdentity {
    pub(crate) const fn uninitialized() -> Self {
        Self([0; 32])
    }

    pub const fn bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RelationalAuthorizationObservationIdentity(pub(crate) [u8; 32]);

impl RelationalAuthorizationObservationIdentity {
    pub const fn bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelationalAuthorizationDecision {
    Allowed,
    Denied,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RelationalAuthorizationObservationCounters {
    pub paths_evaluated: usize,
    pub adjacency_lists_read: usize,
    pub adjacency_edges_inspected: usize,
    pub relation_records_inspected: usize,
    pub entity_records_inspected: usize,
    pub predicate_fields_inspected: usize,
    pub maximum_frontier_width: usize,
    pub reconstructive_graph_scans: usize,
    pub reconstructive_relation_records_scanned: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub struct RelationalAuthorizationPathObservation {
    effect: RelationalAuthorizationPathEffect,
    matched: bool,
    dependencies: RelationalAuthorizationPathDependencies,
    exhaustive: bool,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RelationalAuthorizationAdjacencyDependency {
    entity: EntityId,
    relation_kind: KindId,
    direction: RelationalAuthorizationTraversalDirection,
}

impl RelationalAuthorizationAdjacencyDependency {
    pub(crate) const fn new(
        entity: EntityId,
        relation_kind: KindId,
        direction: RelationalAuthorizationTraversalDirection,
    ) -> Self {
        Self {
            entity,
            relation_kind,
            direction,
        }
    }

    pub const fn entity(&self) -> EntityId {
        self.entity
    }

    pub const fn relation_kind(&self) -> KindId {
        self.relation_kind
    }

    pub const fn direction(&self) -> RelationalAuthorizationTraversalDirection {
        self.direction
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct RelationalAuthorizationPathDependencies {
    pub(crate) entities: Vec<EntityId>,
    pub(crate) relations: Vec<RelationId>,
    pub(crate) adjacency_lists: Vec<RelationalAuthorizationAdjacencyDependency>,
    pub(crate) fields: Vec<(EntityId, AspectFieldLocator)>,
}

impl RelationalAuthorizationPathObservation {
    pub(crate) fn new(
        effect: RelationalAuthorizationPathEffect,
        matched: bool,
        dependencies: RelationalAuthorizationPathDependencies,
        exhaustive: bool,
    ) -> Self {
        Self {
            effect,
            matched,
            dependencies,
            exhaustive,
        }
    }

    pub const fn effect(&self) -> RelationalAuthorizationPathEffect {
        self.effect
    }

    pub const fn matched(&self) -> bool {
        self.matched
    }

    pub fn entities(&self) -> &[EntityId] {
        &self.dependencies.entities
    }

    pub fn relations(&self) -> &[RelationId] {
        &self.dependencies.relations
    }

    pub fn adjacency_lists(&self) -> &[RelationalAuthorizationAdjacencyDependency] {
        &self.dependencies.adjacency_lists
    }

    pub fn fields(&self) -> &[(EntityId, AspectFieldLocator)] {
        &self.dependencies.fields
    }

    pub const fn exhaustive(&self) -> bool {
        self.exhaustive
    }
}

/// Relational-owned evidence minted from one actual immutable snapshot read.
///
/// Private fields and the absence of `Clone`/deserialization prevent callers
/// from restamping descriptive touched identifiers into observation authority.
#[derive(Debug)]
pub struct RelationalAuthorizationObservationEvidence {
    snapshot: SnapshotHandle,
    plan_identity: RelationalAuthorizationPlanIdentity,
    observation_identity: RelationalAuthorizationObservationIdentity,
    principal: EntityId,
    scope: EntityId,
    decision: RelationalAuthorizationDecision,
    paths: Vec<RelationalAuthorizationPathObservation>,
    proposed_effects: Vec<RelationalAuthorizationEffectTarget>,
    counters: RelationalAuthorizationObservationCounters,
}

impl RelationalAuthorizationObservationEvidence {
    pub(crate) fn mint(
        snapshot: SnapshotHandle,
        plan_identity: RelationalAuthorizationPlanIdentity,
        observation_identity: RelationalAuthorizationObservationIdentity,
        principal: EntityId,
        scope: EntityId,
        decision: RelationalAuthorizationDecision,
        paths: Vec<RelationalAuthorizationPathObservation>,
        proposed_effects: Vec<RelationalAuthorizationEffectTarget>,
        counters: RelationalAuthorizationObservationCounters,
    ) -> Self {
        Self {
            snapshot,
            plan_identity,
            observation_identity,
            principal,
            scope,
            decision,
            paths,
            proposed_effects,
            counters,
        }
    }

    pub fn snapshot(&self) -> &SnapshotHandle {
        &self.snapshot
    }

    pub const fn plan_identity(&self) -> RelationalAuthorizationPlanIdentity {
        self.plan_identity
    }

    pub const fn observation_identity(&self) -> RelationalAuthorizationObservationIdentity {
        self.observation_identity
    }

    pub const fn principal(&self) -> EntityId {
        self.principal
    }

    pub const fn scope(&self) -> EntityId {
        self.scope
    }

    pub const fn decision(&self) -> RelationalAuthorizationDecision {
        self.decision
    }

    pub fn paths(&self) -> &[RelationalAuthorizationPathObservation] {
        &self.paths
    }

    pub fn proposed_effects(&self) -> &[RelationalAuthorizationEffectTarget] {
        &self.proposed_effects
    }

    pub const fn counters(&self) -> RelationalAuthorizationObservationCounters {
        self.counters
    }
}
