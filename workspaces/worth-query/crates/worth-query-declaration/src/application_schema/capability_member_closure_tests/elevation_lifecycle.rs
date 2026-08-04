use super::*;
use crate::application_capability::{
    ApplicationCapabilityContextEntitySlotBinding, ApplicationCapabilityElevationDefinition,
    ApplicationCapabilityElevationLifecycleDefinition, ApplicationCapabilityElevationStates,
    ApplicationCapabilityMandatoryReviewDefinition, ApplicationCapabilityOperationBinding,
};
use crate::application_schema::ApplicationFieldPresence;

#[path = "elevation_lifecycle/canonical_identity.rs"]
mod canonical_identity;

struct Elevation;
struct ElevationFacts;
struct Review;
struct ReviewFacts;
struct ElevationIdentity;
struct ElevationReason;
struct ElevationStatus;
struct ElevationNotBefore;
struct ElevationNotAfter;
struct ReviewIdentity;
struct ReviewStatus;
struct Requester;
struct Approver;
struct ElevationGrant;
struct ElevationReview;
struct Reviewer;
struct ElevationSlot;
struct ReviewSlot;
struct RequestOperation;
struct ApproveOperation;
struct RevokeOperation;
struct CompleteReviewOperation;

#[test]
fn governed_elevation_requires_one_closed_distinct_lifecycle() {
    let contract = elevation_contract(
        StatePosture::Distinct,
        ReviewPosture::Distinct,
        LifecyclePosture::Distinct,
    );
    assert_eq!(build_from_members(elevation_members(contract)), Ok(()));
}

#[test]
fn duplicate_elevation_state_value_is_not_a_linear_lifecycle() {
    assert_eq!(
        build_from_members(elevation_members(elevation_contract(
            StatePosture::Duplicate,
            ReviewPosture::Distinct,
            LifecyclePosture::Distinct,
        ))),
        Err(ApplicationSchemaDeclarationDenial::InvalidApplicationCapability)
    );
}

#[test]
fn duplicate_review_value_cannot_distinguish_required_from_completed() {
    assert_eq!(
        build_from_members(elevation_members(elevation_contract(
            StatePosture::Distinct,
            ReviewPosture::Duplicate,
            LifecyclePosture::Distinct,
        ))),
        Err(ApplicationSchemaDeclarationDenial::InvalidApplicationCapability)
    );
}

#[test]
fn one_operation_cannot_own_two_lifecycle_roles() {
    assert_eq!(
        build_from_members(elevation_members(elevation_contract(
            StatePosture::Distinct,
            ReviewPosture::Distinct,
            LifecyclePosture::DuplicateOperation,
        ))),
        Err(ApplicationSchemaDeclarationDenial::InvalidApplicationCapability)
    );
}

#[test]
fn lifecycle_operation_must_exist_in_the_same_schema() {
    let contract = elevation_contract(
        StatePosture::Distinct,
        ReviewPosture::Distinct,
        LifecyclePosture::MissingOperation,
    );
    assert_eq!(
        build_from_members(elevation_members(contract)),
        Err(ApplicationSchemaDeclarationDenial::MissingApplicationCapabilityDependency)
    );
}

#[test]
fn lifecycle_context_slots_must_name_the_exact_elevation_and_review_entities() {
    assert_eq!(
        build_from_members(elevation_members(elevation_contract(
            StatePosture::Distinct,
            ReviewPosture::Distinct,
            LifecyclePosture::WrongElevationSlot,
        ))),
        Err(ApplicationSchemaDeclarationDenial::InvalidApplicationCapability)
    );
}

#[derive(Clone, Copy)]
enum StatePosture {
    Distinct,
    Duplicate,
}

#[derive(Clone, Copy)]
enum ReviewPosture {
    Distinct,
    Duplicate,
}

#[derive(Clone, Copy)]
enum LifecyclePosture {
    Distinct,
    DuplicateOperation,
    MissingOperation,
    WrongElevationSlot,
    SwappedOperations,
}

fn elevation_contract(
    states: StatePosture,
    review: ReviewPosture,
    lifecycle: LifecyclePosture,
) -> crate::application_capability::ErasedApplicationCapabilityContract {
    ApplicationCapabilityContractBuilder::new(
        ApplicationCapabilityRef::<Schema, Capability>::from_schema_identifier("Capability"),
        operation::<Operation>("Operation"),
        ApplicationEntityRef::<Schema, Grant>::from_schema_identifier("Grant"),
    )
    .target(target_definition(false, false))
    .constraints(constraint_definition())
    .delegation(delegation_definition())
    .composition(composition(true))
    .elevation(ApplicationCapabilityElevationRule::governed(
        elevation_definition(states, review, lifecycle),
    ))
    .build()
    .erased()
    .clone()
}

fn elevation_definition(
    states: StatePosture,
    review: ReviewPosture,
    lifecycle: LifecyclePosture,
) -> ApplicationCapabilityElevationDefinition {
    let values = match states {
        StatePosture::Distinct => [1, 2, 3, 4, 5, 6, 7],
        StatePosture::Duplicate => [1, 2, 3, 4, 5, 6, 6],
    };
    let completed = match review {
        ReviewPosture::Distinct => 2,
        ReviewPosture::Duplicate => 1,
    };
    ApplicationCapabilityElevationDefinition::new(
        elevation_binding::<ElevationIdentity>("ElevationIdentity"),
        elevation_binding::<ElevationReason>("ElevationReason"),
        elevation_binding::<ElevationStatus>("ElevationStatus"),
        ApplicationCapabilityElevationStates::new(
            elevation_value(values[0]),
            elevation_value(values[1]),
            elevation_value(values[2]),
            elevation_value(values[3]),
            elevation_value(values[4]),
            elevation_value(values[5]),
            elevation_value(values[6]),
        ),
        ApplicationCapabilityValidityDefinition::new(
            ApplicationCapabilityValidityTimeline::UnixEpochSeconds,
            elevation_binding::<ElevationNotBefore>("ElevationNotBefore"),
            elevation_binding::<ElevationNotAfter>("ElevationNotAfter"),
        ),
        relation::<Requester, Principal, Elevation>("Requester", "Principal", "Elevation"),
        relation::<Approver, Principal, Elevation>("Approver", "Principal", "Elevation"),
        relation::<ElevationGrant, Elevation, Grant>("ElevationGrant", "Elevation", "Grant"),
        lifecycle_definition(lifecycle),
        ApplicationCapabilityMandatoryReviewDefinition::new(
            relation::<ElevationReview, Elevation, Review>(
                "ElevationReview",
                "Elevation",
                "Review",
            ),
            review_binding::<ReviewIdentity>("ReviewIdentity"),
            relation::<Reviewer, Principal, Review>("Reviewer", "Principal", "Review"),
            review_binding::<ReviewStatus>("ReviewStatus"),
            review_value(1),
            review_value(completed),
        ),
    )
}

fn lifecycle_definition(
    posture: LifecyclePosture,
) -> ApplicationCapabilityElevationLifecycleDefinition {
    let (request, approve) = match posture {
        LifecyclePosture::DuplicateOperation => ("Request", "Request"),
        LifecyclePosture::MissingOperation => ("Missing", "Approve"),
        LifecyclePosture::SwappedOperations => ("Approve", "Request"),
        LifecyclePosture::Distinct | LifecyclePosture::WrongElevationSlot => ("Request", "Approve"),
    };
    let elevation_slot = match posture {
        LifecyclePosture::WrongElevationSlot => {
            ApplicationCapabilityContextEntitySlotBinding::from_reference(resource_slot::<
                Context,
                ResourceSlot,
            >(
                "Context",
                "ResourceSlot",
            ))
        }
        _ => ApplicationCapabilityContextEntitySlotBinding::from_reference(context_slot::<
            ElevationSlot,
            Elevation,
        >(
            "ElevationSlot",
            "Elevation",
        )),
    };
    ApplicationCapabilityElevationLifecycleDefinition::new(
        elevation_slot,
        ApplicationCapabilityContextEntitySlotBinding::from_reference(context_slot::<
            ReviewSlot,
            Review,
        >(
            "ReviewSlot", "Review"
        )),
        operation_binding::<RequestOperation>(request),
        operation_binding::<ApproveOperation>(approve),
        operation_binding::<RevokeOperation>("Revoke"),
        operation_binding::<CompleteReviewOperation>("CompleteReview"),
    )
}

fn elevation_members(
    contract: crate::application_capability::ErasedApplicationCapabilityContract,
) -> Vec<ApplicationSchemaMember> {
    let mut result = members(contract);
    result.extend([
        entity_member("Elevation"),
        entity_member("Review"),
        aspect_member("Elevation", "ElevationFacts"),
        aspect_member("Review", "ReviewFacts"),
        elevation_field_member("ElevationIdentity"),
        elevation_field_member("ElevationReason"),
        elevation_field_member("ElevationStatus"),
        elevation_field_member("ElevationNotBefore"),
        elevation_field_member("ElevationNotAfter"),
        review_field_member("ReviewIdentity"),
        review_field_member("ReviewStatus"),
        relation_member("Requester", "Principal", "Elevation"),
        relation_member("Approver", "Principal", "Elevation"),
        relation_member("ElevationGrant", "Elevation", "Grant"),
        relation_member("ElevationReview", "Elevation", "Review"),
        relation_member("Reviewer", "Principal", "Review"),
        context_slot_member(
            "ElevationSlot",
            std::any::type_name::<ElevationSlot>(),
            "Elevation",
        ),
        context_slot_member("ReviewSlot", std::any::type_name::<ReviewSlot>(), "Review"),
        operation_member("Request"),
        operation_member("Approve"),
        operation_member("Revoke"),
        operation_member("CompleteReview"),
    ]);
    result
}

fn elevation_value(value: u64) -> ApplicationCapabilityValueBinding {
    ApplicationCapabilityValueBinding::new(
        elevation_field::<ElevationStatus>("ElevationStatus"),
        value,
    )
}

fn review_value(value: u64) -> ApplicationCapabilityValueBinding {
    ApplicationCapabilityValueBinding::new(review_field::<ReviewStatus>("ReviewStatus"), value)
}

fn elevation_binding<Field>(name: &'static str) -> ApplicationCapabilityFieldBinding {
    ApplicationCapabilityFieldBinding::from_reference(elevation_field::<Field>(name))
}

fn review_binding<Field>(name: &'static str) -> ApplicationCapabilityFieldBinding {
    ApplicationCapabilityFieldBinding::from_reference(review_field::<Field>(name))
}

fn elevation_field<Field>(
    name: &'static str,
) -> ApplicationFieldRef<
    Schema,
    Elevation,
    ElevationFacts,
    Field,
    u64,
    ReadOnly,
    EqualityPredicate,
    NoApplicationCurrency,
> {
    ApplicationFieldRef::from_schema_identifiers("Elevation", "ElevationFacts", name)
}

fn review_field<Field>(
    name: &'static str,
) -> ApplicationFieldRef<
    Schema,
    Review,
    ReviewFacts,
    Field,
    u64,
    ReadOnly,
    EqualityPredicate,
    NoApplicationCurrency,
> {
    ApplicationFieldRef::from_schema_identifiers("Review", "ReviewFacts", name)
}

fn context_slot<Slot, Entity>(
    slot: &'static str,
    entity: &'static str,
) -> ApplicationCapabilityContextEntitySlotRef<Schema, Context, Slot, Entity> {
    ApplicationCapabilityContextEntitySlotRef::from_schema_identifiers(
        ApplicationCapabilityContextRef::from_schema_identifier("Context"),
        slot,
        ApplicationEntityRef::from_schema_identifier(entity),
    )
}

fn operation<Marker>(name: &'static str) -> ApplicationOperationRef<Schema, Marker, ()> {
    ApplicationOperationRef::from_schema_identifier(name)
}

fn operation_binding<Marker>(name: &'static str) -> ApplicationCapabilityOperationBinding {
    ApplicationCapabilityOperationBinding::from_reference(operation::<Marker>(name))
}

fn entity_member(entity: &str) -> ApplicationSchemaMember {
    ApplicationSchemaMember::Entity {
        entity: entity.to_string(),
    }
}

fn aspect_member(entity: &str, aspect: &str) -> ApplicationSchemaMember {
    ApplicationSchemaMember::Aspect {
        entity: entity.to_string(),
        aspect: aspect.to_string(),
    }
}

fn elevation_field_member(field: &str) -> ApplicationSchemaMember {
    typed_field_member("Elevation", "ElevationFacts", field)
}

fn review_field_member(field: &str) -> ApplicationSchemaMember {
    typed_field_member("Review", "ReviewFacts", field)
}

fn typed_field_member(entity: &str, aspect: &str, field: &str) -> ApplicationSchemaMember {
    ApplicationSchemaMember::Field {
        entity: entity.to_string(),
        aspect: aspect.to_string(),
        field: field.to_string(),
        presence: ApplicationFieldPresence::Required,
        scalar_family: ScalarAspectType::UInt64,
        value_type: std::any::type_name::<u64>().to_string(),
        currency: None,
        writable: false,
        equality_queryable: true,
    }
}

fn context_slot_member(slot: &str, slot_type: &str, entity: &str) -> ApplicationSchemaMember {
    ApplicationSchemaMember::ApplicationCapabilityContextEntitySlot {
        context: "Context".to_string(),
        context_type: std::any::type_name::<Context>().to_string(),
        slot: slot.to_string(),
        slot_type: slot_type.to_string(),
        entity: entity.to_string(),
    }
}

fn operation_member(operation: &str) -> ApplicationSchemaMember {
    ApplicationSchemaMember::Operation {
        operation: operation.to_string(),
        input_type: std::any::type_name::<()>().to_string(),
    }
}
