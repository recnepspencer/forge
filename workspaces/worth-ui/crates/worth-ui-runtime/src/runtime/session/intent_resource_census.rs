use crate::inspection::intent::UiIntentEvidenceResourceSnapshot;
use crate::runtime::interaction::UiInteractionStateSnapshot;
use crate::runtime::observation::UiObservationResourceSnapshot;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiIntentResourceCensus {
    active_observation_turns: usize,
    retained_observation_sets: usize,
    retained_observations: usize,
    retained_observation_bytes: usize,
    active_pointer_gestures: usize,
    active_pointer_captures: usize,
    active_input_recipients: usize,
    active_draft_sessions: usize,
    retained_draft_utf8_bytes: usize,
    pending_challenges: usize,
    retained_confirmation_candidates: usize,
    retained_confirmation_payloads: usize,
    execution_entries: usize,
    active_reservations: usize,
    retained_admission_candidates: usize,
    retained_payloads: usize,
    retained_owner_references: usize,
    retained_payload_bytes: usize,
    prepared_executor_handles: usize,
    running_executor_handles: usize,
    recovery_authorities: usize,
    consequence_receipts: usize,
    retained_evidence_references: usize,
    retained_evidence_bytes: usize,
}

pub(crate) struct UiIntentResourceCensusInput {
    pub(crate) observation: UiObservationResourceSnapshot,
    pub(crate) interaction: UiInteractionStateSnapshot,
    pub(crate) confirmation: crate::runtime::intent::UiIntentConfirmationMetrics,
    pub(crate) execution: crate::runtime::intent_execution::UiIntentExecutionAdmissionCensus,
    pub(crate) evidence: UiIntentEvidenceResourceSnapshot,
}

impl UiIntentResourceCensus {
    pub(crate) const fn from_owners(input: UiIntentResourceCensusInput) -> Self {
        Self {
            active_observation_turns: input.observation.active_turns(),
            retained_observation_sets: input.observation.retained_sets(),
            retained_observations: input.observation.retained_observations(),
            retained_observation_bytes: input.observation.retained_bytes(),
            active_pointer_gestures: input.interaction.active_gestures(),
            active_pointer_captures: input.interaction.active_gestures(),
            active_input_recipients: input.interaction.active_recipients(),
            active_draft_sessions: input.interaction.active_draft_sessions(),
            retained_draft_utf8_bytes: input.interaction.retained_draft_utf8_bytes(),
            pending_challenges: input.confirmation.pending_challenges(),
            retained_confirmation_candidates: input.confirmation.retained_candidates(),
            retained_confirmation_payloads: input.confirmation.retained_payloads(),
            execution_entries: input.execution.execution_entries,
            active_reservations: input.execution.active_attempts,
            retained_admission_candidates: input.execution.retained_candidates,
            retained_payloads: input.execution.retained_payloads,
            retained_owner_references: input.execution.retained_owner_references,
            retained_payload_bytes: input.execution.retained_payload_bytes,
            prepared_executor_handles: input.execution.prepared_attempts,
            running_executor_handles: input.execution.running_attempts,
            recovery_authorities: input.execution.recovering_attempts,
            consequence_receipts: input.execution.consequence_pending_attempts,
            retained_evidence_references: input.evidence.retained_references(),
            retained_evidence_bytes: input.evidence.retained_bytes(),
        }
    }

    pub fn is_empty(self) -> bool {
        self == Self::EMPTY
    }

    pub fn is_operationally_empty(mut self) -> bool {
        self.retained_evidence_references = 0;
        self.retained_evidence_bytes = 0;
        self == Self::EMPTY
    }

    pub const EMPTY: Self = Self {
        active_observation_turns: 0,
        retained_observation_sets: 0,
        retained_observations: 0,
        retained_observation_bytes: 0,
        active_pointer_gestures: 0,
        active_pointer_captures: 0,
        active_input_recipients: 0,
        active_draft_sessions: 0,
        retained_draft_utf8_bytes: 0,
        pending_challenges: 0,
        retained_confirmation_candidates: 0,
        retained_confirmation_payloads: 0,
        execution_entries: 0,
        active_reservations: 0,
        retained_admission_candidates: 0,
        retained_payloads: 0,
        retained_owner_references: 0,
        retained_payload_bytes: 0,
        prepared_executor_handles: 0,
        running_executor_handles: 0,
        recovery_authorities: 0,
        consequence_receipts: 0,
        retained_evidence_references: 0,
        retained_evidence_bytes: 0,
    };

    pub const fn active_observation_turns(self) -> usize {
        self.active_observation_turns
    }
    pub const fn retained_observation_sets(self) -> usize {
        self.retained_observation_sets
    }
    pub const fn retained_observations(self) -> usize {
        self.retained_observations
    }
    pub const fn retained_observation_bytes(self) -> usize {
        self.retained_observation_bytes
    }
    pub const fn active_pointer_gestures(self) -> usize {
        self.active_pointer_gestures
    }
    pub const fn active_pointer_captures(self) -> usize {
        self.active_pointer_captures
    }
    pub const fn active_input_recipients(self) -> usize {
        self.active_input_recipients
    }
    pub const fn active_draft_sessions(self) -> usize {
        self.active_draft_sessions
    }
    pub const fn retained_draft_utf8_bytes(self) -> usize {
        self.retained_draft_utf8_bytes
    }
    pub const fn pending_challenges(self) -> usize {
        self.pending_challenges
    }
    pub const fn retained_confirmation_candidates(self) -> usize {
        self.retained_confirmation_candidates
    }
    pub const fn retained_confirmation_payloads(self) -> usize {
        self.retained_confirmation_payloads
    }
    pub const fn execution_entries(self) -> usize {
        self.execution_entries
    }
    pub const fn active_reservations(self) -> usize {
        self.active_reservations
    }
    pub const fn retained_admission_candidates(self) -> usize {
        self.retained_admission_candidates
    }
    pub const fn retained_payloads(self) -> usize {
        self.retained_payloads
    }
    pub const fn retained_owner_references(self) -> usize {
        self.retained_owner_references
    }
    pub const fn retained_payload_bytes(self) -> usize {
        self.retained_payload_bytes
    }
    pub const fn prepared_executor_handles(self) -> usize {
        self.prepared_executor_handles
    }
    pub const fn running_executor_handles(self) -> usize {
        self.running_executor_handles
    }
    pub const fn recovery_authorities(self) -> usize {
        self.recovery_authorities
    }
    pub const fn consequence_receipts(self) -> usize {
        self.consequence_receipts
    }
    pub const fn retained_evidence_references(self) -> usize {
        self.retained_evidence_references
    }
    pub const fn retained_evidence_bytes(self) -> usize {
        self.retained_evidence_bytes
    }
}
