use super::{
    ForgeQueryAdmittedGraphReadRelationDirection, ForgeQueryAdmittedQuerySchemaReferences,
    ForgeQueryGraphReadAccessShape, ForgeQueryGraphReadFanoutPosture,
    ForgeQueryGraphReadLifecycleClass, ForgeQueryGraphReadOperationResolution,
    ForgeQueryGraphReadOrderingPosture, ForgeQueryGraphReadPredicateFamily,
    ForgeQueryGraphReadResolvedOperationKind, ForgeQueryGraphReadResultPressure,
    ForgeQueryGraphReadRootPosture, ForgeQueryGraphReadTraversalOperator,
};
use crate::runtime::{
    ForgeQueryReadBuiltInOperator, ForgeQueryReadGraphFamily, ForgeQueryReadScopeClass,
};

pub(crate) fn derive_graph_read_access_shape(
    operation_resolution: ForgeQueryGraphReadOperationResolution,
) -> ForgeQueryGraphReadAccessShape {
    let references = operation_resolution.references();
    let scope_class = operation_resolution.scope_class().clone();
    let root_posture = root_posture_from_scope_class(&scope_class);
    let relation_directions = admitted_relation_directions(references);
    let traversal_operators = traversal_operators_from_resolution(&operation_resolution);
    let max_depth = maximum_admitted_relation_depth(references);
    let fanout_posture = fanout_posture(references, &traversal_operators);
    let predicate_family = predicate_family(references);
    let ordering_posture = ordering_posture(references);
    let result_pressure = result_pressure(
        operation_resolution.graph_family().clone(),
        references.projections().len(),
    );
    ForgeQueryGraphReadAccessShape::new(
        operation_resolution,
        root_posture,
        scope_class,
        relation_directions,
        traversal_operators,
        max_depth,
        fanout_posture,
        predicate_family,
        ordering_posture,
        result_pressure,
        ForgeQueryGraphReadLifecycleClass::ReusableReadFamily,
    )
}

fn root_posture_from_scope_class(
    scope_class: &ForgeQueryReadScopeClass,
) -> ForgeQueryGraphReadRootPosture {
    match scope_class {
        ForgeQueryReadScopeClass::LocalNeighborhood => ForgeQueryGraphReadRootPosture::Local,
        ForgeQueryReadScopeClass::AnchoredExpansion => ForgeQueryGraphReadRootPosture::Anchored,
        ForgeQueryReadScopeClass::ExplicitBroadSearch => {
            ForgeQueryGraphReadRootPosture::ExplicitBroadSearch
        }
    }
}

fn admitted_relation_directions(
    references: &ForgeQueryAdmittedQuerySchemaReferences,
) -> Vec<ForgeQueryAdmittedGraphReadRelationDirection> {
    references
        .relations()
        .iter()
        .map(|relation| relation.direction().clone())
        .collect()
}

fn traversal_operators_from_resolution(
    resolution: &ForgeQueryGraphReadOperationResolution,
) -> Vec<ForgeQueryGraphReadTraversalOperator> {
    resolution
        .operations()
        .iter()
        .map(|operation| match operation.kind() {
            ForgeQueryGraphReadResolvedOperationKind::BuiltIn(operator) => match operator {
                ForgeQueryReadBuiltInOperator::DirectEdge => {
                    ForgeQueryGraphReadTraversalOperator::DirectEdge
                }
                ForgeQueryReadBuiltInOperator::SuccessorWalk => {
                    ForgeQueryGraphReadTraversalOperator::SuccessorWalk
                }
                ForgeQueryReadBuiltInOperator::BoundedAncestor => {
                    ForgeQueryGraphReadTraversalOperator::BoundedAncestor
                }
                ForgeQueryReadBuiltInOperator::BoundedDescendant => {
                    ForgeQueryGraphReadTraversalOperator::BoundedDescendant
                }
                ForgeQueryReadBuiltInOperator::AnchoredFrontier => {
                    ForgeQueryGraphReadTraversalOperator::AnchoredFrontier
                }
                ForgeQueryReadBuiltInOperator::SharedEndpoint => {
                    ForgeQueryGraphReadTraversalOperator::SharedEndpoint
                }
                ForgeQueryReadBuiltInOperator::SharedAttachment => {
                    ForgeQueryGraphReadTraversalOperator::SharedAttachment
                }
                ForgeQueryReadBuiltInOperator::FrontierSearch => {
                    ForgeQueryGraphReadTraversalOperator::FrontierSearch
                }
            },
            ForgeQueryGraphReadResolvedOperationKind::DomainRegistered(operation) => {
                operation.traversal_operator().clone()
            }
            ForgeQueryGraphReadResolvedOperationKind::DeclarationTraversal => {
                ForgeQueryGraphReadTraversalOperator::DeclarationTraversal
            }
        })
        .collect()
}

fn maximum_admitted_relation_depth(references: &ForgeQueryAdmittedQuerySchemaReferences) -> usize {
    references
        .relations()
        .iter()
        .map(|relation| relation.depth())
        .max()
        .unwrap_or(0)
}

fn fanout_posture(
    references: &ForgeQueryAdmittedQuerySchemaReferences,
    operators: &[ForgeQueryGraphReadTraversalOperator],
) -> ForgeQueryGraphReadFanoutPosture {
    if references.relations().is_empty() {
        return ForgeQueryGraphReadFanoutPosture::None;
    }
    if operators.iter().any(|operator| {
        matches!(
            operator,
            ForgeQueryGraphReadTraversalOperator::AnchoredFrontier
                | ForgeQueryGraphReadTraversalOperator::FrontierSearch
        )
    }) {
        return ForgeQueryGraphReadFanoutPosture::Frontier;
    }
    if references.relations().len() == 1 {
        ForgeQueryGraphReadFanoutPosture::SingleRelation
    } else {
        ForgeQueryGraphReadFanoutPosture::MultiRelation
    }
}

fn predicate_family(
    references: &ForgeQueryAdmittedQuerySchemaReferences,
) -> ForgeQueryGraphReadPredicateFamily {
    let mut families = references
        .predicates()
        .iter()
        .map(|predicate| match predicate.family() {
            "equality" => ForgeQueryGraphReadPredicateFamily::Equality,
            "integer-comparison" => ForgeQueryGraphReadPredicateFamily::Range,
            "string-contains" => ForgeQueryGraphReadPredicateFamily::Text,
            "set-membership" => ForgeQueryGraphReadPredicateFamily::Membership,
            "presence" => ForgeQueryGraphReadPredicateFamily::Presence,
            _ => ForgeQueryGraphReadPredicateFamily::Mixed,
        })
        .collect::<Vec<_>>();
    families.sort_by_key(|family| family.as_str());
    families.dedup();
    match families.as_slice() {
        [] => ForgeQueryGraphReadPredicateFamily::None,
        [single] => single.clone(),
        _ => ForgeQueryGraphReadPredicateFamily::Mixed,
    }
}

fn ordering_posture(
    references: &ForgeQueryAdmittedQuerySchemaReferences,
) -> ForgeQueryGraphReadOrderingPosture {
    if references.orderings().is_empty() {
        ForgeQueryGraphReadOrderingPosture::Unordered
    } else {
        ForgeQueryGraphReadOrderingPosture::Ordered
    }
}

fn result_pressure(
    family: ForgeQueryReadGraphFamily,
    projection_count: usize,
) -> ForgeQueryGraphReadResultPressure {
    match family {
        ForgeQueryReadGraphFamily::Detail => ForgeQueryGraphReadResultPressure::Detail,
        ForgeQueryReadGraphFamily::Collection if projection_count <= 3 => {
            ForgeQueryGraphReadResultPressure::CollectionNarrow
        }
        ForgeQueryReadGraphFamily::Collection => ForgeQueryGraphReadResultPressure::CollectionWide,
    }
}
