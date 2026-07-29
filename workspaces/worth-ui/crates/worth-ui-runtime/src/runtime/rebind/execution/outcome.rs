use super::state::UiRebindReservation;

mod mapping;

use mapping::map_changed_completion;
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
    inner: Box<crate::facade::WorthUiMountedApplicationReplacementInFlight<'session>>,
}

pub struct UiRebindInternalDefectOutcome {
    kind: UiRebindInternalDefectKind,
    publication: Option<Box<UiDefectivePublication>>,
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

impl<'session> UiRebindDenialReceipt<'session> {
    pub(crate) fn capacity(
        denial: super::UiRebindReservationDenial,
        retry: super::UiPreparedRebind<'session>,
    ) -> Self {
        Self {
            predecessor_remains_current: true,
            stopped_phase: UiRebindStoppedPhase::EffectAdmission,
            cause: UiRebindDenialCause::RuntimeCapacity(denial),
            valid_next_action: UiRebindValidNextAction::RetryPrepared,
            retry: Some(Box::new(retry)),
        }
    }

    fn retry(
        plan: crate::runtime::rebind::UiRebindPlan,
        registration: UiRebindReservation,
        replacement: Box<crate::facade::WorthUiPreparedMountedApplicationReplacement<'session>>,
        stopped_phase: UiRebindStoppedPhase,
        cause: UiRebindDenialCause,
    ) -> Self {
        Self {
            predecessor_remains_current: true,
            stopped_phase,
            cause,
            valid_next_action: UiRebindValidNextAction::RetryPrepared,
            retry: Some(Box::new(super::UiPreparedRebind::changed(
                plan,
                registration,
                replacement,
            ))),
        }
    }

    pub const fn predecessor_remains_current(&self) -> bool {
        self.predecessor_remains_current
    }

    pub const fn stopped_phase(&self) -> UiRebindStoppedPhase {
        self.stopped_phase
    }

    pub const fn cause(&self) -> UiRebindDenialCause {
        self.cause
    }

    pub const fn valid_next_action(&self) -> UiRebindValidNextAction {
        self.valid_next_action
    }

    pub fn retry_at(mut self, now_tick: u64) -> UiRebindOutcome<'session> {
        match self.retry.take() {
            Some(retry) => retry.execute(now_tick),
            None => UiRebindOutcome::RejectedBeforeEffects(self),
        }
    }
}

impl<'session> UiRebindCompletionHandle<'session> {
    pub(super) fn new(
        plan: crate::runtime::rebind::UiRebindPlan,
        registration: UiRebindReservation,
        inner: Box<crate::facade::WorthUiMountedApplicationReplacementInFlight<'session>>,
    ) -> Self {
        Self {
            state: Some(Box::new(UiRebindCompletionState {
                plan,
                registration,
                inner,
            })),
        }
    }

    pub fn attempt(&self) -> worth_ui_host_contract::UiMountedPresentationAttemptIdentity {
        self.state().inner.attempt()
    }

    pub fn deadline(&self) -> worth_ui_host_contract::UiPresentationDeadline {
        self.state().inner.deadline()
    }

    pub fn complete(self, now_tick: u64) -> UiRebindOutcome<'session> {
        let state = self.into_state();
        let outcome = state.inner.complete(now_tick);
        map_changed_completion(state.plan, state.registration, outcome)
    }

    pub fn dispose(self) -> UiRebindOutcome<'session> {
        let state = self.into_state();
        let outcome = state.inner.cancel();
        mapping::map_changed_cancellation(state.plan, state.registration, outcome)
    }

    fn state(&self) -> &UiRebindCompletionState<'session> {
        self.state
            .as_deref()
            .expect("live completion handle owns its state")
    }

    fn into_state(mut self) -> Box<UiRebindCompletionState<'session>> {
        self.state
            .take()
            .expect("live completion handle owns its state")
    }
}

impl Drop for UiRebindCompletionHandle<'_> {
    fn drop(&mut self) {
        let Some(state) = self.state.take() else {
            return;
        };
        let outcome = state.inner.cancel();
        drop(mapping::map_changed_cancellation(
            state.plan,
            state.registration,
            outcome,
        ));
    }
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
