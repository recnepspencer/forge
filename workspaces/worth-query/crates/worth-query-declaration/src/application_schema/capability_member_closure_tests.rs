use worth_foundational::facade::ScalarAspectType;

use crate::application_capability::{
    ApplicationCapabilityAcceptedValues, ApplicationCapabilityActorComposition,
    ApplicationCapabilityAllowRule, ApplicationCapabilityCardinalityDimension,
    ApplicationCapabilityComposition, ApplicationCapabilityConflictRule,
    ApplicationCapabilityConstraintDefinition, ApplicationCapabilityContextRef,
    ApplicationCapabilityContractBuilder, ApplicationCapabilityDecisionComposition,
    ApplicationCapabilityDelegationDefinition, ApplicationCapabilityDelegationRule,
    ApplicationCapabilityDenyRule, ApplicationCapabilityDisclosureRule,
    ApplicationCapabilityDistinctActorRule, ApplicationCapabilityFieldBinding,
    ApplicationCapabilityFieldDimension, ApplicationCapabilityGraphClause,
    ApplicationCapabilityGraphRule, ApplicationCapabilityPropagationComposition,
    ApplicationCapabilityProvenanceRef, ApplicationCapabilityRef,
    ApplicationCapabilityRelationBinding, ApplicationCapabilityRelationDimension,
    ApplicationCapabilityScopeGuard, ApplicationCapabilitySeparationOfDutyRule,
    ApplicationCapabilityTargetDefinition, ApplicationCapabilityValidityDefinition,
    ApplicationCapabilityValueBinding,
};

use super::{
    capability_member_closure::validate_application_capability_members,
    ApplicationAuthorizationPathBuilder, ApplicationEntityRef, ApplicationFieldRef,
    ApplicationOperationRef, ApplicationRelationRef, ApplicationSchemaDeclarationDenial,
    ApplicationSchemaMember, EqualityPredicate, NoApplicationCurrency, ReadOnly,
};

struct Schema;
struct Capability;
struct Operation;
struct Grant;
struct Resource;
struct Principal;
struct Facts;
struct Action;
struct Purpose;
struct Field;
struct Amount;
struct Workflow;
struct ValidFrom;
struct ValidThrough;
struct DelegationLimit;
struct ResourceRelation;
struct WrongResourceRelation;
struct ScopedRelation;
struct PrincipalResource;
struct Parent;
struct Grantor;
struct Grantee;
struct Context;
struct Provenance;

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
    let members = members(contract(true, false, true));
    assert_eq!(
        build_from_members(members),
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
    let members = members(contract(false, false, false));
    assert_eq!(
        build_from_members(members),
        Err(ApplicationSchemaDeclarationDenial::InvalidApplicationCapability)
    );
}

fn build_from_members(
    members: Vec<ApplicationSchemaMember>,
) -> Result<(), ApplicationSchemaDeclarationDenial> {
    validate_application_capability_members(&members)
}

fn members(
    contract: crate::application_capability::ErasedApplicationCapabilityContract,
) -> Vec<ApplicationSchemaMember> {
    let mut members = vec![
        ApplicationSchemaMember::Entity {
            entity: "Grant".to_string(),
        },
        ApplicationSchemaMember::Entity {
            entity: "Resource".to_string(),
        },
        ApplicationSchemaMember::Entity {
            entity: "Principal".to_string(),
        },
        ApplicationSchemaMember::Aspect {
            entity: "Grant".to_string(),
            aspect: "Facts".to_string(),
        },
        ApplicationSchemaMember::PrincipalBinding {
            binding: "PrincipalBinding".to_string(),
            mapping_entity: "Grant".to_string(),
            identity_aspect: "Facts".to_string(),
            identity_field: "Action".to_string(),
            status_aspect: "Facts".to_string(),
            status_field: "Purpose".to_string(),
            target_relation: "Grantor".to_string(),
            principal_entity: "Principal".to_string(),
            principal_identity_aspect: "Facts".to_string(),
            principal_identity_field: "Field".to_string(),
            principal_identity_scalar_family: ScalarAspectType::UInt64,
            principal_identity_value_type: std::any::type_name::<u64>().to_string(),
        },
        ApplicationSchemaMember::Operation {
            operation: "Operation".to_string(),
            input_type: std::any::type_name::<()>().to_string(),
        },
        relation_member("ResourceRelation", "Grant", "Resource"),
        relation_member("WrongResourceRelation", "Principal", "Resource"),
        relation_member("ScopedRelation", "Grant", "Resource"),
        relation_member("PrincipalResource", "Principal", "Resource"),
        relation_member("Parent", "Grant", "Grant"),
        relation_member("Grantor", "Principal", "Grant"),
        relation_member("Grantee", "Principal", "Grant"),
    ];
    for field in [
        "Action",
        "Purpose",
        "Field",
        "Amount",
        "Workflow",
        "ValidFrom",
        "ValidThrough",
        "DelegationLimit",
    ] {
        members.push(field_member(field));
    }
    members.push(ApplicationSchemaMember::ApplicationCapability { contract });
    members
}

fn contract(
    wrong_resource_topology: bool,
    changed_purpose: bool,
    disclosure: bool,
) -> crate::application_capability::ErasedApplicationCapabilityContract {
    let resource = if wrong_resource_topology {
        relation::<WrongResourceRelation, Principal, Resource>(
            "WrongResourceRelation",
            "Principal",
            "Resource",
        )
    } else {
        relation::<ResourceRelation, Grant, Resource>("ResourceRelation", "Grant", "Resource")
    };
    let target = ApplicationCapabilityTargetDefinition::new(
        ApplicationCapabilityValueBinding::new(field::<Action>("Action"), 1_u64),
        resource,
        ApplicationCapabilityRelationDimension::Bound(relation::<ScopedRelation, Grant, Resource>(
            "ScopedRelation",
            "Grant",
            "Resource",
        )),
        ApplicationCapabilityFieldDimension::bound(field::<Field>("Field")),
        ApplicationCapabilityValueBinding::new(
            field::<Purpose>("Purpose"),
            if changed_purpose { 2_u64 } else { 1_u64 },
        ),
    );
    let constraints = ApplicationCapabilityConstraintDefinition::new(
        ApplicationCapabilityFieldDimension::bound(field::<Amount>("Amount")),
        ApplicationCapabilityCardinalityDimension::One,
        binding::<Workflow>("Workflow"),
        ApplicationCapabilityValidityDefinition::new(
            binding::<ValidFrom>("ValidFrom"),
            binding::<ValidThrough>("ValidThrough"),
        ),
        ApplicationCapabilityContextRef::<Schema, Context>::from_schema_identifier("Context"),
    );
    let delegation = ApplicationCapabilityDelegationDefinition::new(
        relation::<Parent, Grant, Grant>("Parent", "Grant", "Grant"),
        relation::<Grantor, Principal, Grant>("Grantor", "Principal", "Grant"),
        relation::<Grantee, Principal, Grant>("Grantee", "Principal", "Grant"),
        binding::<DelegationLimit>("DelegationLimit"),
        ApplicationCapabilityProvenanceRef::<Schema, Provenance>::from_schema_identifier(
            "Provenance",
        ),
    );
    ApplicationCapabilityContractBuilder::new(
        ApplicationCapabilityRef::<Schema, Capability>::from_schema_identifier("Capability"),
        ApplicationOperationRef::<Schema, Operation, ()>::from_schema_identifier("Operation"),
        ApplicationEntityRef::<Schema, Grant>::from_schema_identifier("Grant"),
    )
    .target(target)
    .constraints(constraints)
    .delegation(delegation)
    .composition(composition(disclosure))
    .build()
    .erased()
    .clone()
}

fn composition(disclosure: bool) -> ApplicationCapabilityComposition {
    let allow = ApplicationCapabilityGraphRule::any([ApplicationCapabilityGraphClause::new(
        ApplicationAuthorizationPathBuilder::from_principal(ApplicationEntityRef::<
            Schema,
            Principal,
        >::from_schema_identifier(
            "Principal"
        ))
        .forward(ApplicationRelationRef::<
            Schema,
            PrincipalResource,
            Principal,
            Resource,
        >::from_schema_identifiers(
            "PrincipalResource",
            "Principal",
            "Resource",
        ))
        .allow(ApplicationEntityRef::<Schema, Resource>::from_schema_identifier("Resource")),
    )]);
    let disclosure = if disclosure {
        ApplicationCapabilityDisclosureRule::permit([ApplicationCapabilityScopeGuard::requiring([
            ApplicationCapabilityAcceptedValues::one_of(field::<Field>("Field"), [1_u64]),
        ])])
    } else {
        ApplicationCapabilityDisclosureRule::not_applicable()
    };
    ApplicationCapabilityComposition::new(
        ApplicationCapabilityDecisionComposition::new(
            ApplicationCapabilityAllowRule::new(allow),
            ApplicationCapabilityDenyRule::not_applicable(),
            ApplicationCapabilityConflictRule::not_applicable(),
        ),
        ApplicationCapabilityActorComposition::new(
            ApplicationCapabilitySeparationOfDutyRule::not_applicable(),
            ApplicationCapabilityDistinctActorRule::not_applicable(),
        ),
        ApplicationCapabilityPropagationComposition::new(
            ApplicationCapabilityDelegationRule::forbidden(),
            disclosure,
        ),
    )
}

fn field<FieldMarker>(
    name: &'static str,
) -> ApplicationFieldRef<
    Schema,
    Grant,
    Facts,
    FieldMarker,
    u64,
    ReadOnly,
    EqualityPredicate,
    NoApplicationCurrency,
> {
    ApplicationFieldRef::from_schema_identifiers("Grant", "Facts", name)
}

fn binding<FieldMarker>(name: &'static str) -> ApplicationCapabilityFieldBinding {
    ApplicationCapabilityFieldBinding::from_reference(field::<FieldMarker>(name))
}

fn relation<RelationMarker, From, To>(
    name: &'static str,
    from: &'static str,
    to: &'static str,
) -> ApplicationCapabilityRelationBinding {
    ApplicationCapabilityRelationBinding::from_reference(ApplicationRelationRef::<
        Schema,
        RelationMarker,
        From,
        To,
    >::from_schema_identifiers(
        name, from, to
    ))
}

fn field_member(field: &str) -> ApplicationSchemaMember {
    ApplicationSchemaMember::Field {
        entity: "Grant".to_string(),
        aspect: "Facts".to_string(),
        field: field.to_string(),
        scalar_family: ScalarAspectType::UInt64,
        value_type: std::any::type_name::<u64>().to_string(),
        currency: None,
        writable: false,
        equality_queryable: true,
    }
}

fn relation_member(relation: &str, from: &str, to: &str) -> ApplicationSchemaMember {
    ApplicationSchemaMember::Relation {
        relation: relation.to_string(),
        from: from.to_string(),
        to: to.to_string(),
    }
}
