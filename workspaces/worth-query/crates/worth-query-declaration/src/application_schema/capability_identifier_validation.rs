use crate::application_capability::{
    ApplicationCapabilityDisclosureRule, ApplicationCapabilityFieldBinding,
    ApplicationCapabilityFieldDimension, ApplicationCapabilityGraphRule,
    ApplicationCapabilityRelationBinding, ApplicationCapabilityRelationDimension,
    ApplicationCapabilityScopeGuard, ErasedApplicationCapabilityContract,
};

use super::identifier_validation::{validate_authorization_path, validate_simple_identifier};
use super::ApplicationSchemaDeclarationDenial;

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
    let currentness = contract.constraints().currentness();
    validate_field(currentness.active_status().field())?;
    validate_field(currentness.workflow().grant())?;
    validate_field(currentness.workflow().resource())?;
    validate_field(currentness.validity().not_before())?;
    validate_field(currentness.validity().not_after())?;
    validate_relation(contract.delegation().parent())?;
    validate_relation(contract.delegation().grantor())?;
    validate_relation(contract.delegation().grantee())?;
    validate_field(contract.delegation().limit())?;
    if let Some(activation) = contract.delegation().activation() {
        validate_simple_identifier(activation.operation().operation())?;
        validate_field(activation.identity())?;
    }
    validate_composition(contract)
}

fn validate_composition(
    contract: &ErasedApplicationCapabilityContract,
) -> Result<(), ApplicationSchemaDeclarationDenial> {
    let composition = contract.composition();
    validate_graph_rule(composition.decision().allow().graph())?;
    for rule in [
        composition.decision().deny().graph(),
        composition.decision().conflict().graph(),
        composition.actors().separation_of_duty().graph(),
        composition.actors().distinct_actor().graph(),
    ]
    .into_iter()
    .flatten()
    {
        validate_graph_rule(rule)?;
    }
    if let ApplicationCapabilityDisclosureRule::Permit(guards) =
        composition.propagation().disclosure()
    {
        for guard in guards {
            validate_guard(guard)?;
        }
    }
    Ok(())
}

fn validate_graph_rule(
    rule: &ApplicationCapabilityGraphRule,
) -> Result<(), ApplicationSchemaDeclarationDenial> {
    for requirement in rule.requirements() {
        for clause in requirement.clauses() {
            validate_authorization_path(clause.path())?;
            validate_guard(clause.guard())?;
            for anchor in clause.context_anchors() {
                validate_relation(anchor.relation())?;
                validate_identifiers([
                    anchor.slot().context(),
                    anchor.slot().slot(),
                    anchor.slot().entity(),
                ])?;
            }
        }
    }
    Ok(())
}

fn validate_identifiers<'a>(
    values: impl IntoIterator<Item = &'a str>,
) -> Result<(), ApplicationSchemaDeclarationDenial> {
    for value in values {
        validate_simple_identifier(value)?;
    }
    Ok(())
}

fn validate_guard(
    guard: &ApplicationCapabilityScopeGuard,
) -> Result<(), ApplicationSchemaDeclarationDenial> {
    for requirement in guard.requirements() {
        validate_field(requirement.field())?;
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
