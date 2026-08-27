use super::{
    WorthUiActiveApplicationSession, WorthUiMountedApplicationReplacementOutcome,
    WorthUiPreparedApplicationActivation,
};

pub(super) struct WorthUiPresentedApplicationReplacement<'session> {
    session: &'session mut WorthUiActiveApplicationSession,
    application: Box<WorthUiPreparedApplicationActivation>,
    mounted_successor: crate::mounting::UiMountedGraphReplacementSuccessor,
    mounted_receipt: crate::mounting::UiMountedFramePublicationReceipt,
    focus: crate::runtime::focus::UiPreparedFocusMountedReconciliation,
}

impl<'session> WorthUiPresentedApplicationReplacement<'session> {
    pub(super) fn new(
        session: &'session mut WorthUiActiveApplicationSession,
        application: Box<WorthUiPreparedApplicationActivation>,
        mounted_successor: crate::mounting::UiMountedGraphReplacementSuccessor,
        mounted_receipt: crate::mounting::UiMountedFramePublicationReceipt,
    ) -> Self {
        let snapshot = mounted_successor
            .focus_participation_snapshot()
            .expect("a presented mounted successor retains Focus participation");
        let focus = session
            .focus
            .prepare_mounted_reconciliation(&snapshot)
            .expect("mounted Focus participation fits bounded counters");
        Self {
            session,
            application,
            mounted_successor,
            mounted_receipt,
            focus,
        }
    }

    pub(super) fn commit_once(self) -> WorthUiMountedApplicationReplacementOutcome<'session> {
        let application = self
            .session
            .commit_application_activation(self.application, self.mounted_successor);
        self.session
            .reconcile_prepared_focus_after_published_frame(self.focus, &self.mounted_receipt);
        WorthUiMountedApplicationReplacementOutcome::Published {
            application,
            mounted: self.mounted_receipt,
        }
    }
}
