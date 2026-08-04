use super::canonical_components::{
    append_field, append_relation, append_value_binding, text,
    ApplicationCapabilityCanonicalComponent,
};
use super::{
    ApplicationCapabilityContextEntitySlotBinding, ApplicationCapabilityElevationRule,
    ApplicationCapabilityOperationBinding, ErasedApplicationCapabilityContract,
};

pub(super) fn append_elevation(
    components: &mut Vec<ApplicationCapabilityCanonicalComponent>,
    contract: &ErasedApplicationCapabilityContract,
) {
    let ApplicationCapabilityElevationRule::Governed(elevation) = contract.elevation() else {
        text(components, "elevation.posture", "not-applicable");
        return;
    };
    text(components, "elevation.posture", "governed");
    append_field(components, "elevation.identity", elevation.identity());
    append_field(components, "elevation.reason", elevation.reason());
    append_field(components, "elevation.status", elevation.status());
    for (name, state) in [
        ("requested", elevation.states().requested()),
        ("approved", elevation.states().approved()),
        ("active", elevation.states().active()),
        ("expired", elevation.states().expired()),
        ("revoked", elevation.states().revoked()),
        ("review-required", elevation.states().review_required()),
        ("reviewed", elevation.states().reviewed()),
    ] {
        append_value_binding(components, &format!("elevation.state.{name}"), state);
    }
    text(
        components,
        "elevation.validity.timeline",
        elevation.validity().timeline().canonical_name(),
    );
    append_field(
        components,
        "elevation.validity.not-before",
        elevation.validity().not_before(),
    );
    append_field(
        components,
        "elevation.validity.not-after",
        elevation.validity().not_after(),
    );
    append_relation(components, "elevation.requester", elevation.requester());
    append_relation(components, "elevation.approver", elevation.approver());
    append_relation(components, "elevation.grant", elevation.grant());
    append_lifecycle(components, elevation.lifecycle());
    let review = elevation.review();
    append_relation(components, "elevation.review.relation", review.relation());
    append_field(components, "elevation.review.identity", review.identity());
    append_relation(components, "elevation.review.reviewer", review.reviewer());
    append_field(components, "elevation.review.status", review.status());
    append_value_binding(components, "elevation.review.required", review.required());
    append_value_binding(components, "elevation.review.completed", review.completed());
}

fn append_lifecycle(
    components: &mut Vec<ApplicationCapabilityCanonicalComponent>,
    lifecycle: &super::ApplicationCapabilityElevationLifecycleDefinition,
) {
    append_slot(
        components,
        "elevation.lifecycle.elevation-slot",
        lifecycle.elevation_slot(),
    );
    append_slot(
        components,
        "elevation.lifecycle.review-slot",
        lifecycle.review_slot(),
    );
    for (role, operation) in [
        ("request", lifecycle.request()),
        ("approve", lifecycle.approve()),
        ("revoke", lifecycle.revoke()),
        ("complete-review", lifecycle.complete_review()),
    ] {
        append_operation(
            components,
            &format!("elevation.lifecycle.{role}"),
            operation,
        );
    }
}

fn append_slot(
    components: &mut Vec<ApplicationCapabilityCanonicalComponent>,
    prefix: &str,
    slot: &ApplicationCapabilityContextEntitySlotBinding,
) {
    text(components, format!("{prefix}.context"), slot.context());
    text(
        components,
        format!("{prefix}.context-type"),
        slot.context_type(),
    );
    text(components, format!("{prefix}.slot"), slot.slot());
    text(components, format!("{prefix}.slot-type"), slot.slot_type());
    text(components, format!("{prefix}.entity"), slot.entity());
}

fn append_operation(
    components: &mut Vec<ApplicationCapabilityCanonicalComponent>,
    prefix: &str,
    operation: &ApplicationCapabilityOperationBinding,
) {
    text(
        components,
        format!("{prefix}.operation"),
        operation.operation(),
    );
    text(
        components,
        format!("{prefix}.operation-type"),
        operation.operation_type(),
    );
    text(
        components,
        format!("{prefix}.input-type"),
        operation.input_type(),
    );
}
