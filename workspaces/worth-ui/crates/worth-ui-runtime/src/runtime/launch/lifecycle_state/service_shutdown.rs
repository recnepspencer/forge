impl super::WorthUiRuntimeShutdownReceipt {
    pub(in crate::runtime) const fn service_proposal_shutdown(
        &self,
    ) -> crate::runtime::session::service_proposal::UiServiceProposalCompilerShutdownReceipt {
        self.service_proposals
    }

    pub const fn focus_placement(&self) -> crate::mounting::UiFocusHostPlacementShutdownReport {
        self.focus_placement
    }

    pub const fn portal_closed_records(&self) -> usize {
        self.portal.closed_records()
    }

    pub const fn portal_abandoned_indeterminate_records(&self) -> usize {
        self.portal.abandoned_indeterminate_records()
    }

    pub const fn portal_final_active_records(&self) -> usize {
        self.portal.final_active_records()
    }

    pub const fn motion_abandoned_staged_tracks(&self) -> u16 {
        self.motion.abandoned_staged_tracks()
    }

    pub const fn motion_terminated_active_tracks(&self) -> u16 {
        self.motion.terminated_active_tracks()
    }

    pub const fn motion_cancelled_exit_retentions(&self) -> u16 {
        self.motion.cancelled_exit_retentions()
    }

    pub const fn motion_final_census_is_zero(&self) -> bool {
        self.motion.final_census().is_zero()
    }

    pub const fn scroll_owners_released(&self) -> usize {
        self.scroll_owners_released
    }

    pub const fn selection_owners_released(&self) -> usize {
        self.selection_owners_released
    }

    pub(crate) fn bind_rebind(
        mut self,
        report: crate::runtime::rebind::UiRebindShutdownReport,
    ) -> Self {
        self.rebind = report;
        self
    }

    pub(crate) fn bind_focus_placement(
        mut self,
        report: crate::mounting::UiFocusHostPlacementShutdownReport,
    ) -> Self {
        self.focus_placement = report;
        self
    }

    pub(crate) fn bind_portal(
        mut self,
        report: crate::runtime::portal::UiPortalShutdownReport,
    ) -> Self {
        self.portal = report;
        self
    }

    pub(crate) fn bind_motion(
        mut self,
        report: crate::runtime::motion::UiMotionShutdownReport,
    ) -> Self {
        self.motion = report;
        self
    }

    pub(crate) fn bind_scroll_owners_released(mut self, released: usize) -> Self {
        self.scroll_owners_released = released;
        self
    }

    pub(crate) fn bind_selection_owners_released(mut self, released: usize) -> Self {
        self.selection_owners_released = released;
        self
    }
}
