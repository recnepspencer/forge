use super::lane_admission_fixture::{admit_lanes, lane_fixture, spoofed_query_lane_fixture};
use crate::runtime::{
    WorthUiExecutionLane, WorthUiExecutionLaneSupport, WorthUiLaneAdapterHook,
    WorthUiLaneAdapterHookKind, WorthUiLaneAdmissionDenialReason, WorthUiLaneCostRegime,
    WorthUiLaneFailureMode, WorthUiPlanTopologyDenialReason, WorthUiUnsupportedHookDenialReason,
};

#[test]
fn equivalent_lane_descriptors_produce_equivalent_lane_support() {
    let (runtime, _plan_input, planning, _) = lane_fixture();
    let support = WorthUiExecutionLaneSupport::platform_default();
    let permuted_support = crate::runtime::WorthUiExecutionLaneSupport::from_supported_lanes([
        WorthUiExecutionLane::SpecialCaseExtension,
        WorthUiExecutionLane::RenderResource,
        WorthUiExecutionLane::EguiBoundary,
        WorthUiExecutionLane::LaneBoundary,
        WorthUiExecutionLane::DiagnosticsProjection,
        WorthUiExecutionLane::StyleToken,
        WorthUiExecutionLane::CommandSurface,
        WorthUiExecutionLane::QueryBound,
        WorthUiExecutionLane::RealtimeOverlayHud,
        WorthUiExecutionLane::CanvasSpatial,
        WorthUiExecutionLane::VirtualizedData,
        WorthUiExecutionLane::OrdinaryWidgetShell,
    ]);

    let left = admit_lanes(&runtime, &planning, &support);
    let right = admit_lanes(&runtime, &planning, &permuted_support);

    assert_eq!(left.support_digest(), right.support_digest());
    assert_eq!(left.rows(), right.rows());
    assert_eq!(left.counters().raw_lane_string_lookup_count(), 0);
    assert_eq!(left.counters().broad_support_scan_count(), 0);
}

#[test]
fn unsupported_lane_reference_rejected_before_plan_activation() {
    let (runtime, plan_input, planning, allocation) = lane_fixture();
    let support_without_query =
        WorthUiExecutionLaneSupport::without_lane_for_test(WorthUiExecutionLane::QueryBound);

    let denial = runtime
        .admit_execution_lanes(
            &runtime.detached_allocation_lowering_input_for_test(&planning),
            &support_without_query,
        )
        .expect_err("unsupported lane denies before topology activation");

    assert_eq!(
        denial.reason(),
        WorthUiLaneAdmissionDenialReason::UnsupportedLaneReference
    );
    assert_eq!(denial.lane(), Some(WorthUiExecutionLane::QueryBound));
    assert_eq!(denial.counters().unsupported_lane_denial_count(), 1);
    assert_eq!(denial.counters().topology_node_construction_count(), 0);
    let diagnostic = denial
        .diagnostic()
        .expect("unsupported QueryBound support row preserves diagnostic posture");
    assert_eq!(diagnostic.lane(), WorthUiExecutionLane::QueryBound);
    assert_eq!(
        diagnostic.failure_mode(),
        WorthUiLaneFailureMode::QuerySupportDenial
    );

    let admission = admit_lanes(
        &runtime,
        &planning,
        &WorthUiExecutionLaneSupport::platform_default(),
    );
    let plan = runtime
        .assemble_execution_plan_topology_with_lane_admission(
            &runtime.detached_allocation_lowering_input_for_test(&planning),
            &allocation,
            &admission,
        )
        .expect("admitted lanes allow topology activation");
    assert_eq!(
        plan.topology().traversal_order().len(),
        plan_input.node_inputs().len()
    );
}

#[test]
fn query_lane_node_without_query_owned_support_link_denies() {
    let (runtime, _broken_input, broken_planning) = spoofed_query_lane_fixture();

    let denial = runtime
        .admit_execution_lanes(
            &runtime.detached_allocation_lowering_input_for_test(&broken_planning),
            &WorthUiExecutionLaneSupport::platform_default(),
        )
        .expect_err("query-shaped node without Query support link denies");

    assert_eq!(
        denial.reason(),
        WorthUiLaneAdmissionDenialReason::MissingQuerySupportLinks
    );
    assert_eq!(denial.lane(), Some(WorthUiExecutionLane::QueryBound));
    assert_eq!(denial.counters().topology_node_construction_count(), 0);
    assert_eq!(denial.counters().query_posture_reauthoring_count(), 0);
}

#[test]
fn topology_convenience_path_requires_lane_admission() {
    let (runtime, _broken_input, broken_planning) = spoofed_query_lane_fixture();
    let allocation = runtime
        .allocate_runtime_handles(&runtime.detached_allocation_receipt_for_test(&broken_planning))
        .expect("handle allocation still sees typed plan families");

    let denial = runtime
        .assemble_execution_plan_topology(
            &runtime.detached_allocation_lowering_input_for_test(&broken_planning),
            &allocation,
        )
        .expect_err("public topology assembly cannot bypass lane admission");

    assert_eq!(
        denial.reason(),
        WorthUiPlanTopologyDenialReason::LaneAdmissionMismatch
    );
    assert_eq!(denial.counters().topology_node_count(), 0);
}

#[test]
fn lane_taxonomy_distinguishes_cost_and_failure_modes() {
    let support = WorthUiExecutionLaneSupport::platform_default();
    let ordinary = support
        .row_for_lane(WorthUiExecutionLane::OrdinaryWidgetShell)
        .expect("ordinary row exists");
    let virtualized = support
        .row_for_lane(WorthUiExecutionLane::VirtualizedData)
        .expect("virtualized row exists");
    let canvas = support
        .row_for_lane(WorthUiExecutionLane::CanvasSpatial)
        .expect("canvas row exists");
    let realtime = support
        .row_for_lane(WorthUiExecutionLane::RealtimeOverlayHud)
        .expect("realtime row exists");

    assert_eq!(
        ordinary.cost_regime(),
        WorthUiLaneCostRegime::LocalTraversal
    );
    assert_eq!(
        virtualized.cost_regime(),
        WorthUiLaneCostRegime::WindowedTraversal
    );
    assert_eq!(
        canvas.cost_regime(),
        WorthUiLaneCostRegime::SpatialIndexTraversal
    );
    assert_eq!(
        realtime.cost_regime(),
        WorthUiLaneCostRegime::FrameSynchronizedTraversal
    );
    assert_eq!(
        ordinary.failure_mode(),
        WorthUiLaneFailureMode::LocalWidgetFailure
    );
    assert_eq!(
        virtualized.failure_mode(),
        WorthUiLaneFailureMode::WindowInvalidationFailure
    );
    assert_eq!(
        canvas.failure_mode(),
        WorthUiLaneFailureMode::SpatialHitTestFailure
    );
    assert_eq!(
        realtime.failure_mode(),
        WorthUiLaneFailureMode::RealtimeFrameMiss
    );
}

#[test]
fn query_bound_support_row_preserves_query_bound_posture() {
    let support = WorthUiExecutionLaneSupport::platform_default();
    let query_row = support
        .row_for_lane(WorthUiExecutionLane::QueryBound)
        .expect("query support row exists");

    assert!(query_row.descriptor().is_query_bound());
    assert_eq!(
        query_row.cost_regime(),
        WorthUiLaneCostRegime::QueryRuntimeBacked
    );
    assert_eq!(
        query_row.failure_mode(),
        WorthUiLaneFailureMode::QuerySupportDenial
    );
}

#[test]
fn private_component_lane_claim_rejected_without_lane_support() {
    let (runtime, _plan_input, planning, _) = lane_fixture();
    let admission = admit_lanes(
        &runtime,
        &planning,
        &WorthUiExecutionLaneSupport::platform_default(),
    );
    let private_hook = WorthUiLaneAdapterHook::forbidden_for_test(
        "component.private_lane",
        WorthUiLaneAdapterHookKind::PrivateLaneClaim,
    );

    let denial = runtime
        .admit_extension_hook(&admission, private_hook)
        .expect_err("private lane hook claim denies");

    assert_eq!(
        denial.reason(),
        WorthUiUnsupportedHookDenialReason::PrivateLaneClaim
    );
    assert_eq!(denial.counters().private_lane_claim_denial_count(), 1);
    assert_eq!(denial.counters().forbidden_hook_count(), 1);
}

#[test]
fn all_supported_hook_points_have_typed_admission_constructors() {
    let (runtime, _plan_input, planning, _) = lane_fixture();
    let admission = admit_lanes(
        &runtime,
        &planning,
        &WorthUiExecutionLaneSupport::platform_default(),
    );
    let hooks = [
        WorthUiLaneAdapterHook::source_ingress("hook.source"),
        WorthUiLaneAdapterHook::debounce_policy("hook.debounce"),
        WorthUiLaneAdapterHook::identity_seed_contribution("hook.identity_seed"),
        WorthUiLaneAdapterHook::durable_state_family_admission("hook.state_family"),
        WorthUiLaneAdapterHook::component_lowering("hook.component_lowering"),
        WorthUiLaneAdapterHook::lane_adapter_mechanics(
            "hook.virtualized",
            WorthUiExecutionLane::VirtualizedData,
        ),
        WorthUiLaneAdapterHook::canvas_spatial_draw_and_hit_test("hook.canvas"),
        WorthUiLaneAdapterHook::realtime_overlay_mechanics("hook.realtime"),
        WorthUiLaneAdapterHook::diagnostics_projection("hook.diagnostics"),
        WorthUiLaneAdapterHook::counter_families("hook.counters"),
        WorthUiLaneAdapterHook::report_materialization("hook.reports"),
    ];

    for hook in hooks {
        let admitted = runtime
            .admit_extension_hook(&admission, hook)
            .expect("supported hook admits through typed constructor");
        assert_eq!(admitted.counters().forbidden_hook_count(), 0);
        assert_eq!(admitted.counters().hook_admission_count(), 1);
    }
}

#[test]
fn hook_admission_rejects_active_plan_or_query_authority_override() {
    let (runtime, _plan_input, planning, _) = lane_fixture();
    let admission = admit_lanes(
        &runtime,
        &planning,
        &WorthUiExecutionLaneSupport::platform_default(),
    );
    let forbidden = [
        (
            WorthUiLaneAdapterHookKind::ActivePlanTruthOverride,
            WorthUiUnsupportedHookDenialReason::ActivePlanTruthOverride,
        ),
        (
            WorthUiLaneAdapterHookKind::QueryPostureOverride,
            WorthUiUnsupportedHookDenialReason::QueryPostureOverride,
        ),
        (
            WorthUiLaneAdapterHookKind::StateCarryForwardOverride,
            WorthUiUnsupportedHookDenialReason::StateCarryForwardOverride,
        ),
        (
            WorthUiLaneAdapterHookKind::LaneTaxonomyOverride,
            WorthUiUnsupportedHookDenialReason::LaneTaxonomyOverride,
        ),
        (
            WorthUiLaneAdapterHookKind::PerformanceCertificationOverride,
            WorthUiUnsupportedHookDenialReason::PerformanceCertificationOverride,
        ),
    ];

    for (kind, expected_reason) in forbidden {
        let hook = WorthUiLaneAdapterHook::forbidden_for_test("forbidden", kind);
        let denial = runtime
            .admit_extension_hook(&admission, hook)
            .expect_err("authority override hook denies");
        assert_eq!(denial.reason(), expected_reason);
        assert_eq!(denial.counters().forbidden_hook_count(), 1);
        assert_eq!(denial.counters().hook_admission_count(), 0);
    }
}

#[test]
fn admitted_lane_adapter_hook_preserves_lane_counter_contract() {
    let (runtime, _plan_input, planning, _) = lane_fixture();
    let support = WorthUiExecutionLaneSupport::platform_default();
    let admission = admit_lanes(&runtime, &planning, &support);
    let hook = WorthUiLaneAdapterHook::canvas_spatial_draw_and_hit_test("canvas.draw.hit_test");

    let hook_admission = runtime
        .admit_extension_hook(&admission, hook)
        .expect("supported lane adapter hook admits");
    let expected_row = support
        .row_for_lane(WorthUiExecutionLane::CanvasSpatial)
        .expect("canvas support row exists");

    assert_eq!(hook_admission.preserved_lane_support(), expected_row);
    assert_eq!(hook_admission.counters().hook_admission_count(), 1);
    assert_eq!(hook_admission.counters().forbidden_hook_count(), 0);
    assert_eq!(hook_admission.counters().raw_lane_string_lookup_count(), 0);
    assert_eq!(
        hook_admission.counters().query_posture_reauthoring_count(),
        0
    );
}

#[test]
fn query_bound_lane_support_links_are_preserved_not_reauthored() {
    let (runtime, plan_input, planning, _) = lane_fixture();
    let admission = admit_lanes(
        &runtime,
        &planning,
        &WorthUiExecutionLaneSupport::platform_default(),
    );
    let query_inputs = plan_input
        .node_inputs()
        .iter()
        .filter(|input| input.query_binding_posture().is_some())
        .collect::<Vec<_>>();

    assert!(!query_inputs.is_empty());
    assert_eq!(admission.query_support_links().len(), query_inputs.len());
    for (links, input) in admission
        .query_support_links()
        .iter()
        .zip(query_inputs.iter().copied())
    {
        let posture = input
            .query_binding_posture()
            .expect("query support link comes from Query posture");
        assert_eq!(links.posture(), posture);
        assert_eq!(
            links.binding_identity(),
            input
                .query_binding_identity()
                .expect("query support link carries typed binding identity")
        );
        assert_eq!(links.required_surfaces(), input.query_required_surfaces());
    }
    assert_eq!(
        admission.counters().query_support_link_count(),
        query_inputs.len()
    );
    assert_eq!(admission.counters().query_posture_reauthoring_count(), 0);
}
