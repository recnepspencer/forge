use crate::capability::SurfaceId;
use crate::runtime::WorthUiRuntimeHost;

use super::authored_props::primitive_content_authored_props;
use super::denial_receipt::WorthUiPrimitiveContentValueDenialReceipt;
use super::digest::{
    primitive_content_admission_digest, primitive_content_denial_set_digest,
    primitive_content_schema_digest,
};
use super::prop_admission::{
    admit_optional_schema_prop, admit_required_schema_prop, admit_unsupported_reference_if_present,
    authored_span_for, content_item_count, push_unknown_content_prop_denials,
    resolve_content_measurement, schema_for, sort_content_denials,
};
use super::report::{
    WorthUiPrimitiveContentAdmissionCounters, WorthUiPrimitiveContentAdmissionReceipt,
    WorthUiPrimitiveContentAdmissionReport, WorthUiPrimitiveContentValueDenialSet,
    WorthUiValidatedPrimitiveContentPropSet,
};
use super::schema::{
    primitive_content_prop_schemas, CONTENT_ACCESSIBILITY_NAME_PROP, CONTENT_BADGE_TEXT_PROP,
    CONTENT_DIVIDER_THICKNESS_PROP, CONTENT_ICON_PROP, CONTENT_ICON_SIZE_PROP,
    CONTENT_ICON_STROKE_PROP, CONTENT_IMAGE_PROP, CONTENT_KIND_PROP, CONTENT_ORDER_PROP,
    CONTENT_SLOT_PROP, CONTENT_SPACER_SIZE_PROP, CONTENT_TEXT_PROP, CONTENT_TEXT_SIZE_PROP,
};
use super::value::WorthUiValidatedPrimitiveContentValue;

impl WorthUiRuntimeHost {
    pub fn resolve_primitive_content_admission_report(
        &self,
        surface_id: &SurfaceId,
    ) -> WorthUiPrimitiveContentAdmissionReport {
        self.admit_primitive_content_props(surface_id)
    }

    pub(crate) fn admit_primitive_content_props(
        &self,
        surface_id: &SurfaceId,
    ) -> WorthUiPrimitiveContentAdmissionReport {
        let authored_props = primitive_content_authored_props(self, surface_id);
        let mut defaults_applied = 0;
        let mut values_validated = 0;
        let mut denials = Vec::new();
        let schemas = primitive_content_prop_schemas();
        let authored_digest = self
            .active_authoring_snapshot()
            .and_then(|snapshot| {
                snapshot
                    .authored_surface_props()
                    .surface_digest(surface_id.as_str())
            })
            .unwrap_or(0xcbf2_9ce4_8422_2325);

        let kind = admit_required_schema_prop(
            surface_id.as_str(),
            schema_for(CONTENT_KIND_PROP),
            &authored_props,
            &mut defaults_applied,
            &mut values_validated,
            &mut denials,
        )
        .into_kind();
        let order = admit_required_schema_prop(
            surface_id.as_str(),
            schema_for(CONTENT_ORDER_PROP),
            &authored_props,
            &mut defaults_applied,
            &mut values_validated,
            &mut denials,
        )
        .into_order();
        let text = admit_required_schema_prop(
            surface_id.as_str(),
            schema_for(CONTENT_TEXT_PROP),
            &authored_props,
            &mut defaults_applied,
            &mut values_validated,
            &mut denials,
        )
        .into_text();
        let icon_id = admit_optional_schema_prop(
            surface_id.as_str(),
            schema_for(CONTENT_ICON_PROP),
            &authored_props,
            &mut values_validated,
            &mut denials,
        )
        .map(WorthUiValidatedPrimitiveContentValue::into_icon_id);
        let text_size_token = admit_required_schema_prop(
            surface_id.as_str(),
            schema_for(CONTENT_TEXT_SIZE_PROP),
            &authored_props,
            &mut defaults_applied,
            &mut values_validated,
            &mut denials,
        )
        .into_measurement_token();
        let icon_size_token = admit_required_schema_prop(
            surface_id.as_str(),
            schema_for(CONTENT_ICON_SIZE_PROP),
            &authored_props,
            &mut defaults_applied,
            &mut values_validated,
            &mut denials,
        )
        .into_measurement_token();
        let icon_stroke_token = admit_required_schema_prop(
            surface_id.as_str(),
            schema_for(CONTENT_ICON_STROKE_PROP),
            &authored_props,
            &mut defaults_applied,
            &mut values_validated,
            &mut denials,
        )
        .into_measurement_token();
        let spacer_size_token = admit_required_schema_prop(
            surface_id.as_str(),
            schema_for(CONTENT_SPACER_SIZE_PROP),
            &authored_props,
            &mut defaults_applied,
            &mut values_validated,
            &mut denials,
        )
        .into_measurement_token();
        let badge_text = admit_optional_schema_prop(
            surface_id.as_str(),
            schema_for(CONTENT_BADGE_TEXT_PROP),
            &authored_props,
            &mut values_validated,
            &mut denials,
        )
        .map(WorthUiValidatedPrimitiveContentValue::into_text);
        let divider_thickness_token = admit_required_schema_prop(
            surface_id.as_str(),
            schema_for(CONTENT_DIVIDER_THICKNESS_PROP),
            &authored_props,
            &mut defaults_applied,
            &mut values_validated,
            &mut denials,
        )
        .into_measurement_token();
        let accessibility_name = admit_optional_schema_prop(
            surface_id.as_str(),
            schema_for(CONTENT_ACCESSIBILITY_NAME_PROP),
            &authored_props,
            &mut values_validated,
            &mut denials,
        )
        .map(WorthUiValidatedPrimitiveContentValue::into_text);

        admit_unsupported_reference_if_present(
            surface_id.as_str(),
            schema_for(CONTENT_IMAGE_PROP),
            &authored_props,
            &mut values_validated,
            &mut denials,
        );
        admit_unsupported_reference_if_present(
            surface_id.as_str(),
            schema_for(CONTENT_SLOT_PROP),
            &authored_props,
            &mut values_validated,
            &mut denials,
        );
        push_unknown_content_prop_denials(surface_id.as_str(), &authored_props, &mut denials);

        let text_size_points = resolve_content_measurement(
            self,
            surface_id.as_str(),
            schema_for(CONTENT_TEXT_SIZE_PROP),
            &text_size_token,
            authored_span_for(CONTENT_TEXT_SIZE_PROP, &authored_props),
            &mut denials,
        );
        let icon_size_points = resolve_content_measurement(
            self,
            surface_id.as_str(),
            schema_for(CONTENT_ICON_SIZE_PROP),
            &icon_size_token,
            authored_span_for(CONTENT_ICON_SIZE_PROP, &authored_props),
            &mut denials,
        );
        let icon_stroke_width_points = resolve_content_measurement(
            self,
            surface_id.as_str(),
            schema_for(CONTENT_ICON_STROKE_PROP),
            &icon_stroke_token,
            authored_span_for(CONTENT_ICON_STROKE_PROP, &authored_props),
            &mut denials,
        );
        let spacer_size_points = resolve_content_measurement(
            self,
            surface_id.as_str(),
            schema_for(CONTENT_SPACER_SIZE_PROP),
            &spacer_size_token,
            authored_span_for(CONTENT_SPACER_SIZE_PROP, &authored_props),
            &mut denials,
        );
        let divider_thickness_points = resolve_content_measurement(
            self,
            surface_id.as_str(),
            schema_for(CONTENT_DIVIDER_THICKNESS_PROP),
            &divider_thickness_token,
            authored_span_for(CONTENT_DIVIDER_THICKNESS_PROP, &authored_props),
            &mut denials,
        );

        let icon_id = icon_id.and_then(|id| {
            if self.active_capability_snapshot().icons().get(&id).is_some() {
                Some(id)
            } else {
                let mut denial = WorthUiPrimitiveContentValueDenialReceipt::new(
                    surface_id.as_str(),
                    schema_for(CONTENT_ICON_PROP),
                    id.as_str().to_owned(),
                    authored_span_for(CONTENT_ICON_PROP, &authored_props),
                );
                denial.attach_source_span(authored_span_for(CONTENT_ICON_PROP, &authored_props));
                denials.push(denial);
                None
            }
        });
        let items_emitted =
            content_item_count(&order, icon_id.as_ref(), &text, badge_text.as_deref());
        sort_content_denials(&mut denials);
        let counters = WorthUiPrimitiveContentAdmissionCounters::new(
            schemas.len(),
            authored_props.len(),
            defaults_applied,
            values_validated,
            denials.len(),
            items_emitted,
        );
        let schema_digest = primitive_content_schema_digest(schemas);
        if !denials.is_empty() {
            let denial_set_digest =
                primitive_content_denial_set_digest(surface_id.as_str(), &denials);
            return WorthUiPrimitiveContentAdmissionReport::rejected(
                surface_id.as_str(),
                WorthUiPrimitiveContentValueDenialSet::new(
                    surface_id.as_str(),
                    denials,
                    denial_set_digest,
                ),
                counters,
                schema_digest,
            );
        }

        let prop_set = WorthUiValidatedPrimitiveContentPropSet::new(
            kind,
            order,
            text,
            icon_id,
            text_size_token,
            text_size_points,
            icon_size_token,
            icon_size_points,
            icon_stroke_token,
            icon_stroke_width_points,
            spacer_size_token,
            spacer_size_points,
            badge_text,
            divider_thickness_token,
            divider_thickness_points,
            accessibility_name,
        );
        let admission_digest =
            primitive_content_admission_digest(surface_id.as_str(), authored_digest, &prop_set);
        WorthUiPrimitiveContentAdmissionReport::accepted(
            surface_id.as_str(),
            WorthUiPrimitiveContentAdmissionReceipt::new(
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
