mod authored_props;
mod denial_receipt;
mod digest;
mod presentation;
mod report;
mod value;

use crate::capability::SurfaceId;
use crate::runtime::WorthUiRuntimeHost;

use authored_props::{primitive_authored_props, AuthoredPrimitiveProp};
pub use denial_receipt::WorthUiPrimitiveValueDenialReceipt;

pub use presentation::{
    WorthUiPrimitiveDenialPresentation, WorthUiPrimitiveDenialPresentationRow,
    WorthUiPrimitiveSourceSpan,
};
pub use report::{
    WorthUiPrimitivePropAdmissionCounters, WorthUiPrimitivePropAdmissionReceipt,
    WorthUiPrimitivePropAdmissionReport, WorthUiPrimitivePropAdmissionStatus,
    WorthUiPrimitiveValueDenialSet, WorthUiValidatedPrimitivePropSet,
};

use super::{
    primitive_authored_prop_schemas, WorthUiPrimitiveAuthoredPropSchema, PRIMITIVE_ALIGN_PROP,
    PRIMITIVE_BACKGROUND_PROP, PRIMITIVE_CURSOR_PROP, PRIMITIVE_DISABLED_PROP,
    PRIMITIVE_FOCUS_PROP, PRIMITIVE_FOREGROUND_PROP, PRIMITIVE_INTERACTION_ID_PROP,
    PRIMITIVE_INTERACTION_PROP, PRIMITIVE_MOTION_DURATION_PROP, PRIMITIVE_MOTION_EASING_PROP,
    PRIMITIVE_MOTION_PROP, PRIMITIVE_MOTION_TARGET_PROP, PRIMITIVE_PADDING_PROP,
    PRIMITIVE_RADIUS_PROP, PRIMITIVE_SELECTED_PROP, PRIMITIVE_SUBMIT_PAYLOAD_PROP,
    PRIMITIVE_TEXT_PROP,
};
use digest::{primitive_admission_digest, primitive_denial_set_digest, primitive_schema_digest};
use value::{default_primitive_value, validate_primitive_value, WorthUiValidatedPrimitiveValue};

impl WorthUiRuntimeHost {
    pub fn resolve_primitive_prop_admission_report(
        &self,
        surface_id: &SurfaceId,
    ) -> WorthUiPrimitivePropAdmissionReport {
        self.admit_primitive_props(surface_id)
    }

    pub(crate) fn admit_primitive_props(
        &self,
        surface_id: &SurfaceId,
    ) -> WorthUiPrimitivePropAdmissionReport {
        let authored_props = primitive_authored_props(self, surface_id);
        let mut defaults_applied = 0;
        let mut denials = Vec::new();
        let schemas = primitive_authored_prop_schemas();
        let authored_digest = self
            .active_authoring_snapshot()
            .and_then(|snapshot| {
                snapshot
                    .authored_surface_props()
                    .surface_digest(surface_id.as_str())
            })
            .unwrap_or(0xcbf2_9ce4_8422_2325);
        let text = admit_schema_prop(
            surface_id.as_str(),
            text_schema(),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        let align = admit_schema_prop(
            surface_id.as_str(),
            align_schema(),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        let padding = admit_schema_prop(
            surface_id.as_str(),
            padding_schema(),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        let radius = admit_schema_prop(
            surface_id.as_str(),
            radius_schema(),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        let background = admit_schema_prop(
            surface_id.as_str(),
            background_schema(),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        let foreground = admit_schema_prop(
            surface_id.as_str(),
            foreground_schema(),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        let interaction = admit_schema_prop(
            surface_id.as_str(),
            interaction_schema(),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        let cursor = admit_schema_prop(
            surface_id.as_str(),
            cursor_schema(),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        let focus = admit_schema_prop(
            surface_id.as_str(),
            focus_schema(),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        let disabled = admit_schema_prop(
            surface_id.as_str(),
            disabled_schema(),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        let selected = admit_schema_prop(
            surface_id.as_str(),
            selected_schema(),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        let interaction_id = admit_schema_prop(
            surface_id.as_str(),
            interaction_id_schema(),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        let submit_payload = admit_schema_prop(
            surface_id.as_str(),
            submit_payload_schema(),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        let motion = admit_schema_prop(
            surface_id.as_str(),
            motion_schema(),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        let motion_target = admit_schema_prop(
            surface_id.as_str(),
            motion_target_schema(),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        let motion_duration = admit_schema_prop(
            surface_id.as_str(),
            motion_duration_schema(),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        let motion_easing = admit_schema_prop(
            surface_id.as_str(),
            motion_easing_schema(),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        push_unknown_primitive_prop_denials(surface_id.as_str(), &authored_props, &mut denials);
        let counters = WorthUiPrimitivePropAdmissionCounters::new(
            schemas.len(),
            authored_props.len(),
            defaults_applied,
            schemas.len(),
            denials.len(),
        );
        let schema_digest = primitive_schema_digest(schemas);
        if !denials.is_empty() {
            let denial_set_digest = primitive_denial_set_digest(surface_id.as_str(), &denials);
            return WorthUiPrimitivePropAdmissionReport::rejected(
                surface_id.as_str(),
                WorthUiPrimitiveValueDenialSet::new(
                    surface_id.as_str(),
                    denials,
                    denial_set_digest,
                ),
                counters,
                schema_digest,
            );
        }
        let prop_set = WorthUiValidatedPrimitivePropSet::new(
            text.into_text(),
            align.into_align(),
            padding.into_measurement_token(),
            radius.into_measurement_token(),
            background.into_color(),
            foreground.into_color(),
            interaction.into_interaction_kind(),
            cursor.into_cursor(),
            focus.into_focus(),
            disabled.into_boolean(),
            selected.into_boolean(),
            interaction_id.into_text(),
            submit_payload.into_text(),
            motion.into_motion_kind(),
            motion_target.into_motion_target(),
            motion_duration.into_measurement_token(),
            motion_easing.into_easing(),
        );
        let admission_digest =
            primitive_admission_digest(surface_id.as_str(), authored_digest, &prop_set);
        WorthUiPrimitivePropAdmissionReport::accepted(
            surface_id.as_str(),
            WorthUiPrimitivePropAdmissionReceipt::new(
                surface_id.as_str(),
                prop_set,
                authored_digest,
                admission_digest,
            ),
            counters,
            schema_digest,
        )
    }
}

fn admit_schema_prop(
    surface_id: &str,
    schema: &'static WorthUiPrimitiveAuthoredPropSchema,
    authored_props: &[AuthoredPrimitiveProp],
    defaults_applied: &mut usize,
    denials: &mut Vec<WorthUiPrimitiveValueDenialReceipt>,
) -> WorthUiValidatedPrimitiveValue {
    let authored_prop = authored_props
        .iter()
        .find(|prop| prop.key == schema.prop_key());
    let raw_value = authored_prop
        .map(|prop| prop.value.clone())
        .unwrap_or_else(|| {
            *defaults_applied += 1;
            schema.default_value().to_owned()
        });
    match validate_primitive_value(surface_id, schema, raw_value) {
        Ok(value) => value,
        Err(mut denial) => {
            denial.attach_source_span(authored_prop.and_then(|prop| prop.source_span));
            denials.push(denial);
            default_primitive_value(schema)
        }
    }
}

fn push_unknown_primitive_prop_denials(
    surface_id: &str,
    authored_props: &[AuthoredPrimitiveProp],
    denials: &mut Vec<WorthUiPrimitiveValueDenialReceipt>,
) {
    for prop in authored_props {
        if prop.key.starts_with("primitive_")
            && !primitive_authored_prop_schemas()
                .iter()
                .any(|schema| schema.prop_key() == prop.key)
        {
            denials.push(WorthUiPrimitiveValueDenialReceipt::unknown_prop(
                surface_id,
                &prop.key,
                prop.value.clone(),
                prop.source_span,
            ));
        }
    }
}

fn text_schema() -> &'static WorthUiPrimitiveAuthoredPropSchema {
    schema_for(PRIMITIVE_TEXT_PROP)
}

fn align_schema() -> &'static WorthUiPrimitiveAuthoredPropSchema {
    schema_for(PRIMITIVE_ALIGN_PROP)
}

fn padding_schema() -> &'static WorthUiPrimitiveAuthoredPropSchema {
    schema_for(PRIMITIVE_PADDING_PROP)
}

fn radius_schema() -> &'static WorthUiPrimitiveAuthoredPropSchema {
    schema_for(PRIMITIVE_RADIUS_PROP)
}

fn background_schema() -> &'static WorthUiPrimitiveAuthoredPropSchema {
    schema_for(PRIMITIVE_BACKGROUND_PROP)
}

fn foreground_schema() -> &'static WorthUiPrimitiveAuthoredPropSchema {
    schema_for(PRIMITIVE_FOREGROUND_PROP)
}

fn interaction_schema() -> &'static WorthUiPrimitiveAuthoredPropSchema {
    schema_for(PRIMITIVE_INTERACTION_PROP)
}

fn cursor_schema() -> &'static WorthUiPrimitiveAuthoredPropSchema {
    schema_for(PRIMITIVE_CURSOR_PROP)
}

fn focus_schema() -> &'static WorthUiPrimitiveAuthoredPropSchema {
    schema_for(PRIMITIVE_FOCUS_PROP)
}

fn disabled_schema() -> &'static WorthUiPrimitiveAuthoredPropSchema {
    schema_for(PRIMITIVE_DISABLED_PROP)
}

fn selected_schema() -> &'static WorthUiPrimitiveAuthoredPropSchema {
    schema_for(PRIMITIVE_SELECTED_PROP)
}

fn interaction_id_schema() -> &'static WorthUiPrimitiveAuthoredPropSchema {
    schema_for(PRIMITIVE_INTERACTION_ID_PROP)
}

fn submit_payload_schema() -> &'static WorthUiPrimitiveAuthoredPropSchema {
    schema_for(PRIMITIVE_SUBMIT_PAYLOAD_PROP)
}

fn motion_schema() -> &'static WorthUiPrimitiveAuthoredPropSchema {
    schema_for(PRIMITIVE_MOTION_PROP)
}

fn motion_target_schema() -> &'static WorthUiPrimitiveAuthoredPropSchema {
    schema_for(PRIMITIVE_MOTION_TARGET_PROP)
}

fn motion_duration_schema() -> &'static WorthUiPrimitiveAuthoredPropSchema {
    schema_for(PRIMITIVE_MOTION_DURATION_PROP)
}

fn motion_easing_schema() -> &'static WorthUiPrimitiveAuthoredPropSchema {
    schema_for(PRIMITIVE_MOTION_EASING_PROP)
}

fn schema_for(prop_key: &str) -> &'static WorthUiPrimitiveAuthoredPropSchema {
    primitive_authored_prop_schemas()
        .iter()
        .find(|schema| schema.prop_key() == prop_key)
        .expect("primitive prop schema exists")
}
