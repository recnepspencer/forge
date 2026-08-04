use super::*;

pub(super) struct Elevation;
pub(super) struct ElevationFacts;
pub(super) struct Review;
pub(super) struct ReviewFacts;
pub(super) struct ElevationIdentity;
pub(super) struct ElevationReason;
pub(super) struct ElevationStatus;
pub(super) struct ElevationNotBefore;
pub(super) struct ElevationNotAfter;
pub(super) struct ReviewIdentity;
pub(super) struct ReviewKind;
pub(super) struct ReviewStatus;
pub(super) struct Requester;
pub(super) struct Approver;
pub(super) struct ElevationGrant;
pub(super) struct ElevationReview;
pub(super) struct ReviewScope;
pub(super) struct Reviewer;
pub(super) struct ElevationSlot;
pub(super) struct ReviewSlot;
pub(super) struct RequestOperation;
pub(super) struct ApproveOperation;
pub(super) struct RevokeOperation;
pub(super) struct CompleteReviewOperation;
pub(super) struct RequestCapability;
pub(super) struct ApproveCapability;
pub(super) struct RevokeCapability;
pub(super) struct CompleteReviewCapability;

pub(super) fn elevation_value(value: u64) -> ApplicationCapabilityValueBinding {
    ApplicationCapabilityValueBinding::new(
        elevation_field::<ElevationStatus>("ElevationStatus"),
        value,
    )
}

pub(super) fn review_value(value: u64) -> ApplicationCapabilityValueBinding {
    ApplicationCapabilityValueBinding::new(review_field::<ReviewStatus>("ReviewStatus"), value)
}

pub(super) fn elevation_binding<Field>(name: &'static str) -> ApplicationCapabilityFieldBinding {
    ApplicationCapabilityFieldBinding::from_reference(elevation_field::<Field>(name))
}

pub(super) fn review_binding<Field>(name: &'static str) -> ApplicationCapabilityFieldBinding {
    ApplicationCapabilityFieldBinding::from_reference(review_field::<Field>(name))
}

pub(super) fn elevation_field<Field>(
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

pub(super) fn review_field<Field>(
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

pub(super) fn context_slot<Slot, Entity>(
    slot: &'static str,
    entity: &'static str,
) -> ApplicationCapabilityContextEntitySlotRef<Schema, Context, Slot, Entity> {
    ApplicationCapabilityContextEntitySlotRef::from_schema_identifiers(
        ApplicationCapabilityContextRef::from_schema_identifier("Context"),
        slot,
        ApplicationEntityRef::from_schema_identifier(entity),
    )
}

pub(super) fn operation<Marker>(name: &'static str) -> ApplicationOperationRef<Schema, Marker, ()> {
    ApplicationOperationRef::from_schema_identifier(name)
}

pub(super) fn transition_binding<CapabilityMarker, OperationMarker>(
    capability: &'static str,
    operation_name: &'static str,
) -> ApplicationCapabilityTransitionBinding {
    ApplicationCapabilityTransitionBinding::from_references(
        ApplicationCapabilityRef::<Schema, CapabilityMarker>::from_schema_identifier(capability),
        operation::<OperationMarker>(operation_name),
    )
}

pub(super) fn transition_contract<CapabilityMarker, OperationMarker>(
    capability: &'static str,
    operation_name: &'static str,
) -> crate::application_capability::ErasedApplicationCapabilityContract {
    ApplicationCapabilityContractBuilder::new(
        ApplicationCapabilityRef::<Schema, CapabilityMarker>::from_schema_identifier(capability),
        operation::<OperationMarker>(operation_name),
        ApplicationEntityRef::<Schema, Grant>::from_schema_identifier("Grant"),
    )
    .target(target_definition(false, false))
    .constraints(constraint_definition())
    .delegation(delegation_definition())
    .composition(composition(true))
    .elevation(ApplicationCapabilityElevationRule::not_applicable())
    .build()
    .erased()
    .clone()
}
