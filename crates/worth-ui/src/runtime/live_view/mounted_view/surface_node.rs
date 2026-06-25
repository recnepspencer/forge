use crate::runtime::live_view::digest::digest_parts;
use crate::runtime::{
    WorthUiBoxEdges, WorthUiFlowLayoutReceipt, WorthUiPrimitiveColor, WorthUiRuntimeFactId,
    WorthUiStatefulAppearanceRecipeReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiMountedSurfaceNodeReceipt {
    node_id: String,
    background_color: WorthUiPrimitiveColor,
    border_color: WorthUiPrimitiveColor,
    border_width_points: f32,
    radius_points: f32,
    padding_token: String,
    padding_edges: WorthUiBoxEdges,
    semantic_slice: &'static str,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    receipt_digest: u64,
}

impl WorthUiMountedSurfaceNodeReceipt {
    pub(super) fn from_receipts(
        live_view_id: &str,
        flow_layout: &WorthUiFlowLayoutReceipt,
        appearance: &WorthUiStatefulAppearanceRecipeReceipt,
    ) -> Self {
        let resolved = appearance.resolve_rest();
        let consumed_facts = vec![WorthUiRuntimeFactId::live_view_declaration(live_view_id)];
        let receipt_digest = digest_parts([
            live_view_id,
            "surface:form_card",
            resolved.receipt_digest().to_string().as_str(),
            flow_layout.receipt_digest().to_string().as_str(),
        ]);
        Self {
            node_id: "live_view.form_card".to_owned(),
            background_color: resolved.background_color(),
            border_color: resolved.border_color(),
            border_width_points: resolved.border_width_points(),
            radius_points: resolved.radius_points(),
            padding_token: flow_layout.padding_token().to_owned(),
            padding_edges: flow_layout.padding_edges(),
            semantic_slice: "LiveViewSurface",
            consumed_facts,
            receipt_digest,
        }
    }

    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    pub fn background_color(&self) -> WorthUiPrimitiveColor {
        self.background_color
    }

    pub fn border_color(&self) -> WorthUiPrimitiveColor {
        self.border_color
    }

    pub fn border_width_points(&self) -> f32 {
        self.border_width_points
    }

    pub fn radius_points(&self) -> f32 {
        self.radius_points
    }

    pub fn padding_points(&self) -> f32 {
        self.padding_edges.max_axis_point()
    }

    pub fn padding_token(&self) -> &str {
        &self.padding_token
    }

    pub fn padding_edges(&self) -> WorthUiBoxEdges {
        self.padding_edges
    }

    pub fn semantic_slice(&self) -> &'static str {
        self.semantic_slice
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}
