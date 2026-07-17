use crate::runtime::{
    WorthUiExecutionLane, WorthUiExecutionPlan, WorthUiLaneAdmission, WorthUiLaneSupportStatus,
    WorthUiOrdinaryExecutionLane, WorthUiOrdinaryLaneCounters, WorthUiOrdinaryLaneNode,
    WorthUiOrdinaryLanePlan, WorthUiOrdinaryLanePlanDenial, WorthUiOrdinaryLanePlanDenialReason,
    WorthUiPlanExecutionLane, WorthUiPlanNode, WorthUiPlanNodeInputFamily,
};
use std::collections::BTreeMap;

pub(crate) struct WorthUiOrdinaryLanePlanBuilder;

impl WorthUiOrdinaryLanePlanBuilder {
    pub(crate) fn build(
        execution_plan: &WorthUiExecutionPlan,
        lane_admission: &WorthUiLaneAdmission,
    ) -> Result<WorthUiOrdinaryLanePlan, WorthUiOrdinaryLanePlanDenial> {
        let mut counters = WorthUiOrdinaryLaneCounters::default();
        if !lane_is_supported(lane_admission, WorthUiExecutionLane::OrdinaryWidgetShell) {
            counters.record_denial();
            return Err(WorthUiOrdinaryLanePlanDenial::new(
                WorthUiOrdinaryLanePlanDenialReason::LaneAdmissionMissingOrdinarySupport,
                counters,
            ));
        }

        let mut rows = Vec::new();
        let mut component_plan_indexes = Vec::new();
        let mut command_plan_indexes = Vec::new();
        let mut token_plan_indexes = Vec::new();
        let lane_by_plan_index = lane_by_plan_index(execution_plan);

        for node in execution_plan.topology().traversal_order() {
            match ordinary_lane_for_node(&lane_by_plan_index, node) {
                Some(lane) => {
                    require_lane_support(lane, lane_admission, &mut counters)?;
                    counters.record_ordinary_plan_row();
                    remember_index(
                        node.family().input_family(),
                        node.runtime_handle().plan_index(),
                        &mut component_plan_indexes,
                        &mut command_plan_indexes,
                        &mut token_plan_indexes,
                    );
                    rows.push(WorthUiOrdinaryLaneNode::new(
                        node.runtime_handle(),
                        lane,
                        node.child_range(),
                        node.egui_boundary()
                            .map(|boundary| boundary.contacts().len())
                            .unwrap_or_default(),
                    ));
                }
                None => counters.record_skipped_nonordinary_plan_row(),
            }
        }

        rows.sort_by_key(WorthUiOrdinaryLaneNode::plan_index);
        component_plan_indexes.sort_unstable();
        command_plan_indexes.sort_unstable();
        token_plan_indexes.sort_unstable();

        if rows.is_empty() {
            counters.record_denial();
            return Err(WorthUiOrdinaryLanePlanDenial::new(
                WorthUiOrdinaryLanePlanDenialReason::NoOrdinaryRows,
                counters,
            ));
        }

        let digest = digest_ordinary_rows(execution_plan.handle_receipt().basis_digest(), &rows);
        Ok(WorthUiOrdinaryLanePlan::new(
            super::WorthUiOrdinaryLanePlanInput {
                handle_receipt: execution_plan.handle_receipt(),
                support_digest: lane_admission.support_digest(),
                ordinary_plan_digest: digest,
                rows,
                component_plan_indexes,
                command_plan_indexes,
                token_plan_indexes,
                counters,
            },
        ))
    }
}

fn ordinary_lane_for_node(
    lane_by_plan_index: &BTreeMap<u32, WorthUiPlanExecutionLane>,
    node: &WorthUiPlanNode,
) -> Option<WorthUiOrdinaryExecutionLane> {
    match lane_by_plan_index
        .get(&node.runtime_handle().plan_index())
        .copied()?
    {
        WorthUiPlanExecutionLane::UiStructure => match node.family().input_family() {
            WorthUiPlanNodeInputFamily::ComponentInvocation => {
                Some(WorthUiOrdinaryExecutionLane::WidgetShell)
            }
            WorthUiPlanNodeInputFamily::LayoutRegion => {
                Some(WorthUiOrdinaryExecutionLane::ShellRegion)
            }
            WorthUiPlanNodeInputFamily::ChildRange => {
                Some(WorthUiOrdinaryExecutionLane::ChildRangeTraversal)
            }
            _ => None,
        },
        WorthUiPlanExecutionLane::Command => Some(WorthUiOrdinaryExecutionLane::CommandSurface),
        WorthUiPlanExecutionLane::Style => Some(WorthUiOrdinaryExecutionLane::TokenStyleSupport),
        WorthUiPlanExecutionLane::EguiBoundary => {
            Some(WorthUiOrdinaryExecutionLane::EguiBoundarySupport)
        }
        WorthUiPlanExecutionLane::QueryView
        | WorthUiPlanExecutionLane::Diagnostics
        | WorthUiPlanExecutionLane::LaneBoundary
        | WorthUiPlanExecutionLane::RenderResource => None,
    }
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

fn require_lane_support(
    ordinary_lane: WorthUiOrdinaryExecutionLane,
    lane_admission: &WorthUiLaneAdmission,
    counters: &mut WorthUiOrdinaryLaneCounters,
) -> Result<(), WorthUiOrdinaryLanePlanDenial> {
    let required_lane = match ordinary_lane {
        WorthUiOrdinaryExecutionLane::WidgetShell
        | WorthUiOrdinaryExecutionLane::ShellRegion
        | WorthUiOrdinaryExecutionLane::ChildRangeTraversal => {
            WorthUiExecutionLane::OrdinaryWidgetShell
        }
        WorthUiOrdinaryExecutionLane::CommandSurface => WorthUiExecutionLane::CommandSurface,
        WorthUiOrdinaryExecutionLane::TokenStyleSupport => WorthUiExecutionLane::StyleToken,
        WorthUiOrdinaryExecutionLane::EguiBoundarySupport => WorthUiExecutionLane::EguiBoundary,
    };

    if lane_is_supported(lane_admission, required_lane) {
        return Ok(());
    }

    counters.record_denial();
    Err(WorthUiOrdinaryLanePlanDenial::new(
        missing_support_reason(required_lane),
        *counters,
    ))
}

fn lane_is_supported(lane_admission: &WorthUiLaneAdmission, lane: WorthUiExecutionLane) -> bool {
    lane_admission
        .posture_for(lane)
        .is_some_and(|row| row.status() == WorthUiLaneSupportStatus::Supported)
}

fn missing_support_reason(lane: WorthUiExecutionLane) -> WorthUiOrdinaryLanePlanDenialReason {
    match lane {
        WorthUiExecutionLane::OrdinaryWidgetShell => {
            WorthUiOrdinaryLanePlanDenialReason::LaneAdmissionMissingOrdinarySupport
        }
        WorthUiExecutionLane::CommandSurface => {
            WorthUiOrdinaryLanePlanDenialReason::LaneAdmissionMissingCommandSurfaceSupport
        }
        WorthUiExecutionLane::StyleToken => {
            WorthUiOrdinaryLanePlanDenialReason::LaneAdmissionMissingStyleTokenSupport
        }
        WorthUiExecutionLane::EguiBoundary => {
            WorthUiOrdinaryLanePlanDenialReason::LaneAdmissionMissingEguiBoundarySupport
        }
        _ => WorthUiOrdinaryLanePlanDenialReason::LaneAdmissionMissingOrdinarySupport,
    }
}

fn remember_index(
    family: WorthUiPlanNodeInputFamily,
    plan_index: u32,
    component_plan_indexes: &mut Vec<u32>,
    command_plan_indexes: &mut Vec<u32>,
    token_plan_indexes: &mut Vec<u32>,
) {
    match family {
        WorthUiPlanNodeInputFamily::ComponentInvocation => component_plan_indexes.push(plan_index),
        WorthUiPlanNodeInputFamily::Command => command_plan_indexes.push(plan_index),
        WorthUiPlanNodeInputFamily::TokenStyle => token_plan_indexes.push(plan_index),
        _ => {}
    }
}

fn digest_ordinary_rows(seed: u64, rows: &[WorthUiOrdinaryLaneNode]) -> u64 {
    rows.iter().fold(seed, |digest, row| {
        fold(
            fold(digest, row.plan_index().into()),
            row.lane().canonical_tag(),
        )
    })
}

fn fold(mut digest: u64, value: u64) -> u64 {
    digest ^= value;
    digest.wrapping_mul(0x100000001b3)
}
