//! SUPPORT AUTHORITY for component-level mounted-frame certification.

use crate::facade::{WorthUiActiveApplicationSession, WorthUiActiveFrameworkTurnExecution};

pub trait WorthUiMountedFrameExecutionCertificationExt {
    fn classify_mounted_frame_reuse(
        &self,
        request: &crate::mounting::UiMountedFrameRequest,
    ) -> crate::mounting::UiMountedFrameReuse;

    fn prepare_mounted_frame(
        self,
        request: crate::mounting::UiMountedFrameRequest,
    ) -> Result<
        crate::mounting::UiPreparedMountedFrame,
        crate::mounting::UiMountedFramePreparationDenial,
    >;
}

pub trait WorthUiMountedPublicationCertificationExt {
    fn present_prepared_mounted_frame(
        &mut self,
        frame: crate::mounting::UiPreparedMountedFrame,
        deadline: worth_ui_host_contract::UiPresentationDeadline,
        now: u64,
    ) -> crate::mounting::UiMountedFrameOutcome;
}

impl<'session> WorthUiMountedFrameExecutionCertificationExt
    for WorthUiActiveFrameworkTurnExecution<'session>
{
    fn classify_mounted_frame_reuse(
        &self,
        request: &crate::mounting::UiMountedFrameRequest,
    ) -> crate::mounting::UiMountedFrameReuse {
        self.classify_mounted_frame_reuse_internal(request)
    }

    fn prepare_mounted_frame(
        self,
        request: crate::mounting::UiMountedFrameRequest,
    ) -> Result<
        crate::mounting::UiPreparedMountedFrame,
        crate::mounting::UiMountedFramePreparationDenial,
    > {
        self.prepare_mounted_frame_internal(request)
    }
}

impl WorthUiMountedPublicationCertificationExt for WorthUiActiveApplicationSession {
    fn present_prepared_mounted_frame(
        &mut self,
        frame: crate::mounting::UiPreparedMountedFrame,
        deadline: worth_ui_host_contract::UiPresentationDeadline,
        now: u64,
    ) -> crate::mounting::UiMountedFrameOutcome {
        self.present_prepared_mounted_frame_internal(frame, deadline, now)
    }
}
