use super::*;

#[path = "physical_progress/pending_completion.rs"]
mod pending_completion;
#[path = "physical_progress/recovery_progress.rs"]
mod recovery_progress;
#[path = "physical_progress/settlement_progress.rs"]
mod settlement_progress;

impl UiNativeApplicationProgramProgress {
    pub(in crate::native_platform::application_driver) fn physical_work_progressed(
        &mut self,
        shell: &mut WorthUiNativeApplicationShell,
        grant: worth_ui_host_native::UiNativePhysicalProgressGrant,
    ) -> Result<(), ()> {
        let class = grant.class();
        if class == worth_ui_host_native::UiNativePhysicalProgressClass::PresentationRecovery {
            let presentation = grant.presentation().ok_or(())?;
            return self.progress_presentation_recovery(shell, presentation);
        }
        let presentation = grant.presentation();
        let Some(pending) = self.take_pending_for_physical_progress(class, presentation)? else {
            return self.advance(shell);
        };
        let completed = self.complete_pending_physical_progress(shell, pending, presentation)?;
        self.settle_completed_physical_progress(
            shell,
            completed,
            presentation,
            grant.duplicate_presentation_observed(),
        )
    }
}
