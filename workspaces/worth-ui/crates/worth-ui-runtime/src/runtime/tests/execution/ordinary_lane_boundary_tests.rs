use super::ordinary_lane_test_support::{
    ordinary_lane_denial_for_missing_family, ordinary_lane_fixture,
};
use crate::runtime::{
    WorthUiComponentHandle, WorthUiExecutionLane, WorthUiHandlePlanGeneration,
    WorthUiOrdinaryFrameTarget, WorthUiOrdinaryLaneFrameDenialReason,
    WorthUiOrdinaryLanePlanDenialReason, WorthUiPlanNodeInputFamily,
};

#[test]
fn equivalent_ordinary_lane_plans_execute_with_equivalent_traversal_counters() {
    let (left_runtime, left_plan, left_allocation) = ordinary_fixture();
    let (right_runtime, right_plan, right_allocation) = ordinary_fixture();
    let left_handle = left_allocation
        .component_handles()
        .first()
        .copied()
        .expect("fixture has component handle");
    let right_handle = right_allocation
        .component_handles()
        .first()
        .copied()
        .expect("fixture has component handle");

    let left_receipt = left_runtime
        .execute_ordinary_lane_frame(
            &left_plan,
            WorthUiOrdinaryFrameTarget::component(left_handle),
        )
        .expect("ordinary component frame executes");
    let right_receipt = right_runtime
        .execute_ordinary_lane_frame(
            &right_plan,
            WorthUiOrdinaryFrameTarget::component(right_handle),
        )
        .expect("ordinary component frame executes");

    assert_eq!(
        left_receipt.certification().ordinary_plan_digest(),
        right_receipt.certification().ordinary_plan_digest()
    );
    assert_eq!(left_receipt.counters(), right_receipt.counters());
    assert_eq!(left_receipt.counters().ordinary_frame_row_touch_count(), 1);
    assert_eq!(left_receipt.counters().full_plan_scan_count(), 0);
}

#[test]
fn ordinary_lane_rejects_virtualized_or_canvas_surface_claims() {
    let (runtime, ordinary_plan, _) = ordinary_fixture();

    let virtualized = runtime
        .execute_ordinary_lane_frame(
            &ordinary_plan,
            WorthUiOrdinaryFrameTarget::virtualized_data_for_test(31),
        )
        .expect_err("virtualized data cannot enter ordinary lane");
    let canvas = runtime
        .execute_ordinary_lane_frame(
            &ordinary_plan,
            WorthUiOrdinaryFrameTarget::canvas_spatial_for_test(37),
        )
        .expect_err("canvas spatial cannot enter ordinary lane");
    let realtime = runtime
        .execute_ordinary_lane_frame(
            &ordinary_plan,
            WorthUiOrdinaryFrameTarget::realtime_overlay_for_test(41),
        )
        .expect_err("realtime overlay cannot enter ordinary lane");

    assert_eq!(
        virtualized.reason(),
        WorthUiOrdinaryLaneFrameDenialReason::NonOrdinaryLaneClaim
    );
    assert_eq!(canvas.reason(), virtualized.reason());
    assert_eq!(realtime.reason(), virtualized.reason());
    assert_eq!(virtualized.counters().non_ordinary_claim_denial_count(), 1);
    assert_eq!(canvas.counters().non_ordinary_claim_denial_count(), 1);
    assert_eq!(realtime.counters().non_ordinary_claim_denial_count(), 1);
}

#[test]
fn ordinary_frame_path_does_not_parse_or_resolve_source() {
    let (runtime, ordinary_plan, allocation) = ordinary_fixture();
    let command_handle = allocation
        .command_handles()
        .first()
        .copied()
        .expect("fixture has command handle");
    let token_handle = allocation
        .token_handles()
        .first()
        .copied()
        .expect("fixture has token handle");

    let command_receipt = runtime
        .execute_ordinary_lane_frame(
            &ordinary_plan,
            WorthUiOrdinaryFrameTarget::command(command_handle),
        )
        .expect("command surface frame executes from typed handle");
    let token_receipt = runtime
        .execute_ordinary_lane_frame(
            &ordinary_plan,
            WorthUiOrdinaryFrameTarget::token_support(token_handle),
        )
        .expect("token support frame executes from typed handle");

    assert_path_is_source_free(command_receipt.counters());
    assert_path_is_source_free(token_receipt.counters());
    assert_eq!(command_receipt.counters().command_surface_touch_count(), 1);
    assert_eq!(token_receipt.counters().token_support_touch_count(), 1);

    let parse_denial = runtime
        .execute_ordinary_lane_frame(
            &ordinary_plan,
            WorthUiOrdinaryFrameTarget::parse_source_for_test(),
        )
        .expect_err("source parsing path is denied");
    let lookup_denial = runtime
        .execute_ordinary_lane_frame(
            &ordinary_plan,
            WorthUiOrdinaryFrameTarget::registry_lookup_for_test(),
        )
        .expect_err("registry lookup path is denied");
    let artifact_denial = runtime
        .execute_ordinary_lane_frame(
            &ordinary_plan,
            WorthUiOrdinaryFrameTarget::artifact_scan_for_test(),
        )
        .expect_err("artifact scan path is denied");

    assert_eq!(
        parse_denial.reason(),
        WorthUiOrdinaryLaneFrameDenialReason::FramePathSourceParse
    );
    assert_eq!(
        lookup_denial.reason(),
        WorthUiOrdinaryLaneFrameDenialReason::FramePathRegistryLookup
    );
    assert_eq!(
        artifact_denial.reason(),
        WorthUiOrdinaryLaneFrameDenialReason::FramePathArtifactScan
    );
}

#[test]
fn ordinary_lane_counters_fail_when_widget_execution_scans_all_plan_nodes() {
    let (runtime, ordinary_plan, _) = ordinary_fixture();

    let denial = runtime
        .execute_ordinary_lane_frame(
            &ordinary_plan,
            WorthUiOrdinaryFrameTarget::full_plan_scan_for_test(),
        )
        .expect_err("full plan scan is not certified for frame execution");

    assert_eq!(
        denial.reason(),
        WorthUiOrdinaryLaneFrameDenialReason::FullPlanScanCertificationFailure
    );
    assert_eq!(denial.counters().full_plan_scan_count(), 1);
    assert_eq!(denial.counters().certification_failure_count(), 1);
    assert_eq!(denial.counters().denial_count(), 1);
}

#[test]
fn ordinary_plan_requires_admitted_support_for_each_included_surface() {
    assert_mismatched_admission_denies(
        WorthUiPlanNodeInputFamily::Command,
        WorthUiExecutionLane::CommandSurface,
        WorthUiOrdinaryLanePlanDenialReason::LaneAdmissionMissingCommandSurfaceSupport,
    );
    assert_mismatched_admission_denies(
        WorthUiPlanNodeInputFamily::TokenStyle,
        WorthUiExecutionLane::StyleToken,
        WorthUiOrdinaryLanePlanDenialReason::LaneAdmissionMissingStyleTokenSupport,
    );
}

fn assert_mismatched_admission_denies(
    removed_family: WorthUiPlanNodeInputFamily,
    removed_lane: WorthUiExecutionLane,
    expected_reason: WorthUiOrdinaryLanePlanDenialReason,
) {
    let denial = ordinary_lane_denial_for_missing_family(removed_family, removed_lane);

    assert_eq!(denial.reason(), expected_reason);
    assert_eq!(denial.counters().denial_count(), 1);
}

#[test]
fn ordinary_frame_rejects_stale_typed_handle_generation() {
    let (runtime, ordinary_plan, allocation) = ordinary_fixture();
    let fresh_handle = allocation
        .component_handles()
        .first()
        .copied()
        .expect("fixture has component handle");
    let stale_generation =
        WorthUiHandlePlanGeneration::new(fresh_handle.plan_generation().as_u64() ^ 0xfeed);
    let stale_handle = WorthUiComponentHandle::new(fresh_handle.plan_index(), stale_generation);

    let denial = runtime
        .execute_ordinary_lane_frame(
            &ordinary_plan,
            WorthUiOrdinaryFrameTarget::component(stale_handle),
        )
        .expect_err("stale component handle generation cannot execute");

    assert_eq!(
        denial.reason(),
        WorthUiOrdinaryLaneFrameDenialReason::TargetGenerationMismatch
    );
    assert_eq!(denial.plan_index(), Some(fresh_handle.plan_index()));
    assert_eq!(denial.counters().certification_failure_count(), 1);
}

fn assert_path_is_source_free(counters: crate::runtime::WorthUiOrdinaryLaneCounters) {
    assert_eq!(counters.source_parse_count(), 0);
    assert_eq!(counters.registry_lookup_count(), 0);
    assert_eq!(counters.artifact_tree_scan_count(), 0);
    assert_eq!(counters.component_string_resolution_count(), 0);
    assert_eq!(counters.command_string_resolution_count(), 0);
    assert_eq!(counters.full_plan_scan_count(), 0);
}

fn ordinary_fixture() -> (
    crate::runtime::WorthUiRuntimeHost,
    crate::runtime::WorthUiOrdinaryLanePlan,
    crate::runtime::WorthUiRuntimeHandleAllocation,
) {
    ordinary_lane_fixture()
}
