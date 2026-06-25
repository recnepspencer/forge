use crate::runtime::{
    WorthUiAllocatedChildReceipt, WorthUiFlowLayoutCrossAlign, WorthUiLayoutAllocationFrame,
    WorthUiMountedFlowKind,
};

use super::participant::WorthUiLayoutAllocationParticipant;

pub(in crate::runtime::layout_allocation) fn allocate_participants(
    kind: WorthUiMountedFlowKind,
    participants: &[WorthUiLayoutAllocationParticipant],
    origin_x: f32,
    origin_y: f32,
    width: f32,
    height: f32,
    gap: f32,
    cross_align: WorthUiFlowLayoutCrossAlign,
) -> Vec<WorthUiAllocatedChildReceipt> {
    match kind {
        WorthUiMountedFlowKind::Row | WorthUiMountedFlowKind::Inline => {
            allocate_horizontal_participants(
                participants,
                origin_x,
                origin_y,
                width,
                height,
                gap,
                cross_align,
            )
        }
        WorthUiMountedFlowKind::Column | WorthUiMountedFlowKind::Stack => {
            allocate_vertical_participants(
                participants,
                origin_x,
                origin_y,
                width,
                height,
                gap,
                cross_align,
            )
        }
        WorthUiMountedFlowKind::Grid | WorthUiMountedFlowKind::Spacer => Vec::new(),
    }
}

fn allocate_horizontal_participants(
    participants: &[WorthUiLayoutAllocationParticipant],
    origin_x: f32,
    origin_y: f32,
    width: f32,
    height: f32,
    gap: f32,
    cross_align: WorthUiFlowLayoutCrossAlign,
) -> Vec<WorthUiAllocatedChildReceipt> {
    let participating = participating_participants(participants);
    let gap_total = gap * participating.len().saturating_sub(1) as f32;
    let hug_total = participating
        .iter()
        .filter(|participant| participant.sizing.fill_weight().is_none())
        .map(|participant| participant.natural_width)
        .sum::<f32>();
    let fill_weight_total = fill_weight_total(&participating);
    let fill_pool = (width - gap_total - hug_total).max(0.0);
    let baseline_max = participating
        .iter()
        .map(|participant| participant.natural_baseline)
        .fold(0.0, f32::max);
    let mut cursor_x = origin_x;
    let mut receipts = Vec::new();
    for participant in participants {
        let frame_width = horizontal_participant_width(participant, fill_pool, fill_weight_total);
        let frame_height =
            cross_axis_participant_size(participant, participant.natural_height, height);
        let frame_y = cross_axis_offset(
            cross_align,
            origin_y,
            height,
            frame_height,
            baseline_max,
            participant.natural_baseline,
        );
        receipts.push(allocated_child_receipt(
            participant,
            WorthUiLayoutAllocationFrame::new(cursor_x, frame_y, frame_width, frame_height),
        ));
        if participant.participation.participates_in_layout() {
            cursor_x += frame_width + gap;
        }
    }
    receipts
}

fn allocate_vertical_participants(
    participants: &[WorthUiLayoutAllocationParticipant],
    origin_x: f32,
    origin_y: f32,
    width: f32,
    height: f32,
    gap: f32,
    cross_align: WorthUiFlowLayoutCrossAlign,
) -> Vec<WorthUiAllocatedChildReceipt> {
    let participating = participating_participants(participants);
    let gap_total = gap * participating.len().saturating_sub(1) as f32;
    let hug_total = participating
        .iter()
        .filter(|participant| participant.sizing.fill_weight().is_none())
        .map(|participant| participant.natural_height)
        .sum::<f32>();
    let fill_weight_total = fill_weight_total(&participating);
    let fill_pool = (height - gap_total - hug_total).max(0.0);
    let mut cursor_y = origin_y;
    let mut receipts = Vec::new();
    for participant in participants {
        let frame_height = vertical_participant_height(participant, fill_pool, fill_weight_total);
        let frame_width =
            cross_axis_participant_size(participant, participant.natural_width, width);
        let frame_x = cross_axis_offset(cross_align, origin_x, width, frame_width, 0.0, 0.0);
        receipts.push(allocated_child_receipt(
            participant,
            WorthUiLayoutAllocationFrame::new(frame_x, cursor_y, frame_width, frame_height),
        ));
        if participant.participation.participates_in_layout() {
            cursor_y += frame_height + gap;
        }
    }
    receipts
}

fn horizontal_participant_width(
    participant: &WorthUiLayoutAllocationParticipant,
    fill_pool: f32,
    fill_weight_total: f32,
) -> f32 {
    participant_axis_size(
        participant,
        participant.natural_width,
        fill_pool,
        fill_weight_total,
    )
}

fn vertical_participant_height(
    participant: &WorthUiLayoutAllocationParticipant,
    fill_pool: f32,
    fill_weight_total: f32,
) -> f32 {
    participant_axis_size(
        participant,
        participant.natural_height,
        fill_pool,
        fill_weight_total,
    )
}

fn participant_axis_size(
    participant: &WorthUiLayoutAllocationParticipant,
    natural_size: f32,
    fill_pool: f32,
    fill_weight_total: f32,
) -> f32 {
    if !participant.participation.participates_in_layout() {
        return 0.0;
    }
    participant
        .sizing
        .fill_weight()
        .map(|weight| fill_pool * weight as f32 / fill_weight_total.max(1.0))
        .unwrap_or(natural_size)
}

fn cross_axis_participant_size(
    participant: &WorthUiLayoutAllocationParticipant,
    natural_size: f32,
    available_size: f32,
) -> f32 {
    if participant.participation.participates_in_layout() {
        natural_size.min(available_size)
    } else {
        0.0
    }
}

fn participating_participants(
    participants: &[WorthUiLayoutAllocationParticipant],
) -> Vec<&WorthUiLayoutAllocationParticipant> {
    participants
        .iter()
        .filter(|participant| participant.participation.participates_in_layout())
        .collect()
}

fn fill_weight_total(participants: &[&WorthUiLayoutAllocationParticipant]) -> f32 {
    participants
        .iter()
        .filter_map(|participant| participant.sizing.fill_weight())
        .sum::<u32>() as f32
}

fn allocated_child_receipt(
    participant: &WorthUiLayoutAllocationParticipant,
    frame: WorthUiLayoutAllocationFrame,
) -> WorthUiAllocatedChildReceipt {
    WorthUiAllocatedChildReceipt::new(
        participant.parent_id.clone(),
        participant.child_node_id.clone(),
        participant.order,
        participant.sizing,
        participant.sizing.token(),
        participant.participation,
        participant.natural_width,
        participant.natural_height,
        participant.natural_baseline,
        participant.natural_metric_basis.clone(),
        frame,
    )
}

fn cross_axis_offset(
    cross_align: WorthUiFlowLayoutCrossAlign,
    origin: f32,
    available: f32,
    item_size: f32,
    baseline_max: f32,
    item_baseline: f32,
) -> f32 {
    match cross_align {
        WorthUiFlowLayoutCrossAlign::Start => origin,
        WorthUiFlowLayoutCrossAlign::Center => origin + (available - item_size).max(0.0) / 2.0,
        WorthUiFlowLayoutCrossAlign::End => origin + (available - item_size).max(0.0),
        WorthUiFlowLayoutCrossAlign::Baseline => origin + (baseline_max - item_baseline).max(0.0),
    }
}
