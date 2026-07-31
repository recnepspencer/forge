use super::*;

#[test]
fn capability_contract_requires_every_graph_rule_relation() {
    let mut members = members(contract(false, false, true));
    members.retain(|member| {
        !matches!(
            member,
            ApplicationSchemaMember::Relation { relation, .. }
                if relation == "PrincipalResource"
        )
    });
    assert_eq!(
        build_from_members(members),
        Err(ApplicationSchemaDeclarationDenial::InvalidApplicationCapability)
    );
}

#[test]
fn capability_resource_must_begin_at_the_declared_grant_entity() {
    assert_eq!(
        build_from_members(members(contract(true, false, true))),
        Err(ApplicationSchemaDeclarationDenial::InvalidApplicationCapability)
    );
}

#[test]
fn capability_names_are_unique_even_when_other_meaning_differs() {
    let mut members = members(contract(false, false, true));
    members.push(ApplicationSchemaMember::ApplicationCapability {
        contract: contract(false, true, true),
    });
    assert_eq!(
        build_from_members(members),
        Err(ApplicationSchemaDeclarationDenial::DuplicateApplicationCapability)
    );
}

#[test]
fn field_bound_capability_requires_an_explicit_disclosure_contract() {
    assert_eq!(
        build_from_members(members(contract(false, false, false))),
        Err(ApplicationSchemaDeclarationDenial::InvalidApplicationCapability)
    );
}

#[test]
fn capability_currentness_requires_status_and_resource_workflow_fields() {
    for missing in ["Status", "ResourceWorkflow"] {
        let mut members = members(contract(false, false, true));
        members.retain(|member| {
            !matches!(
                member,
                ApplicationSchemaMember::Field { field, .. } if field == missing
            )
        });
        assert_eq!(
            build_from_members(members),
            Err(ApplicationSchemaDeclarationDenial::MissingApplicationCapabilityDependency),
            "{missing} must remain an installed currentness dependency"
        );
    }
}

#[test]
fn capability_validity_timeline_requires_compatible_field_families() {
    let mut members = members(contract(false, false, true));
    for member in &mut members {
        if let ApplicationSchemaMember::Field {
            field,
            scalar_family,
            ..
        } = member
        {
            if field == "ValidFrom" {
                *scalar_family = ScalarAspectType::Int64;
            }
        }
    }
    assert_eq!(
        build_from_members(members),
        Err(ApplicationSchemaDeclarationDenial::MissingApplicationCapabilityDependency)
    );
}
