use super::{
    capability_member_closure::validate_application_capability_members,
    ApplicationAuthorizationPathBuilder, ApplicationEntityRef, ApplicationFieldRef,
    ApplicationOperationRef, ApplicationRelationRef, ApplicationSchemaDeclarationDenial,
    ApplicationSchemaMember, EqualityPredicate, NoApplicationCurrency, ReadOnly,
};
use crate::application_capability::{
    ApplicationCapabilityAcceptedValues, ApplicationCapabilityActorComposition,
    ApplicationCapabilityAllowRule, ApplicationCapabilityCardinalityDimension,
    ApplicationCapabilityComposition, ApplicationCapabilityConflictRule,
    ApplicationCapabilityConstraintDefinition, ApplicationCapabilityContextEntitySlotRef,
    ApplicationCapabilityContextRef, ApplicationCapabilityContractBuilder,
    ApplicationCapabilityCurrentnessDefinition, ApplicationCapabilityDecisionComposition,
    ApplicationCapabilityDelegationDefinition, ApplicationCapabilityDelegationRule,
    ApplicationCapabilityDenyRule, ApplicationCapabilityDisclosureRule,
    ApplicationCapabilityDistinctActorRule, ApplicationCapabilityElevationRule,
    ApplicationCapabilityFieldBinding, ApplicationCapabilityFieldDimension,
    ApplicationCapabilityGraphClause, ApplicationCapabilityGraphRule,
    ApplicationCapabilityPathContextAnchor, ApplicationCapabilityPropagationComposition,
    ApplicationCapabilityProvenanceRef, ApplicationCapabilityRef,
    ApplicationCapabilityRelationBinding, ApplicationCapabilityRelationDimension,
    ApplicationCapabilityScopeGuard, ApplicationCapabilitySeparationOfDutyRule,
    ApplicationCapabilityTargetDefinition, ApplicationCapabilityValidityDefinition,
    ApplicationCapabilityValidityTimeline, ApplicationCapabilityValueBinding,
    ApplicationCapabilityWorkflowDefinition,
};
use worth_foundational::facade::ScalarAspectType;

struct Schema;
struct Capability;
struct Operation;
struct Grant;
struct Resource;
struct Principal;
struct Facts;
struct ResourceFacts;
struct Action;
struct Purpose;
struct Field;
struct Amount;
struct Workflow;
struct ResourceWorkflow;
struct Status;
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
struct ResourceSlot;
struct MissingResourceSlot;
struct OtherContext;
struct OtherResourceSlot;
type ErasedContract = crate::application_capability::ErasedApplicationCapabilityContract;

#[path = "capability_member_closure_tests/context_anchors.rs"]
mod context_anchors;
#[path = "capability_member_closure_tests/contract_validation.rs"]
mod contract_validation;
#[path = "capability_member_closure_tests/declared_dimensions.rs"]
mod declared_dimensions;
#[path = "capability_member_closure_tests/elevation_lifecycle.rs"]
mod elevation_lifecycle;
#[path = "capability_member_closure_tests/fixture_members.rs"]
mod fixture_members;
#[path = "capability_member_closure_tests/population_budget.rs"]
mod population_budget;

use fixture_members::{field_member, relation_member, resource_field_member};

fn build_from_members(
    members: Vec<ApplicationSchemaMember>,
) -> Result<(), ApplicationSchemaDeclarationDenial> {
    validate_application_capability_members(&members)
}

fn members(contract: ErasedContract) -> Vec<ApplicationSchemaMember> {
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
        ApplicationSchemaMember::Aspect {
            entity: "Resource".to_string(),
            aspect: "ResourceFacts".to_string(),
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
        ApplicationSchemaMember::ApplicationCapabilityContext {
            context: "Context".to_string(),
            context_type: std::any::type_name::<Context>().to_string(),
        },
        ApplicationSchemaMember::ApplicationCapabilityContextEntitySlot {
            context: "Context".to_string(),
            context_type: std::any::type_name::<Context>().to_string(),
            slot: "ResourceSlot".to_string(),
            slot_type: std::any::type_name::<ResourceSlot>().to_string(),
            entity: "Resource".to_string(),
        },
        ApplicationSchemaMember::ApplicationCapabilityProvenance {
            provenance: "Provenance".to_string(),
            provenance_type: std::any::type_name::<Provenance>().to_string(),
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
        "Status",
        "ValidFrom",
        "ValidThrough",
        "DelegationLimit",
    ] {
        members.push(field_member(field));
    }
    members.push(resource_field_member("ResourceWorkflow"));
    members.push(ApplicationSchemaMember::ApplicationCapability { contract });
    members
}

fn contract(
    wrong_resource_topology: bool,
    changed_purpose: bool,
    disclosure: bool,
) -> ErasedContract {
    contract_with_name_and_composition(
        "Capability",
        wrong_resource_topology,
        changed_purpose,
        composition(disclosure),
    )
}

fn contract_with_composition(
    wrong_resource_topology: bool,
    changed_purpose: bool,
    composition: ApplicationCapabilityComposition,
) -> ErasedContract {
    contract_with_name_and_composition(
        "Capability",
        wrong_resource_topology,
        changed_purpose,
        composition,
    )
}

fn contract_with_name_and_composition(
    capability_name: &'static str,
    wrong_resource_topology: bool,
    changed_purpose: bool,
    composition: ApplicationCapabilityComposition,
) -> ErasedContract {
    ApplicationCapabilityContractBuilder::new(
        ApplicationCapabilityRef::<Schema, Capability>::from_schema_identifier(capability_name),
        ApplicationOperationRef::<Schema, Operation, ()>::from_schema_identifier("Operation"),
        ApplicationEntityRef::<Schema, Grant>::from_schema_identifier("Grant"),
    )
    .target(target_definition(wrong_resource_topology, changed_purpose))
    .constraints(constraint_definition())
    .delegation(delegation_definition())
    .composition(composition)
    .elevation(ApplicationCapabilityElevationRule::not_applicable())
    .build()
    .erased()
    .clone()
}

fn target_definition(
    wrong_resource_topology: bool,
    changed_purpose: bool,
) -> ApplicationCapabilityTargetDefinition {
    let resource = if wrong_resource_topology {
        relation::<WrongResourceRelation, Principal, Resource>(
            "WrongResourceRelation",
            "Principal",
            "Resource",
        )
    } else {
        relation::<ResourceRelation, Grant, Resource>("ResourceRelation", "Grant", "Resource")
    };
    ApplicationCapabilityTargetDefinition::new(
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
    )
}

fn constraint_definition() -> ApplicationCapabilityConstraintDefinition {
    ApplicationCapabilityConstraintDefinition::new(
        ApplicationCapabilityFieldDimension::bound(field::<Amount>("Amount")),
        ApplicationCapabilityCardinalityDimension::One,
        ApplicationCapabilityCurrentnessDefinition::new(
            ApplicationCapabilityValueBinding::new(field::<Status>("Status"), 1_u64),
            ApplicationCapabilityWorkflowDefinition::new(
                binding::<Workflow>("Workflow"),
                resource_binding::<ResourceWorkflow>("ResourceWorkflow"),
            ),
            ApplicationCapabilityValidityDefinition::new(
                ApplicationCapabilityValidityTimeline::UnixEpochSeconds,
                binding::<ValidFrom>("ValidFrom"),
                binding::<ValidThrough>("ValidThrough"),
            ),
        ),
        ApplicationCapabilityContextRef::<Schema, Context>::from_schema_identifier("Context"),
    )
}

fn delegation_definition() -> ApplicationCapabilityDelegationDefinition {
    ApplicationCapabilityDelegationDefinition::new(
        relation::<Parent, Grant, Grant>("Parent", "Grant", "Grant"),
        relation::<Grantor, Principal, Grant>("Grantor", "Principal", "Grant"),
        relation::<Grantee, Principal, Grant>("Grantee", "Principal", "Grant"),
        binding::<DelegationLimit>("DelegationLimit"),
        ApplicationCapabilityProvenanceRef::<Schema, Provenance>::from_schema_identifier(
            "Provenance",
        ),
    )
}

fn resource_slot<ContextMarker, SlotMarker>(
    context: &'static str,
    slot: &'static str,
) -> ApplicationCapabilityContextEntitySlotRef<Schema, ContextMarker, SlotMarker, Resource> {
    ApplicationCapabilityContextEntitySlotRef::from_schema_identifiers(
        ApplicationCapabilityContextRef::from_schema_identifier(context),
        slot,
        ApplicationEntityRef::from_schema_identifier("Resource"),
    )
}

fn anchored_composition(
    anchor: ApplicationCapabilityPathContextAnchor,
) -> ApplicationCapabilityComposition {
    let allow = ApplicationCapabilityGraphRule::any([ApplicationCapabilityGraphClause::new(
        principal_resource_path(),
    )
    .anchored([anchor])]);
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
            ApplicationCapabilityDisclosureRule::permit([
                ApplicationCapabilityScopeGuard::requiring([
                    ApplicationCapabilityAcceptedValues::one_of(field::<Field>("Field"), [1_u64]),
                ]),
            ]),
        ),
    )
}

fn composition(disclosure: bool) -> ApplicationCapabilityComposition {
    let allow = ApplicationCapabilityGraphRule::any([ApplicationCapabilityGraphClause::new(
        principal_resource_path(),
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

fn principal_resource_path() -> crate::application_schema::ApplicationAuthorizationPath {
    ApplicationAuthorizationPathBuilder::from_principal(
        ApplicationEntityRef::<Schema, Principal>::from_schema_identifier("Principal"),
    )
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
    .allow(ApplicationEntityRef::<Schema, Resource>::from_schema_identifier("Resource"))
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

fn resource_binding<FieldMarker>(name: &'static str) -> ApplicationCapabilityFieldBinding {
    ApplicationCapabilityFieldBinding::from_reference(ApplicationFieldRef::<
        Schema,
        Resource,
        ResourceFacts,
        FieldMarker,
        u64,
        ReadOnly,
        EqualityPredicate,
        NoApplicationCurrency,
    >::from_schema_identifiers(
        "Resource", "ResourceFacts", name
    ))
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
