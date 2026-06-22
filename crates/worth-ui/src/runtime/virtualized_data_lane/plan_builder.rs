use std::collections::BTreeMap;

use crate::runtime::{
    WorthUiExecutionLane, WorthUiExecutionPlan, WorthUiLaneAdmission, WorthUiLaneSupportStatus,
    WorthUiPlanExecutionLane, WorthUiPlanNode, WorthUiPlanNodeInputFamily,
    WorthUiQueryPatchPosture, WorthUiVirtualizedDataCounters, WorthUiVirtualizedDataNode,
    WorthUiVirtualizedDataPlan, WorthUiVirtualizedDataPlanDenial,
    WorthUiVirtualizedDataPlanDenialReason,
};

pub(crate) struct WorthUiVirtualizedDataPlanBuilder;

impl WorthUiVirtualizedDataPlanBuilder {
    pub(crate) fn build(
        execution_plan: &WorthUiExecutionPlan,
        lane_admission: &WorthUiLaneAdmission,
    ) -> Result<WorthUiVirtualizedDataPlan, WorthUiVirtualizedDataPlanDenial> {
        let mut counters = WorthUiVirtualizedDataCounters::default();
        require_lane_support(
            lane_admission,
            WorthUiExecutionLane::VirtualizedData,
            WorthUiVirtualizedDataPlanDenialReason::LaneAdmissionMissingVirtualizedDataSupport,
            &mut counters,
        )?;
        require_lane_support(
            lane_admission,
            WorthUiExecutionLane::QueryBound,
            WorthUiVirtualizedDataPlanDenialReason::LaneAdmissionMissingQuerySupport,
            &mut counters,
        )?;
        require_matching_lane_admission(execution_plan, lane_admission, &mut counters)?;

        let query_links_by_plan_index = query_links_by_plan_index(lane_admission);
        let lane_by_plan_index = lane_by_plan_index(execution_plan);
        let mut rows = Vec::new();
        let mut view_binding_plan_indexes = Vec::new();

        for node in execution_plan.topology().traversal_order() {
            let posture = query_links_by_plan_index
                .get(&node.runtime_handle().plan_index())
                .map(|links| WorthUiQueryPatchPosture::from_query_support_links(links));
            if let (true, Some(posture)) =
                (is_virtualized_data_node(&lane_by_plan_index, node), posture)
            {
                counters.record_data_plan_row();
                view_binding_plan_indexes.push(node.runtime_handle().plan_index());
                rows.push(WorthUiVirtualizedDataNode::new(
                    node.runtime_handle(),
                    posture,
                ));
            } else {
                counters.record_skipped_nondata_plan_row();
            }
        }

        rows.sort_by_key(WorthUiVirtualizedDataNode::plan_index);
        view_binding_plan_indexes.sort_unstable();

        if rows.is_empty() {
            counters.record_denial();
            return Err(WorthUiVirtualizedDataPlanDenial::new(
                WorthUiVirtualizedDataPlanDenialReason::NoVirtualizedDataRows,
                counters,
            ));
        }

        let digest = digest_data_rows(execution_plan.handle_receipt().basis_digest(), &rows);
        Ok(WorthUiVirtualizedDataPlan::new(
            execution_plan.handle_receipt(),
            lane_admission.support_digest(),
            digest,
            rows,
            view_binding_plan_indexes,
            counters,
        ))
    }
}

fn require_matching_lane_admission(
    execution_plan: &WorthUiExecutionPlan,
    lane_admission: &WorthUiLaneAdmission,
    counters: &mut WorthUiVirtualizedDataCounters,
) -> Result<(), WorthUiVirtualizedDataPlanDenial> {
    if lane_admission.plan_input_basis_digest() == execution_plan.handle_receipt().basis_digest() {
        return Ok(());
    }

    counters.record_certification_failure();
    Err(WorthUiVirtualizedDataPlanDenial::new(
        WorthUiVirtualizedDataPlanDenialReason::LaneAdmissionPlanMismatch,
        *counters,
    ))
}

fn is_virtualized_data_node(
    lane_by_plan_index: &BTreeMap<u32, WorthUiPlanExecutionLane>,
    node: &WorthUiPlanNode,
) -> bool {
    lane_by_plan_index
        .get(&node.runtime_handle().plan_index())
        .is_some_and(|lane| *lane == WorthUiPlanExecutionLane::QueryView)
        && node.family().input_family() == WorthUiPlanNodeInputFamily::QueryViewBinding
}

fn query_links_by_plan_index(
    lane_admission: &WorthUiLaneAdmission,
) -> BTreeMap<u32, &crate::runtime::WorthUiQueryLaneSupportLinks> {
    lane_admission
        .query_support_links()
        .iter()
        .map(|links| (links.plan_index(), links))
        .collect()
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
    lane_admission: &WorthUiLaneAdmission,
    lane: WorthUiExecutionLane,
    reason: WorthUiVirtualizedDataPlanDenialReason,
    counters: &mut WorthUiVirtualizedDataCounters,
) -> Result<(), WorthUiVirtualizedDataPlanDenial> {
    if lane_admission
        .posture_for(lane)
        .is_some_and(|row| row.status() == WorthUiLaneSupportStatus::Supported)
    {
        return Ok(());
    }

    counters.record_denial();
    Err(WorthUiVirtualizedDataPlanDenial::new(reason, *counters))
}

fn digest_data_rows(seed: u64, rows: &[WorthUiVirtualizedDataNode]) -> u64 {
    rows.iter().fold(seed, |digest, row| {
        fold_digest(
            fold_digest(digest, u64::from(row.plan_index())),
            row.query_patch_posture().canonical_digest(),
        )
    })
}

fn fold_digest(digest: u64, text: impl IntoDigestFold) -> u64 {
    text.fold_into(digest)
}

trait IntoDigestFold {
    fn fold_into(self, digest: u64) -> u64;
}

impl IntoDigestFold for u64 {
    fn fold_into(self, mut digest: u64) -> u64 {
        digest ^= self;
        digest.wrapping_mul(0x100000001b3)
    }
}

impl IntoDigestFold for &str {
    fn fold_into(self, digest: u64) -> u64 {
        self.as_bytes()
            .iter()
            .fold(digest, |digest, byte| u64::from(*byte).fold_into(digest))
    }
}
