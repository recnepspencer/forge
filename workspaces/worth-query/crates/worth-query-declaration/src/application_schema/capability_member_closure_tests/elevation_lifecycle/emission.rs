use super::*;

use crate::application_capability::{
    ApplicationCapabilityLifecycleEffect, ErasedApplicationCapabilityContract,
};
use crate::application_schema::{
    ApplicationEffectRef, ApplicationOperationProgramTarget, OperationEmits,
};

pub struct ActivityEffect;
struct RejectingInput;

#[derive(Clone, Copy)]
pub(super) enum EffectPosture {
    NotApplicable,
    Derived,
}

#[derive(Clone, Copy)]
pub(super) enum ResourceRelationPosture {
    NotApplicable,
    Governed,
}

impl OperationEmits<RequestOperation> for ActivityEffect {}

impl ApplicationCapabilityLifecycleEffect<Schema, RequestOperation> for () {
    type Effect = ActivityEffect;
    type Payload = String;

    fn effect() -> ApplicationEffectRef<Schema, Self::Effect, Self::Payload> {
        ApplicationEffectRef::from_schema_identifier("ActivityEffect")
    }

    fn lifecycle_effect(&self) -> Option<Self::Payload> {
        Some("estate:access".to_owned())
    }
}

impl ApplicationCapabilityLifecycleEffect<Schema, RequestOperation> for RejectingInput {
    type Effect = ActivityEffect;
    type Payload = String;

    fn effect() -> ApplicationEffectRef<Schema, Self::Effect, Self::Payload> {
        ApplicationEffectRef::from_schema_identifier("ActivityEffect")
    }

    fn lifecycle_effect(&self) -> Option<Self::Payload> {
        None
    }
}

#[test]
fn lifecycle_effect_target_is_framework_owned() {
    let mut members = effect_members();
    members.push(ApplicationSchemaMember::OperationProgram {
        operation: "Request".to_owned(),
        target: ApplicationOperationProgramTarget::Emit {
            effect: "ActivityEffect".to_owned(),
        },
    });
    assert_eq!(
        build_from_members(members),
        Err(ApplicationSchemaDeclarationDenial::InvalidApplicationCapability)
    );
}

#[test]
fn lifecycle_resource_link_target_is_framework_owned() {
    let mut members = effect_members();
    members.push(ApplicationSchemaMember::OperationProgram {
        operation: "Request".to_owned(),
        target: ApplicationOperationProgramTarget::Link {
            relation: "ElevationResource".to_owned(),
            from: "Elevation".to_owned(),
            to: "Resource".to_owned(),
        },
    });
    assert_eq!(
        build_from_members(members),
        Err(ApplicationSchemaDeclarationDenial::InvalidApplicationCapability)
    );
}

#[test]
fn lifecycle_effect_requires_the_exact_declared_payload_type() {
    let mut members = effect_members();
    let effect = members.iter_mut().find(|member| {
        matches!(member, ApplicationSchemaMember::Effect { effect, .. } if effect == "ActivityEffect")
    });
    let Some(ApplicationSchemaMember::Effect { payload_type, .. }) = effect else {
        panic!("fixture must declare the lifecycle effect")
    };
    *payload_type = std::any::type_name::<u64>().to_owned();
    assert_eq!(
        build_from_members(members),
        Err(ApplicationSchemaDeclarationDenial::InvalidApplicationCapability)
    );
}

#[test]
fn bound_lifecycle_effect_derives_only_from_the_typed_input() {
    let transition = request_transition();
    let binding = transition.lifecycle_effect().expect("effect is bound");
    let derived = binding
        .derive_from_retained_input(&() as &dyn std::any::Any)
        .expect("typed input derives one payload");
    assert_eq!(derived.effect(), "ActivityEffect");
    assert_eq!(derived.payload_type(), std::any::type_name::<String>());
    assert!(binding
        .derive_from_retained_input(&1_u64 as &dyn std::any::Any)
        .is_none());
}

#[test]
fn typed_lifecycle_effect_rejection_derives_no_payload() {
    let transition = ApplicationCapabilityTransitionBinding::from_references_with_lifecycle_effect(
        ApplicationCapabilityRef::<Schema, RequestCapability>::from_schema_identifier(
            "RequestCapability",
        ),
        ApplicationOperationRef::<Schema, RequestOperation, RejectingInput>::from_schema_identifier(
            "Request",
        ),
    );
    let binding = transition.lifecycle_effect().expect("effect is bound");

    assert!(binding
        .derive_from_retained_input(&RejectingInput as &dyn std::any::Any)
        .is_none());
}

fn effect_members() -> Vec<ApplicationSchemaMember> {
    let contract = lifecycle_contract(EffectPosture::Derived, ResourceRelationPosture::Governed);
    let mut members = elevation_members(contract);
    members.push(relation_member(
        "ElevationResource",
        "Elevation",
        "Resource",
    ));
    members.push(ApplicationSchemaMember::Effect {
        effect: "ActivityEffect".to_owned(),
        payload_type: std::any::type_name::<String>().to_owned(),
    });
    members
}

pub(super) fn lifecycle_contract(
    effect: EffectPosture,
    resource_relation: ResourceRelationPosture,
) -> ErasedApplicationCapabilityContract {
    let lifecycle = ApplicationCapabilityElevationLifecycleDefinition::new(
        ApplicationCapabilityContextEntitySlotBinding::from_reference(context_slot::<
            ElevationSlot,
            Elevation,
        >(
            "ElevationSlot",
            "Elevation",
        )),
        ApplicationCapabilityContextEntitySlotBinding::from_reference(context_slot::<
            ReviewSlot,
            Review,
        >(
            "ReviewSlot", "Review"
        )),
        match effect {
            EffectPosture::NotApplicable => {
                transition_binding::<RequestCapability, RequestOperation>(
                    "RequestCapability",
                    "Request",
                )
            }
            EffectPosture::Derived => request_transition(),
        },
        transition_binding::<ApproveCapability, ApproveOperation>("ApproveCapability", "Approve"),
        transition_binding::<RevokeCapability, RevokeOperation>("RevokeCapability", "Revoke"),
        transition_binding::<CompleteReviewCapability, CompleteReviewOperation>(
            "CompleteReviewCapability",
            "CompleteReview",
        ),
    );
    let elevation = elevation_definition_with_lifecycle(
        StatePosture::Distinct,
        ReviewPosture::Distinct,
        std::time::Duration::from_secs(1_200),
        lifecycle,
    );
    let elevation = match resource_relation {
        ResourceRelationPosture::NotApplicable => elevation,
        ResourceRelationPosture::Governed => {
            elevation.with_resource_relation(relation::<ElevationResource, Elevation, Resource>(
                "ElevationResource",
                "Elevation",
                "Resource",
            ))
        }
    };
    ApplicationCapabilityContractBuilder::new(
        ApplicationCapabilityRef::<Schema, Capability>::from_schema_identifier("Capability"),
        operation::<Operation>("Operation"),
        ApplicationEntityRef::<Schema, Grant>::from_schema_identifier("Grant"),
    )
    .target(target_definition(false, false))
    .constraints(constraint_definition())
    .delegation(delegation_definition())
    .composition(composition(true))
    .elevation(ApplicationCapabilityElevationRule::governed(elevation))
    .build()
    .erased()
    .clone()
}

fn request_transition() -> ApplicationCapabilityTransitionBinding {
    ApplicationCapabilityTransitionBinding::from_references_with_lifecycle_effect(
        ApplicationCapabilityRef::<Schema, RequestCapability>::from_schema_identifier(
            "RequestCapability",
        ),
        operation::<RequestOperation>("Request"),
    )
}
