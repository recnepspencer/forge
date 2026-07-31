use std::collections::BTreeSet;

use crate::application_capability::{
    ApplicationCapabilityFieldBinding, ApplicationCapabilityFieldDimension,
    ApplicationCapabilityRelationBinding, ApplicationCapabilityRelationDimension,
    ApplicationCapabilityRule, ErasedApplicationCapabilityContract,
};

use super::{ApplicationSchemaDeclarationDenial, ApplicationSchemaMember};

const MAXIMUM_CAPABILITY_CONTRACTS: usize = 1_024;

pub(super) fn validate_application_capability_members(
    members: &[ApplicationSchemaMember],
) -> Result<(), ApplicationSchemaDeclarationDenial> {
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
        if !names.insert(contract.name()) {
            return Err(ApplicationSchemaDeclarationDenial::DuplicateApplicationCapability);
        }
        validate_contract(members, contract)?;
    }
    Ok(())
}

fn validate_contract(
    members: &[ApplicationSchemaMember],
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
        || !rules_exist(members, contract)
    {
        return Err(ApplicationSchemaDeclarationDenial::MissingApplicationCapabilityDependency);
    }
    if !topology_is_valid(contract)
        || contract
            .composition()
            .decision()
            .allow()
            .policy_name()
            .is_none()
    {
        return Err(ApplicationSchemaDeclarationDenial::InvalidApplicationCapability);
    }
    Ok(())
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

fn rules_exist(
    members: &[ApplicationSchemaMember],
    contract: &ErasedApplicationCapabilityContract,
) -> bool {
    rules(contract).into_iter().all(|rule| match rule {
        ApplicationCapabilityRule::NotApplicable => true,
        ApplicationCapabilityRule::Policy(policy) => members.iter().any(
            |member| matches!(member, ApplicationSchemaMember::Policy { policy: found } if found == policy),
        ),
    })
}

fn rules(contract: &ErasedApplicationCapabilityContract) -> [&ApplicationCapabilityRule; 7] {
    let composition = contract.composition();
    [
        composition.decision().allow(),
        composition.decision().deny(),
        composition.decision().conflict(),
        composition.actors().separation_of_duty(),
        composition.actors().distinct_actor(),
        composition.propagation().delegation(),
        composition.propagation().disclosure(),
    ]
}
