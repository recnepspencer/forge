use crate::runtime::{
    WorthUiFlowLayoutFill, WorthUiFlowLayoutFit, WorthUiFlowLayoutKind,
    WorthUiPrimitiveProofReceipt,
};

use super::item_frame::{
    inline_content_height, inline_content_width, stacked_content_height, stacked_content_width,
};

pub(super) fn natural_flow_size(receipt: &WorthUiPrimitiveProofReceipt) -> (f32, f32) {
    let inline_width = inline_content_width(receipt);
    let inline_height = inline_content_height(receipt);
    match receipt.flow_layout().kind() {
        WorthUiFlowLayoutKind::Row | WorthUiFlowLayoutKind::Inline => {
            (inline_width.clamp(120.0, 360.0), inline_height.max(64.0))
        }
        WorthUiFlowLayoutKind::Column | WorthUiFlowLayoutKind::Stack => (
            stacked_content_width(receipt).clamp(120.0, 360.0),
            stacked_content_height(receipt).max(64.0),
        ),
        WorthUiFlowLayoutKind::Grid => (
            inline_width.clamp(120.0, 360.0),
            (inline_height * 2.0 + receipt.flow_layout().gap_points()).max(96.0),
        ),
        WorthUiFlowLayoutKind::Spacer => (receipt.flow_layout().gap_points().max(1.0), 1.0),
    }
}

pub(super) fn resolved_flow_width(
    receipt: &WorthUiPrimitiveProofReceipt,
    content_width: f32,
    natural_width: f32,
) -> f32 {
    if receipt.flow_layout().kind() == WorthUiFlowLayoutKind::Spacer {
        return natural_width;
    }
    match (receipt.flow_layout().fit(), receipt.flow_layout().fill()) {
        (WorthUiFlowLayoutFit::Fill, _) => content_width.max(0.0),
        (_, WorthUiFlowLayoutFill::Width | WorthUiFlowLayoutFill::Both) => content_width.max(0.0),
        _ => natural_width.clamp(120.0, content_width.max(120.0)),
    }
}

pub(super) fn resolved_flow_height(
    receipt: &WorthUiPrimitiveProofReceipt,
    content_height: f32,
    natural_height: f32,
) -> f32 {
    if receipt.flow_layout().kind() == WorthUiFlowLayoutKind::Spacer {
        return natural_height;
    }
    match (receipt.flow_layout().fit(), receipt.flow_layout().fill()) {
        (WorthUiFlowLayoutFit::Fill, _) => content_height.max(0.0),
        (_, WorthUiFlowLayoutFill::Height | WorthUiFlowLayoutFill::Both) => content_height.max(0.0),
        _ => natural_height.clamp(40.0, content_height.max(40.0)),
    }
}
