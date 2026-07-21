use super::canonical_tag::{canonical_execution_lane_tag, canonical_plan_node_family_tag};
use super::hash_fold::WorthUiExecutionPlanHashFold;
use crate::runtime::{
    WorthUiExecutablePlanDecisionKind, WorthUiExecutionPlan, WorthUiExecutionPlanDigest,
    WorthUiExecutionPlanEquivalence, WorthUiExecutionPlanEquivalenceBasis,
    WorthUiExecutionPlanEquivalenceCounters,
};

pub(crate) struct WorthUiExecutionPlanDigestor;

impl WorthUiExecutionPlanDigestor {
    pub(crate) fn regional_digest(
        plan: &WorthUiExecutionPlan,
    ) -> (
        WorthUiExecutionPlanDigest,
        WorthUiExecutionPlanEquivalenceCounters,
    ) {
        let mut counters = WorthUiExecutionPlanEquivalenceCounters::default();
        counters.record_plan_digest();
        let mut digest = WorthUiExecutionPlanHashFold::new(0x8d96_f0fd_6c8e_1f75);
        digest.fold_len(plan.region_count());
        digest.fold_u64(plan.regional_semantic_digest());
        let raw = digest.finish();
        let basis = WorthUiExecutionPlanEquivalenceBasis::new(
            super::WorthUiExecutionPlanEquivalenceBasisInput {
                plan_node_count: plan.region_count(),
                child_range_count: plan
                    .regional_family_count(crate::runtime::WorthUiPlanNodeInputFamily::ChildRange),
                lane_partition_count: regional_lane_partition_count(plan),
                lookup_entry_count: regional_lookup_entry_count(plan),
                render_resource_ref_count: plan.regional_family_count(
                    crate::runtime::WorthUiPlanNodeInputFamily::RenderResourceRef,
                ),
                executable_shape_fingerprint: raw,
            },
        );
        (WorthUiExecutionPlanDigest::new(raw, basis), counters)
    }

    pub(crate) fn digest(
        plan: &WorthUiExecutionPlan,
    ) -> (
        WorthUiExecutionPlanDigest,
        WorthUiExecutionPlanEquivalenceCounters,
    ) {
        if !plan.has_reconstructive_flat_projection() {
            return Self::regional_digest(plan);
        }
        let mut counters = WorthUiExecutionPlanEquivalenceCounters::default();
        let raw = digest_plan(plan, &mut counters);
        let basis = WorthUiExecutionPlanEquivalenceBasis::new(
            super::WorthUiExecutionPlanEquivalenceBasisInput {
                plan_node_count: plan.topology().traversal_order().len(),
                child_range_count: plan.topology().child_ranges().len(),
                lane_partition_count: plan.lane_partitions().len(),
                lookup_entry_count: plan.lookup_index().entry_count(),
                render_resource_ref_count: counters.render_resource_digest_count(),
                executable_shape_fingerprint: raw,
            },
        );
        (WorthUiExecutionPlanDigest::new(raw, basis), counters)
    }

    pub(crate) fn compare(
        previous: &WorthUiExecutionPlan,
        next: &WorthUiExecutionPlan,
    ) -> WorthUiExecutionPlanEquivalence {
        let (previous_digest, previous_counters) = Self::regional_digest(previous);
        let (next_digest, next_counters) = Self::regional_digest(next);
        let mut counters = previous_counters.combine(next_counters);
        counters.record_equivalence_comparison();
        let (regions_match, region_counters) =
            previous.semantically_matches_executable_regions(next);
        counters.record_region_comparison(region_counters);
        let decision_kind = if previous_digest == next_digest && regions_match {
            WorthUiExecutablePlanDecisionKind::ExactSemanticNoOp
        } else {
            WorthUiExecutablePlanDecisionKind::RebuildRequired
        };
        WorthUiExecutionPlanEquivalence::new(previous_digest, next_digest, decision_kind, counters)
    }
}

fn regional_lane_partition_count(plan: &WorthUiExecutionPlan) -> usize {
    use crate::runtime::WorthUiPlanNodeInputFamily as Family;
    [
        plan.regional_family_count(Family::ComponentInvocation)
            + plan.regional_family_count(Family::LayoutRegion)
            + plan.regional_family_count(Family::ChildRange)
            + plan.regional_family_count(Family::StateSlot),
        plan.regional_family_count(Family::QueryViewBinding),
        plan.regional_family_count(Family::Command),
        plan.regional_family_count(Family::TokenStyle),
        plan.regional_family_count(Family::Accessibility)
            + plan.regional_family_count(Family::DiagnosticsRef),
        plan.regional_family_count(Family::LanePartitionRef),
        plan.regional_family_count(Family::RenderResourceRef),
        plan.regional_family_count(Family::CanvasSpatial),
        plan.regional_family_count(Family::RealtimeOverlay),
    ]
    .into_iter()
    .filter(|count| *count > 0)
    .count()
}

fn regional_lookup_entry_count(plan: &WorthUiExecutionPlan) -> usize {
    use crate::runtime::WorthUiPlanNodeInputFamily as Family;
    [
        Family::ComponentInvocation,
        Family::Command,
        Family::TokenStyle,
        Family::QueryViewBinding,
        Family::LanePartitionRef,
        Family::RenderResourceRef,
    ]
    .into_iter()
    .map(|family| plan.regional_family_count(family))
    .sum()
}

fn digest_plan(
    plan: &WorthUiExecutionPlan,
    counters: &mut WorthUiExecutionPlanEquivalenceCounters,
) -> u64 {
    counters.record_plan_digest();
    let mut digest = WorthUiExecutionPlanHashFold::new(0x8d96_f0fd_6c8e_1f75);
    digest.fold_len(plan.topology().traversal_order().len());
    for node in plan.topology().traversal_order() {
        counters.record_plan_node_digest();
        digest.fold_tag(0x10);
        digest.fold_u64(canonical_plan_node_family_tag(
            node.runtime_handle().family(),
        ));
        digest.fold_u64(u64::from(node.runtime_handle().plan_index()));
        digest.fold_u64(canonical_plan_node_family_tag(node.family().input_family()));
        match node.child_range() {
            Some(range) => digest_child_range(&mut digest, range, counters),
            None => digest.fold_tag(0x11),
        }
        match node.region_structure() {
            Some(structure) => {
                digest.fold_tag(0x12);
                digest.fold_bool(structure.structure_declared());
                digest.fold_len(structure.root_region_count());
                digest.fold_len(structure.region_count());
                digest.fold_len(structure.mount_count());
                digest.fold_len(structure.max_region_depth());
            }
            None => digest.fold_tag(0x13),
        }
        match node.render_resource_ref() {
            Some(resource_ref) => {
                counters.record_render_resource_digest();
                digest.fold_tag(0x16);
                digest.fold_u64(u64::from(resource_ref.owner_plan_index()));
            }
            None => digest.fold_tag(0x17),
        }
    }
    digest.fold_len(plan.topology().child_ranges().len());
    for range in plan.topology().child_ranges() {
        digest_child_range(&mut digest, *range, counters);
    }
    digest.fold_len(plan.lane_partitions().len());
    for partition in plan.lane_partitions() {
        counters.record_lane_partition_digest();
        digest.fold_tag(0x30);
        digest.fold_u64(canonical_execution_lane_tag(partition.lane()));
        digest.fold_len(partition.plan_indexes().len());
        for plan_index in partition.plan_indexes() {
            digest.fold_u64(u64::from(*plan_index));
        }
    }
    digest_lookup_index(plan, &mut digest, counters);
    digest.finish()
}

fn digest_child_range(
    digest: &mut WorthUiExecutionPlanHashFold,
    range: crate::runtime::WorthUiPlanChildRange,
    counters: &mut WorthUiExecutionPlanEquivalenceCounters,
) {
    counters.record_child_range_digest();
    digest.fold_tag(0x20);
    digest.fold_u64(u64::from(range.owner_plan_index()));
    digest.fold_u64(u64::from(range.start()));
    digest.fold_u64(u64::from(range.len()));
}

fn digest_lookup_index(
    plan: &WorthUiExecutionPlan,
    digest: &mut WorthUiExecutionPlanHashFold,
    counters: &mut WorthUiExecutionPlanEquivalenceCounters,
) {
    counters.record_lookup_index_digest();
    digest_index_list(digest, 0x40, plan.lookup_index().component_plan_indexes());
    digest_index_list(digest, 0x41, plan.lookup_index().command_plan_indexes());
    digest_index_list(digest, 0x42, plan.lookup_index().token_plan_indexes());
    digest_index_list(digest, 0x43, plan.lookup_index().query_plan_indexes());
    digest_index_list(digest, 0x44, plan.lookup_index().lane_plan_indexes());
    digest_index_list(
        digest,
        0x45,
        plan.lookup_index().render_resource_plan_indexes(),
    );
}

fn digest_index_list(digest: &mut WorthUiExecutionPlanHashFold, tag: u64, indexes: &[u32]) {
    digest.fold_tag(tag);
    digest.fold_len(indexes.len());
    for index in indexes {
        digest.fold_u64(u64::from(*index));
    }
}
