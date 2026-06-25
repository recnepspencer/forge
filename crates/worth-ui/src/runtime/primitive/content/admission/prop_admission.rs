use crate::capability::{DensityTokenId, ImageAssetId, WorthUiDensityValue};
use crate::runtime::{WorthUiPrimitiveContentValueDenialReceipt, WorthUiRuntimeHost};

use super::super::participation::WorthUiPrimitiveContentParticipationPosture;
use super::super::receipt::WorthUiPrimitiveContentItemKind;
use super::authored_props::AuthoredPrimitiveContentProp;
use super::schema::{primitive_content_prop_schemas, WorthUiPrimitiveContentPropSchema};
use super::value::{
    default_primitive_content_value, validate_primitive_content_value,
    WorthUiValidatedPrimitiveContentValue,
};

pub(super) fn admit_required_schema_prop(
    surface_id: &str,
    schema: &'static WorthUiPrimitiveContentPropSchema,
    authored_props: &[AuthoredPrimitiveContentProp],
    defaults_applied: &mut usize,
    values_validated: &mut usize,
    denials: &mut Vec<WorthUiPrimitiveContentValueDenialReceipt>,
) -> WorthUiValidatedPrimitiveContentValue {
    let authored_prop = authored_props
        .iter()
        .find(|prop| prop.key == schema.prop_key());
    let raw_value = authored_prop
        .map(|prop| prop.value.clone())
        .or_else(|| schema.default_value().map(str::to_owned))
        .unwrap_or_default();
    if authored_prop.is_none() && schema.default_value().is_some() {
        *defaults_applied += 1;
    }
    *values_validated += 1;
    match validate_primitive_content_value(surface_id, schema, raw_value) {
        Ok(value) => value,
        Err(mut denial) => {
            denial.attach_source_span(authored_prop.and_then(|prop| prop.source_span));
            denials.push(denial);
            default_primitive_content_value(schema)
                .expect("required content schema default must admit")
        }
    }
}

pub(super) fn admit_optional_schema_prop(
    surface_id: &str,
    schema: &'static WorthUiPrimitiveContentPropSchema,
    authored_props: &[AuthoredPrimitiveContentProp],
    values_validated: &mut usize,
    denials: &mut Vec<WorthUiPrimitiveContentValueDenialReceipt>,
) -> Option<WorthUiValidatedPrimitiveContentValue> {
    let authored_prop = authored_props
        .iter()
        .find(|prop| prop.key == schema.prop_key())?;
    *values_validated += 1;
    match validate_primitive_content_value(surface_id, schema, authored_prop.value.clone()) {
        Ok(value) => Some(value),
        Err(mut denial) => {
            denial.attach_source_span(authored_prop.source_span);
            denials.push(denial);
            None
        }
    }
}

pub(super) fn admit_unsupported_reference_if_present(
    surface_id: &str,
    schema: &'static WorthUiPrimitiveContentPropSchema,
    authored_props: &[AuthoredPrimitiveContentProp],
    values_validated: &mut usize,
    denials: &mut Vec<WorthUiPrimitiveContentValueDenialReceipt>,
) {
    let Some(authored_prop) = authored_props
        .iter()
        .find(|prop| prop.key == schema.prop_key())
    else {
        return;
    };
    *values_validated += 1;
    let mut denial = WorthUiPrimitiveContentValueDenialReceipt::new(
        surface_id,
        schema,
        authored_prop.value.clone(),
        authored_prop.source_span,
    );
    denial.attach_source_span(authored_prop.source_span);
    denials.push(denial);
}

pub(super) fn push_unknown_content_prop_denials(
    surface_id: &str,
    authored_props: &[AuthoredPrimitiveContentProp],
    denials: &mut Vec<WorthUiPrimitiveContentValueDenialReceipt>,
) {
    for prop in authored_props {
        if prop.key.starts_with("content_")
            && !primitive_content_prop_schemas()
                .iter()
                .any(|schema| schema.prop_key() == prop.key)
        {
            denials.push(WorthUiPrimitiveContentValueDenialReceipt::unknown_prop(
                surface_id,
                &prop.key,
                prop.value.clone(),
                prop.source_span,
            ));
        }
    }
}

pub(super) fn resolve_content_measurement(
    runtime: &WorthUiRuntimeHost,
    surface_id: &str,
    schema: &'static WorthUiPrimitiveContentPropSchema,
    token_text: &str,
    source_span: Option<crate::runtime::WorthUiPrimitiveSourceSpan>,
    denials: &mut Vec<WorthUiPrimitiveContentValueDenialReceipt>,
) -> f32 {
    let Some(points) = resolve_content_measurement_points(runtime, token_text) else {
        let mut denial = WorthUiPrimitiveContentValueDenialReceipt::new(
            surface_id,
            schema,
            token_text.to_owned(),
            source_span,
        );
        denial.attach_source_span(source_span);
        denials.push(denial);
        return 0.0;
    };
    points
}

pub(super) fn content_item_count(
    order: &[WorthUiPrimitiveContentItemKind],
    icon_id: Option<&crate::capability::IconId>,
    image_asset_id: Option<&ImageAssetId>,
    text: &str,
    badge_text: Option<&str>,
    participation: WorthUiPrimitiveContentParticipationPosture,
) -> usize {
    if !participation.participates_in_layout() {
        return 0;
    }
    order
        .iter()
        .filter(|kind| match kind {
            WorthUiPrimitiveContentItemKind::Text => !text.is_empty(),
            WorthUiPrimitiveContentItemKind::Icon => icon_id.is_some(),
            WorthUiPrimitiveContentItemKind::Image => image_asset_id.is_some(),
            WorthUiPrimitiveContentItemKind::Spacer | WorthUiPrimitiveContentItemKind::Divider => {
                true
            }
            WorthUiPrimitiveContentItemKind::Badge => badge_text.is_some(),
        })
        .count()
}

pub(super) fn authored_span_for(
    prop_key: &str,
    authored_props: &[AuthoredPrimitiveContentProp],
) -> Option<crate::runtime::WorthUiPrimitiveSourceSpan> {
    authored_props
        .iter()
        .find(|prop| prop.key == prop_key)
        .and_then(|prop| prop.source_span)
}

pub(super) fn schema_for(prop_key: &str) -> &'static WorthUiPrimitiveContentPropSchema {
    primitive_content_prop_schemas()
        .iter()
        .find(|schema| schema.prop_key() == prop_key)
        .expect("primitive content prop schema exists")
}

pub(super) fn sort_content_denials(denials: &mut [WorthUiPrimitiveContentValueDenialReceipt]) {
    denials.sort_by(|left, right| {
        let left_schema_order = content_schema_order(left.prop_key());
        let right_schema_order = content_schema_order(right.prop_key());
        left_schema_order
            .cmp(&right_schema_order)
            .then_with(|| content_source_order(left).cmp(&content_source_order(right)))
            .then_with(|| left.prop_key().cmp(right.prop_key()))
    });
}

fn content_schema_order(prop_key: &str) -> usize {
    primitive_content_prop_schemas()
        .iter()
        .position(|schema| schema.prop_key() == prop_key)
        .unwrap_or(usize::MAX)
}

fn content_source_order(denial: &WorthUiPrimitiveContentValueDenialReceipt) -> usize {
    denial
        .source_span()
        .map(|span| span.start_byte())
        .unwrap_or(usize::MAX)
}

fn resolve_content_measurement_points(
    runtime: &WorthUiRuntimeHost,
    token_text: &str,
) -> Option<f32> {
    let token = DensityTokenId::new(token_text).ok()?;
    let descriptor = runtime.inspect_active_density_token_descriptor(&token)?;
    match descriptor.value() {
        WorthUiDensityValue::Padding(value) => Some(value.horizontal_points()),
        WorthUiDensityValue::Spacing(value) => Some(value.points()),
        WorthUiDensityValue::HitTargetMinimum(value) => Some(value.points()),
        WorthUiDensityValue::Posture(_) => None,
    }
}
