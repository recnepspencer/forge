use worth_ui_host_contract::{
    UiHostObservationFamily, UiHostObservationPresentationBasis, UiSemanticSurfaceIdentity,
    UiSurfaceBindingGeneration,
};

use super::UiDraftSessionIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiLocalInputStopReason {
    NoLocalRecipient,
    MissingInputRecipientAffinity,
    InputRecipientAffinityChanged,
    TextProfileGenerationChanged {
        expected: worth_ui_host_contract::UiTextProfileGeneration,
        observed: Option<worth_ui_host_contract::UiTextProfileGeneration>,
    },
    ForeignBinding {
        expected: UiSurfaceBindingGeneration,
        observed: UiSurfaceBindingGeneration,
    },
    ApplicationGenerationChanged,
    TargetNoLongerCurrent(crate::runtime::interaction::UiInteractionTargetingDenial),
    InputRevisionDiscontinuity {
        previous: u64,
        observed: u64,
    },
    DraftByteBudgetExceeded {
        limit: usize,
        attempted: usize,
    },
    RecipientFamilyMismatch {
        required: super::UiLocalInputRecipientFamily,
        active: super::UiLocalInputRecipientFamily,
    },
    CompositionActive,
    RecipientReplaced,
    ExplicitCancel,
    FocusLost,
    ObservationInvalid,
    ObservationLoss(UiHostObservationFamily),
    SurfaceRebound,
    MountedInstanceRemoved,
    ApplicationRebound,
    Shutdown,
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiLocalInputStop {
    session: Option<UiDraftSessionIdentity>,
    surface: Option<UiSemanticSurfaceIdentity>,
    presentation: Option<UiHostObservationPresentationBasis>,
    settled_recipient: bool,
    settled_session: bool,
    reason: UiLocalInputStopReason,
}

impl UiLocalInputStop {
    pub(super) const fn for_settled_session(
        session: UiDraftSessionIdentity,
        surface: UiSemanticSurfaceIdentity,
        presentation: UiHostObservationPresentationBasis,
        reason: UiLocalInputStopReason,
    ) -> Self {
        Self {
            session: Some(session),
            surface: Some(surface),
            presentation: Some(presentation),
            settled_recipient: true,
            settled_session: true,
            reason,
        }
    }

    pub(super) const fn for_settled_recipient(
        surface: UiSemanticSurfaceIdentity,
        presentation: UiHostObservationPresentationBasis,
        reason: UiLocalInputStopReason,
    ) -> Self {
        Self {
            session: None,
            surface: Some(surface),
            presentation: Some(presentation),
            settled_recipient: true,
            settled_session: false,
            reason,
        }
    }

    pub(super) const fn for_suspended_session(
        session: UiDraftSessionIdentity,
        surface: UiSemanticSurfaceIdentity,
        presentation: UiHostObservationPresentationBasis,
        reason: UiLocalInputStopReason,
    ) -> Self {
        Self {
            session: Some(session),
            surface: Some(surface),
            presentation: Some(presentation),
            settled_recipient: true,
            settled_session: false,
            reason,
        }
    }

    pub(super) const fn for_unsettled_report(
        presentation: UiHostObservationPresentationBasis,
        reason: UiLocalInputStopReason,
    ) -> Self {
        Self {
            session: None,
            surface: None,
            presentation: Some(presentation),
            settled_recipient: false,
            settled_session: false,
            reason,
        }
    }

    pub(super) const fn for_unsettled_session(
        session: UiDraftSessionIdentity,
        surface: UiSemanticSurfaceIdentity,
        presentation: UiHostObservationPresentationBasis,
        reason: UiLocalInputStopReason,
    ) -> Self {
        Self {
            session: Some(session),
            surface: Some(surface),
            presentation: Some(presentation),
            settled_recipient: false,
            settled_session: false,
            reason,
        }
    }

    pub const fn session(&self) -> Option<UiDraftSessionIdentity> {
        self.session
    }

    pub const fn surface(&self) -> Option<UiSemanticSurfaceIdentity> {
        self.surface
    }

    pub const fn presentation(&self) -> Option<UiHostObservationPresentationBasis> {
        self.presentation
    }

    pub const fn settled_session(&self) -> bool {
        self.settled_session
    }

    pub const fn settled_recipient(&self) -> bool {
        self.settled_recipient
    }

    pub const fn reason(&self) -> UiLocalInputStopReason {
        self.reason
    }
}
