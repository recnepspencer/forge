#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiFocusRoutingDenial {
    UnknownScope,
    UnknownParticipant,
    StaleParticipantIncarnation,
    StalePlan,
    RevisionExhausted,
    VisitCounterOverflow,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) struct UiFocusPlan {
    expected_revision: u64,
    next: Option<super::UiSemanticKeyboardFocus>,
    cause: super::UiFocusCause,
    outcome: super::UiFocusOutcome,
    participants_visited: u32,
}

impl UiFocusPlan {
    pub(super) const fn new(
        expected_revision: u64,
        next: Option<super::UiSemanticKeyboardFocus>,
        cause: super::UiFocusCause,
        outcome: super::UiFocusOutcome,
        participants_visited: u32,
    ) -> Self {
        Self {
            expected_revision,
            next,
            cause,
            outcome,
            participants_visited,
        }
    }

    pub(super) const fn expected_revision(self) -> u64 {
        self.expected_revision
    }
    pub(super) const fn next(self) -> Option<super::UiSemanticKeyboardFocus> {
        self.next
    }
    pub(super) const fn cause(self) -> super::UiFocusCause {
        self.cause
    }
    pub(super) const fn outcome(self) -> super::UiFocusOutcome {
        self.outcome
    }
    pub(super) const fn participants_visited(self) -> u32 {
        self.participants_visited
    }
}

impl super::UiFocusRuntimeState {
    pub(in crate::runtime) fn plan(
        &self,
        request: super::UiFocusRequest,
    ) -> Result<UiFocusPlan, UiFocusRoutingDenial> {
        match request {
            super::UiFocusRequest::Direct {
                scope,
                participant,
                incarnation,
                cause,
            } => {
                let participant = self.exact_participant(scope, participant, incarnation)?;
                Ok(self.plan_for(Some(participant), cause, 1))
            }
            super::UiFocusRequest::Traverse {
                scope,
                direction,
                wrap,
            } => self.plan_traversal(scope, direction, wrap),
            super::UiFocusRequest::First { scope, cause } => {
                Ok(self.plan_for(self.first_in_scope(scope), cause, 1))
            }
            super::UiFocusRequest::Last { scope, cause } => {
                Ok(self.plan_for(self.last_in_scope(scope), cause, 1))
            }
            super::UiFocusRequest::Restore(token) => self.plan_restoration(token),
            super::UiFocusRequest::Clear { cause } => Ok(self.plan_for(None, cause, 0)),
        }
    }

    fn plan_traversal(
        &self,
        scope: super::UiFocusScopeIdentity,
        direction: super::UiFocusTraversalDirection,
        wrap: bool,
    ) -> Result<UiFocusPlan, UiFocusRoutingDenial> {
        let scoped = self
            .participants
            .get(&scope)
            .ok_or(UiFocusRoutingDenial::UnknownScope)?;
        let current_index = self.current.and_then(|current| {
            (current.scope() == scope)
                .then(|| {
                    self.participant_index
                        .get(&current.participant())
                        .map(|entry| entry.1)
                })
                .flatten()
        });
        let next = match (direction, current_index) {
            (super::UiFocusTraversalDirection::Forward, Some(index)) => scoped
                .get(index + 1)
                .copied()
                .or_else(|| wrap.then(|| scoped.first().copied()).flatten()),
            (super::UiFocusTraversalDirection::Backward, Some(index)) => index
                .checked_sub(1)
                .and_then(|previous| scoped.get(previous).copied())
                .or_else(|| wrap.then(|| scoped.last().copied()).flatten()),
            (super::UiFocusTraversalDirection::Forward, None) => scoped.first().copied(),
            (super::UiFocusTraversalDirection::Backward, None) => scoped.last().copied(),
        };
        Ok(self.plan_for(
            next,
            super::UiFocusCause::KeyboardTraversal,
            u32::from(next.is_some()),
        ))
    }

    pub(super) fn first_in_scope(
        &self,
        scope: super::UiFocusScopeIdentity,
    ) -> Option<super::UiFocusParticipant> {
        self.participants.get(&scope)?.first().copied()
    }

    fn last_in_scope(
        &self,
        scope: super::UiFocusScopeIdentity,
    ) -> Option<super::UiFocusParticipant> {
        self.participants.get(&scope)?.last().copied()
    }

    pub(super) fn plan_for(
        &self,
        next: Option<super::UiFocusParticipant>,
        cause: super::UiFocusCause,
        participants_visited: u32,
    ) -> UiFocusPlan {
        let next = next.map(super::UiSemanticKeyboardFocus::new);
        let outcome = transition_outcome(self.current, next);
        UiFocusPlan::new(self.revision, next, cause, outcome, participants_visited)
    }
}

pub(super) fn transition_outcome(
    previous: Option<super::UiSemanticKeyboardFocus>,
    next: Option<super::UiSemanticKeyboardFocus>,
) -> super::UiFocusOutcome {
    match (previous, next) {
        (Some(previous), Some(next)) if previous.participant() == next.participant() => {
            super::UiFocusOutcome::Unchanged
        }
        (_, Some(_)) => super::UiFocusOutcome::Moved,
        (Some(_), None) => super::UiFocusOutcome::Cleared,
        (None, None) => super::UiFocusOutcome::NoEligibleParticipant,
    }
}
