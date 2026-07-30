use super::WorthUiActiveApplicationSession;

pub(crate) struct WorthUiPreparedMountedContentRebind<'session> {
    session: &'session mut WorthUiActiveApplicationSession,
    frame: crate::mounting::UiPreparedMountedFrame,
}

pub(crate) struct WorthUiMountedContentRebindInFlight<'session> {
    session: &'session mut WorthUiActiveApplicationSession,
    mounted: crate::mounting::UiMountedPresentationInFlight,
}

pub(crate) struct WorthUiMountedContentRebindIndeterminate<'session> {
    session: &'session mut WorthUiActiveApplicationSession,
    frame: crate::mounting::UiMountedIndeterminateFrame,
}

pub(crate) enum WorthUiMountedContentRebindOutcome<'session> {
    Published(crate::mounting::UiMountedFramePublicationReceipt),
    RejectedBeforeEffects(Box<WorthUiPreparedMountedContentRebind<'session>>),
    InFlight(Box<WorthUiMountedContentRebindInFlight<'session>>),
    PresentationIndeterminate(Box<WorthUiMountedContentRebindIndeterminate<'session>>),
    RetentionDenied {
        denial: crate::mounting::UiMountedFrameRetentionDenial,
        retry: Box<WorthUiPreparedMountedContentRebind<'session>>,
    },
    AdmissionDenied {
        denial: crate::mounting::UiMountedPresentationAdmissionDenial,
        retry: Box<WorthUiPreparedMountedContentRebind<'session>>,
    },
    CompletionDenied(crate::mounting::UiMountedPresentationCompletionDenial),
}

impl<'session> WorthUiPreparedMountedContentRebind<'session> {
    pub(crate) fn new(
        session: &'session mut WorthUiActiveApplicationSession,
        frame: crate::mounting::UiPreparedMountedFrame,
    ) -> Self {
        Self { session, frame }
    }

    pub(crate) fn frame(&self) -> &crate::mounting::UiPreparedMountedFrame {
        &self.frame
    }

    pub(crate) fn present(
        self: Box<Self>,
        deadline: worth_ui_host_contract::UiPresentationDeadline,
        now: u64,
    ) -> WorthUiMountedContentRebindOutcome<'session> {
        let Self { session, frame } = *self;
        let outcome = session.present_prepared_mounted_frame_internal(frame, deadline, now);
        finish(session, outcome)
    }
}

impl<'session> WorthUiMountedContentRebindInFlight<'session> {
    pub(crate) fn attempt(&self) -> worth_ui_host_contract::UiMountedPresentationAttemptIdentity {
        self.mounted.attempt()
    }

    pub(crate) fn deadline(&self) -> worth_ui_host_contract::UiPresentationDeadline {
        self.mounted.deadline()
    }

    pub(crate) fn complete(
        self: Box<Self>,
        now: u64,
    ) -> WorthUiMountedContentRebindOutcome<'session> {
        let Self { session, mounted } = *self;
        let outcome = session.complete_mounted_presentation(mounted, now);
        finish(session, outcome)
    }

    pub(crate) fn cancel(self: Box<Self>) -> WorthUiMountedContentRebindOutcome<'session> {
        let Self { session, mounted } = *self;
        let outcome = session.cancel_mounted_presentation(mounted);
        finish(session, outcome)
    }
}

impl<'session> WorthUiMountedContentRebindIndeterminate<'session> {
    pub(crate) fn frame(&self) -> &crate::mounting::UiMountedIndeterminateFrame {
        &self.frame
    }

    pub(crate) fn into_parts(
        self: Box<Self>,
    ) -> (
        &'session mut WorthUiActiveApplicationSession,
        crate::mounting::UiMountedIndeterminateFrame,
    ) {
        let Self { session, frame } = *self;
        (session, frame)
    }
}

fn finish<'session>(
    session: &'session mut WorthUiActiveApplicationSession,
    outcome: crate::mounting::UiMountedFrameOutcome,
) -> WorthUiMountedContentRebindOutcome<'session> {
    match outcome {
        crate::mounting::UiMountedFrameOutcome::Published(receipt) => {
            WorthUiMountedContentRebindOutcome::Published(receipt)
        }
        crate::mounting::UiMountedFrameOutcome::RejectedBeforeEffects(rejected) => {
            WorthUiMountedContentRebindOutcome::RejectedBeforeEffects(Box::new(
                WorthUiPreparedMountedContentRebind::new(session, rejected.into_frame()),
            ))
        }
        crate::mounting::UiMountedFrameOutcome::InFlight(mounted) => {
            WorthUiMountedContentRebindOutcome::InFlight(Box::new(
                WorthUiMountedContentRebindInFlight { session, mounted },
            ))
        }
        crate::mounting::UiMountedFrameOutcome::PresentationIndeterminate(frame) => {
            WorthUiMountedContentRebindOutcome::PresentationIndeterminate(Box::new(
                WorthUiMountedContentRebindIndeterminate { session, frame },
            ))
        }
        crate::mounting::UiMountedFrameOutcome::RetentionDenied(rejection) => {
            WorthUiMountedContentRebindOutcome::RetentionDenied {
                denial: rejection.denial(),
                retry: Box::new(WorthUiPreparedMountedContentRebind::new(
                    session,
                    rejection.into_frame(),
                )),
            }
        }
        crate::mounting::UiMountedFrameOutcome::AdmissionDenied(rejection) => {
            WorthUiMountedContentRebindOutcome::AdmissionDenied {
                denial: rejection.denial(),
                retry: Box::new(WorthUiPreparedMountedContentRebind::new(
                    session,
                    rejection.into_frame(),
                )),
            }
        }
        crate::mounting::UiMountedFrameOutcome::CompletionDenied(denial) => {
            WorthUiMountedContentRebindOutcome::CompletionDenied(denial)
        }
        crate::mounting::UiMountedFrameOutcome::Unchanged(_)
        | crate::mounting::UiMountedFrameOutcome::Reconciled(_) => {
            unreachable!("explicit content preparation always presents a fresh mounted frame")
        }
    }
}
