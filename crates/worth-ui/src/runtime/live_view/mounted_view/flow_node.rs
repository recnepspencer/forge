use crate::runtime::live_view::digest::digest_parts;
use crate::runtime::{
    WorthUiBoxEdges, WorthUiFlowLayoutAlign, WorthUiFlowLayoutCrossAlign, WorthUiFlowLayoutKind,
    WorthUiFlowLayoutReceipt, WorthUiRuntimeFactId,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiMountedFlowKind {
    Row,
    Column,
    Inline,
    Stack,
    Grid,
    Spacer,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiMountedFlowContainerNodeReceipt {
    node_id: String,
    kind: WorthUiMountedFlowKind,
    gap_token: String,
    gap_points: f32,
    padding_token: String,
    padding_edges: WorthUiBoxEdges,
    align: WorthUiMountedFlowAlign,
    cross_align: WorthUiFlowLayoutCrossAlign,
    semantic_slice: &'static str,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    receipt_digest: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiMountedFlowAlign {
    Start,
    Center,
    End,
}

impl WorthUiMountedFlowContainerNodeReceipt {
    pub(super) fn from_flow_layout_node(
        node_id: &str,
        live_view_id: &str,
        flow_layout: &WorthUiFlowLayoutReceipt,
    ) -> Self {
        let consumed_facts = vec![WorthUiRuntimeFactId::live_view_declaration(live_view_id)];
        let receipt_digest = digest_parts([
            live_view_id,
            node_id,
            flow_layout.receipt_digest().to_string().as_str(),
        ]);
        Self {
            node_id: node_id.to_owned(),
            kind: mounted_flow_kind(flow_layout.kind()),
            gap_token: flow_layout.gap_token().to_owned(),
            gap_points: flow_layout.gap_points(),
            padding_token: flow_layout.padding_token().to_owned(),
            padding_edges: flow_layout.padding_edges(),
            align: mounted_align(flow_layout.align()),
            cross_align: flow_layout.cross_align(),
            semantic_slice: "LiveViewFlow",
            consumed_facts,
            receipt_digest,
        }
    }

    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    pub fn kind(&self) -> WorthUiMountedFlowKind {
        self.kind
    }

    pub fn gap_points(&self) -> f32 {
        self.gap_points
    }

    pub fn gap_token(&self) -> &str {
        &self.gap_token
    }

    pub fn padding_token(&self) -> &str {
        &self.padding_token
    }

    pub fn padding_edges(&self) -> crate::runtime::WorthUiBoxEdges {
        self.padding_edges
    }

    pub fn align(&self) -> WorthUiMountedFlowAlign {
        self.align
    }

    pub fn cross_align(&self) -> crate::runtime::WorthUiFlowLayoutCrossAlign {
        self.cross_align
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

fn mounted_flow_kind(kind: WorthUiFlowLayoutKind) -> WorthUiMountedFlowKind {
    match kind {
        WorthUiFlowLayoutKind::Row => WorthUiMountedFlowKind::Row,
        WorthUiFlowLayoutKind::Column => WorthUiMountedFlowKind::Column,
        WorthUiFlowLayoutKind::Inline => WorthUiMountedFlowKind::Inline,
        WorthUiFlowLayoutKind::Stack => WorthUiMountedFlowKind::Stack,
        WorthUiFlowLayoutKind::Grid => WorthUiMountedFlowKind::Grid,
        WorthUiFlowLayoutKind::Spacer => WorthUiMountedFlowKind::Spacer,
    }
}

fn mounted_align(align: WorthUiFlowLayoutAlign) -> WorthUiMountedFlowAlign {
    match align {
        WorthUiFlowLayoutAlign::Start => WorthUiMountedFlowAlign::Start,
        WorthUiFlowLayoutAlign::Center => WorthUiMountedFlowAlign::Center,
        WorthUiFlowLayoutAlign::End => WorthUiMountedFlowAlign::End,
    }
}
