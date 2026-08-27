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
        let authority = UiNativeProgramReconstructionAuthority::Physical(authority);
        let progress = self.attempt_reconstruction(shell, program_frame, authority)?;
        if self.settle_reconstruction_attempt(program_frame, authority, progress)? {
            self.advance(shell)?;
        }
        Ok(())
    }

    fn attempt_reconstruction(
        &mut self,
        shell: &mut WorthUiNativeApplicationShell,
        program_frame: usize,
        authority: UiNativeProgramReconstructionAuthority,
    ) -> Result<FrameProgress, ()> {
        self.next_completion_tick = self.next_completion_tick.saturating_add(1);
        let outcome = shell
            .reconstruct_current_presentation(u64::MAX, self.next_completion_tick)
            .map_err(|_| ())?;
        self.retain_or_attribute(shell, outcome, program_frame, None, Some(authority), false)
    }

    fn settle_reconstruction_attempt(
        &mut self,
        _program_frame: usize,
        authority: UiNativeProgramReconstructionAuthority,
        progress: FrameProgress,
    ) -> Result<bool, ()> {
        match progress {
            FrameProgress::Retained => Ok(false),
            FrameProgress::Settled => {
                if let UiNativeProgramReconstructionAuthority::Physical(correlation) = authority {
                    self.physical_recovery
                        .commit_settlement(correlation)
                        .map_err(|_| ())?;
                }
                Ok(true)
            }
            FrameProgress::RetryRequired(retry_readiness) => {
                let _ = retry_readiness;
                if self.pending_retry.is_none() {
                    return Err(());
                }
                Ok(false)
            }
            FrameProgress::Failed => Err(()),
        }
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
