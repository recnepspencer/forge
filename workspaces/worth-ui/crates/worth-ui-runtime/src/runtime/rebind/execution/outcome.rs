use super::state::UiRebindReservation;

mod completion_handle;
mod content_mapping;
mod denial_receipt;
mod mapping;

pub(crate) use content_mapping::map_content_first_attempt;
pub(crate) use mapping::map_changed_first_attempt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiDuplicateObservationReceipt {
    turn: crate::runtime::observation::UiObservationTurnIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiRebindValidNextAction {
    RetryPrepared,
    ManageCompletion,
    RecoverHostTruth,
    ReportDefect,
    None,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiRebindStoppedPhase {
    ObservationAdmission,
    FinalAdmission,
    EffectAdmission,
    MountedRetentionAdmission,
    MountedPresentationAdmission,
    HostPresentation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiRebindDenialCause {
    RuntimeCapacity(super::UiRebindReservationDenial),
    MountedRetention(crate::mounting::UiMountedFrameRetentionDenial),
    MountedPresentation(crate::mounting::UiMountedPresentationAdmissionDenial),
    HostRejectedBeforeEffects,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiRebindCancellationReceipt {
    predecessor_remains_current: bool,
    stopped_phase: UiRebindStoppedPhase,
    valid_next_action: UiRebindValidNextAction,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiRebindTimeoutReceipt {
    predecessor_remains_current: bool,
    stopped_phase: UiRebindStoppedPhase,
    valid_next_action: UiRebindValidNextAction,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiRebindSupersededReceipt {
    predecessor_remains_current: bool,
    stopped_phase: UiRebindStoppedPhase,
    valid_next_action: UiRebindValidNextAction,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiRebindInternalDefectKind {
    PlannedRealizedMismatch,
    UnexpectedCancellationPublication,
}

pub struct UiRebindDenialReceipt<'session> {
    predecessor_remains_current: bool,
    stopped_phase: UiRebindStoppedPhase,
    cause: UiRebindDenialCause,
    valid_next_action: UiRebindValidNextAction,
    retry: Option<Box<super::UiPreparedRebind<'session>>>,
}

#[must_use = "completion authority must be completed, disposed, or dropped through cancellation"]
pub struct UiRebindCompletionHandle<'session> {
    state: Option<Box<UiRebindCompletionState<'session>>>,
}

struct UiRebindCompletionState<'session> {
    plan: crate::runtime::rebind::UiRebindPlan,
    registration: UiRebindReservation,
    inner: UiRebindCompletionInner<'session>,
}

enum UiRebindCompletionInner<'session> {
    Changed(Box<crate::facade::WorthUiMountedApplicationReplacementInFlight<'session>>),
    Content {
        generation: crate::facade::prepared_application_authority::
            WorthUiPreparedApplicationGenerationIdentity,
        inner: Box<crate::facade::entry::WorthUiMountedContentRebindInFlight<'session>>,
    },
}

pub struct UiRebindInternalDefectOutcome {
    kind: UiRebindInternalDefectKind,
    publication: Option<Box<UiDefectivePublication>>,
    _unpublished_plan: Option<Box<crate::runtime::rebind::UiRebindPlan>>,
    _registration: UiRebindReservation,
}

enum UiDefectivePublication {
    Changed {
        _plan: crate::runtime::rebind::UiRebindPlan,
        _application: crate::facade::WorthUiApplicationCutoverReceipt,
        _mounted: crate::mounting::UiMountedFramePublicationReceipt,
    },
    EvidenceOnly {
        _plan: crate::runtime::rebind::UiRebindPlan,
        _prior: crate::facade::prepared_application_authority::
            WorthUiPreparedApplicationGenerationIdentity,
        _active: crate::facade::prepared_application_authority::
            WorthUiPreparedApplicationGenerationIdentity,
    },
    Content {
        _plan: crate::runtime::rebind::UiRebindPlan,
        _generation: crate::facade::prepared_application_authority::
            WorthUiPreparedApplicationGenerationIdentity,
        _mounted: crate::mounting::UiMountedFramePublicationReceipt,
    },
}

pub enum UiRebindOutcome<'session> {
    Duplicate(UiDuplicateObservationReceipt),
    ObservedNoChange(crate::runtime::observation::UiObservedNoChangeReceipt),
    RejectedBeforeEffects(UiRebindDenialReceipt<'session>),
    CancelledBeforeEffects(UiRebindCancellationReceipt),
    TimedOutBeforeEffects(UiRebindTimeoutReceipt),
    SupersededBeforeEffects(UiRebindSupersededReceipt),
    Published(super::UiRebindReceipt),
    InFlight(UiRebindCompletionHandle<'session>),
    Indeterminate(super::UiRebindRecoveryHandle<'session>),
    InternalDefect(UiRebindInternalDefectOutcome),
}

impl UiRebindCancellationReceipt {
    pub(crate) const fn cancelled() -> Self {
        Self {
            predecessor_remains_current: true,
            stopped_phase: UiRebindStoppedPhase::FinalAdmission,
            valid_next_action: UiRebindValidNextAction::None,
        }
    }

    pub const fn predecessor_remains_current(self) -> bool {
        self.predecessor_remains_current
    }

    pub const fn stopped_phase(self) -> UiRebindStoppedPhase {
        self.stopped_phase
    }

    pub const fn valid_next_action(self) -> UiRebindValidNextAction {
        self.valid_next_action
    }
}

impl UiDuplicateObservationReceipt {
    pub(crate) const fn new(turn: crate::runtime::observation::UiObservationTurnIdentity) -> Self {
        Self { turn }
    }

    pub const fn turn(self) -> crate::runtime::observation::UiObservationTurnIdentity {
        self.turn
    }
}

impl UiRebindTimeoutReceipt {
    pub(crate) const fn elapsed() -> Self {
        Self {
            predecessor_remains_current: true,
            stopped_phase: UiRebindStoppedPhase::FinalAdmission,
            valid_next_action: UiRebindValidNextAction::None,
        }
    }

    pub const fn predecessor_remains_current(self) -> bool {
        self.predecessor_remains_current
    }

    pub const fn stopped_phase(self) -> UiRebindStoppedPhase {
        self.stopped_phase
    }

    pub const fn valid_next_action(self) -> UiRebindValidNextAction {
        self.valid_next_action
    }
}

impl UiRebindSupersededReceipt {
    pub(crate) const fn before_effects(stopped_phase: UiRebindStoppedPhase) -> Self {
        Self {
            predecessor_remains_current: true,
            stopped_phase,
            valid_next_action: UiRebindValidNextAction::None,
        }
    }

    pub const fn predecessor_remains_current(self) -> bool {
        self.predecessor_remains_current
    }

    pub const fn stopped_phase(self) -> UiRebindStoppedPhase {
        self.stopped_phase
    }

    pub const fn valid_next_action(self) -> UiRebindValidNextAction {
        self.valid_next_action
    }
}

impl UiRebindInternalDefectOutcome {
    pub(crate) fn published_mismatch(
        plan: crate::runtime::rebind::UiRebindPlan,
        registration: UiRebindReservation,
        application: crate::facade::WorthUiApplicationCutoverReceipt,
        mounted: crate::mounting::UiMountedFramePublicationReceipt,
    ) -> Self {
        Self {
            kind: UiRebindInternalDefectKind::PlannedRealizedMismatch,
            publication: Some(Box::new(UiDefectivePublication::Changed {
                _plan: plan,
                _application: application,
                _mounted: mounted,
            })),
            _unpublished_plan: None,
            _registration: registration,
        }
    }

    pub(crate) fn evidence_mismatch(
        plan: crate::runtime::rebind::UiRebindPlan,
        registration: UiRebindReservation,
        prior: crate::facade::prepared_application_authority::
            WorthUiPreparedApplicationGenerationIdentity,
        active: crate::facade::prepared_application_authority::
            WorthUiPreparedApplicationGenerationIdentity,
    ) -> Self {
        Self {
            kind: UiRebindInternalDefectKind::PlannedRealizedMismatch,
            publication: Some(Box::new(UiDefectivePublication::EvidenceOnly {
                _plan: plan,
                _prior: prior,
                _active: active,
            })),
            _unpublished_plan: None,
            _registration: registration,
        }
    }

    pub(crate) fn content_mismatch(
        plan: crate::runtime::rebind::UiRebindPlan,
        registration: UiRebindReservation,
        generation: crate::facade::prepared_application_authority::
            WorthUiPreparedApplicationGenerationIdentity,
        mounted: crate::mounting::UiMountedFramePublicationReceipt,
    ) -> Self {
        Self {
            kind: UiRebindInternalDefectKind::PlannedRealizedMismatch,
            publication: Some(Box::new(UiDefectivePublication::Content {
                _plan: plan,
                _generation: generation,
                _mounted: mounted,
            })),
            _unpublished_plan: None,
            _registration: registration,
        }
    }

    pub(crate) fn unexpected_cancellation_publication(
        plan: crate::runtime::rebind::UiRebindPlan,
        mut registration: UiRebindReservation,
        application: crate::facade::WorthUiApplicationCutoverReceipt,
        mounted: crate::mounting::UiMountedFramePublicationReceipt,
    ) -> Self {
        registration
            .retain_recovery()
            .expect("effect admission reserved recovery capacity");
        Self {
            kind: UiRebindInternalDefectKind::UnexpectedCancellationPublication,
            publication: Some(Box::new(UiDefectivePublication::Changed {
                _plan: plan,
                _application: application,
                _mounted: mounted,
            })),
            _unpublished_plan: None,
            _registration: registration,
        }
    }

    pub(crate) fn unexpected_content_cancellation_publication(
        plan: crate::runtime::rebind::UiRebindPlan,
        mut registration: UiRebindReservation,
        generation: crate::facade::prepared_application_authority::
            WorthUiPreparedApplicationGenerationIdentity,
        mounted: crate::mounting::UiMountedFramePublicationReceipt,
    ) -> Self {
        registration
            .retain_recovery()
            .expect("effect admission reserved recovery capacity");
        Self {
            kind: UiRebindInternalDefectKind::UnexpectedCancellationPublication,
            publication: Some(Box::new(UiDefectivePublication::Content {
                _plan: plan,
                _generation: generation,
                _mounted: mounted,
            })),
            _unpublished_plan: None,
            _registration: registration,
        }
    }

    pub(crate) fn completion_authority_rejected(
        plan: crate::runtime::rebind::UiRebindPlan,
        registration: UiRebindReservation,
    ) -> Self {
        Self {
            kind: UiRebindInternalDefectKind::PlannedRealizedMismatch,
            publication: None,
            _unpublished_plan: Some(Box::new(plan)),
            _registration: registration,
        }
    }

    pub const fn kind(&self) -> UiRebindInternalDefectKind {
        self.kind
    }

    pub const fn publication_occurred(&self) -> bool {
        self.publication.is_some()
    }

    pub const fn valid_next_action(&self) -> UiRebindValidNextAction {
        UiRebindValidNextAction::ReportDefect
    }

    pub const fn retains_recovery_authority(&self) -> bool {
        self.publication.is_some()
    }
}
