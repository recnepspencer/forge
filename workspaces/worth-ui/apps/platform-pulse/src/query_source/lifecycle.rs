use worth_ui::facade::query_binding::{
    UiProjectionObservation, UiScalarProjectionObservation, WorthUiScalarProjectionActionAdvance,
    WorthUiScalarProjectionActionEvidence, WorthUiScalarProjectionActionIndeterminate,
    WorthUiScalarProjectionActionLiveOwner, WorthUiScalarProjectionActionOutcome,
    WorthUiScalarProjectionActionPublicationCompletion, WorthUiScalarProjectionActionRequest,
    WorthUiScalarProjectionAdvanceError, WorthUiScalarProjectionSourceCloseError,
    WorthUiScalarProjectionSourceCloseReceipt, WorthUiScalarProjectionSourceRecord,
};

pub(crate) struct PlatformPulseQueryLifecycle {
    initial: Option<WorthUiScalarProjectionActionAdvance>,
    state: PlatformPulseQueryOwnerState,
}

enum PlatformPulseQueryOwnerState {
    BeforeInitial,
    AwaitingPublication(WorthUiScalarProjectionActionPublicationCompletion),
    Live(WorthUiScalarProjectionActionLiveOwner),
    Indeterminate(WorthUiScalarProjectionActionIndeterminate),
    Closed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PlatformPulseQuerySourceRevision(u64);

pub(crate) enum PlatformPulseQueryActionOutcome {
    Executed {
        evidence: WorthUiScalarProjectionActionEvidence,
        observation: UiProjectionObservation,
    },
    Denied {
        denial: worth_ui::facade::query_binding::WorthUiScalarProjectionActionDenial,
        active_query_source_revision: u64,
        submitted_query_source_revision: u64,
    },
    Indeterminate {
        detail: String,
    },
}

#[derive(Debug)]
pub(crate) enum PlatformPulseQueryLifecycleDenial {
    InitialAlreadyIssued,
    PublicationAlreadyPending,
    PublicationNotPending,
    OwnerNotLive,
    ActionRequest(&'static str),
    Advance(WorthUiScalarProjectionAdvanceError),
    ForeignPublication,
    Close(Box<WorthUiScalarProjectionSourceCloseError>),
}

impl std::fmt::Display for PlatformPulseQueryLifecycleDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InitialAlreadyIssued => formatter.write_str("initial fact already issued"),
            Self::PublicationAlreadyPending => formatter.write_str("publication already pending"),
            Self::PublicationNotPending => formatter.write_str("publication is not pending"),
            Self::OwnerNotLive => formatter.write_str("live owner unavailable"),
            Self::ActionRequest(denial) => write!(formatter, "action request: {denial}"),
            Self::Advance(denial) => write!(formatter, "advance: {denial:?}"),
            Self::ForeignPublication => formatter.write_str("foreign publication fact"),
            Self::Close(denial) => write!(formatter, "close: {denial:?}"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PlatformPulseQueryShutdownReceipt {
    owner_terminal: bool,
    live_source_count: usize,
    live_attempt_count: usize,
    live_resource_count: usize,
    live_consumer_lease_count: usize,
    retained_projection_count: usize,
    projection_receipt_count: usize,
}

impl PlatformPulseQueryLifecycle {
    pub(crate) fn new(initial: WorthUiScalarProjectionActionAdvance) -> Self {
        Self {
            initial: Some(initial),
            state: PlatformPulseQueryOwnerState::BeforeInitial,
        }
    }

    pub(crate) fn issue_initial(
        &mut self,
    ) -> Result<UiProjectionObservation, PlatformPulseQueryLifecycleDenial> {
        if !matches!(self.state, PlatformPulseQueryOwnerState::BeforeInitial) {
            return Err(PlatformPulseQueryLifecycleDenial::InitialAlreadyIssued);
        }
        let initial = self
            .initial
            .take()
            .ok_or(PlatformPulseQueryLifecycleDenial::InitialAlreadyIssued)?;
        let (observation, completion) = initial.into_parts();
        self.state = PlatformPulseQueryOwnerState::AwaitingPublication(completion);
        Ok(observation)
    }

    pub(crate) fn advance(
        &mut self,
        record: WorthUiScalarProjectionSourceRecord,
    ) -> Result<UiProjectionObservation, PlatformPulseQueryLifecycleDenial> {
        let owner = self.take_live_owner()?;
        let advance = owner
            .advance_source(record)
            .map_err(PlatformPulseQueryLifecycleDenial::Advance)?;
        Ok(self.retain_advance(advance))
    }

    pub(crate) fn execute_current_action(
        &mut self,
        status: impl Into<String>,
    ) -> Result<PlatformPulseQueryActionOutcome, PlatformPulseQueryLifecycleDenial> {
        let source_revision = self.current_source_revision()?;
        let request = WorthUiScalarProjectionActionRequest::new(source_revision.value(), status)
            .map_err(PlatformPulseQueryLifecycleDenial::ActionRequest)?;
        let owner = self.take_live_owner()?;
        Ok(self.retain_action_outcome(owner.execute_action(request)))
    }

    /// Exercises the real Query action boundary with a deliberately foreign
    /// revision. This is a Pulse product scenario, not a synthetic denial.
    pub(crate) fn execute_denied_action(
        &mut self,
        status: impl Into<String>,
    ) -> Result<PlatformPulseQueryActionOutcome, PlatformPulseQueryLifecycleDenial> {
        let source_revision = self.current_source_revision()?;
        let submitted = source_revision.value().saturating_add(1);
        let request = WorthUiScalarProjectionActionRequest::new(submitted, status)
            .map_err(PlatformPulseQueryLifecycleDenial::ActionRequest)?;
        let owner = self.take_live_owner()?;
        Ok(self.retain_action_outcome(owner.execute_action(request)))
    }

    fn retain_action_outcome(
        &mut self,
        outcome: WorthUiScalarProjectionActionOutcome,
    ) -> PlatformPulseQueryActionOutcome {
        match outcome {
            WorthUiScalarProjectionActionOutcome::Executed(execution) => {
                let (evidence, advance) = execution.into_parts();
                let observation = self.retain_advance(advance);
                PlatformPulseQueryActionOutcome::Executed {
                    evidence,
                    observation,
                }
            }
            WorthUiScalarProjectionActionOutcome::Denied(denied) => {
                let denial = denied.denial();
                let active_query_source_revision = denied.active_revision();
                let submitted_query_source_revision = denied.submitted_revision();
                self.state = PlatformPulseQueryOwnerState::Live(denied.into_owner());
                PlatformPulseQueryActionOutcome::Denied {
                    denial,
                    active_query_source_revision,
                    submitted_query_source_revision,
                }
            }
            WorthUiScalarProjectionActionOutcome::Indeterminate(indeterminate) => {
                let detail = indeterminate.detail().to_owned();
                self.state = PlatformPulseQueryOwnerState::Indeterminate(indeterminate);
                PlatformPulseQueryActionOutcome::Indeterminate { detail }
            }
        }
    }

    fn current_source_revision(
        &self,
    ) -> Result<PlatformPulseQuerySourceRevision, PlatformPulseQueryLifecycleDenial> {
        match &self.state {
            PlatformPulseQueryOwnerState::Live(owner) => {
                Ok(PlatformPulseQuerySourceRevision(owner.source_revision()))
            }
            _ => Err(PlatformPulseQueryLifecycleDenial::OwnerNotLive),
        }
    }

    pub(crate) fn admit_publication(
        &mut self,
        observation: UiScalarProjectionObservation,
    ) -> Result<(), PlatformPulseQueryLifecycleDenial> {
        let state = std::mem::replace(&mut self.state, PlatformPulseQueryOwnerState::Closed);
        let PlatformPulseQueryOwnerState::AwaitingPublication(completion) = state else {
            self.state = state;
            return Err(PlatformPulseQueryLifecycleDenial::PublicationNotPending);
        };
        match completion.admit_publication(observation) {
            Ok(owner) => {
                self.state = PlatformPulseQueryOwnerState::Live(owner);
                Ok(())
            }
            Err(_) => Err(PlatformPulseQueryLifecycleDenial::ForeignPublication),
        }
    }

    pub(crate) fn close(
        mut self,
    ) -> Result<PlatformPulseQueryShutdownReceipt, PlatformPulseQueryLifecycleDenial> {
        let state = std::mem::replace(&mut self.state, PlatformPulseQueryOwnerState::Closed);
        let receipt = match state {
            PlatformPulseQueryOwnerState::Live(owner) => owner.close(),
            PlatformPulseQueryOwnerState::Indeterminate(owner) => owner.close(),
            _ => return Err(PlatformPulseQueryLifecycleDenial::OwnerNotLive),
        };
        receipt
            .map(PlatformPulseQueryShutdownReceipt::from)
            .map_err(|denial| PlatformPulseQueryLifecycleDenial::Close(Box::new(denial)))
    }

    fn take_live_owner(
        &mut self,
    ) -> Result<WorthUiScalarProjectionActionLiveOwner, PlatformPulseQueryLifecycleDenial> {
        let state = std::mem::replace(&mut self.state, PlatformPulseQueryOwnerState::Closed);
        let PlatformPulseQueryOwnerState::Live(owner) = state else {
            self.state = state;
            return Err(match self.state {
                PlatformPulseQueryOwnerState::AwaitingPublication(_) => {
                    PlatformPulseQueryLifecycleDenial::PublicationAlreadyPending
                }
                _ => PlatformPulseQueryLifecycleDenial::OwnerNotLive,
            });
        };
        Ok(owner)
    }

    fn retain_advance(
        &mut self,
        advance: WorthUiScalarProjectionActionAdvance,
    ) -> UiProjectionObservation {
        let (observation, completion) = advance.into_parts();
        self.state = PlatformPulseQueryOwnerState::AwaitingPublication(completion);
        observation
    }
}

impl PlatformPulseQuerySourceRevision {
    const fn value(self) -> u64 {
        self.0
    }
}

impl From<WorthUiScalarProjectionSourceCloseReceipt> for PlatformPulseQueryShutdownReceipt {
    fn from(receipt: WorthUiScalarProjectionSourceCloseReceipt) -> Self {
        Self {
            owner_terminal: receipt.owner_terminal(),
            live_source_count: receipt.live_source_count(),
            live_attempt_count: receipt.live_attempt_count(),
            live_resource_count: receipt.live_resource_count(),
            live_consumer_lease_count: receipt.live_consumer_lease_count(),
            retained_projection_count: receipt.retained_projection_count(),
            projection_receipt_count: receipt.projection_receipt_count(),
        }
    }
}

impl PlatformPulseQueryShutdownReceipt {
    pub(crate) const fn owner_terminal(self) -> bool {
        self.owner_terminal
    }
    pub(crate) const fn live_source_count(self) -> usize {
        self.live_source_count
    }
    pub(crate) const fn live_attempt_count(self) -> usize {
        self.live_attempt_count
    }
    pub(crate) const fn live_resource_count(self) -> usize {
        self.live_resource_count
    }
    pub(crate) const fn live_consumer_lease_count(self) -> usize {
        self.live_consumer_lease_count
    }
    pub(crate) const fn retained_projection_count(self) -> usize {
        self.retained_projection_count
    }
    pub(crate) const fn projection_receipt_count(self) -> usize {
        self.projection_receipt_count
    }
}
