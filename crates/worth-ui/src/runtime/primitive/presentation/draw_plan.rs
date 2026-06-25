use crate::runtime::{
    WorthUiBoxEdges, WorthUiFlowLayoutAlign, WorthUiFlowLayoutKind, WorthUiPrimitiveProofReceipt,
    WorthUiPrimitiveProvedContentAnatomy,
};

use super::execution_counters::WorthUiPrimitiveLayoutExecutionCounters;
use super::frame::WorthUiPrimitiveFrame;
use super::graph_basis::WorthUiPrimitiveDrawPlanGraphBasis;
use super::item_frame::{
    flow_item_frames, planned_layout_item_count, WorthUiPrimitiveFlowItemFrame,
};
use super::natural_size::{natural_flow_size, resolved_flow_height, resolved_flow_width};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveDrawPlan {
    frame: WorthUiPrimitiveFrame,
    item_frames: Vec<WorthUiPrimitiveFlowItemFrame>,
    flow_padding_edges: WorthUiBoxEdges,
    graph_basis: WorthUiPrimitiveDrawPlanGraphBasis,
    counters: WorthUiPrimitiveLayoutExecutionCounters,
    receipt: WorthUiPrimitiveProofReceipt,
}

impl WorthUiPrimitiveDrawPlan {
    pub(crate) fn from_receipt(
        receipt: WorthUiPrimitiveProofReceipt,
        available_width: f32,
        available_height: f32,
    ) -> Self {
        let proved_content = receipt.content().proved_anatomy();
        Self::from_proved_content(receipt, proved_content, available_width, available_height)
    }

    pub(crate) fn from_proved_content(
        receipt: WorthUiPrimitiveProofReceipt,
        proved_content: WorthUiPrimitiveProvedContentAnatomy,
        available_width: f32,
        available_height: f32,
    ) -> Self {
        let graph_basis = WorthUiPrimitiveDrawPlanGraphBasis::from_primitive_receipt(&receipt);
        let padding = receipt.flow_layout().padding_edges();
        let padded = receipt.flow_layout().kind() != WorthUiFlowLayoutKind::Spacer;
        let content_width = if padded {
            available_width - padding.horizontal()
        } else {
            available_width
        };
        let content_height = if padded {
            available_height - padding.vertical()
        } else {
            available_height
        };
        let (natural_width, natural_height) =
            natural_flow_size(receipt.flow_layout(), &proved_content);
        let resolved_content_width = resolved_flow_width(&receipt, content_width, natural_width);
        let resolved_content_height =
            resolved_flow_height(&receipt, content_height, natural_height);
        let width = if padded {
            resolved_content_width + padding.horizontal()
        } else {
            resolved_content_width
        };
        let height = if padded {
            resolved_content_height + padding.vertical()
        } else {
            resolved_content_height
        };
        let x = match receipt.flow_layout().align() {
            WorthUiFlowLayoutAlign::Start => 0.0,
            WorthUiFlowLayoutAlign::Center => (available_width - width) * 0.5,
            WorthUiFlowLayoutAlign::End => available_width - width,
        };
        let y = (available_height - height) * 0.5;
        let item_offset_x = if padded { padding.left() } else { 0.0 };
        let item_offset_y = if padded { padding.top() } else { 0.0 };
        let item_frames = flow_item_frames(
            receipt.flow_layout(),
            &proved_content,
            resolved_content_width,
            resolved_content_height,
            item_offset_x,
            item_offset_y,
        );
        let content_item_count = proved_content.anatomy().item_count();
        Self {
            frame: WorthUiPrimitiveFrame::new(x, y, width, height),
            item_frames,
            flow_padding_edges: padding,
            graph_basis,
            counters: WorthUiPrimitiveLayoutExecutionCounters::new(
                content_item_count,
                planned_layout_item_count(receipt.flow_layout(), &proved_content),
                0,
                0,
            ),
            receipt,
        }
    }

    pub fn frame(&self) -> WorthUiPrimitiveFrame {
        self.frame
    }

    pub fn item_frames(&self) -> &[WorthUiPrimitiveFlowItemFrame] {
        &self.item_frames
    }

    pub fn flow_padding_edges(&self) -> WorthUiBoxEdges {
        self.flow_padding_edges
    }

    pub fn graph_basis(&self) -> &WorthUiPrimitiveDrawPlanGraphBasis {
        &self.graph_basis
    }

    pub fn receipt(&self) -> &WorthUiPrimitiveProofReceipt {
        &self.receipt
    }

    pub fn counters(&self) -> WorthUiPrimitiveLayoutExecutionCounters {
        self.counters
    }
}
