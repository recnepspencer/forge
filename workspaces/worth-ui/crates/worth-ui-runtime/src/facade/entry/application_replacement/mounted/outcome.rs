use super::super::{
    WorthUiActiveApplicationSession, WorthUiApplicationCutoverReceipt,
    WorthUiApplicationSemanticNoOpReceipt, WorthUiPreparedApplicationActivation,
};

pub struct WorthUiPreparedMountedApplicationReplacement<'session> {
    pub(super) session: &'session mut WorthUiActiveApplicationSession,
    pub(super) application: Box<WorthUiPreparedApplicationActivation>,
    pub(super) mounted_successor: crate::mounting::UiMountedIdentityState,
    pub(super) frame: crate::mounting::UiPreparedMountedFrame,
}

pub struct WorthUiMountedApplicationReplacementInFlight<'session> {
    pub(super) session: &'session mut WorthUiActiveApplicationSession,
    pub(super) application: Box<WorthUiPreparedApplicationActivation>,
    pub(super) mounted_successor: crate::mounting::UiMountedIdentityState,
    pub(super) publication: crate::mounting::UiMountedFramePublicationCandidate,
    pub(super) handle: crate::mounting::UiMountedPresentationInFlight,
}

pub struct WorthUiMountedReplacementAdmissionDenial<'session> {
    pub(super) denial: crate::mounting::UiMountedPresentationAdmissionDenial,
    pub(super) replacement: Box<WorthUiPreparedMountedApplicationReplacement<'session>>,
}

pub struct WorthUiMountedReplacementRetentionDenial<'session> {
    pub(super) denial: crate::mounting::UiMountedFrameRetentionDenial,
    pub(super) replacement: Box<WorthUiPreparedMountedApplicationReplacement<'session>>,
}

pub struct WorthUiMountedReplacementCompletionDenial<'session> {
    pub(super) denial: crate::mounting::UiMountedPresentationCompletionDenial,
    pub(super) in_flight: WorthUiMountedApplicationReplacementInFlight<'session>,
}

pub enum WorthUiMountedReplacementPreparationOutcome<'session> {
    SemanticNoOp(Box<WorthUiApplicationSemanticNoOpReceipt>),
    Prepared(Box<WorthUiPreparedMountedApplicationReplacement<'session>>),
}

pub enum WorthUiMountedApplicationReplacementOutcome<'session> {
    Published {
        application: WorthUiApplicationCutoverReceipt,
        mounted: crate::mounting::UiMountedFramePublicationReceipt,
    },
    RejectedBeforeEffects(Box<WorthUiPreparedMountedApplicationReplacement<'session>>),
    InFlight(Box<WorthUiMountedApplicationReplacementInFlight<'session>>),
    PresentationIndeterminate(Box<crate::mounting::UiMountedIndeterminateFrame>),
    RetentionDenied(WorthUiMountedReplacementRetentionDenial<'session>),
    AdmissionDenied(WorthUiMountedReplacementAdmissionDenial<'session>),
    CompletionDenied(Box<WorthUiMountedReplacementCompletionDenial<'session>>),
}

impl<'session> WorthUiMountedReplacementAdmissionDenial<'session> {
    pub fn denial(&self) -> crate::mounting::UiMountedPresentationAdmissionDenial {
        self.denial
    }

    pub fn into_replacement(self) -> Box<WorthUiPreparedMountedApplicationReplacement<'session>> {
        self.replacement
    }
}

impl<'session> WorthUiMountedReplacementRetentionDenial<'session> {
    pub fn denial(&self) -> crate::mounting::UiMountedFrameRetentionDenial {
        self.denial
    }

    pub fn into_replacement(self) -> Box<WorthUiPreparedMountedApplicationReplacement<'session>> {
        self.replacement
    }
}

impl<'session> WorthUiMountedReplacementCompletionDenial<'session> {
    pub fn denial(&self) -> crate::mounting::UiMountedPresentationCompletionDenial {
        self.denial
    }

    pub fn into_in_flight(
        self: Box<Self>,
    ) -> WorthUiMountedApplicationReplacementInFlight<'session> {
        self.in_flight
    }
}
