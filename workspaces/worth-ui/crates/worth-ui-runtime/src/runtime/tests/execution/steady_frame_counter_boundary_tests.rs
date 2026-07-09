use super::canvas_spatial_lane_test_support::canvas_spatial_fixture;
use super::ordinary_lane_test_support::ordinary_lane_fixture;
use super::realtime_overlay_lane_test_support::realtime_overlay_fixture;
use super::virtualized_data_lane_test_support::virtualized_data_fixture;
use super::{
    WorthUiCanvasSpatialFrameTarget, WorthUiCanvasSpatialLane, WorthUiFrameCostCounter,
    WorthUiFrameReportMaterializationBoundary, WorthUiLaneFrameReceiptKind,
    WorthUiOrdinaryFrameTarget, WorthUiOrdinaryLaneCounters, WorthUiRealtimeFrameTarget,
    WorthUiRealtimeLaneCounters, WorthUiRealtimeOverlayLane, WorthUiRuntimeCounterFamily,
    WorthUiSteadyFrameCounterBoundary, WorthUiSteadyFrameCounterDenialReason,
    WorthUiSteadyFrameFoundationalBridge, WorthUiSteadyFrameReportPlanner,
    WorthUiViewBindingHandle, WorthUiVirtualizedDataFrameTarget, WorthUiVirtualizedDataLane,
    WorthUiVisibleRange,
};
use worth_foundational::FoundationalPerformanceReportMaterializationBoundary;

#[test]
fn steady_frame_counters_replay_for_equivalent_active_plan() {
    let left = complete_steady_frame_receipt(77);
    let right = complete_steady_frame_receipt(77);

    assert_eq!(left.counters(), right.counters());
    assert_eq!(
        left.aggregate_packet().replay_digest(),
        right.aggregate_packet().replay_digest()
    );
    assert_eq!(
        left.lane_receipts()
            .iter()
            .map(|receipt| receipt.packet().replay_digest())
            .collect::<Vec<_>>(),
        right
            .lane_receipts()
            .iter()
            .map(|receipt| receipt.packet().replay_digest())
            .collect::<Vec<_>>()
    );
}

#[test]
fn active_plan_digest_is_carried_by_every_frame_counter_packet() {
    let first_plan = complete_steady_frame_receipt(77);
    let second_plan = complete_steady_frame_receipt(78);

    assert_eq!(first_plan.active_plan_digest(), 77);
    assert_eq!(first_plan.aggregate_packet().active_plan_digest(), 77);
    assert!(first_plan
        .lane_receipts()
        .iter()
        .all(|receipt| receipt.packet().active_plan_digest() == 77));
    assert_ne!(
        first_plan.aggregate_packet().replay_digest(),
        second_plan.aggregate_packet().replay_digest()
    );
    assert_ne!(
        first_plan
            .lane_receipts()
            .iter()
            .map(|receipt| receipt.packet().replay_digest())
            .collect::<Vec<_>>(),
        second_plan
            .lane_receipts()
            .iter()
            .map(|receipt| receipt.packet().replay_digest())
            .collect::<Vec<_>>()
    );
}

#[test]
fn steady_frame_source_parse_or_registry_lookup_counter_must_be_zero() {
    let mut ordinary = WorthUiOrdinaryLaneCounters::default();
    ordinary.record_frame_row_touch();
    ordinary.record_source_parse();

    let denial = WorthUiSteadyFrameCounterBoundary::for_active_plan(5)
        .record_ordinary_counters_for_test(ordinary)
        .seal()
        .expect_err("steady frames must not parse source");

    assert_eq!(
        denial.reason(),
        WorthUiSteadyFrameCounterDenialReason::ForbiddenFramePathWork
    );
}

#[test]
fn lane_specific_frame_counters_expose_expected_work_breadth() {
    let receipt = complete_steady_frame_receipt(88);
    let kinds = receipt
        .lane_receipts()
        .iter()
        .map(|receipt| receipt.kind())
        .collect::<Vec<_>>();

    assert_eq!(
        kinds,
        vec![
            WorthUiLaneFrameReceiptKind::Ordinary,
            WorthUiLaneFrameReceiptKind::VirtualizedData,
            WorthUiLaneFrameReceiptKind::CanvasSpatial,
            WorthUiLaneFrameReceiptKind::RealtimeOverlay,
        ]
    );
    assert_eq!(
        receipt
            .counters()
            .ordinary()
            .ordinary_frame_row_touch_count(),
        1
    );
    assert_eq!(
        receipt
            .counters()
            .virtualized_data()
            .visible_row_touch_count(),
        40
    );
    assert_eq!(
        receipt
            .counters()
            .virtualized_data()
            .visible_column_touch_count(),
        12
    );
    assert_eq!(
        receipt.counters().canvas_spatial().spatial_hit_test_count(),
        1
    );
    assert_eq!(
        receipt
            .counters()
            .realtime_overlay()
            .frame_synchronized_pass_count(),
        1
    );
}

#[test]
fn text_shape_and_glyph_upload_counters_are_frozen_explicitly() {
    let mut ordinary = WorthUiOrdinaryLaneCounters::default();
    ordinary.record_frame_row_touch();
    ordinary.record_text_shape();
    ordinary.record_glyph_upload();

    let receipt = WorthUiSteadyFrameCounterBoundary::for_active_plan(91)
        .record_ordinary_counters_for_test(ordinary)
        .seal()
        .expect("steady frame receipt seals with rendering counters");

    assert_eq!(receipt.counters().ordinary().text_shape_count(), 1);
    assert_eq!(receipt.counters().ordinary().glyph_upload_count(), 1);
    assert_eq!(
        packet_row_value(
            receipt.aggregate_packet(),
            "frame.steady_rendering.text_shapes"
        ),
        1
    );
    assert_eq!(
        packet_row_value(
            receipt.aggregate_packet(),
            "frame.steady_rendering.glyph_uploads"
        ),
        1
    );
    assert_eq!(
        packet_row_value(
            receipt.lane_receipts()[0].packet(),
            "lane.ordinary.execution.text_shapes"
        ),
        1
    );
    assert_eq!(
        packet_row_value(
            receipt.lane_receipts()[0].packet(),
            "lane.ordinary.execution.glyph_uploads"
        ),
        1
    );
}

#[test]
fn steady_frame_counters_fail_on_diagnostic_materialization_by_default() {
    let mut realtime = WorthUiRealtimeLaneCounters::default();
    realtime.record_frame_synchronized_pass();
    realtime.record_forbidden_work();

    let denial = WorthUiSteadyFrameCounterBoundary::for_active_plan(9)
        .record_realtime_counters_for_test(realtime)
        .seal()
        .expect_err("minimal steady frame diagnostics cannot materialize reports");

    assert_eq!(
        denial.reason(),
        WorthUiSteadyFrameCounterDenialReason::ForbiddenFramePathWork
    );

    let materialization_denial = WorthUiSteadyFrameCounterBoundary::for_active_plan(9)
        .record_realtime_counters_for_test(realtime_counters())
        .record_frame_path_report_materialization_for_test()
        .seal()
        .expect_err("report materialization must be outside ordinary frame sealing");

    assert_eq!(
        materialization_denial.reason(),
        WorthUiSteadyFrameCounterDenialReason::DiagnosticMaterializationOnMinimalPolicy
    );
}

#[test]
fn foundational_receipt_counter_rows_match_steady_frame_specs_exactly() {
    let certified = complete_steady_frame_receipt(99)
        .certify()
        .expect("steady frame receipt certifies");
    let evidence = WorthUiSteadyFrameFoundationalBridge::lower_counter_receipts(&certified)
        .expect("steady frame counters lower to Foundational receipts");

    assert_eq!(
        evidence.receipt_count(),
        certified.receipt().lane_receipts().len() + 1
    );
    for receipt_evidence in evidence.evidence() {
        assert_eq!(
            receipt_evidence.counter_specs().len(),
            receipt_evidence.counter_rows().len()
        );
        assert!(receipt_evidence.canonical_basis_entry_count() > 0);
        for (spec, row) in receipt_evidence
            .counter_specs()
            .iter()
            .zip(receipt_evidence.counter_rows())
        {
            assert_eq!(spec.name(), row.name());
            assert_eq!(spec.expected_exact_count(), row.observed_count());
        }
    }

    let plan = WorthUiSteadyFrameReportPlanner::support_report()
        .plan_from_foundational_receipts(&evidence)
        .expect("Foundational report planning succeeds");
    assert_eq!(
        plan.materialization_boundary(),
        WorthUiFrameReportMaterializationBoundary::ReportAssembly
    );
    assert_eq!(plan.source_receipt_count(), evidence.receipt_count());
    assert_eq!(
        plan.foundational_boundaries(),
        vec![
            FoundationalPerformanceReportMaterializationBoundary::ReportAssembly;
            evidence.receipt_count()
        ]
    );
}

#[test]
fn duplicate_lane_frame_receipts_fail_before_foundational_lowering() {
    let mut ordinary = WorthUiOrdinaryLaneCounters::default();
    ordinary.record_frame_row_touch();

    let denial = WorthUiSteadyFrameCounterBoundary::for_active_plan(10)
        .record_ordinary_counters_for_test(ordinary)
        .record_ordinary_counters_for_test(ordinary)
        .seal()
        .expect_err("one steady frame cannot double-count one lane kind");

    assert_eq!(
        denial.reason(),
        WorthUiSteadyFrameCounterDenialReason::DuplicateLaneFrameReceipt
    );
}

fn complete_steady_frame_receipt(active_plan_digest: u64) -> super::WorthUiFrameExecutionReceipt {
    let (ordinary_runtime, ordinary_plan, ordinary_allocation) = ordinary_lane_fixture();
    let ordinary_handle = ordinary_allocation.component_handles()[0];
    let ordinary_receipt = ordinary_runtime
        .execute_ordinary_lane_frame(
            &ordinary_plan,
            WorthUiOrdinaryFrameTarget::component(ordinary_handle),
        )
        .expect("ordinary frame executes");

    let virtualized = virtualized_data_fixture();
    let data_handle = first_view_binding_handle(&virtualized.allocation);
    let range = WorthUiVisibleRange::grid(120, 40, 0, 12).expect("range is valid");
    let virtualized_receipt = virtualized
        .runtime
        .execute_virtualized_data_frame(
            &virtualized.data_plan,
            WorthUiVirtualizedDataFrameTarget::view_binding(data_handle, range),
        )
        .expect("virtualized frame executes");

    let canvas = canvas_spatial_fixture();
    let canvas_lane = canvas.canvas_plan.rows()[0].lane_handle();
    let canvas_receipt = canvas
        .runtime
        .execute_canvas_spatial_frame(
            &canvas.canvas_plan,
            WorthUiCanvasSpatialFrameTarget::hit_test(
                super::WorthUiSpatialHitTestPlan::for_viewport_point(
                    canvas_lane,
                    super::WorthUiSpatialViewportPoint::viewport(144, 96),
                ),
            ),
        )
        .expect("canvas frame executes");

    let realtime = realtime_overlay_fixture();
    let surface = realtime.hud_plan.renderer_surfaces()[0].handle();
    let realtime_receipt = realtime
        .runtime
        .execute_realtime_frame(
            &realtime.hud_plan,
            WorthUiRealtimeFrameTarget::renderer_surface(surface),
        )
        .expect("realtime frame executes");

    assert_eq!(
        virtualized_receipt.lane(),
        WorthUiVirtualizedDataLane::CellGrid
    );
    assert_eq!(canvas_receipt.lane(), WorthUiCanvasSpatialLane::HitTest);
    assert_eq!(
        realtime_receipt.lane(),
        WorthUiRealtimeOverlayLane::HudOverlay
    );

    let receipt = WorthUiSteadyFrameCounterBoundary::for_active_plan(active_plan_digest)
        .record_ordinary_lane_frame(ordinary_receipt)
        .record_virtualized_data_frame(virtualized_receipt)
        .record_canvas_spatial_frame(canvas_receipt)
        .record_realtime_overlay_frame(realtime_receipt)
        .seal()
        .expect("steady frame receipt seals");

    assert_eq!(
        receipt.aggregate_packet().family(),
        WorthUiRuntimeCounterFamily::SteadyFrameRendering
    );
    receipt
}

fn first_view_binding_handle(
    allocation: &super::WorthUiRuntimeHandleAllocation,
) -> WorthUiViewBindingHandle {
    allocation
        .view_binding_handles()
        .first()
        .copied()
        .expect("fixture has view binding handle")
}

fn realtime_counters() -> WorthUiRealtimeLaneCounters {
    let mut counters = WorthUiRealtimeLaneCounters::default();
    counters.record_frame_synchronized_pass();
    counters.record_renderer_surface_handoff();
    counters
}

fn packet_row_value(packet: &super::WorthUiMeasurementCounterPacket, name: &str) -> u64 {
    packet
        .counters()
        .iter()
        .find(|counter| counter.name() == name)
        .map(WorthUiFrameCostCounter::value)
        .expect("counter row exists")
}
