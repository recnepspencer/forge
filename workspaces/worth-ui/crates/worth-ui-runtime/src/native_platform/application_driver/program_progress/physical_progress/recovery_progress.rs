use super::super::{
    UiNativeApplicationProgramProgress, UiNativePhysicalRecoverySettlement,
    WorthUiNativeApplicationShell,
};

impl UiNativeApplicationProgramProgress {
    pub(super) fn progress_presentation_recovery(
        &mut self,
        shell: &mut WorthUiNativeApplicationShell,
        presentation: worth_ui_host_native::UiNativePhysicalPresentationCorrelation,
    ) -> Result<(), ()> {
        self.physical_recovery
            .observe_scheduled(presentation)
            .or_else(|denial| {
                (denial
                    == super::super::super::physical_recovery_tracker::UiNativePhysicalRecoveryTrackingDenial::DuplicateCorrelation)
                    .then_some(())
                    .ok_or(denial)
            })
            .map_err(|_| ())?;
        let settlement = self
            .physical_recovery
            .classify_settlement(presentation)
            .map_err(|_| ())?;
        if settlement == UiNativePhysicalRecoverySettlement::AttemptStillPending {
            self.physical_recovery
                .commit_settlement(presentation)
                .map_err(|_| ())?;
            return Ok(());
        }
        let recovery_program_frame = self.next_frame.saturating_sub(1);
        self.resume_reconstruction(shell, recovery_program_frame, presentation)
    }
}
