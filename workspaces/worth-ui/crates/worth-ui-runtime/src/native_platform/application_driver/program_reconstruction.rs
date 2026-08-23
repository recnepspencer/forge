use super::program_progress::{
    FrameProgress, UiNativeApplicationProgramProgress, UiNativeProgramReconstructionAuthority,
};
use crate::facade::WorthUiNativeApplicationShell;

impl UiNativeApplicationProgramProgress {
    pub(super) fn resume_reconstruction(
        &mut self,
        shell: &mut WorthUiNativeApplicationShell,
        program_frame: usize,
        authority: worth_ui_host_native::UiNativePhysicalPresentationCorrelation,
    ) -> Result<(), ()> {
        for _ in 0..2 {
            self.next_completion_tick = self.next_completion_tick.saturating_add(1);
            let outcome =
                shell.reconstruct_current_presentation(u64::MAX, self.next_completion_tick)?;
            let progress = self.retain_or_attribute(
                shell,
                outcome,
                program_frame,
                None,
                Some(UiNativeProgramReconstructionAuthority::Physical(authority)),
                false,
            )?;
            match progress {
                FrameProgress::Retained => return Ok(()),
                FrameProgress::Settled => {
                    self.physical_recovery
                        .commit_settlement(authority)
                        .map_err(|_| ())?;
                    return self.advance(shell);
                }
                FrameProgress::RetryRequired => {}
                FrameProgress::Failed => return Err(()),
            }
        }
        Err(())
    }

    pub(super) fn resume_host_reconstruction(
        &mut self,
        shell: &mut WorthUiNativeApplicationShell,
        program_frame: usize,
    ) -> Result<(), ()> {
        for _ in 0..2 {
            self.next_completion_tick = self.next_completion_tick.saturating_add(1);
            let outcome =
                shell.reconstruct_current_presentation(u64::MAX, self.next_completion_tick)?;
            match self.retain_or_attribute(
                shell,
                outcome,
                program_frame,
                None,
                Some(UiNativeProgramReconstructionAuthority::HostRequired),
                false,
            )? {
                FrameProgress::Retained => return Ok(()),
                FrameProgress::Settled => return self.advance(shell),
                FrameProgress::RetryRequired => {}
                FrameProgress::Failed => return Err(()),
            }
        }
        Err(())
    }
}

pub(super) fn retry_text_atlas_deferred(
    shell: &mut WorthUiNativeApplicationShell,
    outcome: crate::mounting::UiMountedFrameOutcome,
    deadline: worth_ui_host_contract::UiPresentationDeadline,
    now_tick: u64,
) -> crate::mounting::UiMountedFrameOutcome {
    match outcome {
        crate::mounting::UiMountedFrameOutcome::RejectedBeforeEffects(rejected)
            if is_text_atlas_rejection(&rejected) =>
        {
            shell.retry_rejected_frame_presentation(rejected, deadline, now_tick)
        }
        outcome => outcome,
    }
}

pub(super) fn is_text_atlas_deferred(outcome: &crate::mounting::UiMountedFrameOutcome) -> bool {
    matches!(
        outcome,
        crate::mounting::UiMountedFrameOutcome::RejectedBeforeEffects(rejected)
            if is_text_atlas_rejection(rejected)
    )
}

fn is_text_atlas_rejection(rejected: &crate::mounting::UiMountedRejectedFrame) -> bool {
    rejected.rejections().iter().all(|rejection| {
        rejection.denial()
            == worth_ui_host_contract::UiHostSurfacePresentationDenial::TextAtlasPresentationDeferred
    })
}
