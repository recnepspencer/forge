use crate::capability::{DensityTokenId, WorthUiDensityValue};
use crate::runtime::{WorthUiBoxEdges, WorthUiPrimitiveSourceSpan, WorthUiRuntimeHost};

use super::denial_receipt::WorthUiFlowLayoutValueDenialReceipt;
use super::schema::WorthUiFlowLayoutPropSchema;

pub(super) fn resolve_flow_measurement(
    runtime: &WorthUiRuntimeHost,
    surface_id: &str,
    schema: &'static WorthUiFlowLayoutPropSchema,
    token_text: &str,
    source_span: Option<WorthUiPrimitiveSourceSpan>,
    denials: &mut Vec<WorthUiFlowLayoutValueDenialReceipt>,
) -> f32 {
    let Some(points) = resolve_flow_measurement_points(runtime, token_text) else {
        let mut denial = WorthUiFlowLayoutValueDenialReceipt::new(
            surface_id,
            schema,
            token_text.to_owned(),
            None,
        );
        denial.attach_source_span(source_span);
        denials.push(denial);
        return 0.0;
    };
    points
}

pub(super) fn resolve_flow_padding(
    runtime: &WorthUiRuntimeHost,
    surface_id: &str,
    schema: &'static WorthUiFlowLayoutPropSchema,
    token_text: &str,
    source_span: Option<WorthUiPrimitiveSourceSpan>,
    denials: &mut Vec<WorthUiFlowLayoutValueDenialReceipt>,
) -> WorthUiBoxEdges {
    let Some(edges) = resolve_flow_measurement_edges(runtime, token_text) else {
        let mut denial = WorthUiFlowLayoutValueDenialReceipt::new(
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

fn resolve_flow_measurement_points(runtime: &WorthUiRuntimeHost, token_text: &str) -> Option<f32> {
    let token = DensityTokenId::new(token_text).ok()?;
    let descriptor = runtime.inspect_active_density_token_descriptor(&token)?;
    match descriptor.value() {
        WorthUiDensityValue::Padding(value) => Some(value.horizontal_points()),
        WorthUiDensityValue::Spacing(value) => Some(value.points()),
        WorthUiDensityValue::HitTargetMinimum(value) => Some(value.points()),
        WorthUiDensityValue::Posture(_) => None,
    }
}

fn resolve_flow_measurement_edges(
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
