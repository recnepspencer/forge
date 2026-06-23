use crate::capability::CommandId;
use crate::runtime::WorthUiRuntimeHost;

use super::{admit_required, schema_for};
use crate::runtime::interaction_lane::authored_props::AuthoredInteractionProp;
use crate::runtime::interaction_lane::denial_receipt::WorthUiInteractionValueDenialReceipt;
use crate::runtime::interaction_lane::payload::WorthUiInteractionKind;
use crate::runtime::interaction_lane::receipt::WorthUiInteractionTarget;
use crate::runtime::interaction_lane::schema::{
    WorthUiInteractionPropSchema, INTERACTION_COMMAND_PROP, INTERACTION_FOCUS_TARGET_PROP,
    INTERACTION_OPEN_TARGET_PROP, INTERACTION_TARGET_PROP, INTERACTION_TOGGLE_VALUE_PROP,
};
use crate::runtime::interaction_lane::value::{
    validate_interaction_value, WorthUiValidatedInteractionValue,
};

pub(super) fn resolve_target(
    runtime: &WorthUiRuntimeHost,
    surface_id: &str,
    kind: Option<&WorthUiValidatedInteractionValue>,
    authored_props: &[AuthoredInteractionProp],
    values_validated: &mut usize,
    denials: &mut Vec<WorthUiInteractionValueDenialReceipt>,
) -> Option<WorthUiInteractionTarget> {
    let Some(WorthUiValidatedInteractionValue::Kind(kind)) = kind else {
        return None;
    };
    match kind {
        WorthUiInteractionKind::Click | WorthUiInteractionKind::Submit => {
            optional_identifier_target(
                surface_id,
                schema_for(INTERACTION_TARGET_PROP),
                authored_props,
                values_validated,
                denials,
                WorthUiInteractionTarget::Surface(surface_id.to_owned()),
                WorthUiInteractionTarget::Surface,
            )
        }
        WorthUiInteractionKind::Command => required_identifier_target(
            runtime,
            surface_id,
            schema_for(INTERACTION_COMMAND_PROP),
            authored_props,
            values_validated,
            denials,
            WorthUiInteractionTarget::Command,
        ),
        WorthUiInteractionKind::Toggle => required_identifier_target(
            runtime,
            surface_id,
            schema_for(INTERACTION_TOGGLE_VALUE_PROP),
            authored_props,
            values_validated,
            denials,
            WorthUiInteractionTarget::Toggle,
        ),
        WorthUiInteractionKind::Open => required_identifier_target(
            runtime,
            surface_id,
            schema_for(INTERACTION_OPEN_TARGET_PROP),
            authored_props,
            values_validated,
            denials,
            WorthUiInteractionTarget::Open,
        ),
        WorthUiInteractionKind::Focus => required_identifier_target(
            runtime,
            surface_id,
            schema_for(INTERACTION_FOCUS_TARGET_PROP),
            authored_props,
            values_validated,
            denials,
            WorthUiInteractionTarget::Focus,
        ),
    }
}

fn optional_identifier_target(
    surface_id: &str,
    schema: &'static WorthUiInteractionPropSchema,
    authored_props: &[AuthoredInteractionProp],
    values_validated: &mut usize,
    denials: &mut Vec<WorthUiInteractionValueDenialReceipt>,
    default_target: WorthUiInteractionTarget,
    build: fn(String) -> WorthUiInteractionTarget,
) -> Option<WorthUiInteractionTarget> {
    let Some(prop) = authored_props
        .iter()
        .find(|prop| prop.key == schema.prop_key())
    else {
        return Some(default_target);
    };
    *values_validated += 1;
    match validate_interaction_value(surface_id, schema, prop.value.clone(), prop.source_span) {
        Ok(value) => Some(build(value.into_identifier())),
        Err(denial) => {
            denials.push(denial);
            None
        }
    }
}

fn required_identifier_target(
    runtime: &WorthUiRuntimeHost,
    surface_id: &str,
    schema: &'static WorthUiInteractionPropSchema,
    authored_props: &[AuthoredInteractionProp],
    values_validated: &mut usize,
    denials: &mut Vec<WorthUiInteractionValueDenialReceipt>,
    build: fn(String) -> WorthUiInteractionTarget,
) -> Option<WorthUiInteractionTarget> {
    let mut defaults_applied = 0;
    let value = admit_required(
        surface_id,
        schema,
        authored_props,
        &mut defaults_applied,
        values_validated,
        denials,
    )?;
    let identifier = value.into_identifier();
    if !interaction_target_reference_is_known(runtime, schema.prop_key(), &identifier) {
        let authored_value = authored_props
            .iter()
            .find(|prop| prop.key == schema.prop_key())
            .expect("required target admission already found authored prop");
        denials.push(WorthUiInteractionValueDenialReceipt::target_reference(
            surface_id,
            schema,
            authored_value.value.clone(),
            authored_value.source_span,
        ));
        return None;
    }
    Some(build(identifier))
}

fn interaction_target_reference_is_known(
    runtime: &WorthUiRuntimeHost,
    prop_key: &str,
    identifier: &str,
) -> bool {
    if prop_key != INTERACTION_COMMAND_PROP {
        return true;
    }
    CommandId::new(identifier)
        .ok()
        .and_then(|command_id| {
            runtime
                .active_capability_snapshot()
                .commands()
                .get(&command_id)
                .map(|_| ())
        })
        .is_some()
}
