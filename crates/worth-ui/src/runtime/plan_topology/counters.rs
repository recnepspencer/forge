#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiPlanTopologyCounters {
    plan_node_input_count: usize,
    topology_node_count: usize,
    child_range_count: usize,
    lane_partition_count: usize,
    lookup_entry_count: usize,
    egui_boundary_count: usize,
    render_resource_ref_count: usize,
    topology_validation_count: usize,
    artifact_tree_scan_count: usize,
    registry_string_lookup_count: usize,
    broad_registry_scan_count: usize,
    ambient_egui_access_count: usize,
    denial_count: usize,
}

impl WorthUiPlanTopologyCounters {
    pub(crate) fn record_plan_node_input(&mut self) {
        self.plan_node_input_count += 1;
    }

    pub(crate) fn record_topology_node(&mut self) {
        self.topology_node_count += 1;
    }

    pub(crate) fn record_child_range(&mut self) {
        self.child_range_count += 1;
    }

    pub(crate) fn record_lane_partition(&mut self) {
        self.lane_partition_count += 1;
    }

    pub(crate) fn record_lookup_entry(&mut self) {
        self.lookup_entry_count += 1;
    }

    pub(crate) fn record_egui_boundary(&mut self) {
        self.egui_boundary_count += 1;
    }

    pub(crate) fn record_render_resource_ref(&mut self) {
        self.render_resource_ref_count += 1;
    }

    pub(crate) fn record_validation(&mut self) {
        self.topology_validation_count += 1;
    }

    pub(crate) fn record_denial(&mut self) {
        self.denial_count += 1;
    }

    pub fn plan_node_input_count(self) -> usize {
        self.plan_node_input_count
    }

    pub fn topology_node_count(self) -> usize {
        self.topology_node_count
    }

    pub fn child_range_count(self) -> usize {
        self.child_range_count
    }

    pub fn lane_partition_count(self) -> usize {
        self.lane_partition_count
    }

    pub fn lookup_entry_count(self) -> usize {
        self.lookup_entry_count
    }

    pub fn egui_boundary_count(self) -> usize {
        self.egui_boundary_count
    }

    pub fn render_resource_ref_count(self) -> usize {
        self.render_resource_ref_count
    }

    pub fn topology_validation_count(self) -> usize {
        self.topology_validation_count
    }

    pub fn artifact_tree_scan_count(self) -> usize {
        self.artifact_tree_scan_count
    }

    pub fn registry_string_lookup_count(self) -> usize {
        self.registry_string_lookup_count
    }

    pub fn broad_registry_scan_count(self) -> usize {
        self.broad_registry_scan_count
    }

    pub fn ambient_egui_access_count(self) -> usize {
        self.ambient_egui_access_count
    }

    pub fn denial_count(self) -> usize {
        self.denial_count
    }
}
