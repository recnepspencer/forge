use worth_query_decl::facade::{
    application_capability::{
        ApplicationCapabilityContextEntitySlotRef, ApplicationCapabilityContextRef,
        ApplicationCapabilityPathContextAnchor,
    },
    application_schema::{ApplicationEntityRef, ApplicationRelationRef},
};

struct Schema;
struct Context;
struct Slot;
struct Principal;
struct Resource;
struct WrongResource;
struct PrincipalResource;

fn main() {
    let relation = ApplicationRelationRef::<Schema, PrincipalResource, Principal, Resource>::
        from_schema_identifiers("PrincipalResource", "Principal", "Resource");
    let slot = ApplicationCapabilityContextEntitySlotRef::<
        Schema,
        Context,
        Slot,
        WrongResource,
    >::from_schema_identifiers(
        ApplicationCapabilityContextRef::from_schema_identifier("Context"),
        "Slot",
        ApplicationEntityRef::from_schema_identifier("WrongResource"),
    );

    let _ = ApplicationCapabilityPathContextAnchor::after_forward(relation, slot);
}
