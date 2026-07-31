use worth_foundational::facade::AspectFieldLocator;

use crate::identity::data::{EntityId, KindId};
use crate::snapshots::data::SnapshotHandle;
use crate::transactions::data::RecordRef;

use super::{
    identity::observation_plan_identity, RelationalAuthorizationEntityAnchor,
    RelationalAuthorizationPlanDenial, RelationalAuthorizationPlanIdentity,
    RelationalAuthorizationPredicate, RelationalAuthorizationRelatedEntityConstraint,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelationalAuthorizationPathEffect {
    Allow,
    Deny,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RelationalAuthorizationTraversalDirection {
    Forward,
    Reverse,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationalAuthorizationTraversal {
    relation_kind: KindId,
    from_kind: KindId,
    to_kind: KindId,
    direction: RelationalAuthorizationTraversalDirection,
}

impl RelationalAuthorizationTraversal {
    pub fn new(
        relation_kind: KindId,
        from_kind: KindId,
        to_kind: KindId,
        direction: RelationalAuthorizationTraversalDirection,
    ) -> Self {
        Self {
            relation_kind,
            from_kind,
            to_kind,
            direction,
        }
    }

    pub const fn relation_kind(&self) -> KindId {
        self.relation_kind
    }

    pub const fn from_kind(&self) -> KindId {
        self.from_kind
    }

    pub const fn to_kind(&self) -> KindId {
        self.to_kind
    }

    pub const fn direction(&self) -> RelationalAuthorizationTraversalDirection {
        self.direction
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationalAuthorizationPathPlan {
    effect: RelationalAuthorizationPathEffect,
    traversals: Vec<RelationalAuthorizationTraversal>,
    predicates: Vec<RelationalAuthorizationPredicate>,
    entity_anchors: Vec<RelationalAuthorizationEntityAnchor>,
    related_entities: Vec<RelationalAuthorizationRelatedEntityConstraint>,
}

impl RelationalAuthorizationPathPlan {
    pub fn new(
        effect: RelationalAuthorizationPathEffect,
        traversals: impl IntoIterator<Item = RelationalAuthorizationTraversal>,
        predicates: impl IntoIterator<Item = RelationalAuthorizationPredicate>,
    ) -> Self {
        Self {
            effect,
            traversals: traversals.into_iter().collect(),
            predicates: predicates.into_iter().collect(),
            entity_anchors: Vec::new(),
            related_entities: Vec::new(),
        }
    }

    pub fn with_entity_anchors(
        mut self,
        anchors: impl IntoIterator<Item = RelationalAuthorizationEntityAnchor>,
    ) -> Self {
        self.entity_anchors = anchors.into_iter().collect();
        self
    }

    pub fn with_related_entities(
        mut self,
        constraints: impl IntoIterator<Item = RelationalAuthorizationRelatedEntityConstraint>,
    ) -> Self {
        self.related_entities = constraints.into_iter().collect();
        self
    }

    pub const fn effect(&self) -> RelationalAuthorizationPathEffect {
        self.effect
    }

    pub fn traversals(&self) -> &[RelationalAuthorizationTraversal] {
        &self.traversals
    }

    pub fn predicates(&self) -> &[RelationalAuthorizationPredicate] {
        &self.predicates
    }

    pub fn entity_anchors(&self) -> &[RelationalAuthorizationEntityAnchor] {
        &self.entity_anchors
    }

    pub fn related_entities(&self) -> &[RelationalAuthorizationRelatedEntityConstraint] {
        &self.related_entities
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationalAuthorizationEffectTarget {
    record: RecordRef,
    field: Option<AspectFieldLocator>,
}

impl RelationalAuthorizationEffectTarget {
    pub fn record(record: RecordRef) -> Self {
        Self {
            record,
            field: None,
        }
    }

    pub fn field(record: RecordRef, field: AspectFieldLocator) -> Self {
        Self {
            record,
            field: Some(field),
        }
    }

    pub const fn record_ref(&self) -> &RecordRef {
        &self.record
    }

    pub fn field_locator(&self) -> Option<&AspectFieldLocator> {
        self.field.as_ref()
    }
}

#[derive(Debug)]
pub struct RelationalAuthorizationObservationPlan {
    snapshot: SnapshotHandle,
    principal: EntityId,
    scope: EntityId,
    principal_kind: KindId,
    scope_kind: KindId,
    paths: Vec<RelationalAuthorizationPathPlan>,
    proposed_effects: Vec<RelationalAuthorizationEffectTarget>,
    identity: RelationalAuthorizationPlanIdentity,
}

impl RelationalAuthorizationObservationPlan {
    pub fn try_new(
        snapshot: SnapshotHandle,
        principal: EntityId,
        scope: EntityId,
        principal_kind: KindId,
        scope_kind: KindId,
        paths: impl IntoIterator<Item = RelationalAuthorizationPathPlan>,
        proposed_effects: impl IntoIterator<Item = RelationalAuthorizationEffectTarget>,
    ) -> Result<Self, RelationalAuthorizationPlanDenial> {
        let mut plan = Self {
            snapshot,
            principal,
            scope,
            principal_kind,
            scope_kind,
            paths: paths.into_iter().collect(),
            proposed_effects: proposed_effects.into_iter().collect(),
            identity: RelationalAuthorizationPlanIdentity::uninitialized(),
        };
        validate_plan(&plan)?;
        plan.identity = observation_plan_identity(&plan);
        Ok(plan)
    }

    pub fn snapshot(&self) -> &SnapshotHandle {
        &self.snapshot
    }

    pub const fn principal(&self) -> EntityId {
        self.principal
    }

    pub const fn scope(&self) -> EntityId {
        self.scope
    }

    pub const fn principal_kind(&self) -> KindId {
        self.principal_kind
    }

    pub const fn scope_kind(&self) -> KindId {
        self.scope_kind
    }

    pub fn paths(&self) -> &[RelationalAuthorizationPathPlan] {
        &self.paths
    }

    pub fn proposed_effects(&self) -> &[RelationalAuthorizationEffectTarget] {
        &self.proposed_effects
    }

    pub const fn identity(&self) -> RelationalAuthorizationPlanIdentity {
        self.identity
    }

    pub(crate) fn comparison_at(
        &self,
        snapshot: SnapshotHandle,
    ) -> Result<Self, RelationalAuthorizationPlanDenial> {
        let plan = Self {
            snapshot,
            principal: self.principal,
            scope: self.scope,
            principal_kind: self.principal_kind,
            scope_kind: self.scope_kind,
            paths: self.paths.clone(),
            proposed_effects: self.proposed_effects.clone(),
            identity: RelationalAuthorizationPlanIdentity::uninitialized(),
        };
        validate_plan(&plan)?;
        Ok(plan)
    }
}

fn validate_plan(
    plan: &RelationalAuthorizationObservationPlan,
) -> Result<(), RelationalAuthorizationPlanDenial> {
    if plan.paths.is_empty() {
        return Err(RelationalAuthorizationPlanDenial::NoPaths);
    }
    if !plan
        .paths
        .iter()
        .any(|path| path.effect == RelationalAuthorizationPathEffect::Allow)
    {
        return Err(RelationalAuthorizationPlanDenial::NoAllowPath);
    }
    for (path_index, path) in plan.paths.iter().enumerate() {
        validate_path(plan, path_index, path)?;
    }
    Ok(())
}

fn validate_path(
    plan: &RelationalAuthorizationObservationPlan,
    path_index: usize,
    path: &RelationalAuthorizationPathPlan,
) -> Result<(), RelationalAuthorizationPlanDenial> {
    let mut kinds = Vec::with_capacity(path.traversals.len() + 1);
    kinds.push(plan.principal_kind);
    for (traversal_index, traversal) in path.traversals.iter().enumerate() {
        let expected = kinds[traversal_index];
        let (actual, next) = match traversal.direction {
            RelationalAuthorizationTraversalDirection::Forward => {
                (traversal.from_kind, traversal.to_kind)
            }
            RelationalAuthorizationTraversalDirection::Reverse => {
                (traversal.to_kind, traversal.from_kind)
            }
        };
        if actual != expected {
            return Err(RelationalAuthorizationPlanDenial::DiscontinuousTraversal {
                path: path_index,
                traversal: traversal_index,
                expected,
                actual,
            });
        }
        if traversal_index == 0 && actual != plan.principal_kind {
            return Err(RelationalAuthorizationPlanDenial::PathStartsAtWrongKind {
                path: path_index,
                expected: plan.principal_kind,
                actual,
            });
        }
        kinds.push(next);
    }
    let final_kind = *kinds.last().expect("principal kind is always present");
    if final_kind != plan.scope_kind {
        return Err(RelationalAuthorizationPlanDenial::PathEndsAtWrongKind {
            path: path_index,
            expected: plan.scope_kind,
            actual: final_kind,
        });
    }
    for predicate in &path.predicates {
        if predicate.field().field_path().fields().len() != 1 {
            return Err(
                RelationalAuthorizationPlanDenial::PredicateFieldPathNotSingle {
                    path: path_index,
                    ordinal: predicate.traversal_ordinal(),
                    fields: predicate.field().field_path().fields().len(),
                },
            );
        }
        let Some(expected) = kinds.get(predicate.traversal_ordinal()).copied() else {
            return Err(RelationalAuthorizationPlanDenial::PredicateOutsidePath {
                path: path_index,
                ordinal: predicate.traversal_ordinal(),
                traversals: path.traversals.len(),
            });
        };
        if predicate.entity_kind() != expected {
            return Err(
                RelationalAuthorizationPlanDenial::PredicateTargetsWrongKind {
                    path: path_index,
                    ordinal: predicate.traversal_ordinal(),
                    expected,
                    actual: predicate.entity_kind(),
                },
            );
        }
    }
    for anchor in &path.entity_anchors {
        let Some(expected) = kinds.get(anchor.traversal_ordinal()).copied() else {
            return Err(RelationalAuthorizationPlanDenial::EntityAnchorOutsidePath {
                path: path_index,
                ordinal: anchor.traversal_ordinal(),
                traversals: path.traversals.len(),
            });
        };
        if anchor.entity_kind() != expected {
            return Err(
                RelationalAuthorizationPlanDenial::EntityAnchorTargetsWrongKind {
                    path: path_index,
                    ordinal: anchor.traversal_ordinal(),
                    expected,
                    actual: anchor.entity_kind(),
                },
            );
        }
    }
    for related in &path.related_entities {
        let Some(expected) = kinds.get(related.traversal_ordinal()).copied() else {
            return Err(
                RelationalAuthorizationPlanDenial::RelatedEntityOutsidePath {
                    path: path_index,
                    ordinal: related.traversal_ordinal(),
                    traversals: path.traversals.len(),
                },
            );
        };
        let actual = traversal_start_kind(related.traversal());
        if actual != expected {
            return Err(
                RelationalAuthorizationPlanDenial::RelatedEntityStartsAtWrongKind {
                    path: path_index,
                    ordinal: related.traversal_ordinal(),
                    expected,
                    actual,
                },
            );
        }
    }
    Ok(())
}

const fn traversal_start_kind(traversal: &RelationalAuthorizationTraversal) -> KindId {
    match traversal.direction {
        RelationalAuthorizationTraversalDirection::Forward => traversal.from_kind,
        RelationalAuthorizationTraversalDirection::Reverse => traversal.to_kind,
    }
}
