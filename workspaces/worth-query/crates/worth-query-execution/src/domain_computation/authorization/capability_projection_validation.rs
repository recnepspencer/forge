//! Non-observing validation of typed capability-request shape.

use std::collections::BTreeSet;

use worth_foundational::facade::AspectValue;
use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityCardinalityDimension, ApplicationCapabilityRelationDimension,
    ApplicationCapabilityRequestProjection,
};

use super::capability_registry::{
    WorthQueryCapabilityContextAnchor, WorthQueryInstalledCapabilityPlan,
};
use super::capability_request_resolution::WorthQueryCapabilityContextKey;
use super::retained_capability_request::WorthQueryRetainedCapabilityRequest;
use super::{WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind};

pub(super) fn validate_projected_capability_shape<Schema, Scope, Context>(
    installed: &WorthQueryInstalledCapabilityPlan,
    projection: &ApplicationCapabilityRequestProjection<Schema, Scope, Context>,
) -> Result<(), WorthQueryOperationAuthorizationDenial> {
    let context = projection
        .context_value()
        .entities()
        .iter()
        .map(|selected| {
            let slot = selected.slot();
            (selected.selector().entity() == slot.entity())
                .then(|| WorthQueryCapabilityContextKey {
                    context: slot.context().to_string(),
                    context_type: slot.context_type().to_string(),
                    slot: slot.slot().to_string(),
                    slot_type: slot.slot_type().to_string(),
                    entity: slot.entity().to_string(),
                })
                .ok_or_else(|| projection_denial(installed.contract.name()))
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    validate_shape(
        installed,
        projection.action(),
        projection.purpose(),
        projection.resource().entity(),
        projection
            .related()
            .map(|related| related.relation().relation()),
        projection.field_value().is_some(),
        projection.amount_value().is_some(),
        projection.cardinality_value(),
        projection.context_value().context(),
        projection.context_value().context_type(),
        &context,
    )
}

pub(super) fn validate_retained_capability_shape(
    installed: &WorthQueryInstalledCapabilityPlan,
    projection: &WorthQueryRetainedCapabilityRequest,
) -> Result<(), WorthQueryOperationAuthorizationDenial> {
    let context = projection.context.keys().cloned().collect::<BTreeSet<_>>();
    validate_shape(
        installed,
        &projection.action,
        &projection.purpose,
        projection.resource_entity.as_ref(),
        projection
            .related_relation
            .as_ref()
            .map(|relation| relation.relation()),
        projection.field.is_some(),
        projection.amount.is_some(),
        projection.cardinality,
        projection.context_name.as_ref(),
        projection.context_type.as_ref(),
        &context,
    )
}

#[allow(clippy::too_many_arguments)]
fn validate_shape(
    installed: &WorthQueryInstalledCapabilityPlan,
    action: &AspectValue,
    purpose: &AspectValue,
    resource_entity: &str,
    related_relation: Option<&str>,
    has_field: bool,
    has_amount: bool,
    cardinality: u32,
    context_name: &str,
    context_type: &str,
    context: &BTreeSet<WorthQueryCapabilityContextKey>,
) -> Result<(), WorthQueryOperationAuthorizationDenial> {
    let request = &installed.request;
    if action != &request.action
        || purpose != &request.purpose
        || resource_entity != request.resource_entity
        || context_name != request.context
        || context_type != request.context_type
        || !cardinality_admitted(request.cardinality, cardinality)
        || has_field != request.field.is_some()
        || has_amount != request.amount.is_some()
        || !relation_admitted(installed, related_relation)
    {
        return Err(projection_denial(installed.contract.name()));
    }
    let expected_context = installed
        .paths
        .iter()
        .flat_map(|path| path.context_anchors.iter().map(context_key))
        .collect::<BTreeSet<_>>();
    if expected_context != *context {
        return Err(projection_denial(installed.contract.name()));
    }
    Ok(())
}

fn relation_admitted(
    installed: &WorthQueryInstalledCapabilityPlan,
    requested: Option<&str>,
) -> bool {
    match (installed.contract.target().relation(), requested) {
        (ApplicationCapabilityRelationDimension::NotApplicable, None) => true,
        (ApplicationCapabilityRelationDimension::Bound(expected), Some(actual)) => {
            expected.relation() == actual
        }
        _ => false,
    }
}

const fn cardinality_admitted(
    installed: ApplicationCapabilityCardinalityDimension,
    requested: u32,
) -> bool {
    match installed {
        ApplicationCapabilityCardinalityDimension::One => requested == 1,
        ApplicationCapabilityCardinalityDimension::Many => requested > 0,
        ApplicationCapabilityCardinalityDimension::Bounded(maximum) => {
            requested > 0 && requested <= maximum
        }
    }
}

pub(super) fn context_key(
    anchor: &WorthQueryCapabilityContextAnchor,
) -> WorthQueryCapabilityContextKey {
    WorthQueryCapabilityContextKey {
        context: anchor.context.clone(),
        context_type: anchor.context_type.clone(),
        slot: anchor.slot.clone(),
        slot_type: anchor.slot_type.clone(),
        entity: anchor.entity.clone(),
    }
}

pub(super) fn projection_denial(
    subject: impl Into<String>,
) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(
        WorthQueryOperationAuthorizationDenialKind::CapabilityProjectionRejected,
        subject,
    )
}
