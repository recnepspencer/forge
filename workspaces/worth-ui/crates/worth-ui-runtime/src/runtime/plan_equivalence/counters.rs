#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiExecutionPlanEquivalenceCounters {
    plan_digest_count: usize,
    plan_node_digest_count: usize,
    child_range_digest_count: usize,
    lane_partition_digest_count: usize,
    lookup_index_digest_count: usize,
    egui_boundary_digest_count: usize,
    render_resource_digest_count: usize,
    equivalence_comparison_count: usize,
    artifact_tree_scan_count: usize,
    pointer_identity_comparison_count: usize,
    diagnostic_policy_read_count: usize,
}

impl WorthUiExecutionPlanEquivalenceCounters {
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

    pub(crate) fn record_egui_boundary_digest(&mut self) {
        self.egui_boundary_digest_count += 1;
    }

    pub(crate) fn record_render_resource_digest(&mut self) {
        self.render_resource_digest_count += 1;
    }

    pub(crate) fn record_equivalence_comparison(&mut self) {
        self.equivalence_comparison_count += 1;
    }

    pub(crate) fn combine(mut self, other: Self) -> Self {
        self.plan_digest_count += other.plan_digest_count;
        self.plan_node_digest_count += other.plan_node_digest_count;
        self.child_range_digest_count += other.child_range_digest_count;
        self.lane_partition_digest_count += other.lane_partition_digest_count;
        self.lookup_index_digest_count += other.lookup_index_digest_count;
        self.egui_boundary_digest_count += other.egui_boundary_digest_count;
        self.render_resource_digest_count += other.render_resource_digest_count;
        self.equivalence_comparison_count += other.equivalence_comparison_count;
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

    pub fn egui_boundary_digest_count(self) -> usize {
        self.egui_boundary_digest_count
    }

    pub fn render_resource_digest_count(self) -> usize {
        self.render_resource_digest_count
    }

    pub fn equivalence_comparison_count(self) -> usize {
        self.equivalence_comparison_count
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
