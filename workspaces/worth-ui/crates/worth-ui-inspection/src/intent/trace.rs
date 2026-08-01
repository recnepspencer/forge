use super::{
    UiIntentEvidenceReference, UiIntentInteractionEvidence, UiIntentInteractionEvidenceInput,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentCausalTraceEvidence {
    reference: UiIntentEvidenceReference,
    interaction: UiIntentInteractionEvidenceInput,
    route: Option<UiIntentCausalTraceRouteEvidence>,
    payload: Option<UiIntentCausalTracePayloadEvidence>,
    operability: Option<UiIntentCausalTraceOperabilityEvidence>,
    admission: Option<UiIntentCausalTraceAdmissionEvidence>,
    attempt: Option<UiIntentCausalTraceAttemptEvidence>,
    completion: Option<UiIntentCausalTraceCompletionEvidence>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentCausalTraceRouteEvidence {
    graph_node: u64,
    definition_digest: u64,
    declaration_digest: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentCausalTracePayloadEvidence {
    owner_revision_count: u8,
    primary_owner_revision: Option<u64>,
    owner_revision_digest: u64,
    admitted_utf8_bytes: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentCausalTraceOperabilityPosture {
    Operable,
    ConfirmationRequired,
    Inoperable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentCausalTraceOperabilityEvidence {
    posture: UiIntentCausalTraceOperabilityPosture,
    dependency_count: usize,
    decision_digest: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentCausalTraceAdmissionEvidence {
    slot: u8,
    generation: u64,
    lineage: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentCausalTraceAttemptPosture {
    Prepared,
    Started,
    PendingBeforeEffect,
    PendingEffectMayHaveBegun,
    Completed,
    StoppedBeforeEffect,
    Partial,
    Indeterminate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentCausalTraceAttemptEvidence {
    slot: u8,
    generation: u64,
    idempotency_session: u64,
    idempotency_lineage: u64,
    posture: UiIntentCausalTraceAttemptPosture,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentCausalTraceCompletionEvidence {
    outcome_schema_digest: u64,
    consequence_pending_at_completion: bool,
}

impl UiIntentCausalTraceEvidence {
    #[doc(hidden)]
    pub const fn from_interaction(
        reference: UiIntentEvidenceReference,
        interaction: UiIntentInteractionEvidenceInput,
    ) -> Self {
        Self {
            reference,
            interaction,
            route: None,
            payload: None,
            operability: None,
            admission: None,
            attempt: None,
            completion: None,
        }
    }

    #[doc(hidden)]
    pub fn record_admission(
        &mut self,
        route: UiIntentCausalTraceRouteEvidence,
        payload: UiIntentCausalTracePayloadEvidence,
        operability: UiIntentCausalTraceOperabilityEvidence,
        admission: UiIntentCausalTraceAdmissionEvidence,
    ) {
        self.route = Some(route);
        self.payload = Some(payload);
        self.operability = Some(operability);
        self.admission = Some(admission);
    }

    #[doc(hidden)]
    pub fn record_attempt(&mut self, attempt: UiIntentCausalTraceAttemptEvidence) {
        self.attempt = Some(attempt);
    }

    #[doc(hidden)]
    pub fn record_completion(&mut self, completion: UiIntentCausalTraceCompletionEvidence) {
        self.completion = Some(completion);
    }

    pub const fn reference(self) -> UiIntentEvidenceReference {
        self.reference
    }

    pub const fn interaction(self) -> UiIntentInteractionEvidenceInput {
        self.interaction
    }

    pub const fn interaction_evidence(self) -> UiIntentInteractionEvidence {
        UiIntentInteractionEvidence::from_retained_input(self.reference, self.interaction)
    }

    pub const fn route(self) -> Option<UiIntentCausalTraceRouteEvidence> {
        self.route
    }

    pub const fn payload(self) -> Option<UiIntentCausalTracePayloadEvidence> {
        self.payload
    }

    pub const fn operability(self) -> Option<UiIntentCausalTraceOperabilityEvidence> {
        self.operability
    }

    pub const fn admission(self) -> Option<UiIntentCausalTraceAdmissionEvidence> {
        self.admission
    }

    pub const fn attempt(self) -> Option<UiIntentCausalTraceAttemptEvidence> {
        self.attempt
    }

    pub const fn completion(self) -> Option<UiIntentCausalTraceCompletionEvidence> {
        self.completion
    }

    pub const fn is_complete_through_product_outcome(self) -> bool {
        self.route.is_some()
            && self.payload.is_some()
            && self.operability.is_some()
            && self.admission.is_some()
            && matches!(
                self.attempt,
                Some(UiIntentCausalTraceAttemptEvidence {
                    posture: UiIntentCausalTraceAttemptPosture::Completed,
                    ..
                })
            )
            && matches!(
                self.completion,
                Some(UiIntentCausalTraceCompletionEvidence {
                    consequence_pending_at_completion: true,
                    ..
                })
            )
    }
}

impl UiIntentCausalTraceRouteEvidence {
    #[doc(hidden)]
    pub const fn new(graph_node: u64, definition_digest: u64, declaration_digest: u64) -> Self {
        Self {
            graph_node,
            definition_digest,
            declaration_digest,
        }
    }

    pub const fn graph_node(self) -> u64 {
        self.graph_node
    }

    pub const fn definition_digest(self) -> u64 {
        self.definition_digest
    }

    pub const fn declaration_digest(self) -> u64 {
        self.declaration_digest
    }
}

impl UiIntentCausalTracePayloadEvidence {
    #[doc(hidden)]
    pub const fn new(
        owner_revision_count: u8,
        primary_owner_revision: Option<u64>,
        owner_revision_digest: u64,
        admitted_utf8_bytes: usize,
    ) -> Self {
        Self {
            owner_revision_count,
            primary_owner_revision,
            owner_revision_digest,
            admitted_utf8_bytes,
        }
    }

    pub const fn owner_revision_count(self) -> u8 {
        self.owner_revision_count
    }

    pub const fn primary_owner_revision(self) -> Option<u64> {
        self.primary_owner_revision
    }

    pub const fn owner_revision_digest(self) -> u64 {
        self.owner_revision_digest
    }

    pub const fn admitted_utf8_bytes(self) -> usize {
        self.admitted_utf8_bytes
    }
}

impl UiIntentCausalTraceOperabilityEvidence {
    #[doc(hidden)]
    pub const fn new(
        posture: UiIntentCausalTraceOperabilityPosture,
        dependency_count: usize,
        decision_digest: u64,
    ) -> Self {
        Self {
            posture,
            dependency_count,
            decision_digest,
        }
    }

    pub const fn posture(self) -> UiIntentCausalTraceOperabilityPosture {
        self.posture
    }

    pub const fn dependency_count(self) -> usize {
        self.dependency_count
    }

    pub const fn decision_digest(self) -> u64 {
        self.decision_digest
    }
}

impl UiIntentCausalTraceAdmissionEvidence {
    #[doc(hidden)]
    pub const fn new(slot: u8, generation: u64, lineage: u64) -> Self {
        Self {
            slot,
            generation,
            lineage,
        }
    }

    pub const fn slot(self) -> u8 {
        self.slot
    }

    pub const fn generation(self) -> u64 {
        self.generation
    }

    pub const fn lineage(self) -> u64 {
        self.lineage
    }
}

impl UiIntentCausalTraceAttemptEvidence {
    #[doc(hidden)]
    pub const fn new(
        slot: u8,
        generation: u64,
        idempotency_session: u64,
        idempotency_lineage: u64,
        posture: UiIntentCausalTraceAttemptPosture,
    ) -> Self {
        Self {
            slot,
            generation,
            idempotency_session,
            idempotency_lineage,
            posture,
        }
    }

    pub const fn slot(self) -> u8 {
        self.slot
    }

    pub const fn generation(self) -> u64 {
        self.generation
    }

    pub const fn idempotency_session(self) -> u64 {
        self.idempotency_session
    }

    pub const fn idempotency_lineage(self) -> u64 {
        self.idempotency_lineage
    }

    pub const fn posture(self) -> UiIntentCausalTraceAttemptPosture {
        self.posture
    }
}

impl UiIntentCausalTraceCompletionEvidence {
    #[doc(hidden)]
    pub const fn new(outcome_schema_digest: u64, consequence_pending_at_completion: bool) -> Self {
        Self {
            outcome_schema_digest,
            consequence_pending_at_completion,
        }
    }

    pub const fn outcome_schema_digest(self) -> u64 {
        self.outcome_schema_digest
    }

    pub const fn consequence_pending_at_completion(self) -> bool {
        self.consequence_pending_at_completion
    }
}
