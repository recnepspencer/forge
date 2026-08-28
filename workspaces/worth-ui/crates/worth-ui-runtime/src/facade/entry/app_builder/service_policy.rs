use super::WorthUiApplicationBuilder;

impl<ChangeProfileState, IntentWiringState>
    WorthUiApplicationBuilder<ChangeProfileState, IntentWiringState>
{
    pub fn with_portal_policy_defaults(
        mut self,
        policy: crate::declaration::UiPortalPolicy,
    ) -> Self {
        self.service_policy_defaults = self.service_policy_defaults.with_portal(policy);
        self
    }

    pub fn with_focus_policy_defaults(mut self, policy: crate::declaration::UiFocusPolicy) -> Self {
        self.service_policy_defaults = self.service_policy_defaults.with_focus(policy);
        self
    }

    pub fn with_motion_policy_defaults(
        mut self,
        policy: crate::declaration::UiMotionPolicy,
    ) -> Self {
        self.service_policy_defaults = self.service_policy_defaults.with_motion(policy);
        self
    }

    pub fn with_command_routing_policy_defaults(
        mut self,
        policy: crate::declaration::UiCommandRoutingPolicy,
    ) -> Self {
        self.service_policy_defaults = self.service_policy_defaults.with_command_routing(policy);
        self
    }

    pub fn with_scroll_policy_defaults(
        mut self,
        policy: crate::declaration::UiScrollPolicy,
    ) -> Self {
        self.service_policy_defaults = self.service_policy_defaults.with_scroll(policy);
        self
    }

    pub fn with_selection_policy_defaults(
        mut self,
        policy: crate::declaration::UiSelectionPolicy,
    ) -> Self {
        self.service_policy_defaults = self.service_policy_defaults.with_selection(policy);
        self
    }
}
