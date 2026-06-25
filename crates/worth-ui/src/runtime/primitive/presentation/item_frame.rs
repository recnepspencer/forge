use crate::runtime::{
    WorthUiFlowLayoutCrossAlign, WorthUiFlowLayoutKind, WorthUiFlowLayoutReceipt,
    WorthUiPrimitiveContentAnatomyItemReceipt, WorthUiPrimitiveContentItemKind,
    WorthUiPrimitiveProvedContentAnatomy,
};

use super::frame::WorthUiPrimitiveFrame;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveFlowItemKind {
    Badge,
    Divider,
    Icon,
    Image,
    Spacer,
    Text,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveFlowItemFrame {
    item_index: usize,
    item_kind: WorthUiPrimitiveFlowItemKind,
    frame: WorthUiPrimitiveFrame,
}

impl WorthUiPrimitiveFlowItemFrame {
    pub(crate) fn new(
        item_index: usize,
        item_kind: WorthUiPrimitiveFlowItemKind,
        frame: WorthUiPrimitiveFrame,
    ) -> Self {
        Self {
            item_index,
            item_kind,
            frame,
        }
    }

    pub fn item_index(&self) -> usize {
        self.item_index
    }

    pub fn item_kind(&self) -> WorthUiPrimitiveFlowItemKind {
        self.item_kind
    }

    pub fn frame(&self) -> WorthUiPrimitiveFrame {
        self.frame
    }
}

pub(super) fn flow_item_frames(
    flow_layout: &WorthUiFlowLayoutReceipt,
    content: &WorthUiPrimitiveProvedContentAnatomy,
    width: f32,
    height: f32,
    origin_x: f32,
    origin_y: f32,
) -> Vec<WorthUiPrimitiveFlowItemFrame> {
    match flow_layout.kind() {
        WorthUiFlowLayoutKind::Column | WorthUiFlowLayoutKind::Stack => {
            stacked_flow_item_frames(flow_layout, content, width, height, origin_x, origin_y)
        }
        WorthUiFlowLayoutKind::Spacer => Vec::new(),
        _ => inline_flow_item_frames(flow_layout, content, width, height, origin_x, origin_y),
    }
}

pub(super) fn inline_content_width(
    flow_layout: &WorthUiFlowLayoutReceipt,
    content: &WorthUiPrimitiveProvedContentAnatomy,
) -> f32 {
    let item_width: f32 = content
        .anatomy()
        .items()
        .iter()
        .map(inline_item_width)
        .sum();
    let gap_width =
        content.anatomy().items().len().saturating_sub(1) as f32 * flow_layout.gap_points();
    item_width + gap_width
}

pub(super) fn inline_content_height(content: &WorthUiPrimitiveProvedContentAnatomy) -> f32 {
    content
        .anatomy()
        .items()
        .iter()
        .map(inline_item_height)
        .fold(0.0, f32::max)
}

pub(super) fn stacked_content_width(content: &WorthUiPrimitiveProvedContentAnatomy) -> f32 {
    content
        .anatomy()
        .items()
        .iter()
        .map(inline_item_width)
        .fold(0.0, f32::max)
}

pub(super) fn stacked_content_height(
    flow_layout: &WorthUiFlowLayoutReceipt,
    content: &WorthUiPrimitiveProvedContentAnatomy,
) -> f32 {
    let item_height: f32 = content
        .anatomy()
        .items()
        .iter()
        .map(inline_item_height)
        .sum();
    let gap_height =
        content.anatomy().items().len().saturating_sub(1) as f32 * flow_layout.gap_points();
    item_height + gap_height
}

pub(super) fn planned_layout_item_count(
    flow_layout: &WorthUiFlowLayoutReceipt,
    content: &WorthUiPrimitiveProvedContentAnatomy,
) -> usize {
    match flow_layout.kind() {
        WorthUiFlowLayoutKind::Spacer => 0,
        _ => content.anatomy().item_count(),
    }
}

fn inline_flow_item_frames(
    flow_layout: &WorthUiFlowLayoutReceipt,
    content: &WorthUiPrimitiveProvedContentAnatomy,
    width: f32,
    height: f32,
    origin_x: f32,
    origin_y: f32,
) -> Vec<WorthUiPrimitiveFlowItemFrame> {
    let content_width = inline_content_width(flow_layout, content);
    let baseline = inline_content_baseline(content);
    let mut cursor_x = (width - content_width) * 0.5;
    content
        .anatomy()
        .items()
        .iter()
        .enumerate()
        .map(|(item_index, item)| {
            let item_width = inline_item_width(item);
            let item_height = inline_item_height(item);
            let frame = WorthUiPrimitiveFrame::new(
                origin_x + cursor_x,
                origin_y + cross_axis_y(flow_layout.cross_align(), height, item, baseline),
                item_width,
                item_height,
            );
            cursor_x += item_width + flow_layout.gap_points();
            WorthUiPrimitiveFlowItemFrame::new(item_index, flow_item_kind(item), frame)
        })
        .collect()
}

fn stacked_flow_item_frames(
    flow_layout: &WorthUiFlowLayoutReceipt,
    content: &WorthUiPrimitiveProvedContentAnatomy,
    width: f32,
    height: f32,
    origin_x: f32,
    origin_y: f32,
) -> Vec<WorthUiPrimitiveFlowItemFrame> {
    let content_height = stacked_content_height(flow_layout, content);
    let mut cursor_y = (height - content_height) * 0.5;
    content
        .anatomy()
        .items()
        .iter()
        .enumerate()
        .map(|(item_index, item)| {
            let item_width = inline_item_width(item);
            let item_height = inline_item_height(item);
            let frame = WorthUiPrimitiveFrame::new(
                origin_x + cross_axis_x(flow_layout.cross_align(), width, item_width),
                origin_y + cursor_y,
                item_width,
                item_height,
            );
            cursor_y += item_height + flow_layout.gap_points();
            WorthUiPrimitiveFlowItemFrame::new(item_index, flow_item_kind(item), frame)
        })
        .collect()
}

fn cross_axis_y(
    cross_align: WorthUiFlowLayoutCrossAlign,
    height: f32,
    item: &WorthUiPrimitiveContentAnatomyItemReceipt,
    content_baseline: f32,
) -> f32 {
    match cross_align {
        WorthUiFlowLayoutCrossAlign::Start => 0.0,
        WorthUiFlowLayoutCrossAlign::Center => (height - inline_item_height(item)) * 0.5,
        WorthUiFlowLayoutCrossAlign::End => height - inline_item_height(item),
        WorthUiFlowLayoutCrossAlign::Baseline => content_baseline - inline_item_baseline(item),
    }
}

fn cross_axis_x(cross_align: WorthUiFlowLayoutCrossAlign, width: f32, item_width: f32) -> f32 {
    match cross_align {
        WorthUiFlowLayoutCrossAlign::Start | WorthUiFlowLayoutCrossAlign::Baseline => 0.0,
        WorthUiFlowLayoutCrossAlign::Center => (width - item_width) * 0.5,
        WorthUiFlowLayoutCrossAlign::End => width - item_width,
    }
}

fn flow_item_kind(
    item: &WorthUiPrimitiveContentAnatomyItemReceipt,
) -> WorthUiPrimitiveFlowItemKind {
    match item.item_kind() {
        WorthUiPrimitiveContentItemKind::Badge => WorthUiPrimitiveFlowItemKind::Badge,
        WorthUiPrimitiveContentItemKind::Divider => WorthUiPrimitiveFlowItemKind::Divider,
        WorthUiPrimitiveContentItemKind::Icon => WorthUiPrimitiveFlowItemKind::Icon,
        WorthUiPrimitiveContentItemKind::Image => WorthUiPrimitiveFlowItemKind::Image,
        WorthUiPrimitiveContentItemKind::Spacer => WorthUiPrimitiveFlowItemKind::Spacer,
        WorthUiPrimitiveContentItemKind::Text => WorthUiPrimitiveFlowItemKind::Text,
    }
}

fn inline_item_width(item: &WorthUiPrimitiveContentAnatomyItemReceipt) -> f32 {
    item.width_points()
}

fn inline_item_height(item: &WorthUiPrimitiveContentAnatomyItemReceipt) -> f32 {
    item.height_points()
}

fn inline_content_baseline(content: &WorthUiPrimitiveProvedContentAnatomy) -> f32 {
    content
        .anatomy()
        .items()
        .iter()
        .map(inline_item_baseline)
        .fold(0.0, f32::max)
}

fn inline_item_baseline(item: &WorthUiPrimitiveContentAnatomyItemReceipt) -> f32 {
    item.baseline_points()
}
