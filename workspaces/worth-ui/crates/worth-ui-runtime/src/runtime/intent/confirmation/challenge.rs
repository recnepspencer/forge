use super::{
    UiIntentConfirmationStop, UiIntentConfirmationStopReason, UiIntentConfirmationTimeBasisKind,
};

pub const UI_PENDING_INTENT_CONFIRMATION_LIMIT: usize = 16;
pub const UI_INTENT_CONFIRMATION_TTL_MILLIS: u64 = 30_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentConfirmationSlotIdentity {
    slot: u8,
    generation: u64,
}

#[must_use]
pub struct UiIntentConfirmationChallenge {
    pub(super) candidate: super::super::payload::UiPreparedIntentPayload,
    pub(super) decision: super::super::operability::UiIntentOperabilityDecision,
    pub(super) policy_identity: Box<str>,
    pub(super) issued_at_millis: u64,
    pub(super) expires_at_millis: u64,
    pub(super) lineage: super::super::UiIntentAttemptLineage,
    pub(super) slot_identity: UiIntentConfirmationSlotIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiPendingIntentConfirmation {
    declaration: Box<str>,
    definition: crate::capability::UiIntentId,
    policy_identity: Box<str>,
    lineage: super::super::UiIntentAttemptLineage,
    slot_identity: UiIntentConfirmationSlotIdentity,
    expires_at_millis: u64,
}

#[must_use]
pub enum UiIntentConfirmationIssueOutcome {
    Pending(UiPendingIntentConfirmation),
    Stopped(UiIntentConfirmationStop),
}

pub(super) struct UiPreparedConfirmationChallenge {
    candidate: super::super::payload::UiPreparedIntentPayload,
    decision: super::super::operability::UiIntentOperabilityDecision,
    policy_identity: Box<str>,
    issued_at_millis: u64,
    expires_at_millis: u64,
}

pub(super) fn prepare_challenge(
    candidate: super::super::operability::UiInoperableIntentCandidate,
) -> Result<UiPreparedConfirmationChallenge, UiIntentConfirmationStopReason> {
    let (candidate, decision) = candidate.into_parts();
    let policy_identity = exclusively_required_policy(&decision)
        .ok_or(UiIntentConfirmationStopReason::CandidateNotExclusivelyConfirmable)?;
    let issued_at_millis = monotonic_millis(candidate.interaction_time_basis())?;
    let expires_at_millis = issued_at_millis
        .checked_add(UI_INTENT_CONFIRMATION_TTL_MILLIS)
        .ok_or(UiIntentConfirmationStopReason::ChallengeExpiryOverflow)?;
    Ok(UiPreparedConfirmationChallenge {
        candidate,
        decision,
        policy_identity,
        issued_at_millis,
        expires_at_millis,
    })
}

impl UiPreparedConfirmationChallenge {
    pub(super) fn seal(
        self,
        lineage: super::super::UiIntentAttemptLineage,
        slot_identity: UiIntentConfirmationSlotIdentity,
    ) -> (UiIntentConfirmationChallenge, UiPendingIntentConfirmation) {
        let pending = UiPendingIntentConfirmation {
            declaration: self.candidate.declaration_identity().into(),
            definition: self.candidate.definition_id(),
            policy_identity: self.policy_identity.clone(),
            lineage,
            slot_identity,
            expires_at_millis: self.expires_at_millis,
        };
        let challenge = UiIntentConfirmationChallenge {
            candidate: self.candidate,
            decision: self.decision,
            policy_identity: self.policy_identity,
            issued_at_millis: self.issued_at_millis,
            expires_at_millis: self.expires_at_millis,
            lineage,
            slot_identity,
        };
        (challenge, pending)
    }
}

impl UiIntentConfirmationSlotIdentity {
    pub(super) const fn new(slot: u8, generation: u64) -> Self {
        Self { slot, generation }
    }

    pub const fn slot(self) -> u8 {
        self.slot
    }

    pub const fn generation(self) -> u64 {
        self.generation
    }
}

impl UiPendingIntentConfirmation {
    pub fn declaration_identity(&self) -> &str {
        &self.declaration
    }

    pub const fn definition_id(&self) -> crate::capability::UiIntentId {
        self.definition
    }

    pub fn policy_identity(&self) -> &str {
        &self.policy_identity
    }

    pub const fn lineage(&self) -> super::super::UiIntentAttemptLineage {
        self.lineage
    }

    pub const fn slot_identity(&self) -> UiIntentConfirmationSlotIdentity {
        self.slot_identity
    }

    pub const fn expires_at_millis(&self) -> u64 {
        self.expires_at_millis
    }
}

fn exclusively_required_policy(
    decision: &super::super::operability::UiIntentOperabilityDecision,
) -> Option<Box<str>> {
    let mut causes = decision.causes();
    let super::super::operability::UiIntentInoperableCause::ConfirmationRequired {
        policy_identity,
    } = causes.next()?
    else {
        return None;
    };
    causes.next().is_none().then_some(policy_identity)
}

fn monotonic_millis(
    basis: worth_ui_host_contract::UiHostObservationTimeBasis,
) -> Result<u64, UiIntentConfirmationStopReason> {
    match basis {
        worth_ui_host_contract::UiHostObservationTimeBasis::HostMonotonicMillis(millis) => {
            Ok(millis)
        }
        worth_ui_host_contract::UiHostObservationTimeBasis::HostWallClockMicros(_) => {
            Err(UiIntentConfirmationStopReason::MonotonicTimeRequired {
                observed: UiIntentConfirmationTimeBasisKind::HostWallClock,
            })
        }
        worth_ui_host_contract::UiHostObservationTimeBasis::PresentationRelativeTick(_) => {
            Err(UiIntentConfirmationStopReason::MonotonicTimeRequired {
                observed: UiIntentConfirmationTimeBasisKind::PresentationRelative,
            })
        }
    }
}
