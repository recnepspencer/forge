use crate::runtime::live_view::digest::digest_parts;
use crate::runtime::{
    WorthUiFlowLayoutReceipt, WorthUiLiveViewPayloadProjectionReceipt,
    WorthUiLiveViewReadinessProjectionReceipt, WorthUiPrimitiveEventGeometryReceipt,
    WorthUiQueryGraphExecutionReceipt, WorthUiStatefulAppearanceRecipeReceipt,
};

use super::declaration::{
    WorthUiLiveViewInteractionIntentDeclaration, WorthUiLiveViewInteractionIntentKind,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiLiveViewInteractionIntentReceipt {
    live_view_id: String,
    interaction_id: String,
    kind: WorthUiLiveViewInteractionIntentKind,
    effect: String,
    label: String,
    readiness: WorthUiLiveViewReadinessProjectionReceipt,
    payload: WorthUiLiveViewPayloadProjectionReceipt,
    flow_layout: WorthUiFlowLayoutReceipt,
    appearance: WorthUiStatefulAppearanceRecipeReceipt,
    event_geometry: WorthUiPrimitiveEventGeometryReceipt,
    graph_execution: WorthUiQueryGraphExecutionReceipt,
    interaction_intent_digest: u64,
}

impl WorthUiLiveViewInteractionIntentReceipt {
    pub(crate) fn new(
        live_view_id: &str,
        declaration: &WorthUiLiveViewInteractionIntentDeclaration,
        readiness: WorthUiLiveViewReadinessProjectionReceipt,
        payload: WorthUiLiveViewPayloadProjectionReceipt,
        flow_layout: WorthUiFlowLayoutReceipt,
        appearance: WorthUiStatefulAppearanceRecipeReceipt,
        event_geometry: WorthUiPrimitiveEventGeometryReceipt,
        graph_execution: WorthUiQueryGraphExecutionReceipt,
    ) -> Self {
        let interaction_intent_digest = digest_parts([
            live_view_id,
            declaration.interaction_id(),
            declaration.kind().token(),
            declaration.effect(),
            readiness.readiness_digest().to_string().as_str(),
            payload.payload_projection_digest().to_string().as_str(),
            flow_layout.receipt_digest().to_string().as_str(),
            appearance.receipt_digest().to_string().as_str(),
            event_geometry.receipt_digest().to_string().as_str(),
        ]);
        Self {
            live_view_id: live_view_id.to_owned(),
            interaction_id: declaration.interaction_id().to_owned(),
            kind: declaration.kind().clone(),
            effect: declaration.effect().to_owned(),
            label: declaration.label().to_owned(),
            readiness,
            payload,
            flow_layout,
            appearance,
            event_geometry,
            graph_execution,
            interaction_intent_digest,
        }
    }

    pub fn live_view_id(&self) -> &str {
        &self.live_view_id
    }

    pub fn interaction_id(&self) -> &str {
        &self.interaction_id
    }

    pub fn kind(&self) -> &WorthUiLiveViewInteractionIntentKind {
        &self.kind
    }

    pub fn effect(&self) -> &str {
        &self.effect
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn readiness(&self) -> &WorthUiLiveViewReadinessProjectionReceipt {
        &self.readiness
    }

    pub fn payload_projection(&self) -> &WorthUiLiveViewPayloadProjectionReceipt {
        &self.payload
    }

    pub fn flow_layout(&self) -> &WorthUiFlowLayoutReceipt {
        &self.flow_layout
    }

    pub fn appearance(&self) -> &WorthUiStatefulAppearanceRecipeReceipt {
        &self.appearance
    }

    pub fn event_geometry(&self) -> &WorthUiPrimitiveEventGeometryReceipt {
        &self.event_geometry
    }

    pub fn query_graph_execution(&self) -> &WorthUiQueryGraphExecutionReceipt {
        &self.graph_execution
    }

    pub fn interaction_intent_digest(&self) -> u64 {
        self.interaction_intent_digest
    }
}
