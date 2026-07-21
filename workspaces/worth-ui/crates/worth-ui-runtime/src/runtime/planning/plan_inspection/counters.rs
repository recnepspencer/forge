#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiPlanInspectionCounters {
    inspection_count: usize,
    plan_digest_count: usize,
    node_inspection_count: usize,
    lane_inspection_count: usize,
    provenance_link_count: usize,
    query_link_preservation_count: usize,
    projection_consumption_link_count: usize,
    causal_inspection_reference_count: usize,
    ordinary_outcome_reference_count: usize,
    artifact_tree_scan_count: usize,
    source_archaeology_count: usize,
    registry_lookup_count: usize,
    diagnostic_policy_read_count: usize,
    frame_path_materialization_count: usize,
    denial_count: usize,
}

impl WorthUiPlanInspectionCounters {
    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn record_inspection(&mut self) {
        self.inspection_count += 1;
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn record_plan_digest(&mut self) {
        self.plan_digest_count += 1;
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn record_node_inspection(&mut self) {
        self.node_inspection_count += 1;
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn record_lane_inspection(&mut self) {
        self.lane_inspection_count += 1;
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn record_provenance_link(&mut self) {
        self.provenance_link_count += 1;
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn record_query_link_preservation(&mut self) {
        self.query_link_preservation_count += 1;
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn record_projection_consumption_link(&mut self) {
        self.projection_consumption_link_count += 1;
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn record_denial(&mut self) {
        self.denial_count += 1;
    }

    pub fn inspection_count(self) -> usize {
        self.inspection_count
    }

    pub fn plan_digest_count(self) -> usize {
        self.plan_digest_count
    }

    pub fn node_inspection_count(self) -> usize {
        self.node_inspection_count
    }

    pub fn lane_inspection_count(self) -> usize {
        self.lane_inspection_count
    }

    pub fn provenance_link_count(self) -> usize {
        self.provenance_link_count
    }

    pub fn query_link_preservation_count(self) -> usize {
        self.query_link_preservation_count
    }

    pub fn projection_consumption_link_count(self) -> usize {
        self.projection_consumption_link_count
    }

    pub fn causal_inspection_reference_count(self) -> usize {
        self.causal_inspection_reference_count
    }

    pub fn ordinary_outcome_reference_count(self) -> usize {
        self.ordinary_outcome_reference_count
    }

    pub fn artifact_tree_scan_count(self) -> usize {
        self.artifact_tree_scan_count
    }

    pub fn source_archaeology_count(self) -> usize {
        self.source_archaeology_count
    }

    pub fn registry_lookup_count(self) -> usize {
        self.registry_lookup_count
    }

    pub fn diagnostic_policy_read_count(self) -> usize {
        self.diagnostic_policy_read_count
    }

    pub fn frame_path_materialization_count(self) -> usize {
        self.frame_path_materialization_count
    }

    pub fn denial_count(self) -> usize {
        self.denial_count
    }
}
