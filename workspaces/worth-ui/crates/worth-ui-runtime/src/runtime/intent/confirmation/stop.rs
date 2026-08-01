#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentConfirmationTimeBasisKind {
    HostWallClock,
    PresentationRelative,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentConfirmationCancellationReason {
    MountedInstanceRemoved,
    SurfaceRebound,
    ApplicationRebound,
    Shutdown,
    AmbiguousContinuation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiIntentConfirmationStopReason {
    CandidateNotExclusivelyConfirmable,
    MonotonicTimeRequired {
        observed: UiIntentConfirmationTimeBasisKind,
    },
    ChallengeExpiryOverflow,
    ChallengeCapacityExceeded {
        maximum: usize,
    },
    ChallengeIdentityExhausted,
    NoPendingChallenge {
        declaration: Box<str>,
    },
    AmbiguousPendingChallenges {
        declaration: Box<str>,
        observed: usize,
    },
    LifecycleCancelled(UiIntentConfirmationCancellationReason),
    AlreadyContinued,
    AlreadyStopped,
    MonotonicTimeRegressed {
        issued_at: u64,
        observed: u64,
    },
    Expired {
        expires_at: u64,
        observed: u64,
    },
    ApplicationWorldChanged,
    ApplicationGenerationChanged,
    ConfirmationRouteChanged,
    ProductRouteChanged,
    ConfirmationNotPresented,
    ConfirmationPresentationStale,
    ConfirmationTargetChanged(crate::runtime::interaction::UiInteractionTargetingDenial),
    TargetChanged(crate::runtime::interaction::UiInteractionTargetingDenial),
    PayloadInputChanged,
    OperabilityDependencyChanged,
    PolicyChanged,
    ConfirmationPolicyChanged,
    OccupancyChanged,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiIntentConfirmationLookupCost {
    slots_inspected: usize,
}

#[must_use]
#[derive(Debug, Eq, PartialEq)]
pub struct UiIntentConfirmationStop {
    reason: UiIntentConfirmationStopReason,
    cost: UiIntentConfirmationLookupCost,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentConfirmationSettlementReceipt {
    reason: UiIntentConfirmationCancellationReason,
    settled_challenges: usize,
    pending_after: usize,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiIntentConfirmationShutdownReport {
    settled_challenges: usize,
    pending_after: usize,
}

impl UiIntentConfirmationLookupCost {
    pub(super) const fn new(slots_inspected: usize) -> Self {
        Self { slots_inspected }
    }

    pub const fn slots_inspected(self) -> usize {
        self.slots_inspected
    }
}

impl UiIntentConfirmationStop {
    pub(super) const fn new(
        reason: UiIntentConfirmationStopReason,
        cost: UiIntentConfirmationLookupCost,
    ) -> Self {
        Self { reason, cost }
    }

    pub const fn reason(&self) -> &UiIntentConfirmationStopReason {
        &self.reason
    }

    pub const fn cost(&self) -> UiIntentConfirmationLookupCost {
        self.cost
    }
}

impl UiIntentConfirmationSettlementReceipt {
    pub(super) const fn new(
        reason: UiIntentConfirmationCancellationReason,
        settled_challenges: usize,
        pending_after: usize,
    ) -> Self {
        Self {
            reason,
            settled_challenges,
            pending_after,
        }
    }

    pub const fn reason(self) -> UiIntentConfirmationCancellationReason {
        self.reason
    }

    pub const fn settled_challenges(self) -> usize {
        self.settled_challenges
    }

    pub const fn pending_after(self) -> usize {
        self.pending_after
    }
}

impl UiIntentConfirmationShutdownReport {
    pub(super) const fn new(settled_challenges: usize, pending_after: usize) -> Self {
        Self {
            settled_challenges,
            pending_after,
        }
    }

    pub const fn settled_challenges(self) -> usize {
        self.settled_challenges
    }

    pub const fn pending_after(self) -> usize {
        self.pending_after
    }
}
