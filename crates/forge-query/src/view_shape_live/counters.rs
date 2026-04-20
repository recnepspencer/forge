use crate::live::LivePolicyCounters;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ViewShapeLiveCounters {
    core: LivePolicyCounters,
    view_patch_width: usize,
    view_delivery_width: usize,
    view_refresh_fallback_count: usize,
    view_shape_executor_rediscovery_count: usize,
    view_family_fallback_denial_count: usize,
    view_family_refresh_admission_count: usize,
    view_family_refresh_forbidden_count: usize,
    grouped_desired_state_row_count: usize,
    grouped_delta_row_count: usize,
    grouped_membership_transition_count: usize,
    grouped_lane_count: usize,
    grouped_full_regroup_denial_count: usize,
    observed_inspector_delivery_width: usize,
    focused_inspector_aspect_focus_width: usize,
    focused_inspector_projection_width: usize,
    focused_inspector_widening_denial_count: usize,
    table_ordering_key_count: usize,
    cosmetic_view_semantics_denial_count: usize,
    complexity_status_debt_count: usize,
}

impl ViewShapeLiveCounters {
    pub(crate) fn with_core(mut self, core: LivePolicyCounters) -> Self {
        self.core = core;
        self
    }

    pub(crate) fn set_view_patch_width(&mut self, value: usize) {
        self.view_patch_width = value;
    }

    pub(crate) fn set_view_delivery_width(&mut self, value: usize) {
        self.view_delivery_width = value;
    }

    pub(crate) fn set_observed_inspector_delivery_width(&mut self, value: usize) {
        self.observed_inspector_delivery_width = value;
    }

    pub(crate) fn set_focused_inspector_projection_width(&mut self, value: usize) {
        self.focused_inspector_projection_width = value;
    }

    pub(crate) fn set_focused_inspector_aspect_focus_width(&mut self, value: usize) {
        self.focused_inspector_aspect_focus_width = value;
    }

    pub(crate) fn set_table_ordering_key_count(&mut self, value: usize) {
        self.table_ordering_key_count = value;
    }

    pub(crate) fn add_view_refresh_fallback(&mut self) {
        self.view_refresh_fallback_count += 1;
    }

    pub(crate) fn add_view_family_fallback_denial(&mut self) {
        self.view_family_fallback_denial_count += 1;
    }

    pub(crate) fn add_view_family_refresh_admission(&mut self) {
        self.view_family_refresh_admission_count += 1;
    }

    pub(crate) fn add_view_family_refresh_forbidden(&mut self) {
        self.view_family_refresh_forbidden_count += 1;
    }

    pub(crate) fn set_grouped_desired_state_row_count(&mut self, value: usize) {
        self.grouped_desired_state_row_count = value;
    }

    pub(crate) fn set_grouped_delta_row_count(&mut self, value: usize) {
        self.grouped_delta_row_count = value;
    }

    pub(crate) fn set_grouped_membership_transition_count(&mut self, value: usize) {
        self.grouped_membership_transition_count = value;
    }

    pub(crate) fn set_grouped_lane_count(&mut self, value: usize) {
        self.grouped_lane_count = value;
    }

    pub(crate) fn add_grouped_full_regroup_denial(&mut self) {
        self.grouped_full_regroup_denial_count += 1;
    }

    pub(crate) fn add_focused_inspector_widening_denial(&mut self) {
        self.focused_inspector_widening_denial_count += 1;
    }

    pub(crate) fn add_cosmetic_view_semantics_denial(&mut self) {
        self.cosmetic_view_semantics_denial_count += 1;
    }

    pub(crate) fn add_complexity_status_debt(&mut self) {
        self.complexity_status_debt_count += 1;
    }

    pub fn core(&self) -> &LivePolicyCounters {
        &self.core
    }

    pub fn view_patch_width(&self) -> usize {
        self.view_patch_width
    }

    pub fn view_delivery_width(&self) -> usize {
        self.view_delivery_width
    }

    pub fn view_refresh_fallback_count(&self) -> usize {
        self.view_refresh_fallback_count
    }

    pub fn view_shape_executor_rediscovery_count(&self) -> usize {
        self.view_shape_executor_rediscovery_count
    }

    pub fn view_family_fallback_denial_count(&self) -> usize {
        self.view_family_fallback_denial_count
    }

    pub fn view_family_refresh_admission_count(&self) -> usize {
        self.view_family_refresh_admission_count
    }

    pub fn view_family_refresh_forbidden_count(&self) -> usize {
        self.view_family_refresh_forbidden_count
    }

    pub fn grouped_desired_state_row_count(&self) -> usize {
        self.grouped_desired_state_row_count
    }

    pub fn grouped_delta_row_count(&self) -> usize {
        self.grouped_delta_row_count
    }

    pub fn grouped_membership_transition_count(&self) -> usize {
        self.grouped_membership_transition_count
    }

    pub fn grouped_lane_count(&self) -> usize {
        self.grouped_lane_count
    }

    pub fn grouped_full_regroup_denial_count(&self) -> usize {
        self.grouped_full_regroup_denial_count
    }

    pub fn observed_inspector_delivery_width(&self) -> usize {
        self.observed_inspector_delivery_width
    }

    pub fn focused_inspector_aspect_focus_width(&self) -> usize {
        self.focused_inspector_aspect_focus_width
    }

    pub fn focused_inspector_projection_width(&self) -> usize {
        self.focused_inspector_projection_width
    }

    pub fn focused_inspector_widening_denial_count(&self) -> usize {
        self.focused_inspector_widening_denial_count
    }

    pub fn table_ordering_key_count(&self) -> usize {
        self.table_ordering_key_count
    }

    pub fn cosmetic_view_semantics_denial_count(&self) -> usize {
        self.cosmetic_view_semantics_denial_count
    }

    pub fn complexity_status_debt_count(&self) -> usize {
        self.complexity_status_debt_count
    }
}
