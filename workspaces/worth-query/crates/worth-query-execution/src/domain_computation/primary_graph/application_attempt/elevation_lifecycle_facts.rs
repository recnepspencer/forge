use worth_foundational::facade::{AspectFieldLocator, AspectValue};
use worth_relational::facade::identity::{EntityId, KindId};

use super::WorthQueryApplicationObservedFact;

pub(super) enum WorthQueryExpectedLifecycleRelation {
    Absent,
    Present(EntityId),
}

pub(super) struct WorthQueryElevationLifecycleFactExpectation<'a> {
    pub(super) elevation: EntityId,
    pub(super) review: EntityId,
    pub(super) requester: EntityId,
    pub(super) approver: WorthQueryExpectedLifecycleRelation,
    pub(super) grant: EntityId,
    pub(super) reviewer: WorthQueryExpectedLifecycleRelation,
    pub(super) elevation_identity: (&'a AspectFieldLocator, &'a AspectValue),
    pub(super) reason: (&'a AspectFieldLocator, &'a AspectValue),
    pub(super) status: (&'a AspectFieldLocator, &'a AspectValue),
    pub(super) not_before: (&'a AspectFieldLocator, &'a AspectValue),
    pub(super) not_after: (&'a AspectFieldLocator, &'a AspectValue),
    pub(super) review_identity: (&'a AspectFieldLocator, &'a AspectValue),
    pub(super) review_status: (&'a AspectFieldLocator, &'a AspectValue),
    pub(super) requester_relation: KindId,
    pub(super) approver_relation: KindId,
    pub(super) grant_relation: KindId,
    pub(super) review_relation: KindId,
    pub(super) reviewer_relation: KindId,
}

pub(super) fn lifecycle_facts_are_exact(
    facts: &[WorthQueryApplicationObservedFact],
    expected: WorthQueryElevationLifecycleFactExpectation<'_>,
) -> bool {
    let fields = [
        (expected.elevation, expected.elevation_identity),
        (expected.elevation, expected.reason),
        (expected.elevation, expected.status),
        (expected.elevation, expected.not_before),
        (expected.elevation, expected.not_after),
        (expected.review, expected.review_identity),
        (expected.review, expected.review_status),
    ];
    facts.len() == 12
        && fields
            .into_iter()
            .all(|(entity, (locator, value))| exact_field(facts, entity, locator, value))
        && exact_relation(
            facts,
            expected.requester_relation,
            expected.requester,
            expected.elevation,
            1,
        )
        && expected_relation(
            facts,
            expected.approver_relation,
            expected.approver,
            expected.elevation,
        )
        && exact_relation(
            facts,
            expected.grant_relation,
            expected.elevation,
            expected.grant,
            1,
        )
        && exact_relation(
            facts,
            expected.review_relation,
            expected.elevation,
            expected.review,
            1,
        )
        && expected_relation(
            facts,
            expected.reviewer_relation,
            expected.reviewer,
            expected.review,
        )
}

fn expected_relation(
    facts: &[WorthQueryApplicationObservedFact],
    kind: KindId,
    expected: WorthQueryExpectedLifecycleRelation,
    target: EntityId,
) -> bool {
    match expected {
        WorthQueryExpectedLifecycleRelation::Absent => {
            facts
                .iter()
                .filter(|fact| relation_kind(fact) == Some(kind))
                .filter(|fact| {
                    matches!(
                        fact,
                        WorthQueryApplicationObservedFact::Relation {
                            to,
                            matching_relations,
                            ..
                        } if *to == target && matching_relations.is_empty()
                    )
                })
                .count()
                == 1
        }
        WorthQueryExpectedLifecycleRelation::Present(source) => {
            exact_relation(facts, kind, source, target, 1)
        }
    }
}

fn relation_kind(fact: &WorthQueryApplicationObservedFact) -> Option<KindId> {
    match fact {
        WorthQueryApplicationObservedFact::Relation { relation_kind, .. } => Some(*relation_kind),
        WorthQueryApplicationObservedFact::Entity { .. }
        | WorthQueryApplicationObservedFact::Field { .. }
        | WorthQueryApplicationObservedFact::Adjacency { .. } => None,
    }
}

fn exact_field(
    facts: &[WorthQueryApplicationObservedFact],
    entity: EntityId,
    locator: &AspectFieldLocator,
    value: &AspectValue,
) -> bool {
    facts
        .iter()
        .filter(|fact| {
            matches!(
                fact,
                WorthQueryApplicationObservedFact::Field {
                    entity_id,
                    locator: observed_locator,
                    value: observed_value,
                    ..
                } if *entity_id == entity && observed_locator == locator && observed_value == value
            )
        })
        .count()
        == 1
}

fn exact_relation(
    facts: &[WorthQueryApplicationObservedFact],
    kind: KindId,
    from: EntityId,
    to: EntityId,
    count: usize,
) -> bool {
    facts
        .iter()
        .filter(|fact| {
            matches!(
                fact,
                WorthQueryApplicationObservedFact::Relation {
                    relation_kind,
                    from: observed_from,
                    to: observed_to,
                    matching_relations,
                    ..
                } if *relation_kind == kind
                    && (count == 0 || *observed_from == from)
                    && *observed_to == to
                    && matching_relations.len() == count
            )
        })
        .count()
        == 1
}
