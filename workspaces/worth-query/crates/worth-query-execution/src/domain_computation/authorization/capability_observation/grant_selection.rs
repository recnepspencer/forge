use worth_relational::facade::authorization::{
    RelationalAuthorizationFieldComparison, RelationalAuthorizationFieldConstraint,
    RelationalAuthorizationFieldOperand, RelationalAuthorizationObservationCounters,
    RelationalAuthorizationObservationPlan, RelationalAuthorizationPathPlan,
    RelationalAuthorizationPredicate, RelationalAuthorizationRelatedEntityConstraint,
    RelationalAuthorizationTraversal, RelationalAuthorizationTraversalDirection,
};

pub(super) struct SelectedCapabilityGrant {
    grant: worth_relational::facade::identity::EntityId,
    work: RelationalAuthorizationObservationCounters,
}

impl SelectedCapabilityGrant {
    pub(super) const fn into_parts(
        self,
    ) -> (
        worth_relational::facade::identity::EntityId,
        RelationalAuthorizationObservationCounters,
    ) {
        (self.grant, self.work)
    }
}

use super::super::capability_registry::WorthQueryInstalledCapabilityPlan;
use super::super::retained_capability_request::WorthQueryRetainedCapabilityRequest;
use super::super::{
    WorthQueryAuthorizationTimeSample, WorthQueryOperationAuthorizationDenial,
    WorthQueryOperationAuthorizationDenialKind,
};

pub(super) fn select_exact_grant(
    relational: &worth_relational::facade::runtime::RelationalRuntime,
    snapshot: worth_relational::facade::snapshots::SnapshotHandle,
    installed: &WorthQueryInstalledCapabilityPlan,
    request: &WorthQueryRetainedCapabilityRequest,
    sample: &WorthQueryAuthorizationTimeSample,
) -> Result<SelectedCapabilityGrant, WorthQueryOperationAuthorizationDenial> {
    let path = prepare_grant_selection_path(installed, request, sample)?;
    let plan = RelationalAuthorizationObservationPlan::try_new(
        snapshot,
        request.resource,
        request.principal,
        installed.scope_kind,
        installed.principal_kind,
        [path],
        [],
    )
    .map_err(|_| invalid_policy(installed.contract.name()))?;
    let evidence = relational.observe_authorization(plan).map_err(|_| {
        denial(
            WorthQueryOperationAuthorizationDenialKind::RelationalObservationRejected,
            installed.contract.name(),
        )
    })?;
    let grant = evidence
        .paths()
        .first()
        .and_then(|path| path.witness())
        .and_then(|witness| witness.entity_at(1))
        .ok_or_else(|| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::PermissionDenied,
                installed.contract.name(),
            )
        })?;
    Ok(SelectedCapabilityGrant {
        grant,
        work: evidence.counters(),
    })
}

pub(super) fn prepare_grant_path(
    installed: &WorthQueryInstalledCapabilityPlan,
    request: &WorthQueryRetainedCapabilityRequest,
    sample: &WorthQueryAuthorizationTimeSample,
) -> Result<RelationalAuthorizationPathPlan, WorthQueryOperationAuthorizationDenial> {
    let template = grant_template(installed)?;
    let mut predicates = template.plan.predicates().to_vec();
    append_grant_predicates(installed, request, sample, &mut predicates);
    let mut plan = template.plan.clone().with_predicates(predicates);
    if let (Some(traversal), Some(entity)) = (&installed.request.related_relation, request.related)
    {
        plan = plan.with_related_entities([RelationalAuthorizationRelatedEntityConstraint::new(
            installed.grant_witness.entity_ordinal(),
            traversal.clone(),
            entity,
        )]);
    }
    Ok(plan)
}

fn prepare_grant_selection_path(
    installed: &WorthQueryInstalledCapabilityPlan,
    request: &WorthQueryRetainedCapabilityRequest,
    sample: &WorthQueryAuthorizationTimeSample,
) -> Result<RelationalAuthorizationPathPlan, WorthQueryOperationAuthorizationDenial> {
    let template = grant_template(installed)?;
    let mut predicates = template.plan.predicates().to_vec();
    append_grant_predicates(installed, request, sample, &mut predicates);
    let mut path = RelationalAuthorizationPathPlan::new(
        [
            reverse(&installed.delegation.resource),
            installed.delegation.grantee_from_grant.clone(),
        ],
        predicates,
    )
    .with_field_constraints([RelationalAuthorizationFieldConstraint::new(
        RelationalAuthorizationFieldOperand::new(
            1,
            installed.grant_kind,
            installed.delegation.grant_workflow.clone(),
        ),
        RelationalAuthorizationFieldComparison::Equal,
        RelationalAuthorizationFieldOperand::new(
            0,
            installed.scope_kind,
            installed.delegation.resource_workflow.clone(),
        ),
    )]);
    if let (Some(traversal), Some(entity)) = (&installed.request.related_relation, request.related)
    {
        path = path.with_related_entities([RelationalAuthorizationRelatedEntityConstraint::new(
            1,
            traversal.clone(),
            entity,
        )]);
    }
    Ok(path)
}

fn grant_template(
    installed: &WorthQueryInstalledCapabilityPlan,
) -> Result<
    &super::super::capability_registry::WorthQueryCapabilityPathTemplate,
    WorthQueryOperationAuthorizationDenial,
> {
    let template = installed
        .paths
        .get(installed.grant_witness.path_index())
        .ok_or_else(|| invalid_policy(installed.contract.name()))?;
    if !template.context_anchors.is_empty()
        || template.grant_ordinal != Some(installed.grant_witness.entity_ordinal())
    {
        return Err(invalid_policy(installed.contract.name()));
    }
    Ok(template)
}

fn append_grant_predicates(
    installed: &WorthQueryInstalledCapabilityPlan,
    projection: &WorthQueryRetainedCapabilityRequest,
    sample: &WorthQueryAuthorizationTimeSample,
    predicates: &mut Vec<RelationalAuthorizationPredicate>,
) {
    let ordinal = installed.grant_witness.entity_ordinal();
    predicates.push(RelationalAuthorizationPredicate::compare(
        ordinal,
        installed.grant_kind,
        installed.request.not_before.clone(),
        RelationalAuthorizationFieldComparison::AtMost,
        sample.value().clone(),
    ));
    predicates.push(RelationalAuthorizationPredicate::compare(
        ordinal,
        installed.grant_kind,
        installed.request.not_after.clone(),
        RelationalAuthorizationFieldComparison::AtLeast,
        sample.value().clone(),
    ));
    if let (Some(field), Some(value)) = (&installed.request.field, projection.field.as_ref()) {
        predicates.push(RelationalAuthorizationPredicate::new(
            ordinal,
            installed.grant_kind,
            field.clone(),
            value.clone(),
        ));
    }
    if let (Some(field), Some(value)) = (&installed.request.amount, projection.amount.as_ref()) {
        predicates.push(RelationalAuthorizationPredicate::compare(
            ordinal,
            installed.grant_kind,
            field.clone(),
            RelationalAuthorizationFieldComparison::AtLeast,
            value.clone(),
        ));
    }
}

fn reverse(traversal: &RelationalAuthorizationTraversal) -> RelationalAuthorizationTraversal {
    RelationalAuthorizationTraversal::new(
        traversal.relation_kind(),
        traversal.from_kind(),
        traversal.to_kind(),
        match traversal.direction() {
            RelationalAuthorizationTraversalDirection::Forward => {
                RelationalAuthorizationTraversalDirection::Reverse
            }
            RelationalAuthorizationTraversalDirection::Reverse => {
                RelationalAuthorizationTraversalDirection::Forward
            }
        },
    )
}

fn invalid_policy(subject: &str) -> WorthQueryOperationAuthorizationDenial {
    denial(
        WorthQueryOperationAuthorizationDenialKind::InvalidInstalledPolicy,
        subject,
    )
}

fn denial(
    kind: WorthQueryOperationAuthorizationDenialKind,
    subject: impl Into<String>,
) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(kind, subject)
}
