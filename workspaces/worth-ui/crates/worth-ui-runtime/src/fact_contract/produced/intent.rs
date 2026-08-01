#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentPostureKind {
    Admitted,
    ConfirmationRequired,
    Completed,
    Denied,
    StaleConfirmation,
    Cancelled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentPostureReference {
    Route(crate::capability::UiIntentId),
    Confirmation {
        slot: crate::runtime::intent::UiIntentConfirmationSlotIdentity,
        lineage: crate::runtime::intent::UiIntentAttemptLineage,
    },
    Attempt {
        attempt: crate::runtime::intent_execution::UiIntentExecutionAttemptIdentity,
        idempotency: crate::runtime::intent_execution::UiIntentExecutionIdempotencyIdentity,
    },
}

pub struct UiIntentPostureChangedFact {
    graph_node: crate::graph::UiGraphNodeIdentity,
    target: crate::runtime::interaction::UiPresentedInteractionTargetView,
    reference: UiIntentPostureReference,
    posture: UiIntentPostureKind,
}

impl UiIntentPostureChangedFact {
    pub(crate) const fn new(
        graph_node: crate::graph::UiGraphNodeIdentity,
        target: crate::runtime::interaction::UiPresentedInteractionTargetView,
        reference: UiIntentPostureReference,
        posture: UiIntentPostureKind,
    ) -> Self {
        Self {
            graph_node,
            target,
            reference,
            posture,
        }
    }

    pub const fn graph_node(&self) -> crate::graph::UiGraphNodeIdentity {
        self.graph_node
    }

    pub const fn target(&self) -> crate::runtime::interaction::UiPresentedInteractionTargetView {
        self.target
    }

    pub const fn reference(&self) -> UiIntentPostureReference {
        self.reference
    }

    pub const fn attempt(
        &self,
    ) -> Option<crate::runtime::intent_execution::UiIntentExecutionAttemptIdentity> {
        match self.reference {
            UiIntentPostureReference::Attempt { attempt, .. } => Some(attempt),
            UiIntentPostureReference::Route(_) | UiIntentPostureReference::Confirmation { .. } => {
                None
            }
        }
    }

    pub const fn idempotency(
        &self,
    ) -> Option<crate::runtime::intent_execution::UiIntentExecutionIdempotencyIdentity> {
        match self.reference {
            UiIntentPostureReference::Attempt { idempotency, .. } => Some(idempotency),
            UiIntentPostureReference::Route(_) | UiIntentPostureReference::Confirmation { .. } => {
                None
            }
        }
    }

    pub const fn posture(&self) -> UiIntentPostureKind {
        self.posture
    }
}
