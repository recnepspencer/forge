use crate::runtime::{WorthUiFlowLayoutAlign, WorthUiFlowLayoutKind, WorthUiPrimitiveProofReceipt};

use super::execution_counters::WorthUiPrimitiveLayoutExecutionCounters;
use super::frame::WorthUiPrimitiveFrame;
use super::item_frame::{
    flow_item_frames, planned_layout_item_count, WorthUiPrimitiveFlowItemFrame,
};
use super::natural_size::{natural_flow_size, resolved_flow_height, resolved_flow_width};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveDrawPlan {
    frame: WorthUiPrimitiveFrame,
    item_frames: Vec<WorthUiPrimitiveFlowItemFrame>,
    counters: WorthUiPrimitiveLayoutExecutionCounters,
    receipt: WorthUiPrimitiveProofReceipt,
}

impl WorthUiPrimitiveDrawPlan {
    pub(crate) fn from_receipt(
        receipt: WorthUiPrimitiveProofReceipt,
        available_width: f32,
        available_height: f32,
    ) -> Self {
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
        let (natural_width, natural_height) = natural_flow_size(&receipt);
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
            &receipt,
            resolved_content_width,
            resolved_content_height,
            item_offset_x,
            item_offset_y,
        );
        let content_item_count = receipt.content().items().len();
        Self {
            frame: WorthUiPrimitiveFrame::new(x, y, width, height),
            item_frames,
            counters: WorthUiPrimitiveLayoutExecutionCounters::new(
                content_item_count,
                planned_layout_item_count(&receipt),
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

    pub fn receipt(&self) -> &WorthUiPrimitiveProofReceipt {
        &self.receipt
    }

    pub fn counters(&self) -> WorthUiPrimitiveLayoutExecutionCounters {
        self.counters
    }
}
