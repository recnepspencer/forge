#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiExecutionPlanEquivalenceCounters {
    plan_digest_count: usize,
    plan_node_digest_count: usize,
    child_range_digest_count: usize,
    lane_partition_digest_count: usize,
    lookup_index_digest_count: usize,
    render_resource_digest_count: usize,
    equivalence_comparison_count: usize,
    region_exact_comparison_count: usize,
    region_fingerprint_rejection_count: usize,
    artifact_tree_scan_count: usize,
    pointer_identity_comparison_count: usize,
    diagnostic_policy_read_count: usize,
}

impl WorthUiExecutionPlanEquivalenceCounters {
    pub(crate) fn for_reload_receipt(summary: super::WorthUiPlanEquivalenceSummary) -> Self {
        Self {
            plan_digest_count: 1,
            equivalence_comparison_count: 1,
            region_exact_comparison_count: summary.exact_region_comparison_count(),
            ..Self::default()
        }
    }
    pub(crate) fn record_plan_digest(&mut self) {
        self.plan_digest_count += 1;
    }

    pub(crate) fn record_plan_node_digest(&mut self) {
        self.plan_node_digest_count += 1;
    }

    pub(crate) fn record_child_range_digest(&mut self) {
        self.child_range_digest_count += 1;
    }

    pub(crate) fn record_lane_partition_digest(&mut self) {
        self.lane_partition_digest_count += 1;
    }

    pub(crate) fn record_lookup_index_digest(&mut self) {
        self.lookup_index_digest_count += 1;
    }

    pub(crate) fn record_render_resource_digest(&mut self) {
        self.render_resource_digest_count += 1;
    }

    pub(crate) fn record_equivalence_comparison(&mut self) {
        self.equivalence_comparison_count += 1;
    }

    pub(crate) fn record_region_comparison(
        &mut self,
        counters: crate::runtime::plan_topology::WorthUiPlanRegionStorageCounters,
    ) {
        self.region_exact_comparison_count += counters.exact_comparison_count();
        self.region_fingerprint_rejection_count += counters.fingerprint_rejection_count();
    }

    pub(crate) fn combine(mut self, other: Self) -> Self {
        self.plan_digest_count += other.plan_digest_count;
        self.plan_node_digest_count += other.plan_node_digest_count;
        self.child_range_digest_count += other.child_range_digest_count;
        self.lane_partition_digest_count += other.lane_partition_digest_count;
        self.lookup_index_digest_count += other.lookup_index_digest_count;
        self.render_resource_digest_count += other.render_resource_digest_count;
        self.equivalence_comparison_count += other.equivalence_comparison_count;
        self.region_exact_comparison_count += other.region_exact_comparison_count;
        self.region_fingerprint_rejection_count += other.region_fingerprint_rejection_count;
        self
    }

    pub fn plan_digest_count(self) -> usize {
        self.plan_digest_count
    }

    pub fn plan_node_digest_count(self) -> usize {
        self.plan_node_digest_count
    }

    pub fn child_range_digest_count(self) -> usize {
        self.child_range_digest_count
    }

    pub fn lane_partition_digest_count(self) -> usize {
        self.lane_partition_digest_count
    }

    pub fn lookup_index_digest_count(self) -> usize {
        self.lookup_index_digest_count
    }

    pub fn render_resource_digest_count(self) -> usize {
        self.render_resource_digest_count
    }

    pub fn equivalence_comparison_count(self) -> usize {
        self.equivalence_comparison_count
    }

    pub fn region_exact_comparison_count(self) -> usize {
        self.region_exact_comparison_count
    }

    pub fn region_fingerprint_rejection_count(self) -> usize {
        self.region_fingerprint_rejection_count
    }

    pub fn artifact_tree_scan_count(self) -> usize {
        self.artifact_tree_scan_count
    }

    pub fn pointer_identity_comparison_count(self) -> usize {
        self.pointer_identity_comparison_count
    }

    pub fn diagnostic_policy_read_count(self) -> usize {
        self.diagnostic_policy_read_count
    }
}
