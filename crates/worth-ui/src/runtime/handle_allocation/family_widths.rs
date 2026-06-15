#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiRuntimeHandleFamilyWidths {
    runtime_handle_count: usize,
    component_handle_count: usize,
    command_handle_count: usize,
    token_handle_count: usize,
    child_range_handle_count: usize,
    view_binding_handle_count: usize,
    lane_handle_count: usize,
    state_slot_handle_count: usize,
}

impl WorthUiRuntimeHandleFamilyWidths {
    pub(crate) fn record_runtime_handle(&mut self) {
        self.runtime_handle_count += 1;
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

    pub fn runtime_handle_count(self) -> usize {
        self.runtime_handle_count
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
}
