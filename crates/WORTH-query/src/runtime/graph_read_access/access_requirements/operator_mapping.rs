use crate::runtime::{
    WorthQueryAdmittedGraphReadRelation, WorthQueryAdmittedGraphReadRelationDirection,
    WorthQueryGraphReadAccessShape, WorthQueryGraphReadResolvedOperationKind,
    WorthQueryGraphReadTraversalOperator, WorthQueryReadBuiltInOperator,
};

pub(crate) fn traversal_operators_for_relation(
    access_shape: &WorthQueryGraphReadAccessShape,
    relation: &WorthQueryAdmittedGraphReadRelation,
) -> Vec<WorthQueryGraphReadTraversalOperator> {
    let mut operators = access_shape
        .operation_resolution()
        .operations()
        .iter()
        .filter_map(|operation| match operation.kind() {
            WorthQueryGraphReadResolvedOperationKind::BuiltIn(operator)
                if built_in_operator_applies_to_relation(operator, relation) =>
            {
                Some(traversal_operator_for_built_in(operator))
            }
            WorthQueryGraphReadResolvedOperationKind::DomainRegistered(operation)
                if operation
                    .accepted_relation_names()
                    .iter()
                    .any(|name| name == relation.relation_name()) =>
            {
                Some(operation.traversal_operator().clone())
            }
            WorthQueryGraphReadResolvedOperationKind::DeclarationTraversal => {
                Some(WorthQueryGraphReadTraversalOperator::DeclarationTraversal)
            }
            WorthQueryGraphReadResolvedOperationKind::BuiltIn(_)
            | WorthQueryGraphReadResolvedOperationKind::DomainRegistered(_) => None,
        })
        .collect::<Vec<_>>();
    operators.sort_by_key(|operator| operator.as_str());
    operators.dedup();
    operators
}

fn built_in_operator_applies_to_relation(
    operator: &WorthQueryReadBuiltInOperator,
    relation: &WorthQueryAdmittedGraphReadRelation,
) -> bool {
    matches!(
        (operator, relation.direction()),
        (
            WorthQueryReadBuiltInOperator::BoundedAncestor,
            WorthQueryAdmittedGraphReadRelationDirection::Ancestor
        ) | (
            WorthQueryReadBuiltInOperator::BoundedDescendant,
            WorthQueryAdmittedGraphReadRelationDirection::Descendant
        ) | (
            WorthQueryReadBuiltInOperator::FrontierSearch,
            WorthQueryAdmittedGraphReadRelationDirection::Forward
                | WorthQueryAdmittedGraphReadRelationDirection::Ancestor
                | WorthQueryAdmittedGraphReadRelationDirection::Descendant
        ) | (
            WorthQueryReadBuiltInOperator::DirectEdge
                | WorthQueryReadBuiltInOperator::SuccessorWalk
                | WorthQueryReadBuiltInOperator::AnchoredFrontier
                | WorthQueryReadBuiltInOperator::SharedEndpoint
                | WorthQueryReadBuiltInOperator::SharedAttachment,
            WorthQueryAdmittedGraphReadRelationDirection::Forward
        )
    )
}

fn traversal_operator_for_built_in(
    operator: &WorthQueryReadBuiltInOperator,
) -> WorthQueryGraphReadTraversalOperator {
    match operator {
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
    }
}
