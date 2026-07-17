use super::canonical_tag::{
    canonical_egui_boundary_contact_tag, canonical_egui_boundary_input_tag,
    canonical_execution_lane_tag, canonical_plan_node_family_tag,
};
use super::hash_fold::WorthUiExecutionPlanHashFold;
use crate::runtime::{
    WorthUiExecutionPlan, WorthUiExecutionPlanDigest, WorthUiExecutionPlanEquivalence,
    WorthUiExecutionPlanEquivalenceBasis, WorthUiExecutionPlanEquivalenceCounters,
    WorthUiPlanReuseClassification,
};

pub(crate) struct WorthUiExecutionPlanDigestor;

impl WorthUiExecutionPlanDigestor {
    pub(crate) fn digest(
        plan: &WorthUiExecutionPlan,
    ) -> (
        WorthUiExecutionPlanDigest,
        WorthUiExecutionPlanEquivalenceCounters,
    ) {
        let mut counters = WorthUiExecutionPlanEquivalenceCounters::default();
        let raw = digest_plan(plan, &mut counters);
        let basis = WorthUiExecutionPlanEquivalenceBasis::new(
            super::WorthUiExecutionPlanEquivalenceBasisInput {
                handle_receipt: plan.handle_receipt(),
                plan_node_count: plan.topology().traversal_order().len(),
                child_range_count: plan.topology().child_ranges().len(),
                lane_partition_count: plan.lane_partitions().len(),
                lookup_entry_count: plan.lookup_index().entry_count(),
                egui_boundary_count: counters.egui_boundary_digest_count(),
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
        let (previous_digest, previous_counters) = Self::digest(previous);
        let (next_digest, next_counters) = Self::digest(next);
        let mut counters = previous_counters.combine(next_counters);
        counters.record_equivalence_comparison();
        let reuse_classification = if previous_digest == next_digest {
            WorthUiPlanReuseClassification::Reusable
        } else {
            WorthUiPlanReuseClassification::RebuildRequired
        };
        WorthUiExecutionPlanEquivalence::new(
            previous_digest,
            next_digest,
            reuse_classification,
            counters,
        )
    }
}

fn digest_plan(
    plan: &WorthUiExecutionPlan,
    counters: &mut WorthUiExecutionPlanEquivalenceCounters,
) -> u64 {
    counters.record_plan_digest();
    let mut digest = WorthUiExecutionPlanHashFold::new(0x8d96_f0fd_6c8e_1f75);
    digest.fold_u64(plan.handle_receipt().basis_digest());
    digest.fold_u64(plan.handle_receipt().plan_generation().as_u64());
    digest.fold_len(plan.topology().traversal_order().len());
    for node in plan.topology().traversal_order() {
        counters.record_plan_node_digest();
        digest.fold_tag(0x10);
        digest.fold_u64(canonical_plan_node_family_tag(
            node.runtime_handle().family(),
        ));
        digest.fold_u64(u64::from(node.runtime_handle().plan_index()));
        digest.fold_u64(node.runtime_handle().plan_generation().as_u64());
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
        match node.egui_boundary() {
            Some(boundary) => {
                counters.record_egui_boundary_digest();
                digest.fold_tag(0x14);
                digest.fold_u64(canonical_egui_boundary_input_tag(boundary.input()));
                digest.fold_u64(u64::from(boundary.owner_plan_index()));
                digest.fold_len(boundary.contacts().len());
                for contact in boundary.contacts() {
                    digest.fold_u64(canonical_egui_boundary_contact_tag(*contact));
                }
            }
            None => digest.fold_tag(0x15),
        }
        match node.render_resource_ref() {
            Some(resource_ref) => {
                counters.record_render_resource_digest();
                digest.fold_tag(0x16);
                digest.fold_u64(u64::from(resource_ref.owner_plan_index()));
                digest.fold_u64(resource_ref.plan_generation().as_u64());
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
