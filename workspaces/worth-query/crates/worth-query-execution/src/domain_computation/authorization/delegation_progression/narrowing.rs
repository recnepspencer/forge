use worth_query_declaration::facade::application_capability::ApplicationCapabilityDelegationRequestProjection;
use worth_relational::facade::authorization::{
    RelationalAuthorizationFieldComparison as Comparison, RelationalAuthorizationObservationPlan,
    RelationalAuthorizationPathPlan, RelationalAuthorizationPredicate,
    RelationalAuthorizationRelatedEntityConstraint,
};

use super::super::capability_registry::WorthQueryInstalledCapabilityPlan;
use super::denial;
use super::support::WorthQueryDelegationResolvedRequest;
use crate::domain_computation::authorization::{
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
};

pub(super) fn observe_narrowing<Schema, Scope, Context>(
    relational: &worth_relational::facade::runtime::RelationalRuntime,
    snapshot: worth_relational::facade::snapshots::SnapshotHandle,
    installed: &WorthQueryInstalledCapabilityPlan,
    proposed: &ApplicationCapabilityDelegationRequestProjection<Schema, Scope, Context>,
    resolved: &WorthQueryDelegationResolvedRequest,
) -> Result<
    worth_relational::facade::authorization::RelationalAuthorizationObservationEvidence,
    WorthQueryOperationAuthorizationDenial,
> {
    let mut related = vec![RelationalAuthorizationRelatedEntityConstraint::new(
        0,
        installed.delegation.grantee_from_grant.clone(),
        resolved.grantor,
    )];
    related.extend(resolved.activation_context.iter().map(|context| {
        RelationalAuthorizationRelatedEntityConstraint::new(
            0,
            context.traversal.clone(),
            context.entity,
        )
    }));
    let path = RelationalAuthorizationPathPlan::new([], predicates(installed, proposed))
        .with_related_entities(related);
    let plan = RelationalAuthorizationObservationPlan::try_new(
        snapshot,
        resolved.parent,
        resolved.parent,
        installed.grant_kind,
        installed.grant_kind,
        [path],
        [],
    )
    .map_err(|_| rejected(installed))?;
    let evidence = relational
        .observe_authorization(plan)
        .map_err(|_| rejected(installed))?;
    let [path] = evidence.paths() else {
        return Err(rejected(installed));
    };
    if path.matched() && path.exhaustive() && evidence.counters().maximum_frontier_width <= 1 {
        Ok(evidence)
    } else {
        Err(rejected(installed))
    }
}

fn predicates<Schema, Scope, Context>(
    installed: &WorthQueryInstalledCapabilityPlan,
    proposed: &ApplicationCapabilityDelegationRequestProjection<Schema, Scope, Context>,
) -> Vec<RelationalAuthorizationPredicate> {
    let target = proposed.target();
    let mut predicates = vec![
        predicate(
            installed,
            &installed.delegation.action,
            Comparison::Equal,
            target.action(),
        ),
        predicate(
            installed,
            &installed.delegation.purpose,
            Comparison::Equal,
            target.purpose(),
        ),
        predicate(
            installed,
            &installed.delegation.active_status.0,
            Comparison::Equal,
            &installed.delegation.active_status.1,
        ),
        predicate(
            installed,
            &installed.delegation.grant_workflow,
            Comparison::Equal,
            proposed.workflow().value(),
        ),
        predicate(
            installed,
            &installed.delegation.not_before,
            Comparison::AtMost,
            proposed.not_before().value(),
        ),
        predicate(
            installed,
            &installed.delegation.not_after,
            Comparison::AtLeast,
            proposed.not_after().value(),
        ),
        predicate(
            installed,
            &installed.delegation.remaining,
            Comparison::StrictlyGreater,
            proposed.remaining_delegations().value(),
        ),
    ];
    if let (Some(field), Some(value)) = (&installed.delegation.disclosure, target.field_value()) {
        predicates.push(predicate(installed, field, Comparison::Equal, value));
    }
    if let (Some(field), Some(value)) = (&installed.delegation.magnitude, target.magnitude_value())
    {
        predicates.push(predicate(installed, field, Comparison::AtLeast, value));
    }
    predicates
}

fn predicate(
    installed: &WorthQueryInstalledCapabilityPlan,
    field: &worth_foundational::facade::AspectFieldLocator,
    comparison: Comparison,
    value: &worth_foundational::facade::AspectValue,
) -> RelationalAuthorizationPredicate {
    RelationalAuthorizationPredicate::compare(
        0,
        installed.grant_kind,
        field.clone(),
        comparison,
        value.clone(),
    )
}

fn rejected(
    installed: &WorthQueryInstalledCapabilityPlan,
) -> WorthQueryOperationAuthorizationDenial {
    denial(
        WorthQueryOperationAuthorizationDenialKind::DelegationRejected,
        installed.contract.name(),
    )
}
