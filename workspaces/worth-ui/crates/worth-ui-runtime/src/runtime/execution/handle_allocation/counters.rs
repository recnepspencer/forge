#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiRuntimeHandleAllocationCounters {
    plan_node_input_count: usize,
    component_handle_count: usize,
    command_handle_count: usize,
    token_handle_count: usize,
    child_range_handle_count: usize,
    view_binding_handle_count: usize,
    lane_handle_count: usize,
    state_slot_handle_count: usize,
    collision_check_count: usize,
    collision_denial_count: usize,
    source_parse_count: usize,
    registry_string_lookup_count: usize,
    broad_registry_scan_count: usize,
}

impl WorthUiRuntimeHandleAllocationCounters {
    pub(crate) fn record_plan_node_input(&mut self) {
        self.plan_node_input_count += 1;
    }

    pub(crate) fn record_component_handle(&mut self) {
        self.component_handle_count += 1;
    }

    pub(crate) fn record_command_handle(&mut self) {
        self.command_handle_count += 1;
    }

    pub(crate) fn record_token_handle(&mut self) {
        self.token_handle_count += 1;
    }

    pub(crate) fn record_child_range_handle(&mut self) {
        self.child_range_handle_count += 1;
    }

    pub(crate) fn record_view_binding_handle(&mut self) {
        self.view_binding_handle_count += 1;
    }

    pub(crate) fn record_lane_handle(&mut self) {
        self.lane_handle_count += 1;
    }

    pub(crate) fn record_state_slot_handle(&mut self) {
        self.state_slot_handle_count += 1;
    }

    pub(crate) fn record_collision_check(&mut self) {
        self.collision_check_count += 1;
    }

    pub(crate) fn record_collision_denial(&mut self) {
        self.collision_denial_count += 1;
    }

    pub fn plan_node_input_count(self) -> usize {
        self.plan_node_input_count
    }

    pub fn component_handle_count(self) -> usize {
        self.component_handle_count
    }

    pub fn command_handle_count(self) -> usize {
        self.command_handle_count
    }

    pub fn token_handle_count(self) -> usize {
        self.token_handle_count
    }

    pub fn child_range_handle_count(self) -> usize {
        self.child_range_handle_count
    }

    pub fn view_binding_handle_count(self) -> usize {
        self.view_binding_handle_count
    }

    pub fn lane_handle_count(self) -> usize {
        self.lane_handle_count
    }

    pub fn state_slot_handle_count(self) -> usize {
        self.state_slot_handle_count
    }

    pub fn collision_check_count(self) -> usize {
        self.collision_check_count
    }

    pub fn collision_denial_count(self) -> usize {
        self.collision_denial_count
    }

    pub fn source_parse_count(self) -> usize {
        self.source_parse_count
    }

    pub fn registry_string_lookup_count(self) -> usize {
        self.registry_string_lookup_count
    }

    pub fn broad_registry_scan_count(self) -> usize {
        self.broad_registry_scan_count
    }
}
