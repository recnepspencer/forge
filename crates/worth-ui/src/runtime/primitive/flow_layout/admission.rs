use crate::capability::SurfaceId;
use crate::runtime::WorthUiRuntimeHost;

use super::authored_props::{flow_layout_authored_props, AuthoredFlowLayoutProp};
use super::denial_receipt::WorthUiFlowLayoutValueDenialReceipt;
use super::digest::{
    flow_layout_admission_digest, flow_layout_denial_set_digest, flow_layout_receipt_digest,
    flow_layout_schema_digest,
};
use super::measurement_resolution::{resolve_flow_measurement, resolve_flow_padding};
use super::report::{
    WorthUiFlowLayoutAdmissionCounters, WorthUiFlowLayoutAdmissionReceipt,
    WorthUiFlowLayoutAdmissionReport, WorthUiFlowLayoutValueDenialSet,
    WorthUiValidatedFlowLayoutPropSet,
};
use super::schema::{
    flow_layout_prop_schemas, WorthUiFlowLayoutPropSchema, FLOW_ALIGN_PROP, FLOW_CROSS_ALIGN_PROP,
    FLOW_FILL_PROP, FLOW_FIT_PROP, FLOW_GAP_PROP, FLOW_KIND_PROP, FLOW_PADDING_PROP,
};
use super::value::{
    default_flow_layout_value, validate_flow_layout_value, WorthUiValidatedFlowLayoutValue,
};
use super::WorthUiFlowLayoutReceipt;

impl WorthUiRuntimeHost {
    pub fn resolve_flow_layout_admission_report(
        &self,
        surface_id: &SurfaceId,
    ) -> WorthUiFlowLayoutAdmissionReport {
        self.admit_flow_layout_props(surface_id)
    }

    pub(crate) fn admit_flow_layout_props(
        &self,
        surface_id: &SurfaceId,
    ) -> WorthUiFlowLayoutAdmissionReport {
        let authored_props = flow_layout_authored_props(self, surface_id);
        let authored_digest = self
            .active_authoring_snapshot()
            .and_then(|snapshot| {
                snapshot
                    .authored_surface_props()
                    .surface_digest(surface_id.as_str())
            })
            .unwrap_or(0xcbf2_9ce4_8422_2325);
        self.admit_flow_layout_props_for_subject(
            surface_id.as_str(),
            authored_props,
            authored_digest,
        )
    }

    pub(crate) fn admit_flow_layout_props_for_subject(
        &self,
        subject_id: &str,
        authored_props: Vec<AuthoredFlowLayoutProp>,
        authored_digest: u64,
    ) -> WorthUiFlowLayoutAdmissionReport {
        let mut defaults_applied = 0;
        let mut denials = Vec::new();
        let schemas = flow_layout_prop_schemas();
        let kind = admit_schema_prop(
            subject_id,
            schema_for(FLOW_KIND_PROP),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        let gap = admit_schema_prop(
            subject_id,
            schema_for(FLOW_GAP_PROP),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        let padding = admit_schema_prop(
            subject_id,
            schema_for(FLOW_PADDING_PROP),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        let align = admit_schema_prop(
            subject_id,
            schema_for(FLOW_ALIGN_PROP),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        let cross_align = admit_schema_prop(
            subject_id,
            schema_for(FLOW_CROSS_ALIGN_PROP),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        let fit = admit_schema_prop(
            subject_id,
            schema_for(FLOW_FIT_PROP),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        let fill = admit_schema_prop(
            subject_id,
            schema_for(FLOW_FILL_PROP),
            &authored_props,
            &mut defaults_applied,
            &mut denials,
        );
        push_unknown_flow_layout_prop_denials(subject_id, &authored_props, &mut denials);
        let gap_token = gap.into_measurement_token();
        let padding_token = padding.into_measurement_token();
        let gap_points = resolve_flow_measurement(
            self,
            subject_id,
            schema_for(FLOW_GAP_PROP),
            &gap_token,
            authored_span_for(FLOW_GAP_PROP, &authored_props),
            &mut denials,
        );
        let padding_edges = resolve_flow_padding(
            self,
            subject_id,
            schema_for(FLOW_PADDING_PROP),
            &padding_token,
            authored_span_for(FLOW_PADDING_PROP, &authored_props),
            &mut denials,
        );
        let counters = WorthUiFlowLayoutAdmissionCounters::new(
            schemas.len(),
            authored_props.len(),
            defaults_applied,
            schemas.len(),
            denials.len(),
        );
        let schema_digest = flow_layout_schema_digest(schemas);
        if !denials.is_empty() {
            let denial_set_digest = flow_layout_denial_set_digest(subject_id, &denials);
            return WorthUiFlowLayoutAdmissionReport::rejected(
                subject_id,
                WorthUiFlowLayoutValueDenialSet::new(subject_id, denials, denial_set_digest),
                counters,
                schema_digest,
            );
        }
        let prop_set = WorthUiValidatedFlowLayoutPropSet::new(
            kind.into_kind(),
            gap_token,
            gap_points,
            padding_token,
            padding_edges,
            align.into_align(),
            cross_align.into_cross_align(),
            fit.into_fit(),
            fill.into_fill(),
        );
        let admission_digest = flow_layout_admission_digest(subject_id, authored_digest, &prop_set);
        WorthUiFlowLayoutAdmissionReport::accepted(
            subject_id,
            WorthUiFlowLayoutAdmissionReceipt::new(
                subject_id,
                prop_set,
                authored_digest,
                admission_digest,
            ),
            counters,
            schema_digest,
        )
    }
}

impl WorthUiFlowLayoutAdmissionReceipt {
    pub(crate) fn resolved_receipt(&self) -> WorthUiFlowLayoutReceipt {
        let prop_set = self.prop_set();
        WorthUiFlowLayoutReceipt::new(
            prop_set.kind(),
            prop_set.gap_token(),
            prop_set.gap_points(),
            prop_set.padding_token(),
            prop_set.padding_edges(),
            prop_set.align(),
            prop_set.cross_align(),
            prop_set.fit(),
            prop_set.fill(),
            flow_layout_receipt_digest(self.admission_digest(), prop_set),
        )
    }
}

fn admit_schema_prop(
    surface_id: &str,
    schema: &'static WorthUiFlowLayoutPropSchema,
    authored_props: &[AuthoredFlowLayoutProp],
    defaults_applied: &mut usize,
    denials: &mut Vec<WorthUiFlowLayoutValueDenialReceipt>,
) -> WorthUiValidatedFlowLayoutValue {
    let authored_prop = authored_props
        .iter()
        .find(|prop| prop.key == schema.prop_key());
    let raw_value = authored_prop
        .map(|prop| prop.value.clone())
        .unwrap_or_else(|| {
            *defaults_applied += 1;
            schema.default_value().to_owned()
        });
    match validate_flow_layout_value(surface_id, schema, raw_value) {
        Ok(value) => value,
        Err(mut denial) => {
            denial.attach_source_span(authored_prop.and_then(|prop| prop.source_span));
            denials.push(denial);
            default_flow_layout_value(schema)
        }
    }
}

fn push_unknown_flow_layout_prop_denials(
    surface_id: &str,
    authored_props: &[AuthoredFlowLayoutProp],
    denials: &mut Vec<WorthUiFlowLayoutValueDenialReceipt>,
) {
    for prop in authored_props {
        if prop.key.starts_with("flow_")
            && !flow_layout_prop_schemas()
                .iter()
                .any(|schema| schema.prop_key() == prop.key)
        {
            denials.push(WorthUiFlowLayoutValueDenialReceipt::unknown_prop(
                surface_id,
                &prop.key,
                prop.value.clone(),
                prop.source_span,
            ));
        }
    }
}

fn authored_span_for(
    prop_key: &str,
    authored_props: &[AuthoredFlowLayoutProp],
) -> Option<crate::runtime::WorthUiPrimitiveSourceSpan> {
    authored_props
        .iter()
        .find(|prop| prop.key == prop_key)
        .and_then(|prop| prop.source_span)
}

fn schema_for(prop_key: &str) -> &'static WorthUiFlowLayoutPropSchema {
    flow_layout_prop_schemas()
        .iter()
        .find(|schema| schema.prop_key() == prop_key)
        .expect("flow layout prop schema exists")
}
