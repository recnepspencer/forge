use crate::application_capability::{
    ApplicationCapabilityFieldBinding, ApplicationCapabilityFieldDimension,
    ApplicationCapabilityRelationBinding, ApplicationCapabilityRelationDimension,
    ApplicationCapabilityRule, ErasedApplicationCapabilityContract,
};

use super::{
    identifier_validation::validate_simple_identifier, ApplicationSchemaDeclarationDenial,
};

pub(super) fn validate_capability_identifiers(
    contract: &ErasedApplicationCapabilityContract,
) -> Result<(), ApplicationSchemaDeclarationDenial> {
    for value in [
        contract.name(),
        contract.operation(),
        contract.grant_entity(),
        contract.constraints().context(),
        contract.delegation().provenance(),
    ] {
        validate_simple_identifier(value)?;
    }
    validate_field(contract.target().action().field())?;
    validate_relation(contract.target().resource())?;
    validate_relation_dimension(contract.target().relation())?;
    validate_field_dimension(contract.target().field())?;
    validate_field(contract.target().purpose().field())?;
    validate_field_dimension(contract.constraints().amount())?;
    validate_field(contract.constraints().workflow_stage())?;
    validate_field(contract.constraints().validity().not_before())?;
    validate_field(contract.constraints().validity().not_after())?;
    validate_relation(contract.delegation().parent())?;
    validate_relation(contract.delegation().grantor())?;
    validate_relation(contract.delegation().grantee())?;
    validate_field(contract.delegation().limit())?;
    for rule in rules(contract) {
        if let Some(policy) = rule.policy_name() {
            validate_simple_identifier(policy)?;
        }
    }
    Ok(())
}

fn validate_field(
    field: &ApplicationCapabilityFieldBinding,
) -> Result<(), ApplicationSchemaDeclarationDenial> {
    for value in [
        field.entity(),
        field.aspect(),
        field.field(),
        field.value_type(),
    ] {
        validate_simple_identifier(value)?;
    }
    Ok(())
}

fn validate_relation(
    relation: &ApplicationCapabilityRelationBinding,
) -> Result<(), ApplicationSchemaDeclarationDenial> {
    for value in [relation.relation(), relation.from(), relation.to()] {
        validate_simple_identifier(value)?;
    }
    Ok(())
}

fn validate_field_dimension(
    dimension: &ApplicationCapabilityFieldDimension,
) -> Result<(), ApplicationSchemaDeclarationDenial> {
    match dimension {
        ApplicationCapabilityFieldDimension::NotApplicable => Ok(()),
        ApplicationCapabilityFieldDimension::Bound(field) => validate_field(field),
    }
}

fn validate_relation_dimension(
    dimension: &ApplicationCapabilityRelationDimension,
) -> Result<(), ApplicationSchemaDeclarationDenial> {
    match dimension {
        ApplicationCapabilityRelationDimension::NotApplicable => Ok(()),
        ApplicationCapabilityRelationDimension::Bound(relation) => validate_relation(relation),
    }
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
