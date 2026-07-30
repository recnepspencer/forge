use worth_ui::facade::query_binding::{
    UiProjectionObservation, UiScalarProjectionFactReceipt, WorthUiScalarProjectionAdvance,
    WorthUiScalarProjectionAdvanceError, WorthUiScalarProjectionLiveOwner,
    WorthUiScalarProjectionPublicationCompletion, WorthUiScalarProjectionSourceCloseError,
    WorthUiScalarProjectionSourceCloseReceipt, WorthUiScalarProjectionSourceRecord,
};

pub(crate) struct PlatformPulseQueryLifecycle {
    initial: Option<WorthUiScalarProjectionAdvance>,
    state: PlatformPulseQueryOwnerState,
}

enum PlatformPulseQueryOwnerState {
    BeforeInitial,
    AwaitingPublication(WorthUiScalarProjectionPublicationCompletion),
    Live(WorthUiScalarProjectionLiveOwner),
    Closed,
}

#[derive(Debug)]
pub(crate) enum PlatformPulseQueryLifecycleDenial {
    InitialAlreadyIssued,
    PublicationAlreadyPending,
    PublicationNotPending,
    OwnerNotLive,
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
    pub(crate) fn new(initial: WorthUiScalarProjectionAdvance) -> Self {
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
        let advance = owner
            .advance(record)
            .map_err(PlatformPulseQueryLifecycleDenial::Advance)?;
        let (observation, completion) = advance.into_parts();
        self.state = PlatformPulseQueryOwnerState::AwaitingPublication(completion);
        Ok(observation)
    }

    pub(crate) fn admit_publication(
        &mut self,
        fact: UiScalarProjectionFactReceipt,
    ) -> Result<(), PlatformPulseQueryLifecycleDenial> {
        let state = std::mem::replace(&mut self.state, PlatformPulseQueryOwnerState::Closed);
        let PlatformPulseQueryOwnerState::AwaitingPublication(completion) = state else {
            self.state = state;
            return Err(PlatformPulseQueryLifecycleDenial::PublicationNotPending);
        };
        match completion.admit_publication(fact) {
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
        match state {
            PlatformPulseQueryOwnerState::Live(owner) => owner
                .close()
                .map(PlatformPulseQueryShutdownReceipt::from)
                .map_err(|denial| PlatformPulseQueryLifecycleDenial::Close(Box::new(denial))),
            _ => Err(PlatformPulseQueryLifecycleDenial::OwnerNotLive),
        }
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
    pub(crate) fn owner_terminal(self) -> bool {
        self.owner_terminal
    }

    pub(crate) fn live_source_count(self) -> usize {
        self.live_source_count
    }

    pub(crate) fn live_attempt_count(self) -> usize {
        self.live_attempt_count
    }

    pub(crate) fn live_resource_count(self) -> usize {
        self.live_resource_count
    }

    pub(crate) fn live_consumer_lease_count(self) -> usize {
        self.live_consumer_lease_count
    }

    pub(crate) fn retained_projection_count(self) -> usize {
        self.retained_projection_count
    }

    pub(crate) fn projection_receipt_count(self) -> usize {
        self.projection_receipt_count
    }
}
