#[derive(Debug, Eq, PartialEq)]
pub enum UiIntentExecutionCurrentnessStop {
    DefinitionContractMismatch {
        candidate: crate::capability::UiIntentId,
        requested: crate::capability::UiIntentId,
    },
    ApplicationWorldChanged,
    ApplicationGenerationChanged,
    PresentationInFlight,
    TargetChanged(crate::runtime::interaction::UiInteractionTargetingDenial),
    ProductRouteChanged,
    CommandContextChanged,
    PayloadInputChanged,
    OperabilityDependencyChanged,
    PolicyChanged,
    ConfirmationPolicyChanged,
}

#[derive(Debug, Eq, PartialEq)]
pub enum UiIntentExecutionDispatchStopReason {
    AdmissionSettled(crate::runtime::intent::UiIntentAdmissionSettlementPosture),
    Currentness(UiIntentExecutionCurrentnessStop),
}

#[must_use]
pub enum UiIntentExecutionDispatchOutcome {
    AttemptPrepared(UiIntentExecutionDispatchReceipt),
    Stopped(UiIntentExecutionDispatchStop),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentExecutionDispatchReceipt {
    attempt: super::UiIntentExecutionAttemptIdentity,
    idempotency: super::UiIntentExecutionIdempotencyIdentity,
    deadline: super::UiIntentExecutionDeadline,
    currentness_checks: usize,
    evidence_reference: Option<worth_ui_inspection::UiIntentEvidenceReference>,
}

#[must_use]
#[derive(Debug, Eq, PartialEq)]
pub struct UiIntentExecutionDispatchStop {
    reason: UiIntentExecutionDispatchStopReason,
    active_after: usize,
}

impl UiIntentExecutionDispatchReceipt {
    pub(crate) const fn new(
        attempt: super::UiIntentExecutionAttemptIdentity,
        idempotency: super::UiIntentExecutionIdempotencyIdentity,
        deadline: super::UiIntentExecutionDeadline,
        currentness_checks: usize,
    ) -> Self {
        Self {
            attempt,
            idempotency,
            deadline,
            currentness_checks,
            evidence_reference: None,
        }
    }

    pub const fn attempt(self) -> super::UiIntentExecutionAttemptIdentity {
        self.attempt
    }

    pub const fn idempotency(self) -> super::UiIntentExecutionIdempotencyIdentity {
        self.idempotency
    }

    pub const fn deadline(self) -> super::UiIntentExecutionDeadline {
        self.deadline
    }

    pub const fn currentness_checks(self) -> usize {
        self.currentness_checks
    }

    pub const fn evidence_reference(
        self,
    ) -> Option<worth_ui_inspection::UiIntentEvidenceReference> {
        self.evidence_reference
    }

    pub(crate) const fn with_evidence_reference(
        mut self,
        reference: Option<worth_ui_inspection::UiIntentEvidenceReference>,
    ) -> Self {
        self.evidence_reference = reference;
        self
    }
}

impl UiIntentExecutionDispatchStop {
    pub(crate) const fn new(
        reason: UiIntentExecutionDispatchStopReason,
        active_after: usize,
    ) -> Self {
        Self {
            reason,
            active_after,
        }
    }

    pub const fn reason(&self) -> &UiIntentExecutionDispatchStopReason {
        &self.reason
    }

    pub const fn active_after(&self) -> usize {
        self.active_after
    }
}
