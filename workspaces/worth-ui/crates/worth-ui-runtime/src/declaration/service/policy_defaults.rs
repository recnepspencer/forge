#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct UiServicePolicyDefaults {
    portal: Option<super::UiPortalPolicy>,
    focus: Option<super::UiFocusPolicy>,
    motion: Option<super::UiMotionPolicy>,
    command_routing: Option<super::UiCommandRoutingPolicy>,
    scroll: Option<super::UiScrollPolicy>,
    selection: Option<super::UiSelectionPolicy>,
}

impl UiServicePolicyDefaults {
    pub(crate) const fn with_portal(mut self, policy: super::UiPortalPolicy) -> Self {
        self.portal = Some(policy);
        self
    }

    pub(crate) const fn with_focus(mut self, policy: super::UiFocusPolicy) -> Self {
        self.focus = Some(policy);
        self
    }

    pub(crate) const fn with_motion(mut self, policy: super::UiMotionPolicy) -> Self {
        self.motion = Some(policy);
        self
    }

    pub(crate) const fn with_command_routing(
        mut self,
        policy: super::UiCommandRoutingPolicy,
    ) -> Self {
        self.command_routing = Some(policy);
        self
    }

    pub(crate) const fn with_scroll(mut self, policy: super::UiScrollPolicy) -> Self {
        self.scroll = Some(policy);
        self
    }

    pub(crate) const fn with_selection(mut self, policy: super::UiSelectionPolicy) -> Self {
        self.selection = Some(policy);
        self
    }

    pub(crate) const fn portal(self) -> Option<super::UiPortalPolicy> {
        self.portal
    }

    pub(crate) const fn focus(self) -> Option<super::UiFocusPolicy> {
        self.focus
    }

    pub(crate) const fn motion(self) -> Option<super::UiMotionPolicy> {
        self.motion
    }

    pub(crate) const fn command_routing(self) -> Option<super::UiCommandRoutingPolicy> {
        self.command_routing
    }

    pub(crate) const fn scroll(self) -> Option<super::UiScrollPolicy> {
        self.scroll
    }

    pub(crate) const fn selection(self) -> Option<super::UiSelectionPolicy> {
        self.selection
    }
}
