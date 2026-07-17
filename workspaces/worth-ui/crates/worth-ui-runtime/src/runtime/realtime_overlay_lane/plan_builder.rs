use crate::runtime::{
    WorthUiExecutionLane, WorthUiExecutionPlan, WorthUiExtensionHookAdmission,
    WorthUiHighFrequencyFramePolicy, WorthUiHudNode, WorthUiHudPlan, WorthUiHudPlanDenial,
    WorthUiHudPlanDenialReason, WorthUiLaneAdapterHookKind, WorthUiLaneAdmission,
    WorthUiLaneSupportStatus, WorthUiPlanExecutionLane, WorthUiPlanNode,
    WorthUiPlanNodeInputFamily, WorthUiRealtimeLaneCounters, WorthUiRealtimeOverlayHook,
    WorthUiRendererSurfaceAdmission, WorthUiRuntimeHandleAllocation,
};
use std::collections::BTreeMap;

pub(crate) struct WorthUiHudPlanBuilder;

struct WorthUiHudPlanRows {
    rows: Vec<WorthUiHudNode>,
    renderer_surface_admissions: Vec<WorthUiRendererSurfaceAdmission>,
}

#[derive(Clone, Copy)]
struct WorthUiRealtimePostureCounts {
    command_identity_count: usize,
    accessibility_posture_count: usize,
    diagnostics_posture_count: usize,
}

impl WorthUiHudPlanBuilder {
    pub(crate) fn build(
        execution_plan: &WorthUiExecutionPlan,
        handle_allocation: &WorthUiRuntimeHandleAllocation,
        lane_admission: &WorthUiLaneAdmission,
        hook_admissions: &[WorthUiExtensionHookAdmission],
        frame_policy: WorthUiHighFrequencyFramePolicy,
    ) -> Result<WorthUiHudPlan, WorthUiHudPlanDenial> {
        let mut counters = WorthUiRealtimeLaneCounters::default();
        require_lane_support(
            lane_admission,
            WorthUiExecutionLane::RealtimeOverlayHud,
            WorthUiHudPlanDenialReason::LaneAdmissionMissingRealtimeSupport,
            &mut counters,
        )?;
        require_lane_support(
            lane_admission,
            WorthUiExecutionLane::RenderResource,
            WorthUiHudPlanDenialReason::LaneAdmissionMissingRenderResourceSupport,
            &mut counters,
        )?;
        require_matching_lane_admission(execution_plan, lane_admission, &mut counters)?;
        require_matching_handle_allocation(execution_plan, handle_allocation, &mut counters)?;
        let hooks = realtime_overlay_hooks(hook_admissions, &mut counters)?;

        let lane_by_plan_index = lane_by_plan_index(execution_plan);
        let command_plan_indexes = command_plan_indexes(execution_plan);
        let accessibility_plan_indexes = accessibility_plan_indexes(execution_plan);
        let diagnostics_plan_indexes = diagnostics_plan_indexes(execution_plan);
        let posture_counts = WorthUiRealtimePostureCounts {
            command_identity_count: command_plan_indexes.len(),
            accessibility_posture_count: accessibility_plan_indexes.len(),
            diagnostics_posture_count: diagnostics_plan_indexes.len(),
        };
        let hud_rows = hud_plan_rows(
            execution_plan,
            lane_admission,
            frame_policy,
            &lane_by_plan_index,
            posture_counts,
            &mut counters,
        );

        if hud_rows.rows.is_empty() {
            counters.record_denial();
            return Err(WorthUiHudPlanDenial::new(
                WorthUiHudPlanDenialReason::NoHudRows,
                counters,
            ));
        }

        counters.record_command_identity_preservation(command_plan_indexes.len());
        counters.record_accessibility_posture(accessibility_plan_indexes.len());
        counters.record_diagnostics_posture(diagnostics_plan_indexes.len());
        let hud_plan_digest = digest_hud_plan(
            execution_plan.handle_receipt().basis_digest(),
            frame_policy,
            &hud_rows.rows,
            &command_plan_indexes,
            &accessibility_plan_indexes,
            &diagnostics_plan_indexes,
            &hooks,
        );

        Ok(WorthUiHudPlan::new(super::WorthUiHudPlanInput {
            handle_receipt: execution_plan.handle_receipt(),
            support_digest: lane_admission.support_digest(),
            hud_plan_digest,
            frame_policy,
            rows: hud_rows.rows,
            renderer_surface_admissions: hud_rows.renderer_surface_admissions,
            command_plan_indexes,
            accessibility_plan_indexes,
            diagnostics_plan_indexes,
            overlay_hooks: hooks,
            counters,
        }))
    }
}

fn hud_plan_rows(
    execution_plan: &WorthUiExecutionPlan,
    lane_admission: &WorthUiLaneAdmission,
    frame_policy: WorthUiHighFrequencyFramePolicy,
    lane_by_plan_index: &BTreeMap<u32, WorthUiPlanExecutionLane>,
    posture_counts: WorthUiRealtimePostureCounts,
    counters: &mut WorthUiRealtimeLaneCounters,
) -> WorthUiHudPlanRows {
    let mut rows = Vec::new();

    for node in execution_plan.topology().traversal_order() {
        if is_realtime_render_resource_node(lane_by_plan_index, node) {
            counters.record_hud_plan_row();
            counters.record_renderer_surface_admission();
            rows.push(hud_node_for_render_resource(
                node,
                lane_admission,
                frame_policy,
                posture_counts,
            ));
        } else {
            counters.record_skipped_nonrealtime_plan_row();
        }
    }

    rows.sort_by_key(WorthUiHudNode::plan_index);
    let renderer_surface_admissions = rows
        .iter()
        .map(WorthUiHudNode::renderer_surface_admission)
        .collect::<Vec<_>>();
    WorthUiHudPlanRows {
        rows,
        renderer_surface_admissions,
    }
}

fn hud_node_for_render_resource(
    node: &WorthUiPlanNode,
    lane_admission: &WorthUiLaneAdmission,
    frame_policy: WorthUiHighFrequencyFramePolicy,
    posture_counts: WorthUiRealtimePostureCounts,
) -> WorthUiHudNode {
    WorthUiHudNode::new(
        node.runtime_handle(),
        WorthUiRendererSurfaceAdmission::new(
            node.runtime_handle(),
            frame_policy,
            lane_admission.support_digest(),
            posture_counts.command_identity_count,
            posture_counts.accessibility_posture_count,
            posture_counts.diagnostics_posture_count,
        ),
    )
}

fn realtime_overlay_hooks(
    hook_admissions: &[WorthUiExtensionHookAdmission],
    counters: &mut WorthUiRealtimeLaneCounters,
) -> Result<Vec<WorthUiRealtimeOverlayHook>, WorthUiHudPlanDenial> {
    let mut hooks = Vec::new();
    for admission in hook_admissions {
        if admission.hook().kind() != WorthUiLaneAdapterHookKind::RealtimeOverlayMechanics {
            counters.record_denial();
            return Err(WorthUiHudPlanDenial::new(
                WorthUiHudPlanDenialReason::UnsupportedRealtimeOverlayHook,
                *counters,
            ));
        }
        counters.record_overlay_hook();
        hooks.push(WorthUiRealtimeOverlayHook::from_admission(admission));
    }

    if hooks.is_empty() {
        counters.record_denial();
        return Err(WorthUiHudPlanDenial::new(
            WorthUiHudPlanDenialReason::MissingRealtimeOverlayHook,
            *counters,
        ));
    }

    hooks.sort();
    Ok(hooks)
}

fn require_matching_lane_admission(
    execution_plan: &WorthUiExecutionPlan,
    lane_admission: &WorthUiLaneAdmission,
    counters: &mut WorthUiRealtimeLaneCounters,
) -> Result<(), WorthUiHudPlanDenial> {
    if lane_admission.plan_input_basis_digest() == execution_plan.handle_receipt().basis_digest() {
        return Ok(());
    }

    counters.record_certification_failure();
    Err(WorthUiHudPlanDenial::new(
        WorthUiHudPlanDenialReason::LaneAdmissionPlanMismatch,
        *counters,
    ))
}

fn require_matching_handle_allocation(
    execution_plan: &WorthUiExecutionPlan,
    handle_allocation: &WorthUiRuntimeHandleAllocation,
    counters: &mut WorthUiRealtimeLaneCounters,
) -> Result<(), WorthUiHudPlanDenial> {
    if handle_allocation.receipt() == execution_plan.handle_receipt() {
        return Ok(());
    }

    counters.record_certification_failure();
    Err(WorthUiHudPlanDenial::new(
        WorthUiHudPlanDenialReason::HandleAllocationPlanMismatch,
        *counters,
    ))
}

fn require_lane_support(
    lane_admission: &WorthUiLaneAdmission,
    lane: WorthUiExecutionLane,
    reason: WorthUiHudPlanDenialReason,
    counters: &mut WorthUiRealtimeLaneCounters,
) -> Result<(), WorthUiHudPlanDenial> {
    if lane_admission
        .posture_for(lane)
        .is_some_and(|row| row.status() == WorthUiLaneSupportStatus::Supported)
    {
        return Ok(());
    }

    counters.record_denial();
    Err(WorthUiHudPlanDenial::new(reason, *counters))
}

fn is_realtime_render_resource_node(
    lane_by_plan_index: &BTreeMap<u32, WorthUiPlanExecutionLane>,
    node: &WorthUiPlanNode,
) -> bool {
    lane_by_plan_index
        .get(&node.runtime_handle().plan_index())
        .is_some_and(|lane| *lane == WorthUiPlanExecutionLane::RenderResource)
}

fn lane_by_plan_index(
    execution_plan: &WorthUiExecutionPlan,
) -> BTreeMap<u32, WorthUiPlanExecutionLane> {
    let mut lane_by_plan_index = BTreeMap::new();
    for partition in execution_plan.lane_partitions() {
        for plan_index in partition.plan_indexes() {
            lane_by_plan_index.insert(*plan_index, partition.lane());
        }
    }
    lane_by_plan_index
}

fn command_plan_indexes(execution_plan: &WorthUiExecutionPlan) -> Vec<u32> {
    plan_indexes_for_family(execution_plan, WorthUiPlanNodeInputFamily::Command)
}

fn accessibility_plan_indexes(execution_plan: &WorthUiExecutionPlan) -> Vec<u32> {
    plan_indexes_for_family(execution_plan, WorthUiPlanNodeInputFamily::Accessibility)
}

fn diagnostics_plan_indexes(execution_plan: &WorthUiExecutionPlan) -> Vec<u32> {
    plan_indexes_for_family(execution_plan, WorthUiPlanNodeInputFamily::DiagnosticsRef)
}

fn plan_indexes_for_family(
    execution_plan: &WorthUiExecutionPlan,
    family: WorthUiPlanNodeInputFamily,
) -> Vec<u32> {
    let mut plan_indexes = execution_plan
        .topology()
        .traversal_order()
        .iter()
        .filter(|node| node.family().input_family() == family)
        .map(|node| node.runtime_handle().plan_index())
        .collect::<Vec<_>>();
    plan_indexes.sort_unstable();
    plan_indexes
}

fn digest_hud_plan(
    seed: u64,
    frame_policy: WorthUiHighFrequencyFramePolicy,
    rows: &[WorthUiHudNode],
    command_plan_indexes: &[u32],
    accessibility_plan_indexes: &[u32],
    diagnostics_plan_indexes: &[u32],
    hooks: &[WorthUiRealtimeOverlayHook],
) -> u64 {
    let digest = fold(seed, frame_policy.canonical_digest());
    let digest = rows.iter().fold(digest, |digest, row| {
        let admission = row.renderer_surface_admission();
        fold(
            fold(digest, u64::from(row.plan_index())),
            admission.policy_digest(),
        )
    });
    let digest = command_plan_indexes
        .iter()
        .fold(digest, |digest, plan_index| {
            fold(digest, u64::from(*plan_index))
        });
    let digest = accessibility_plan_indexes
        .iter()
        .fold(digest, |digest, plan_index| {
            fold(digest, u64::from(*plan_index))
        });
    let digest = diagnostics_plan_indexes
        .iter()
        .fold(digest, |digest, plan_index| {
            fold(digest, u64::from(*plan_index))
        });
    hooks
        .iter()
        .fold(digest, |digest, hook| fold(digest, hook.canonical_digest()))
}

fn fold(mut digest: u64, value: u64) -> u64 {
    digest ^= value;
    digest.wrapping_mul(0x100000001b3)
}
