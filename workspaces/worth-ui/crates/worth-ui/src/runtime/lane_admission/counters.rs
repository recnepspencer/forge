#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiLaneAdmissionCounters {
    admission_count: usize,
    plan_node_count: usize,
    distinct_lane_count: usize,
    support_row_lookup_count: usize,
    query_support_link_count: usize,
    hook_admission_count: usize,
    forbidden_hook_count: usize,
    unsupported_lane_denial_count: usize,
    private_lane_claim_denial_count: usize,
    raw_lane_string_lookup_count: usize,
    broad_support_scan_count: usize,
    artifact_tree_scan_count: usize,
    source_archaeology_count: usize,
    frame_path_materialization_count: usize,
    query_posture_reauthoring_count: usize,
    topology_node_construction_count: usize,
    denial_count: usize,
}

impl WorthUiLaneAdmissionCounters {
    pub(crate) fn record_admission(&mut self) {
        self.admission_count += 1;
    }

    pub(crate) fn record_plan_node(&mut self) {
        self.plan_node_count += 1;
    }

    pub(crate) fn record_distinct_lanes(&mut self, count: usize) {
        self.distinct_lane_count = count;
    }

    pub(crate) fn record_support_row_lookup(&mut self) {
        self.support_row_lookup_count += 1;
    }

    pub(crate) fn record_query_support_link(&mut self) {
        self.query_support_link_count += 1;
    }

    pub(crate) fn record_hook_admission(&mut self) {
        self.hook_admission_count += 1;
    }

    pub(crate) fn record_forbidden_hook(&mut self) {
        self.forbidden_hook_count += 1;
    }

    pub(crate) fn record_unsupported_lane_denial(&mut self) {
        self.unsupported_lane_denial_count += 1;
    }

    pub(crate) fn record_private_lane_claim_denial(&mut self) {
        self.private_lane_claim_denial_count += 1;
    }

    pub(crate) fn record_denial(&mut self) {
        self.denial_count += 1;
    }

    pub fn admission_count(self) -> usize {
        self.admission_count
    }

    pub fn plan_node_count(self) -> usize {
        self.plan_node_count
    }

    pub fn distinct_lane_count(self) -> usize {
        self.distinct_lane_count
    }

    pub fn support_row_lookup_count(self) -> usize {
        self.support_row_lookup_count
    }

    pub fn query_support_link_count(self) -> usize {
        self.query_support_link_count
    }

    pub fn hook_admission_count(self) -> usize {
        self.hook_admission_count
    }

    pub fn forbidden_hook_count(self) -> usize {
        self.forbidden_hook_count
    }

    pub fn unsupported_lane_denial_count(self) -> usize {
        self.unsupported_lane_denial_count
    }

    pub fn private_lane_claim_denial_count(self) -> usize {
        self.private_lane_claim_denial_count
    }

    pub fn raw_lane_string_lookup_count(self) -> usize {
        self.raw_lane_string_lookup_count
    }

    pub fn broad_support_scan_count(self) -> usize {
        self.broad_support_scan_count
    }

    pub fn artifact_tree_scan_count(self) -> usize {
        self.artifact_tree_scan_count
    }

    pub fn source_archaeology_count(self) -> usize {
        self.source_archaeology_count
    }

    pub fn frame_path_materialization_count(self) -> usize {
        self.frame_path_materialization_count
    }

    pub fn query_posture_reauthoring_count(self) -> usize {
        self.query_posture_reauthoring_count
    }

    pub fn topology_node_construction_count(self) -> usize {
        self.topology_node_construction_count
    }

    pub fn denial_count(self) -> usize {
        self.denial_count
    }
}
