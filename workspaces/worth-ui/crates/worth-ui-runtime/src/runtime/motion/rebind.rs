impl super::UiMotionRuntimeState {
    pub(in crate::runtime) fn terminalize_rebound_target(
        &mut self,
        target: super::UiMotionTargetIdentity,
    ) -> Option<super::UiMotionTerminalReceipt> {
        self.terminalize_target(target, super::UiMotionTerminalCause::ReboundAway)
    }
}
