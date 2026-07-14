use super::{
    WorthQueryAdmittedGraphReadRelationDirection, WorthQueryAdmittedQuerySchemaReferences,
    WorthQueryGraphReadAccessShape, WorthQueryGraphReadFanoutPosture,
    WorthQueryGraphReadLifecycleClass, WorthQueryGraphReadOperationResolution,
    WorthQueryGraphReadOrderingPosture, WorthQueryGraphReadPredicateFamily,
    WorthQueryGraphReadResolvedOperationKind, WorthQueryGraphReadResultPressure,
    WorthQueryGraphReadRootPosture, WorthQueryGraphReadTraversalOperator,
};
use crate::runtime::{
    WorthQueryReadBuiltInOperator, WorthQueryReadGraphFamily, WorthQueryReadScopeClass,
};

pub(crate) fn derive_graph_read_access_shape(
    operation_resolution: WorthQueryGraphReadOperationResolution,
) -> WorthQueryGraphReadAccessShape {
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
    WorthQueryGraphReadAccessShape::new(
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
        WorthQueryGraphReadLifecycleClass::ReusableReadFamily,
    )
}

fn root_posture_from_scope_class(
    scope_class: &WorthQueryReadScopeClass,
) -> WorthQueryGraphReadRootPosture {
    match scope_class {
        WorthQueryReadScopeClass::LocalNeighborhood => WorthQueryGraphReadRootPosture::Local,
        WorthQueryReadScopeClass::AnchoredExpansion => WorthQueryGraphReadRootPosture::Anchored,
        WorthQueryReadScopeClass::ExplicitBroadSearch => {
            WorthQueryGraphReadRootPosture::ExplicitBroadSearch
        }
    }
}

fn admitted_relation_directions(
    references: &WorthQueryAdmittedQuerySchemaReferences,
) -> Vec<WorthQueryAdmittedGraphReadRelationDirection> {
    references
        .relations()
        .iter()
        .map(|relation| relation.direction().clone())
        .collect()
}

fn traversal_operators_from_resolution(
    resolution: &WorthQueryGraphReadOperationResolution,
) -> Vec<WorthQueryGraphReadTraversalOperator> {
    resolution
        .operations()
        .iter()
        .map(|operation| match operation.kind() {
            WorthQueryGraphReadResolvedOperationKind::BuiltIn(operator) => match operator {
                WorthQueryReadBuiltInOperator::DirectEdge => {
                    WorthQueryGraphReadTraversalOperator::DirectEdge
                }
                WorthQueryReadBuiltInOperator::SuccessorWalk => {
                    WorthQueryGraphReadTraversalOperator::SuccessorWalk
                }
                WorthQueryReadBuiltInOperator::BoundedAncestor => {
                    WorthQueryGraphReadTraversalOperator::BoundedAncestor
                }
                WorthQueryReadBuiltInOperator::BoundedDescendant => {
                    WorthQueryGraphReadTraversalOperator::BoundedDescendant
                }
                WorthQueryReadBuiltInOperator::AnchoredFrontier => {
                    WorthQueryGraphReadTraversalOperator::AnchoredFrontier
                }
                WorthQueryReadBuiltInOperator::SharedEndpoint => {
                    WorthQueryGraphReadTraversalOperator::SharedEndpoint
                }
                WorthQueryReadBuiltInOperator::SharedAttachment => {
                    WorthQueryGraphReadTraversalOperator::SharedAttachment
                }
                WorthQueryReadBuiltInOperator::FrontierSearch => {
                    WorthQueryGraphReadTraversalOperator::FrontierSearch
                }
            },
            WorthQueryGraphReadResolvedOperationKind::DomainRegistered(operation) => {
                operation.traversal_operator().clone()
            }
            WorthQueryGraphReadResolvedOperationKind::DeclarationTraversal => {
                WorthQueryGraphReadTraversalOperator::DeclarationTraversal
            }
        })
        .collect()
}

fn maximum_admitted_relation_depth(references: &WorthQueryAdmittedQuerySchemaReferences) -> usize {
    references
        .relations()
        .iter()
        .map(|relation| relation.depth())
        .max()
        .unwrap_or(0)
}

fn fanout_posture(
    references: &WorthQueryAdmittedQuerySchemaReferences,
    operators: &[WorthQueryGraphReadTraversalOperator],
) -> WorthQueryGraphReadFanoutPosture {
    if references.relations().is_empty() {
        return WorthQueryGraphReadFanoutPosture::None;
    }
    if operators.iter().any(|operator| {
        matches!(
            operator,
            WorthQueryGraphReadTraversalOperator::AnchoredFrontier
                | WorthQueryGraphReadTraversalOperator::FrontierSearch
        )
    }) {
        return WorthQueryGraphReadFanoutPosture::Frontier;
    }
    if references.relations().len() == 1 {
        WorthQueryGraphReadFanoutPosture::SingleRelation
    } else {
        WorthQueryGraphReadFanoutPosture::MultiRelation
    }
}

fn predicate_family(
    references: &WorthQueryAdmittedQuerySchemaReferences,
) -> WorthQueryGraphReadPredicateFamily {
    let mut families = references
        .predicates()
        .iter()
        .map(|predicate| match predicate.family() {
            "equality" => WorthQueryGraphReadPredicateFamily::Equality,
            "integer-comparison" => WorthQueryGraphReadPredicateFamily::Range,
            "string-contains" => WorthQueryGraphReadPredicateFamily::Text,
            "set-membership" => WorthQueryGraphReadPredicateFamily::Membership,
            "presence" => WorthQueryGraphReadPredicateFamily::Presence,
            _ => WorthQueryGraphReadPredicateFamily::Mixed,
        })
        .collect::<Vec<_>>();
    families.sort_by_key(|family| family.as_str());
    families.dedup();
    match families.as_slice() {
        [] => WorthQueryGraphReadPredicateFamily::None,
        [single] => single.clone(),
        _ => WorthQueryGraphReadPredicateFamily::Mixed,
    }
}

fn ordering_posture(
    references: &WorthQueryAdmittedQuerySchemaReferences,
) -> WorthQueryGraphReadOrderingPosture {
    if references.orderings().is_empty() {
        WorthQueryGraphReadOrderingPosture::Unordered
    } else {
        WorthQueryGraphReadOrderingPosture::Ordered
    }
}

fn result_pressure(
    family: WorthQueryReadGraphFamily,
    projection_count: usize,
) -> WorthQueryGraphReadResultPressure {
    match family {
        WorthQueryReadGraphFamily::Detail => WorthQueryGraphReadResultPressure::Detail,
        WorthQueryReadGraphFamily::Collection if projection_count <= 3 => {
            WorthQueryGraphReadResultPressure::CollectionNarrow
        }
        WorthQueryReadGraphFamily::Collection => WorthQueryGraphReadResultPressure::CollectionWide,
    }
}
