use worth_ui_host_contract::UiPresentationDeadline;

use super::{
    WorthUiMountedPreviewAdmissionRejection, WorthUiMountedPreviewCompletionRejection,
    WorthUiMountedPreviewOutcome, WorthUiMountedPreviewPreparationDenial,
    WorthUiMountedPreviewPreparationRejection, WorthUiPreparedMountedPreview,
    WorthUiResolvedMountedPreview,
};

impl WorthUiResolvedMountedPreview {
    pub fn disposition(&self) -> &super::WorthUiMountedPreviewDisposition {
        &self.disposition
    }
    pub fn isolation(&self) -> crate::runtime::UiPreviewPaintIsolationOutcome {
        self.isolation
    }
    pub fn follow_on(&self) -> &crate::runtime::WorthUiMountedPreviewFollowOn {
        &self.follow_on
    }
    pub fn planning_counters(&self) -> crate::runtime::UiFrameworkTransitionPlanningCounters {
        self.planning_counters
    }
}

impl<'session> WorthUiMountedPreviewPreparationRejection<'session> {
    pub fn denial(&self) -> &WorthUiMountedPreviewPreparationDenial {
        &self.denial
    }
    pub fn retry(
        self,
        mounted_instance: worth_ui_host_contract::UiMountedInstanceIdentity,
    ) -> Result<
        WorthUiPreparedMountedPreview<'session>,
        WorthUiMountedPreviewPreparationRejection<'session>,
    > {
        (*self.pending).prepare(mounted_instance)
    }
    pub fn supersede(self) -> WorthUiResolvedMountedPreview {
        (*self.pending).supersede()
    }
}

impl<'session> WorthUiMountedPreviewAdmissionRejection<'session> {
    pub fn denial(&self) -> crate::mounting::UiMountedPresentationAdmissionDenial {
        self.denial
    }
    pub fn retry(
        self,
        deadline: UiPresentationDeadline,
        now: u64,
    ) -> WorthUiMountedPreviewOutcome<'session> {
        self.preview.present(deadline, now)
    }
    pub fn supersede(self) -> WorthUiResolvedMountedPreview {
        self.preview.supersede()
    }
}

impl<'session> WorthUiMountedPreviewCompletionRejection<'session> {
    pub fn denial(&self) -> crate::mounting::UiMountedPresentationCompletionDenial {
        self.denial
    }
    pub fn retry(self, now: u64) -> WorthUiMountedPreviewOutcome<'session> {
        self.in_flight.complete(now)
    }
}
