use std::collections::BTreeMap;

use worth_foundational::facade::{AspectFieldLocator, AspectValue};
use worth_query_declaration::facade::application_capability::ApplicationCapabilityDelegationRequestProjection;
use worth_query_declaration::facade::application_schema::ApplicationOperationProgramTarget;
use worth_query_installation::facade::{
    derive_delegation_proposal_identity, WorthQueryDelegationProposalIdentityBasis,
};
use worth_relational::facade::identity::{EntityId, KindId};

use super::super::capability_binding_lowering::field_locator;
use super::super::capability_registry::WorthQueryInstalledCapabilityPlan;
use super::denial;
use super::support::WorthQueryDelegationResolvedRequest;
use crate::domain_computation::authorization::{
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
};

pub(in crate::domain_computation) struct WorthQueryDelegationActivationBinding {
    proposal_identity: [u8; 32],
    pub(in crate::domain_computation) required_program_targets:
        Vec<ApplicationOperationProgramTarget>,
    pub(in crate::domain_computation) child_kind: KindId,
    pub(in crate::domain_computation) child_key: String,
    pub(in crate::domain_computation) fields: BTreeMap<AspectFieldLocator, AspectValue>,
    pub(in crate::domain_computation) parent_relation: KindId,
    pub(in crate::domain_computation) grantor_relation: KindId,
    pub(in crate::domain_computation) grantee_relation: KindId,
    pub(in crate::domain_computation) resource_relation: KindId,
    pub(in crate::domain_computation) related_relation: Option<KindId>,
    pub(in crate::domain_computation) parent: EntityId,
    pub(in crate::domain_computation) grantor: EntityId,
    pub(in crate::domain_computation) grantee: EntityId,
    pub(in crate::domain_computation) resource: EntityId,
    pub(in crate::domain_computation) related: Option<EntityId>,
    pub(in crate::domain_computation) activation_context: Vec<(KindId, EntityId)>,
}

impl WorthQueryDelegationActivationBinding {
    pub(in crate::domain_computation) const fn proposal_identity(&self) -> &[u8; 32] {
        &self.proposal_identity
    }
}

pub(super) struct WorthQueryPreparedDelegationActivation {
    pub(super) binding: WorthQueryDelegationActivationBinding,
    pub(super) canonical_work: worth_query_installation::facade::WorthQueryCanonicalWorkEvidence,
}

pub(super) fn bind_activation<Schema, Scope, Context>(
    runtime: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<
        Schema,
    >,
    installed: &WorthQueryInstalledCapabilityPlan,
    proposed: &ApplicationCapabilityDelegationRequestProjection<Schema, Scope, Context>,
    resolved: WorthQueryDelegationResolvedRequest,
    target_capability_identity: [u8; 32],
    required_program_targets: &[ApplicationOperationProgramTarget],
    proposal_budget: worth_foundational::facade::CanonicalDigestWorkBudget,
) -> Result<WorthQueryPreparedDelegationActivation, WorthQueryOperationAuthorizationDenial>
where
    Schema: worth_query_installation::facade::ApplicationSchema,
{
    let activation = installed
        .delegation
        .activation
        .as_ref()
        .ok_or_else(|| delegation_denial(installed))?;
    validate_field_bindings(runtime, installed, proposed, activation)?;
    let fields = child_fields(installed, proposed, &activation.identity)?;
    let related = match (&installed.delegation.related, resolved.related) {
        (Some(relation), Some(entity)) => Some((relation.relation_kind(), entity)),
        (None, None) => None,
        _ => return Err(delegation_denial(installed)),
    };
    let activation_context = resolved
        .activation_context
        .into_iter()
        .map(|context| (context.traversal.relation_kind(), context.entity))
        .collect::<Vec<_>>();
    let canonical_activation_context = activation_context
        .iter()
        .copied()
        .map(|(relation, entity)| canonical_relation(relation, entity))
        .collect::<Vec<_>>();
    let (proposal_identity, canonical_work) = derive_delegation_proposal_identity(
        WorthQueryDelegationProposalIdentityBasis {
            target_capability_identity,
            child_kind: installed.grant_kind.as_u32(),
            child_key: proposed.child_key(),
            fields: &fields,
            parent: canonical_relation(
                installed.delegation.parent.relation_kind(),
                resolved.parent,
            ),
            grantor: canonical_relation(
                installed.delegation.grantor.relation_kind(),
                resolved.grantor,
            ),
            grantee: canonical_relation(
                installed.delegation.grantee.relation_kind(),
                resolved.grantee,
            ),
            resource: canonical_relation(
                installed.delegation.resource.relation_kind(),
                resolved.resource,
            ),
            related: related.map(|(relation, entity)| canonical_relation(relation, entity)),
            activation_context: &canonical_activation_context,
        },
        proposal_budget,
    )
    .map_err(|_| delegation_denial(installed))?;
    let binding = WorthQueryDelegationActivationBinding {
        proposal_identity,
        required_program_targets: required_program_targets.to_vec(),
        child_kind: installed.grant_kind,
        child_key: proposed.child_key().to_string(),
        fields,
        parent_relation: installed.delegation.parent.relation_kind(),
        grantor_relation: installed.delegation.grantor.relation_kind(),
        grantee_relation: installed.delegation.grantee.relation_kind(),
        resource_relation: installed.delegation.resource.relation_kind(),
        related_relation: related.map(|(relation, _)| relation),
        parent: resolved.parent,
        grantor: resolved.grantor,
        grantee: resolved.grantee,
        resource: resolved.resource,
        related: related.map(|(_, entity)| entity),
        activation_context,
    };
    Ok(WorthQueryPreparedDelegationActivation {
        binding,
        canonical_work,
    })
}

fn canonical_relation(relation: KindId, entity: EntityId) -> (u32, (u32, u64, u32)) {
    (
        relation.as_u32(),
        (
            entity.partition_id.0,
            entity.local_slot.0,
            entity.generation.0,
        ),
    )
}

fn validate_field_bindings<Schema, Scope, Context>(
    runtime: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<
        Schema,
    >,
    installed: &WorthQueryInstalledCapabilityPlan,
    proposed: &ApplicationCapabilityDelegationRequestProjection<Schema, Scope, Context>,
    activation: &super::super::capability_registry::WorthQueryCapabilityDelegationActivationBindings,
) -> Result<(), WorthQueryOperationAuthorizationDenial>
where
    Schema: worth_query_installation::facade::ApplicationSchema,
{
    let graph = runtime
        .runtime
        .primary_graph()
        .ok_or_else(|| delegation_denial(installed))?;
    let layout = graph.layout();
    let bindings = [
        (proposed.child_identity().field(), &activation.identity),
        (
            proposed.workflow().field(),
            &installed.delegation.grant_workflow,
        ),
        (
            proposed.not_before().field(),
            &installed.delegation.not_before,
        ),
        (
            proposed.not_after().field(),
            &installed.delegation.not_after,
        ),
        (
            proposed.remaining_delegations().field(),
            &installed.delegation.remaining,
        ),
    ];
    if bindings.into_iter().all(|(declared, expected)| {
        field_locator(layout, declared).is_ok_and(|found| &found == expected)
    }) && proposed.not_before().value() <= proposed.not_after().value()
    {
        Ok(())
    } else {
        Err(delegation_denial(installed))
    }
}

fn child_fields<Schema, Scope, Context>(
    installed: &WorthQueryInstalledCapabilityPlan,
    proposed: &ApplicationCapabilityDelegationRequestProjection<Schema, Scope, Context>,
    identity: &AspectFieldLocator,
) -> Result<BTreeMap<AspectFieldLocator, AspectValue>, WorthQueryOperationAuthorizationDenial> {
    let target = proposed.target();
    let mut fields = BTreeMap::from([
        (identity.clone(), proposed.child_identity().value().clone()),
        (installed.delegation.action.clone(), target.action().clone()),
        (
            installed.delegation.purpose.clone(),
            target.purpose().clone(),
        ),
        (
            installed.delegation.active_status.0.clone(),
            installed.delegation.active_status.1.clone(),
        ),
        (
            installed.delegation.grant_workflow.clone(),
            proposed.workflow().value().clone(),
        ),
        (
            installed.delegation.not_before.clone(),
            proposed.not_before().value().clone(),
        ),
        (
            installed.delegation.not_after.clone(),
            proposed.not_after().value().clone(),
        ),
        (
            installed.delegation.remaining.clone(),
            proposed.remaining_delegations().value().clone(),
        ),
    ]);
    bind_optional(
        &mut fields,
        installed.delegation.disclosure.as_ref(),
        target.field_value(),
    )?;
    bind_optional(
        &mut fields,
        installed.delegation.magnitude.as_ref(),
        target.magnitude_value(),
    )?;
    let expected = 8
        + usize::from(installed.delegation.disclosure.is_some())
        + usize::from(installed.delegation.magnitude.is_some());
    (fields.len() == expected)
        .then_some(fields)
        .ok_or_else(|| delegation_denial(installed))
}

fn bind_optional(
    fields: &mut BTreeMap<AspectFieldLocator, AspectValue>,
    locator: Option<&AspectFieldLocator>,
    value: Option<&AspectValue>,
) -> Result<(), WorthQueryOperationAuthorizationDenial> {
    match (locator, value) {
        (Some(locator), Some(value)) => {
            fields.insert(locator.clone(), value.clone());
            Ok(())
        }
        (None, None) => Ok(()),
        _ => Err(denial(
            WorthQueryOperationAuthorizationDenialKind::DelegationRejected,
            "delegated capability optional dimension",
        )),
    }
}

fn delegation_denial(
    installed: &WorthQueryInstalledCapabilityPlan,
) -> WorthQueryOperationAuthorizationDenial {
    denial(
        WorthQueryOperationAuthorizationDenialKind::DelegationRejected,
        installed.contract.name(),
    )
}
