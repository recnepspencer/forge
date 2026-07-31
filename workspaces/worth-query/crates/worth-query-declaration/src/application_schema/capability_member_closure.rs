use std::collections::BTreeSet;

use crate::application_capability::{
    ApplicationCapabilityDisclosureRule, ApplicationCapabilityFieldBinding,
    ApplicationCapabilityFieldDimension, ApplicationCapabilityGraphRule,
    ApplicationCapabilityRelationBinding, ApplicationCapabilityRelationDimension,
    ApplicationCapabilityScopeGuard, ErasedApplicationCapabilityContract,
};

use super::member_closure::ClosureIndex;
use super::{
    ApplicationAuthorizationPathEffect, ApplicationSchemaDeclarationDenial, ApplicationSchemaMember,
};

const MAXIMUM_CAPABILITY_CONTRACTS: usize = 1_024;

pub(super) fn validate_application_capability_members(
    members: &[ApplicationSchemaMember],
) -> Result<(), ApplicationSchemaDeclarationDenial> {
    let closure = ClosureIndex::new(members);
    let contracts = members
        .iter()
        .filter_map(|member| match member {
            ApplicationSchemaMember::ApplicationCapability { contract } => Some(contract),
            _ => None,
        })
        .collect::<Vec<_>>();
    if contracts.len() > MAXIMUM_CAPABILITY_CONTRACTS {
        return Err(ApplicationSchemaDeclarationDenial::InvalidApplicationCapability);
    }
    let mut names = BTreeSet::new();
    for contract in contracts {
        if !names.insert((contract.name(), contract.capability_type())) {
            return Err(ApplicationSchemaDeclarationDenial::DuplicateApplicationCapability);
        }
        validate_contract(members, &closure, contract)?;
    }
    Ok(())
}

fn validate_contract(
    members: &[ApplicationSchemaMember],
    closure: &ClosureIndex<'_>,
    contract: &ErasedApplicationCapabilityContract,
) -> Result<(), ApplicationSchemaDeclarationDenial> {
    if !operation_exists(members, contract)
        || !entity_exists(members, contract.grant_entity())
        || !field_exists(members, contract.target().action().field())
        || !field_exists(members, contract.target().purpose().field())
        || !relation_exists(members, contract.target().resource())
        || !field_dimension_exists(members, contract.target().field())
        || !relation_dimension_exists(members, contract.target().relation())
        || !field_dimension_exists(members, contract.constraints().amount())
        || !field_exists(members, contract.constraints().workflow_stage())
        || !field_exists(members, contract.constraints().validity().not_before())
        || !field_exists(members, contract.constraints().validity().not_after())
        || !relation_exists(members, contract.delegation().parent())
        || !relation_exists(members, contract.delegation().grantor())
        || !relation_exists(members, contract.delegation().grantee())
        || !field_exists(members, contract.delegation().limit())
    {
        return Err(ApplicationSchemaDeclarationDenial::MissingApplicationCapabilityDependency);
    }
    if !topology_is_valid(contract) || !composition_is_closed(closure, contract) {
        return Err(ApplicationSchemaDeclarationDenial::InvalidApplicationCapability);
    }
    Ok(())
}

fn composition_is_closed(
    closure: &ClosureIndex<'_>,
    contract: &ErasedApplicationCapabilityContract,
) -> bool {
    let composition = contract.composition();
    graph_rule_is_closed(
        closure,
        contract,
        composition.decision().allow().graph(),
        ApplicationAuthorizationPathEffect::Allow,
    ) && optional_graph_rule_is_closed(closure, contract, composition.decision().deny().graph())
        && optional_graph_rule_is_closed(
            closure,
            contract,
            composition.decision().conflict().graph(),
        )
        && optional_graph_rule_is_closed(
            closure,
            contract,
            composition.actors().separation_of_duty().graph(),
        )
        && optional_graph_rule_is_closed(
            closure,
            contract,
            composition.actors().distinct_actor().graph(),
        )
        && disclosure_rule_is_closed(contract, composition.propagation().disclosure())
}

fn optional_graph_rule_is_closed(
    closure: &ClosureIndex<'_>,
    contract: &ErasedApplicationCapabilityContract,
    rule: Option<&ApplicationCapabilityGraphRule>,
) -> bool {
    rule.is_none_or(|rule| {
        graph_rule_is_closed(
            closure,
            contract,
            rule,
            ApplicationAuthorizationPathEffect::Deny,
        )
    })
}

fn graph_rule_is_closed(
    closure: &ClosureIndex<'_>,
    contract: &ErasedApplicationCapabilityContract,
    rule: &ApplicationCapabilityGraphRule,
    expected_effect: ApplicationAuthorizationPathEffect,
) -> bool {
    !rule.clauses().is_empty()
        && rule.clauses().iter().all(|clause| {
            clause.path().effect() == expected_effect
                && closure
                    .authorization_path_is_closed(contract.target().resource().to(), clause.path())
                && guard_is_owned(contract, clause.guard())
        })
}

fn disclosure_rule_is_closed(
    contract: &ErasedApplicationCapabilityContract,
    rule: &ApplicationCapabilityDisclosureRule,
) -> bool {
    match (contract.target().field(), rule) {
        (
            ApplicationCapabilityFieldDimension::NotApplicable,
            ApplicationCapabilityDisclosureRule::NotApplicable,
        ) => true,
        (
            ApplicationCapabilityFieldDimension::Bound(disclosed_field),
            ApplicationCapabilityDisclosureRule::Permit(guards),
        ) => {
            !guards.is_empty()
                && guards.iter().all(|guard| {
                    guard_is_owned(contract, guard)
                        && guard
                            .requirements()
                            .iter()
                            .any(|requirement| requirement.field() == disclosed_field)
                })
        }
        _ => false,
    }
}

fn guard_is_owned(
    contract: &ErasedApplicationCapabilityContract,
    guard: &ApplicationCapabilityScopeGuard,
) -> bool {
    guard
        .requirements()
        .iter()
        .all(|requirement| requirement_is_owned(contract, requirement))
}

fn requirement_is_owned(
    contract: &ErasedApplicationCapabilityContract,
    requirement: &crate::application_capability::ApplicationCapabilityAcceptedValues,
) -> bool {
    if requirement.values().is_empty() {
        return false;
    }
    let target = contract.target();
    let constraints = contract.constraints();
    let delegation = contract.delegation();
    requirement_matches_fixed(requirement, target.action())
        || requirement_matches_fixed(requirement, target.purpose())
        || field_dimension_owns(target.field(), requirement.field())
        || field_dimension_owns(constraints.amount(), requirement.field())
        || requirement.field() == constraints.workflow_stage()
        || requirement.field() == constraints.validity().not_before()
        || requirement.field() == constraints.validity().not_after()
        || requirement.field() == delegation.limit()
}

fn requirement_matches_fixed(
    requirement: &crate::application_capability::ApplicationCapabilityAcceptedValues,
    fixed: &crate::application_capability::ApplicationCapabilityValueBinding,
) -> bool {
    requirement.field() == fixed.field()
        && requirement.values().len() == 1
        && requirement.values().first() == Some(fixed.value())
}

fn topology_is_valid(contract: &ErasedApplicationCapabilityContract) -> bool {
    let grant = contract.grant_entity();
    let target = contract.target();
    let constraints = contract.constraints();
    let delegation = contract.delegation();
    target.action().field().entity() == grant
        && target.purpose().field().entity() == grant
        && target.resource().from() == grant
        && constraints.workflow_stage().entity() == grant
        && constraints.validity().not_before().entity() == grant
        && constraints.validity().not_after().entity() == grant
        && delegation.limit().entity() == grant
        && delegation.parent().from() == grant
        && delegation.parent().to() == grant
        && delegation.grantor().to() == grant
        && delegation.grantee().to() == grant
        && field_dimension_belongs_to(target.field(), grant)
        && field_dimension_belongs_to(constraints.amount(), grant)
}

fn operation_exists(
    members: &[ApplicationSchemaMember],
    contract: &ErasedApplicationCapabilityContract,
) -> bool {
    members.iter().any(|member| {
        matches!(
            member,
            ApplicationSchemaMember::Operation {
                operation,
                input_type,
            } if operation == contract.operation() && input_type == contract.input_type()
        )
    })
}

fn entity_exists(members: &[ApplicationSchemaMember], entity: &str) -> bool {
    members.iter().any(
        |member| matches!(member, ApplicationSchemaMember::Entity { entity: found } if found == entity),
    )
}

fn field_exists(
    members: &[ApplicationSchemaMember],
    binding: &ApplicationCapabilityFieldBinding,
) -> bool {
    members.iter().any(|member| {
        matches!(
            member,
            ApplicationSchemaMember::Field {
                entity,
                aspect,
                field,
                value_type,
                ..
            } if entity == binding.entity()
                && aspect == binding.aspect()
                && field == binding.field()
                && value_type == binding.value_type()
        )
    })
}

fn relation_exists(
    members: &[ApplicationSchemaMember],
    binding: &ApplicationCapabilityRelationBinding,
) -> bool {
    members.iter().any(|member| {
        matches!(
            member,
            ApplicationSchemaMember::Relation { relation, from, to }
                if relation == binding.relation()
                    && from == binding.from()
                    && to == binding.to()
        )
    })
}

fn field_dimension_exists(
    members: &[ApplicationSchemaMember],
    dimension: &ApplicationCapabilityFieldDimension,
) -> bool {
    match dimension {
        ApplicationCapabilityFieldDimension::NotApplicable => true,
        ApplicationCapabilityFieldDimension::Bound(field) => field_exists(members, field),
    }
}

fn relation_dimension_exists(
    members: &[ApplicationSchemaMember],
    dimension: &ApplicationCapabilityRelationDimension,
) -> bool {
    match dimension {
        ApplicationCapabilityRelationDimension::NotApplicable => true,
        ApplicationCapabilityRelationDimension::Bound(relation) => {
            relation_exists(members, relation)
        }
    }
}

fn field_dimension_belongs_to(
    dimension: &ApplicationCapabilityFieldDimension,
    grant_entity: &str,
) -> bool {
    match dimension {
        ApplicationCapabilityFieldDimension::NotApplicable => true,
        ApplicationCapabilityFieldDimension::Bound(field) => field.entity() == grant_entity,
    }
}

fn field_dimension_owns(
    dimension: &ApplicationCapabilityFieldDimension,
    field: &ApplicationCapabilityFieldBinding,
) -> bool {
    matches!(dimension, ApplicationCapabilityFieldDimension::Bound(bound) if bound == field)
}
