use crate::runtime::{
    WorthUiCanvasDrawHook, WorthUiCanvasSpatialCounters, WorthUiCanvasSpatialNode,
    WorthUiCanvasSpatialPlan, WorthUiCanvasSpatialPlanDenial, WorthUiCanvasSpatialPlanDenialReason,
    WorthUiExecutionLane, WorthUiExecutionPlan, WorthUiExtensionHookAdmission,
    WorthUiLaneAdapterHookKind, WorthUiLaneAdmission, WorthUiLaneHandle, WorthUiLaneSupportStatus,
    WorthUiPlanExecutionLane, WorthUiPlanNode, WorthUiPlanNodeInputFamily,
    WorthUiRuntimeHandleAllocation, WorthUiSpatialHitTestHook, WorthUiSpatialToolStateHook,
    WorthUiStateSlotHandle,
};
use std::collections::BTreeMap;

pub(crate) struct WorthUiCanvasSpatialPlanBuilder;

impl WorthUiCanvasSpatialPlanBuilder {
    pub(crate) fn build(
        execution_plan: &WorthUiExecutionPlan,
        handle_allocation: &WorthUiRuntimeHandleAllocation,
        lane_admission: &WorthUiLaneAdmission,
        hook_admissions: &[WorthUiExtensionHookAdmission],
    ) -> Result<WorthUiCanvasSpatialPlan, WorthUiCanvasSpatialPlanDenial> {
        let mut counters = WorthUiCanvasSpatialCounters::default();
        require_lane_support(lane_admission, &mut counters)?;
        require_matching_lane_admission(execution_plan, lane_admission, &mut counters)?;
        require_matching_handle_allocation(execution_plan, handle_allocation, &mut counters)?;
        let hooks = canvas_hooks(hook_admissions, &mut counters)?;

        let lane_by_plan_index = lane_by_plan_index(execution_plan);
        let mut rows = Vec::new();
        let mut command_plan_indexes = Vec::new();
        let mut diagnostics_plan_indexes = Vec::new();
        let selection_state_slot_handles =
            allocated_selection_state_slot_handles(execution_plan, handle_allocation);
        let render_resource_ref_count = render_resource_ref_count(execution_plan);

        for node in execution_plan.topology().traversal_order() {
            match node.family().input_family() {
                WorthUiPlanNodeInputFamily::LanePartitionRef
                    if is_lane_boundary_node(&lane_by_plan_index, node) =>
                {
                    counters.record_canvas_plan_row();
                    counters.record_renderer_reference(render_resource_ref_count);
                    rows.push(WorthUiCanvasSpatialNode::new(
                        node.runtime_handle(),
                        WorthUiLaneHandle::new(
                            node.runtime_handle().plan_index(),
                            node.runtime_handle().plan_generation(),
                        ),
                        render_resource_ref_count,
                    ));
                }
                WorthUiPlanNodeInputFamily::Command => {
                    command_plan_indexes.push(node.runtime_handle().plan_index());
                    counters.record_skipped_noncanvas_plan_row();
                }
                WorthUiPlanNodeInputFamily::DiagnosticsRef => {
                    diagnostics_plan_indexes.push(node.runtime_handle().plan_index());
                    counters.record_skipped_noncanvas_plan_row();
                }
                _ => counters.record_skipped_noncanvas_plan_row(),
            }
        }

        rows.sort_by_key(WorthUiCanvasSpatialNode::plan_index);
        command_plan_indexes.sort_unstable();
        diagnostics_plan_indexes.sort_unstable();

        if rows.is_empty() {
            counters.record_denial();
            return Err(WorthUiCanvasSpatialPlanDenial::new(
                WorthUiCanvasSpatialPlanDenialReason::NoCanvasSpatialRows,
                counters,
            ));
        }

        counters.record_command_identity_preservation(command_plan_indexes.len());
        counters.record_diagnostics_posture(diagnostics_plan_indexes.len());
        let digest = digest_canvas_rows(
            execution_plan.handle_receipt().basis_digest(),
            &rows,
            &selection_state_slot_handles,
        );

        Ok(WorthUiCanvasSpatialPlan::new(
            execution_plan.handle_receipt(),
            lane_admission.support_digest(),
            digest,
            rows,
            command_plan_indexes,
            diagnostics_plan_indexes,
            selection_state_slot_handles,
            hooks.draw_hooks,
            hooks.hit_test_hooks,
            hooks.tool_state_hooks,
            counters,
        ))
    }
}

struct CanvasHookFamilies {
    draw_hooks: Vec<WorthUiCanvasDrawHook>,
    hit_test_hooks: Vec<WorthUiSpatialHitTestHook>,
    tool_state_hooks: Vec<WorthUiSpatialToolStateHook>,
}

fn canvas_hooks(
    hook_admissions: &[WorthUiExtensionHookAdmission],
    counters: &mut WorthUiCanvasSpatialCounters,
) -> Result<CanvasHookFamilies, WorthUiCanvasSpatialPlanDenial> {
    let mut draw_hooks = Vec::new();
    let mut hit_test_hooks = Vec::new();
    let mut tool_state_hooks = Vec::new();

    for admission in hook_admissions {
        if admission.hook().kind() != WorthUiLaneAdapterHookKind::CanvasSpatialDrawAndHitTest {
            counters.record_denial();
            return Err(WorthUiCanvasSpatialPlanDenial::new(
                WorthUiCanvasSpatialPlanDenialReason::UnsupportedCanvasSpatialHook,
                *counters,
            ));
        }
        counters.record_admitted_hook_family();
        draw_hooks.push(WorthUiCanvasDrawHook::from_admission(admission));
        hit_test_hooks.push(WorthUiSpatialHitTestHook::from_admission(admission));
        tool_state_hooks.push(WorthUiSpatialToolStateHook::from_admission(admission));
    }

    if draw_hooks.is_empty() {
        counters.record_denial();
        return Err(WorthUiCanvasSpatialPlanDenial::new(
            WorthUiCanvasSpatialPlanDenialReason::MissingCanvasSpatialHook,
            *counters,
        ));
    }

    draw_hooks.sort();
    hit_test_hooks.sort();
    tool_state_hooks.sort();
    Ok(CanvasHookFamilies {
        draw_hooks,
        hit_test_hooks,
        tool_state_hooks,
    })
}

fn require_lane_support(
    lane_admission: &WorthUiLaneAdmission,
    counters: &mut WorthUiCanvasSpatialCounters,
) -> Result<(), WorthUiCanvasSpatialPlanDenial> {
    if lane_admission
        .posture_for(WorthUiExecutionLane::CanvasSpatial)
        .is_some_and(|row| row.status() == WorthUiLaneSupportStatus::Supported)
    {
        return Ok(());
    }

    counters.record_denial();
    Err(WorthUiCanvasSpatialPlanDenial::new(
        WorthUiCanvasSpatialPlanDenialReason::LaneAdmissionMissingCanvasSpatialSupport,
        *counters,
    ))
}

fn require_matching_lane_admission(
    execution_plan: &WorthUiExecutionPlan,
    lane_admission: &WorthUiLaneAdmission,
    counters: &mut WorthUiCanvasSpatialCounters,
) -> Result<(), WorthUiCanvasSpatialPlanDenial> {
    if lane_admission.plan_input_basis_digest() == execution_plan.handle_receipt().basis_digest() {
        return Ok(());
    }

    counters.record_certification_failure();
    Err(WorthUiCanvasSpatialPlanDenial::new(
        WorthUiCanvasSpatialPlanDenialReason::LaneAdmissionPlanMismatch,
        *counters,
    ))
}

fn require_matching_handle_allocation(
    execution_plan: &WorthUiExecutionPlan,
    handle_allocation: &WorthUiRuntimeHandleAllocation,
    counters: &mut WorthUiCanvasSpatialCounters,
) -> Result<(), WorthUiCanvasSpatialPlanDenial> {
    if handle_allocation.receipt() == execution_plan.handle_receipt() {
        return Ok(());
    }

    counters.record_certification_failure();
    Err(WorthUiCanvasSpatialPlanDenial::new(
        WorthUiCanvasSpatialPlanDenialReason::HandleAllocationPlanMismatch,
        *counters,
    ))
}

fn is_lane_boundary_node(
    lane_by_plan_index: &BTreeMap<u32, WorthUiPlanExecutionLane>,
    node: &WorthUiPlanNode,
) -> bool {
    lane_by_plan_index
        .get(&node.runtime_handle().plan_index())
        .is_some_and(|lane| *lane == WorthUiPlanExecutionLane::LaneBoundary)
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

fn render_resource_ref_count(execution_plan: &WorthUiExecutionPlan) -> usize {
    execution_plan
        .topology()
        .traversal_order()
        .iter()
        .filter(|node| {
            node.family().input_family() == WorthUiPlanNodeInputFamily::RenderResourceRef
                && node.render_resource_ref().is_some()
        })
        .count()
}

fn allocated_selection_state_slot_handles(
    execution_plan: &WorthUiExecutionPlan,
    handle_allocation: &WorthUiRuntimeHandleAllocation,
) -> Vec<WorthUiStateSlotHandle> {
    let family_by_plan_index = family_by_plan_index(execution_plan);
    let mut handles = handle_allocation
        .state_slot_handles()
        .iter()
        .copied()
        .filter(|handle| {
            family_by_plan_index
                .get(&handle.plan_index())
                .is_some_and(|family| selection_identity_family(*family))
        })
        .collect::<Vec<_>>();
    handles.sort_unstable();
    handles
}

fn family_by_plan_index(
    execution_plan: &WorthUiExecutionPlan,
) -> BTreeMap<u32, WorthUiPlanNodeInputFamily> {
    execution_plan
        .topology()
        .traversal_order()
        .iter()
        .map(|node| {
            (
                node.runtime_handle().plan_index(),
                node.family().input_family(),
            )
        })
        .collect()
}

fn selection_identity_family(family: WorthUiPlanNodeInputFamily) -> bool {
    matches!(
        family,
        WorthUiPlanNodeInputFamily::ComponentInvocation
            | WorthUiPlanNodeInputFamily::LayoutRegion
            | WorthUiPlanNodeInputFamily::TokenStyle
            | WorthUiPlanNodeInputFamily::ChildRange
    )
}

fn digest_canvas_rows(
    seed: u64,
    rows: &[WorthUiCanvasSpatialNode],
    selection_state_slot_handles: &[WorthUiStateSlotHandle],
) -> u64 {
    let digest = rows.iter().fold(seed, |digest, row| {
        fold(
            fold(digest, u64::from(row.plan_index())),
            row.render_resource_ref_count() as u64,
        )
    });
    selection_state_slot_handles
        .iter()
        .fold(digest, |digest, handle| {
            fold(
                fold(digest, u64::from(handle.plan_index())),
                handle.plan_generation().as_u64(),
            )
        })
}

fn fold(mut digest: u64, value: u64) -> u64 {
    digest ^= value;
    digest.wrapping_mul(0x100000001b3)
}
