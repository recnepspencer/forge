use crate::runtime::{
    ForgeQueryAdmittedGraphReadRelation, ForgeQueryAdmittedGraphReadRelationDirection,
    ForgeQueryGraphReadAccessShape, ForgeQueryGraphReadResolvedOperationKind,
    ForgeQueryGraphReadTraversalOperator, ForgeQueryReadBuiltInOperator,
};

pub(crate) fn traversal_operators_for_relation(
    access_shape: &ForgeQueryGraphReadAccessShape,
    relation: &ForgeQueryAdmittedGraphReadRelation,
) -> Vec<ForgeQueryGraphReadTraversalOperator> {
    let mut operators = access_shape
        .operation_resolution()
        .operations()
        .iter()
        .filter_map(|operation| match operation.kind() {
            ForgeQueryGraphReadResolvedOperationKind::BuiltIn(operator)
                if built_in_operator_applies_to_relation(operator, relation) =>
            {
                Some(traversal_operator_for_built_in(operator))
            }
            ForgeQueryGraphReadResolvedOperationKind::DomainRegistered(operation)
                if operation
                    .accepted_relation_names()
                    .iter()
                    .any(|name| name == relation.relation_name()) =>
            {
                Some(operation.traversal_operator().clone())
            }
            ForgeQueryGraphReadResolvedOperationKind::DeclarationTraversal => {
                Some(ForgeQueryGraphReadTraversalOperator::DeclarationTraversal)
            }
            ForgeQueryGraphReadResolvedOperationKind::BuiltIn(_)
            | ForgeQueryGraphReadResolvedOperationKind::DomainRegistered(_) => None,
        })
        .collect::<Vec<_>>();
    operators.sort_by_key(|operator| operator.as_str());
    operators.dedup();
    operators
}

fn built_in_operator_applies_to_relation(
    operator: &ForgeQueryReadBuiltInOperator,
    relation: &ForgeQueryAdmittedGraphReadRelation,
) -> bool {
    matches!(
        (operator, relation.direction()),
        (
            ForgeQueryReadBuiltInOperator::BoundedAncestor,
            ForgeQueryAdmittedGraphReadRelationDirection::Ancestor
        ) | (
            ForgeQueryReadBuiltInOperator::BoundedDescendant,
            ForgeQueryAdmittedGraphReadRelationDirection::Descendant
        ) | (
            ForgeQueryReadBuiltInOperator::FrontierSearch,
            ForgeQueryAdmittedGraphReadRelationDirection::Forward
                | ForgeQueryAdmittedGraphReadRelationDirection::Ancestor
                | ForgeQueryAdmittedGraphReadRelationDirection::Descendant
        ) | (
            ForgeQueryReadBuiltInOperator::DirectEdge
                | ForgeQueryReadBuiltInOperator::SuccessorWalk
                | ForgeQueryReadBuiltInOperator::AnchoredFrontier
                | ForgeQueryReadBuiltInOperator::SharedEndpoint
                | ForgeQueryReadBuiltInOperator::SharedAttachment,
            ForgeQueryAdmittedGraphReadRelationDirection::Forward
        )
    )
}

fn traversal_operator_for_built_in(
    operator: &ForgeQueryReadBuiltInOperator,
) -> ForgeQueryGraphReadTraversalOperator {
    match operator {
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
    }
}
