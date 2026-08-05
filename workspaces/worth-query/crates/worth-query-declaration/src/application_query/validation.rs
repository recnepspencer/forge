use std::collections::BTreeSet;

use super::{ApplicationQueryDefinition, ApplicationQueryResultShape};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApplicationQueryDefinitionDenial {
    EmptyIdentifier,
    ResultRootMismatch,
    ResultQueryMismatch,
    DuplicateResultSlot,
    DuplicateParameter,
    UnknownPredicateParameter,
    UnknownOrderingResultSlot,
    OrderingResultFieldMismatch,
    UnknownContinuationResultSlot,
    ContinuationResultRelationMismatch,
    ContinuationOrderingMissing,
    ContinuationOrderingOutsideTarget,
    ContinuationRequiresPinnedBasis,
    ContinuationRequiresExactlyOneParentPath,
    ContinuationRequiresSingleRoot,
    ContinuationRequiresSingleManyCollection,
    InvalidRootPath,
    DependencyCeilingExceeded,
    PreviewLaneWithoutBasisSupport,
    LiveLaneWithoutCauseContract,
    LiveCauseContractWithoutLane,
    LiveCauseRequiresContinuation,
    LiveCauseScopeSelectorMismatch,
    LiveCauseTargetSelectorMismatch,
    InvalidLiveResourceContract,
}

pub(super) fn validate_definition<Schema, Query, Parameters, QueryResult, Scope>(
    definition: &ApplicationQueryDefinition<Schema, Query, Parameters, QueryResult, Scope>,
) -> Result<(), ApplicationQueryDefinitionDenial> {
    if definition.name().trim().is_empty() {
        return Err(ApplicationQueryDefinitionDenial::EmptyIdentifier);
    }
    if definition.root_entity() != definition.result_shape().root_entity() {
        return Err(ApplicationQueryDefinitionDenial::ResultRootMismatch);
    }
    if definition.root_paths().iter().any(|path| {
        path.steps().is_empty()
            || path.start_entity() != definition.scope_entity()
            || path.terminal_entity() != definition.root_entity()
            || path
                .steps()
                .windows(2)
                .any(|pair| pair[0].child_entity() != pair[1].parent_entity())
            || path.guards().iter().any(|guard| {
                guard.after_step() > path.steps().len()
                    || root_path_entity_after_step(path, guard.after_step()) != Some(guard.entity())
            })
    }) {
        return Err(ApplicationQueryDefinitionDenial::InvalidRootPath);
    }
    if definition.result_shape().query_type() != std::any::type_name::<Query>()
        || !shape_query_is_valid(definition.result_shape(), std::any::type_name::<Query>())
    {
        return Err(ApplicationQueryDefinitionDenial::ResultQueryMismatch);
    }
    let mut slots = BTreeSet::new();
    if !shape_slots_are_unique(definition.result_shape(), &mut slots) {
        return Err(ApplicationQueryDefinitionDenial::DuplicateResultSlot);
    }
    if definition
        .parameters()
        .windows(2)
        .any(|pair| pair[0].name() == pair[1].name())
    {
        return Err(ApplicationQueryDefinitionDenial::DuplicateParameter);
    }
    if definition.predicates().iter().any(|predicate| {
        !definition
            .parameters()
            .iter()
            .any(|parameter| parameter.name() == predicate.parameter())
    }) {
        return Err(ApplicationQueryDefinitionDenial::UnknownPredicateParameter);
    }
    if definition
        .predicates()
        .iter()
        .any(|predicate| !shape_contains_entity(definition.result_shape(), predicate.field().0))
    {
        return Err(ApplicationQueryDefinitionDenial::ResultRootMismatch);
    }
    for ordering in definition.ordering() {
        let Some(field) = shape_field_by_slot(definition.result_shape(), ordering.slot_type())
        else {
            return Err(ApplicationQueryDefinitionDenial::UnknownOrderingResultSlot);
        };
        if field.query_type() != ordering.query_type()
            || (field.entity(), field.aspect(), field.field()) != ordering.field()
            || field.output_name() != ordering.output_name()
            || field.scalar_family() != ordering.scalar_family()
            || field.value_type() != ordering.value_type()
        {
            return Err(ApplicationQueryDefinitionDenial::OrderingResultFieldMismatch);
        }
    }
    validate_continuation(definition)?;
    validate_live_cause(definition)?;
    let shape_counts = count_shape(definition.result_shape());
    let root_path_depth = definition
        .root_paths()
        .iter()
        .map(|path| path.steps().len())
        .max()
        .unwrap_or(0);
    let root_path_relations = definition
        .root_paths()
        .iter()
        .map(|path| path.steps().len())
        .sum::<usize>();
    let ceiling = definition.dependency_ceiling();
    if shape_counts.0.max(root_path_depth) > ceiling.maximum_traversal_depth()
        || shape_counts.1.saturating_add(root_path_relations) > ceiling.maximum_relation_count()
        || shape_counts.2 > ceiling.maximum_projected_field_count()
    {
        return Err(ApplicationQueryDefinitionDenial::DependencyCeilingExceeded);
    }
    if definition.lanes().preview_enabled() && !definition.basis_support().preview() {
        return Err(ApplicationQueryDefinitionDenial::PreviewLaneWithoutBasisSupport);
    }
    Ok(())
}

fn root_path_entity_after_step(
    path: &super::ApplicationQueryRootPathMeaning,
    after_step: usize,
) -> Option<&str> {
    if after_step == 0 {
        Some(path.start_entity())
    } else {
        path.steps()
            .get(after_step - 1)
            .map(super::ApplicationQueryRootPathStep::child_entity)
    }
}

fn validate_live_cause<Schema, Query, Parameters, QueryResult, Scope>(
    definition: &ApplicationQueryDefinition<Schema, Query, Parameters, QueryResult, Scope>,
) -> Result<(), ApplicationQueryDefinitionDenial> {
    let Some(live) = definition.live_cause() else {
        return if definition.lanes().live_enabled() {
            Err(ApplicationQueryDefinitionDenial::LiveLaneWithoutCauseContract)
        } else {
            Ok(())
        };
    };
    if !definition.lanes().live_enabled() {
        return Err(ApplicationQueryDefinitionDenial::LiveCauseContractWithoutLane);
    }
    let Some(continuation) = definition.continuation() else {
        return Err(ApplicationQueryDefinitionDenial::LiveCauseRequiresContinuation);
    };
    let scope_matches = definition.result_shape().fields().iter().any(|field| {
        field.slot_type() == live.scope_slot_type()
            && (field.entity(), field.aspect(), field.field()) == live.scope_field()
            && field.value_type() == live.scope_value_type()
            && field.entity() == definition.scope_entity()
    });
    if !scope_matches {
        return Err(ApplicationQueryDefinitionDenial::LiveCauseScopeSelectorMismatch);
    }
    let target_matches =
        shape_relation_by_slot(definition.result_shape(), continuation.slot_type()).is_some_and(
            |relation| {
                relation.nested_shape().fields().iter().any(|field| {
                    field.slot_type() == live.target_slot_type()
                        && (field.entity(), field.aspect(), field.field()) == live.target_field()
                        && field.value_type() == live.target_value_type()
                        && field.entity() == continuation.child_entity()
                })
            },
        );
    if !target_matches {
        return Err(ApplicationQueryDefinitionDenial::LiveCauseTargetSelectorMismatch);
    }
    let resources = live.resources();
    if resources.maximum_buffered_causes() == 0
        || resources.maximum_work_per_delivery() == 0
        || resources.maximum_retained_payload_bytes() == 0
    {
        return Err(ApplicationQueryDefinitionDenial::InvalidLiveResourceContract);
    }
    Ok(())
}

fn validate_continuation<Schema, Query, Parameters, QueryResult, Scope>(
    definition: &ApplicationQueryDefinition<Schema, Query, Parameters, QueryResult, Scope>,
) -> Result<(), ApplicationQueryDefinitionDenial> {
    let Some(continuation) = definition.continuation() else {
        return Ok(());
    };
    if definition.cardinality() != super::ApplicationQueryCardinality::ExactlyOne {
        return Err(ApplicationQueryDefinitionDenial::ContinuationRequiresSingleRoot);
    }
    if !definition.basis_support().pinned() {
        return Err(ApplicationQueryDefinitionDenial::ContinuationRequiresPinnedBasis);
    }
    let Some(relation) =
        shape_relation_by_slot(definition.result_shape(), continuation.slot_type())
    else {
        return Err(ApplicationQueryDefinitionDenial::UnknownContinuationResultSlot);
    };
    if continuation.query_type() != std::any::type_name::<Query>()
        || relation.query_type() != continuation.query_type()
        || relation.relation() != continuation.relation()
        || relation.cardinality() != continuation.cardinality()
        || relation.direction() != continuation.direction()
        || relation.nested_shape().root_entity() != continuation.child_entity()
        || relation_parent_entity(relation) != continuation.parent_entity()
    {
        return Err(ApplicationQueryDefinitionDenial::ContinuationResultRelationMismatch);
    }
    if count_many_relations(definition.result_shape()) != 1 {
        return Err(ApplicationQueryDefinitionDenial::ContinuationRequiresSingleManyCollection);
    }
    if continuation_parent_path_is_exact(definition.result_shape(), continuation.slot_type())
        != Some(true)
    {
        return Err(ApplicationQueryDefinitionDenial::ContinuationRequiresExactlyOneParentPath);
    }
    let ordering = definition.ordering();
    if ordering.is_empty() {
        return Err(ApplicationQueryDefinitionDenial::ContinuationOrderingMissing);
    }
    if ordering.iter().any(|term| {
        relation
            .nested_shape()
            .fields()
            .iter()
            .all(|field| field.slot_type() != term.slot_type())
    }) {
        return Err(ApplicationQueryDefinitionDenial::ContinuationOrderingOutsideTarget);
    }
    Ok(())
}

fn continuation_parent_path_is_exact(
    shape: &ApplicationQueryResultShape,
    target_slot_type: &str,
) -> Option<bool> {
    for relation in shape.relations() {
        if relation.slot_type() == target_slot_type {
            return Some(true);
        }
        if let Some(descendant_is_exact) =
            continuation_parent_path_is_exact(relation.nested_shape(), target_slot_type)
        {
            return Some(
                descendant_is_exact
                    && relation.cardinality() == super::ApplicationQueryCardinality::ExactlyOne,
            );
        }
    }
    None
}

fn shape_query_is_valid(shape: &ApplicationQueryResultShape, query_type: &str) -> bool {
    shape.query_type() == query_type
        && shape
            .fields()
            .iter()
            .all(|field| field.query_type() == query_type)
        && shape.relations().iter().all(|relation| {
            relation.query_type() == query_type
                && shape_query_is_valid(relation.nested_shape(), query_type)
        })
}

fn shape_slots_are_unique<'a>(
    shape: &'a ApplicationQueryResultShape,
    slots: &mut BTreeSet<&'a str>,
) -> bool {
    shape
        .fields()
        .iter()
        .all(|field| !field.slot_type().is_empty() && slots.insert(field.slot_type()))
        && shape.relations().iter().all(|relation| {
            !relation.slot_type().is_empty()
                && slots.insert(relation.slot_type())
                && shape_slots_are_unique(relation.nested_shape(), slots)
        })
}

fn count_shape(shape: &ApplicationQueryResultShape) -> (usize, usize, usize) {
    let mut maximum_depth = 0;
    let mut relation_count = 0;
    let mut field_count = shape.fields().len();
    for relation in shape.relations() {
        let nested = count_shape(relation.nested_shape());
        maximum_depth = maximum_depth.max(nested.0 + 1);
        relation_count += nested.1 + 1;
        field_count += nested.2;
    }
    (maximum_depth, relation_count, field_count)
}

fn shape_contains_entity(shape: &ApplicationQueryResultShape, entity: &str) -> bool {
    shape.root_entity() == entity
        || shape
            .relations()
            .iter()
            .any(|relation| shape_contains_entity(relation.nested_shape(), entity))
}

fn shape_field_by_slot<'a>(
    shape: &'a ApplicationQueryResultShape,
    slot_type: &str,
) -> Option<&'a super::ApplicationQueryResultField> {
    shape
        .fields()
        .iter()
        .find(|field| field.slot_type() == slot_type)
        .or_else(|| {
            shape
                .relations()
                .iter()
                .find_map(|relation| shape_field_by_slot(relation.nested_shape(), slot_type))
        })
}

fn shape_relation_by_slot<'a>(
    shape: &'a ApplicationQueryResultShape,
    slot_type: &str,
) -> Option<&'a super::ApplicationQueryResultRelation> {
    shape
        .relations()
        .iter()
        .find(|relation| relation.slot_type() == slot_type)
        .or_else(|| {
            shape
                .relations()
                .iter()
                .find_map(|relation| shape_relation_by_slot(relation.nested_shape(), slot_type))
        })
}

fn count_many_relations(shape: &ApplicationQueryResultShape) -> usize {
    shape
        .relations()
        .iter()
        .map(|relation| {
            usize::from(relation.cardinality() == super::ApplicationQueryCardinality::Many)
                + count_many_relations(relation.nested_shape())
        })
        .sum()
}

fn relation_parent_entity(relation: &super::ApplicationQueryResultRelation) -> &str {
    match relation.direction() {
        super::ApplicationQueryResultTraversalDirection::Forward => relation.from(),
        super::ApplicationQueryResultTraversalDirection::Reverse => relation.to(),
    }
}
