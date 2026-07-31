use worth_query_installation::facade::{
    ApplicationAuthorizationPath, ApplicationAuthorizationTraversalDirection,
};
use worth_relational::facade::authorization::{
    RelationalAuthorizationPathPlan, RelationalAuthorizationPredicate,
    RelationalAuthorizationTraversal, RelationalAuthorizationTraversalDirection,
};

use super::super::schema_layout::WorthQueryPrimaryGraphLayout;
use super::{authorization_denial, WorthQueryOperationAuthorizationDenial};

pub(super) fn lower_authorization_path(
    layout: &WorthQueryPrimaryGraphLayout,
    path: &ApplicationAuthorizationPath,
) -> Result<RelationalAuthorizationPathPlan, WorthQueryOperationAuthorizationDenial> {
    let mut traversals = Vec::with_capacity(path.traversals().len());
    for traversal in path.traversals() {
        let relation = layout.relation(traversal.relation()).ok_or_else(|| {
            authorization_denial(
                traversal.relation(),
                "authorization relation is not installed",
            )
        })?;
        let from = layout.entity_kind(traversal.from()).ok_or_else(|| {
            authorization_denial(
                traversal.from(),
                "authorization source kind is not installed",
            )
        })?;
        let to = layout.entity_kind(traversal.to()).ok_or_else(|| {
            authorization_denial(traversal.to(), "authorization target kind is not installed")
        })?;
        if relation.from != from || relation.to != to {
            return Err(authorization_denial(
                traversal.relation(),
                "authorization traversal endpoints changed",
            ));
        }
        traversals.push(RelationalAuthorizationTraversal::new(
            relation.kind,
            from,
            to,
            match traversal.direction() {
                ApplicationAuthorizationTraversalDirection::Forward => {
                    RelationalAuthorizationTraversalDirection::Forward
                }
                ApplicationAuthorizationTraversalDirection::Reverse => {
                    RelationalAuthorizationTraversalDirection::Reverse
                }
            },
        ));
    }
    let mut predicates = Vec::with_capacity(path.predicates().len());
    for predicate in path.predicates() {
        let entity_kind = layout.entity_kind(predicate.entity()).ok_or_else(|| {
            authorization_denial(
                predicate.entity(),
                "authorization predicate kind is not installed",
            )
        })?;
        let field = layout
            .field_locator(predicate.entity(), predicate.aspect(), predicate.field())
            .cloned()
            .ok_or_else(|| {
                authorization_denial(
                    predicate.field(),
                    "authorization predicate field is not installed",
                )
            })?;
        predicates.push(RelationalAuthorizationPredicate::new(
            predicate.traversal_ordinal(),
            entity_kind,
            field,
            predicate.value().clone(),
        ));
    }
    Ok(RelationalAuthorizationPathPlan::new(traversals, predicates))
}
