use super::{
    WorthUiCounterPacketBuilder, WorthUiFrameCostCounter, WorthUiLaneFrameReceiptKind,
    WorthUiRuntimeCounterFamily, WorthUiSteadyFrameCounterBoundary,
    WorthUiSteadyFrameCounterDenialReason,
};

#[test]
fn steady_frame_schema_rejects_missing_or_unexpected_lane_rows() {
    let missing_required_row = ordinary_packet_without_glyph_uploads(21);
    let missing_denial = WorthUiSteadyFrameCounterBoundary::for_active_plan(21)
        .record_lane_packet_for_test(WorthUiLaneFrameReceiptKind::Ordinary, missing_required_row)
        .seal()
        .expect_err("missing steady-frame lane row must fail schema validation");

    assert_eq!(
        missing_denial.reason(),
        WorthUiSteadyFrameCounterDenialReason::MissingRequiredCounterRow
    );

    let unexpected_row = ordinary_packet_with_unexpected_extra_row(22);
    let unexpected_denial = WorthUiSteadyFrameCounterBoundary::for_active_plan(22)
        .record_lane_packet_for_test(WorthUiLaneFrameReceiptKind::Ordinary, unexpected_row)
        .seal()
        .expect_err("unexpected steady-frame lane row must fail schema validation");

    assert_eq!(
        unexpected_denial.reason(),
        WorthUiSteadyFrameCounterDenialReason::UnexpectedCounterRow
    );
}

fn ordinary_packet_without_glyph_uploads(
    active_plan_digest: u64,
) -> super::WorthUiMeasurementCounterPacket {
    ordinary_packet_builder(active_plan_digest)
        .record(WorthUiFrameCostCounter::count(
            "lane.ordinary.execution.nodes_visited",
            1,
        ))
        .record(WorthUiFrameCostCounter::count(
            "lane.ordinary.execution.layout_recompute_breadth",
            0,
        ))
        .record(WorthUiFrameCostCounter::count(
            "lane.ordinary.execution.command_surfaces_touched",
            0,
        ))
        .record(WorthUiFrameCostCounter::count(
            "lane.ordinary.execution.token_support_touched",
            0,
        ))
        .record(WorthUiFrameCostCounter::count(
            "lane.ordinary.execution.text_shapes",
            0,
        ))
        .record(WorthUiFrameCostCounter::count(
            "lane.ordinary.execution.source_parse_count",
            0,
        ))
        .record(WorthUiFrameCostCounter::count(
            "lane.ordinary.execution.registry_lookup_count",
            0,
        ))
        .record(WorthUiFrameCostCounter::count(
            "lane.ordinary.execution.artifact_tree_scan_count",
            0,
        ))
        .record(WorthUiFrameCostCounter::count(
            "lane.ordinary.execution.full_plan_scan_count",
            0,
        ))
        .seal()
        .expect("malformed but sealed packet")
}

fn ordinary_packet_with_unexpected_extra_row(
    active_plan_digest: u64,
) -> super::WorthUiMeasurementCounterPacket {
    ordinary_packet_builder(active_plan_digest)
        .record(WorthUiFrameCostCounter::count(
            "lane.ordinary.execution.nodes_visited",
            1,
        ))
        .record(WorthUiFrameCostCounter::count(
            "lane.ordinary.execution.layout_recompute_breadth",
            0,
        ))
        .record(WorthUiFrameCostCounter::count(
            "lane.ordinary.execution.command_surfaces_touched",
            0,
        ))
        .record(WorthUiFrameCostCounter::count(
            "lane.ordinary.execution.token_support_touched",
            0,
        ))
        .record(WorthUiFrameCostCounter::count(
            "lane.ordinary.execution.text_shapes",
            0,
        ))
        .record(WorthUiFrameCostCounter::count(
            "lane.ordinary.execution.glyph_uploads",
            0,
        ))
        .record(WorthUiFrameCostCounter::count(
            "lane.ordinary.execution.source_parse_count",
            0,
        ))
        .record(WorthUiFrameCostCounter::count(
            "lane.ordinary.execution.registry_lookup_count",
            0,
        ))
        .record(WorthUiFrameCostCounter::count(
            "lane.ordinary.execution.artifact_tree_scan_count",
            0,
        ))
        .record(WorthUiFrameCostCounter::count(
            "lane.ordinary.execution.full_plan_scan_count",
            0,
        ))
        .record(WorthUiFrameCostCounter::count(
            "lane.ordinary.execution.extra_counter",
            1,
        ))
        .seal()
        .expect("malformed but sealed packet")
}

fn ordinary_packet_builder(active_plan_digest: u64) -> WorthUiCounterPacketBuilder {
    WorthUiRuntimeCounterFamily::OrdinaryLaneExecution
        .at_boundary(WorthUiRuntimeCounterFamily::OrdinaryLaneExecution.allowed_boundary())
        .with_active_plan_digest(active_plan_digest)
}
