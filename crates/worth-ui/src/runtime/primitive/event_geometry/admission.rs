use crate::capability::{DensityTokenId, SurfaceId, WorthUiDensityValue};
use crate::runtime::WorthUiRuntimeHost;

use super::super::WorthUiBoxEdges;
use super::authored_props::{event_geometry_authored_props, AuthoredEventGeometryProp};
use super::denial_receipt::WorthUiEventGeometryValueDenialReceipt;
use super::digest::{
    event_geometry_admission_digest, event_geometry_denial_set_digest,
    event_geometry_receipt_digest, event_geometry_schema_digest,
};
use super::report::{
    WorthUiEventGeometryAdmissionCounters, WorthUiEventGeometryAdmissionReceipt,
    WorthUiEventGeometryAdmissionReport, WorthUiEventGeometryValueDenialSet,
    WorthUiValidatedEventGeometryPropSet,
};
use super::schema::{
    event_geometry_prop_schemas, WorthUiEventGeometryPropSchema, EVENT_CAPTURE_PROP,
    EVENT_CONTAINMENT_PROP, EVENT_CURSOR_PROP, EVENT_HIT_AREA_PROP, EVENT_HIT_SLOP_PROP,
};
use super::value::{
    default_event_geometry_value, validate_event_geometry_value, WorthUiValidatedEventGeometryValue,
};
use super::WorthUiPrimitiveEventGeometryReceipt;

impl WorthUiRuntimeHost {
    pub fn resolve_event_geometry_admission_report(
        &self,
        surface_id: &SurfaceId,
    ) -> WorthUiEventGeometryAdmissionReport {
        self.admit_event_geometry_props(surface_id)
    }

    pub(crate) fn admit_event_geometry_props(
        &self,
        surface_id: &SurfaceId,
    ) -> WorthUiEventGeometryAdmissionReport {
        let authored_props = event_geometry_authored_props(self, surface_id);
        let mut defaults_applied = 0;
        let mut denials = Vec::new();
        let schemas = event_geometry_prop_schemas();
        let authored_digest = self
            .active_authoring_snapshot()
            .and_then(|snapshot| {
                snapshot
                    .authored_surface_props()
                    .surface_digest(surface_id.as_str())
            })
            .unwrap_or(0xcbf2_9ce4_8422_2325);
        let cursor = admit_schema_prop(
            surface_id.as_str(),
            schema_for(EVENT_CURSOR_PROP),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        let hit_area = admit_schema_prop(
            surface_id.as_str(),
            schema_for(EVENT_HIT_AREA_PROP),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        let hit_slop = admit_schema_prop(
            surface_id.as_str(),
            schema_for(EVENT_HIT_SLOP_PROP),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        let containment = admit_schema_prop(
            surface_id.as_str(),
            schema_for(EVENT_CONTAINMENT_PROP),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        let capture = admit_schema_prop(
            surface_id.as_str(),
            schema_for(EVENT_CAPTURE_PROP),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        push_unknown_event_geometry_prop_denials(
            surface_id.as_str(),
            &authored_props,
            &mut denials,
        );
        let hit_slop_token = hit_slop.into_measurement_token();
        let hit_slop_edges = resolve_event_measurement_edges(
            self,
            surface_id.as_str(),
            schema_for(EVENT_HIT_SLOP_PROP),
            &hit_slop_token,
            authored_span_for(EVENT_HIT_SLOP_PROP, &authored_props),
            &mut denials,
        );
        let counters = WorthUiEventGeometryAdmissionCounters::new(
            schemas.len(),
            authored_event_prop_count(&authored_props),
            defaults_applied,
            schemas.len(),
            denials.len(),
        );
        let schema_digest = event_geometry_schema_digest(schemas);
        if !denials.is_empty() {
            let denial_set_digest = event_geometry_denial_set_digest(surface_id.as_str(), &denials);
            return WorthUiEventGeometryAdmissionReport::rejected(
                surface_id.as_str(),
                WorthUiEventGeometryValueDenialSet::new(
                    surface_id.as_str(),
                    denials,
                    denial_set_digest,
                ),
                counters,
                schema_digest,
            );
        }
        let prop_set = WorthUiValidatedEventGeometryPropSet::new(
            cursor.into_cursor(),
            hit_area.into_hit_area(),
            hit_slop_token,
            hit_slop_edges,
            containment.into_containment(),
            capture.into_capture(),
        );
        let admission_digest =
            event_geometry_admission_digest(surface_id.as_str(), authored_digest, &prop_set);
        WorthUiEventGeometryAdmissionReport::accepted(
            surface_id.as_str(),
            WorthUiEventGeometryAdmissionReceipt::new(
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

impl WorthUiEventGeometryAdmissionReceipt {
    pub(crate) fn resolved_receipt(&self) -> WorthUiPrimitiveEventGeometryReceipt {
        let prop_set = self.prop_set();
        WorthUiPrimitiveEventGeometryReceipt::new(
            prop_set.cursor(),
            prop_set.hit_area(),
            prop_set.hit_slop_token(),
            prop_set.hit_slop_edges(),
            prop_set.containment(),
            prop_set.capture(),
            event_geometry_receipt_digest(self.admission_digest(), prop_set),
        )
    }
}

fn admit_schema_prop(
    surface_id: &str,
    schema: &'static WorthUiEventGeometryPropSchema,
    authored_props: &[AuthoredEventGeometryProp],
    defaults_applied: &mut usize,
    denials: &mut Vec<WorthUiEventGeometryValueDenialReceipt>,
) -> WorthUiValidatedEventGeometryValue {
    let authored_prop = authored_props
        .iter()
        .find(|prop| prop.key == schema.prop_key());
    let raw_value = authored_prop
        .map(|prop| prop.value.clone())
        .unwrap_or_else(|| {
            *defaults_applied += 1;
            schema.default_value().to_owned()
        });
    match validate_event_geometry_value(surface_id, schema, raw_value) {
        Ok(value) => value,
        Err(mut denial) => {
            denial.attach_source_span(authored_prop.and_then(|prop| prop.source_span));
            denials.push(denial);
            default_event_geometry_value(schema)
        }
    }
}

fn push_unknown_event_geometry_prop_denials(
    surface_id: &str,
    authored_props: &[AuthoredEventGeometryProp],
    denials: &mut Vec<WorthUiEventGeometryValueDenialReceipt>,
) {
    for prop in authored_props {
        if prop.key.starts_with("event_")
            && !event_geometry_prop_schemas()
                .iter()
                .any(|schema| schema.prop_key() == prop.key)
        {
            denials.push(WorthUiEventGeometryValueDenialReceipt::unknown_prop(
                surface_id,
                &prop.key,
                prop.value.clone(),
                prop.source_span,
            ));
        }
    }
}

fn resolve_event_measurement_edges(
    runtime: &WorthUiRuntimeHost,
    surface_id: &str,
    schema: &'static WorthUiEventGeometryPropSchema,
    token_text: &str,
    source_span: Option<crate::runtime::WorthUiPrimitiveSourceSpan>,
    denials: &mut Vec<WorthUiEventGeometryValueDenialReceipt>,
) -> WorthUiBoxEdges {
    let Some(edges) = resolve_density_measurement_edges(runtime, token_text) else {
        let mut denial = WorthUiEventGeometryValueDenialReceipt::new(
            surface_id,
            schema,
            token_text.to_owned(),
            None,
        );
        denial.attach_source_span(source_span);
        denials.push(denial);
        return WorthUiBoxEdges::uniform(0.0);
    };
    edges
}

fn resolve_density_measurement_edges(
    runtime: &WorthUiRuntimeHost,
    token_text: &str,
) -> Option<WorthUiBoxEdges> {
    let token = DensityTokenId::new(token_text).ok()?;
    let descriptor = runtime.inspect_active_density_token_descriptor(&token)?;
    match descriptor.value() {
        WorthUiDensityValue::Padding(value) => Some(WorthUiBoxEdges::new(
            value.top().points(),
            value.right().points(),
            value.bottom().points(),
            value.left().points(),
        )),
        WorthUiDensityValue::Spacing(value) => Some(WorthUiBoxEdges::uniform(value.points())),
        WorthUiDensityValue::HitTargetMinimum(value) => {
            Some(WorthUiBoxEdges::uniform(value.points()))
        }
        WorthUiDensityValue::Posture(_) => None,
    }
}

fn authored_span_for(
    prop_key: &str,
    authored_props: &[AuthoredEventGeometryProp],
) -> Option<crate::runtime::WorthUiPrimitiveSourceSpan> {
    authored_props
        .iter()
        .find(|prop| prop.key == prop_key)
        .and_then(|prop| prop.source_span)
}

fn authored_event_prop_count(authored_props: &[AuthoredEventGeometryProp]) -> usize {
    authored_props
        .iter()
        .filter(|prop| prop.key.starts_with("event_"))
        .count()
}

fn schema_for(prop_key: &str) -> &'static WorthUiEventGeometryPropSchema {
    event_geometry_prop_schemas()
        .iter()
        .find(|schema| schema.prop_key() == prop_key)
        .expect("event geometry prop schema exists")
}
