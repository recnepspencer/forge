use worth_foundational::facade::{AspectFieldLocator, AspectValue};
use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityDelegationRule, ApplicationCapabilityRelationDimension,
    ErasedApplicationCapabilityContract,
};
use worth_relational::facade::authorization::{
    RelationalAuthorizationTraversal, RelationalAuthorizationTraversalDirection,
};

use super::field_binding;
use crate::domain_computation::authorization::capability_binding_lowering::{
    field_locator, relation,
};
use crate::domain_computation::authorization::WorthQueryOperationAuthorizationDenial;
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphLayout;

pub(in crate::domain_computation::authorization) struct WorthQueryCapabilityDelegationBindings {
    pub(in crate::domain_computation::authorization) rule: ApplicationCapabilityDelegationRule,
    pub(in crate::domain_computation::authorization) parent: RelationalAuthorizationTraversal,
    pub(in crate::domain_computation::authorization) grantor: RelationalAuthorizationTraversal,
    pub(in crate::domain_computation::authorization) grantee: RelationalAuthorizationTraversal,
    pub(in crate::domain_computation::authorization) grantee_from_grant:
        RelationalAuthorizationTraversal,
    pub(in crate::domain_computation::authorization) resource: RelationalAuthorizationTraversal,
    pub(in crate::domain_computation::authorization) related:
        Option<RelationalAuthorizationTraversal>,
    pub(in crate::domain_computation::authorization) action: AspectFieldLocator,
    pub(in crate::domain_computation::authorization) purpose: AspectFieldLocator,
    pub(in crate::domain_computation::authorization) disclosure: Option<AspectFieldLocator>,
    pub(in crate::domain_computation::authorization) amount: Option<AspectFieldLocator>,
    pub(in crate::domain_computation::authorization) active_status:
        (AspectFieldLocator, AspectValue),
    pub(in crate::domain_computation::authorization) grant_workflow: AspectFieldLocator,
    pub(in crate::domain_computation::authorization) resource_workflow: AspectFieldLocator,
    pub(in crate::domain_computation::authorization) not_before: AspectFieldLocator,
    pub(in crate::domain_computation::authorization) not_after: AspectFieldLocator,
    pub(in crate::domain_computation::authorization) remaining: AspectFieldLocator,
}

impl WorthQueryCapabilityDelegationBindings {
    pub(in crate::domain_computation::authorization) fn compile(
        contract: &ErasedApplicationCapabilityContract,
        layout: &WorthQueryPrimaryGraphLayout,
    ) -> Result<Self, WorthQueryOperationAuthorizationDenial> {
        let delegation = contract.delegation();
        let currentness = contract.constraints().currentness();
        let workflow = currentness.workflow();
        let validity = currentness.validity();
        Ok(Self {
            rule: contract.composition().propagation().delegation(),
            parent: relation(
                layout,
                delegation.parent(),
                RelationalAuthorizationTraversalDirection::Forward,
            )?,
            grantor: relation(
                layout,
                delegation.grantor(),
                RelationalAuthorizationTraversalDirection::Reverse,
            )?,
            grantee: relation(
                layout,
                delegation.grantee(),
                RelationalAuthorizationTraversalDirection::Forward,
            )?,
            grantee_from_grant: relation(
                layout,
                delegation.grantee(),
                RelationalAuthorizationTraversalDirection::Reverse,
            )?,
            resource: relation(
                layout,
                contract.target().resource(),
                RelationalAuthorizationTraversalDirection::Forward,
            )?,
            related: match contract.target().relation() {
                ApplicationCapabilityRelationDimension::NotApplicable => None,
                ApplicationCapabilityRelationDimension::Bound(binding) => Some(relation(
                    layout,
                    binding,
                    RelationalAuthorizationTraversalDirection::Forward,
                )?),
            },
            action: field_locator(layout, contract.target().action().field())?,
            purpose: field_locator(layout, contract.target().purpose().field())?,
            disclosure: field_binding(contract.target().field())
                .map(|binding| field_locator(layout, binding))
                .transpose()?,
            amount: field_binding(contract.constraints().amount())
                .map(|binding| field_locator(layout, binding))
                .transpose()?,
            active_status: (
                field_locator(layout, currentness.active_status().field())?,
                currentness.active_status().value().clone(),
            ),
            grant_workflow: field_locator(layout, workflow.grant())?,
            resource_workflow: field_locator(layout, workflow.resource())?,
            not_before: field_locator(layout, validity.not_before())?,
            not_after: field_locator(layout, validity.not_after())?,
            remaining: field_locator(layout, delegation.limit())?,
        })
    }
}
