use serde::{Deserialize, Serialize};

mod causal_trace;
mod projection;
mod watcher_shutdown;

pub use causal_trace::{
    PlatformPulseIntentAdmissionTrace, PlatformPulseIntentCausalTraceObservation,
    PlatformPulseIntentEvidenceReferenceObservation, PlatformPulseIntentInteractionFamily,
    PlatformPulseIntentOperabilityTrace, PlatformPulseIntentOutcomeTrace,
    PlatformPulseIntentPayloadTrace, PlatformPulseIntentRouteTrace, PlatformPulseIntentSourceTrace,
    PlatformPulseIntentTraceProjectionDenial,
};
pub use watcher_shutdown::PlatformPulseIntentWatcherShutdownEvidence;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlatformPulseIntentOperabilityObservation {
    Ready,
    Disabled,
    Denied,
    ConfirmationRequired,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlatformPulseIntentExecutorGateObservation {
    Held,
    Released,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseIntentInputObservation {
    revision: u64,
    operability: PlatformPulseIntentOperabilityObservation,
    executor_gate: PlatformPulseIntentExecutorGateObservation,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseIntentAttemptObservationReference {
    attempt_slot: u8,
    attempt_generation: u64,
    idempotency_session: u64,
    idempotency_lineage: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseIntentExecutorStartedObservation {
    reference: PlatformPulseIntentAttemptObservationReference,
    transition_count: u64,
    active_slots_visited: u64,
    provider_calls: u64,
    provider_polls: u64,
    cancellation_calls: u64,
    settlements: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "posture", content = "evidence")]
pub enum PlatformPulseQueryActionObservation {
    Executed {
        reference: PlatformPulseIntentAttemptObservationReference,
        action_input_revision: u64,
        query_source_revision: u64,
        status: String,
        query_receipt_digest: String,
        affected_live_view_ids: Vec<String>,
    },
    Denied {
        reference: PlatformPulseIntentAttemptObservationReference,
        action_input_revision: u64,
        active_query_source_revision: u64,
        submitted_query_source_revision: u64,
    },
    Indeterminate {
        reference: PlatformPulseIntentAttemptObservationReference,
        action_input_revision: u64,
        detail: String,
    },
    CancelledBeforeEffect {
        reference: PlatformPulseIntentAttemptObservationReference,
        action_input_revision: u64,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "posture", content = "evidence")]
pub enum PlatformPulseIntentPostureObservation {
    Admitted {
        reference: PlatformPulseIntentAttemptObservationReference,
    },
    ConfirmationRequired {
        slot: u8,
        generation: u64,
        lineage: u64,
        expires_at_tick: u64,
    },
    Completed {
        reference: PlatformPulseIntentAttemptObservationReference,
    },
    Denied,
    StaleConfirmation,
    Cancelled {
        reference: PlatformPulseIntentAttemptObservationReference,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseIntentPosturePublished {
    posture: PlatformPulseIntentPostureObservation,
    frame: super::lifecycle::PlatformPulseMountedFrameObservation,
}

impl PlatformPulseIntentInputObservation {
    pub fn revision(self) -> u64 {
        self.revision
    }

    pub fn operability(self) -> PlatformPulseIntentOperabilityObservation {
        self.operability
    }

    pub fn executor_gate(self) -> PlatformPulseIntentExecutorGateObservation {
        self.executor_gate
    }

    pub fn from_record(record: &crate::intent::PlatformPulseIntentInputRecord) -> Self {
        use crate::intent::{
            PlatformPulseExecutorGatePosture as Gate,
            PlatformPulseIntentInputOperability as Operability,
        };
        let operability = match record.operability() {
            Operability::Ready => PlatformPulseIntentOperabilityObservation::Ready,
            Operability::Disabled => PlatformPulseIntentOperabilityObservation::Disabled,
            Operability::Denied => PlatformPulseIntentOperabilityObservation::Denied,
            Operability::ConfirmationRequired => {
                PlatformPulseIntentOperabilityObservation::ConfirmationRequired
            }
        };
        let executor_gate = match record.executor_gate() {
            Gate::Held => PlatformPulseIntentExecutorGateObservation::Held,
            Gate::Released => PlatformPulseIntentExecutorGateObservation::Released,
        };
        Self {
            revision: record.revision(),
            operability,
            executor_gate,
        }
    }
}

impl PlatformPulseIntentAttemptObservationReference {
    pub(super) const fn from_diagnostic_parts(
        attempt_slot: u8,
        attempt_generation: u64,
        idempotency_session: u64,
        idempotency_lineage: u64,
    ) -> Self {
        Self {
            attempt_slot,
            attempt_generation,
            idempotency_session,
            idempotency_lineage,
        }
    }

    pub fn from_product(reference: crate::intent::PlatformPulseActionAttemptReference) -> Self {
        Self {
            attempt_slot: reference.attempt_slot(),
            attempt_generation: reference.attempt_generation(),
            idempotency_session: reference.idempotency_session(),
            idempotency_lineage: reference.idempotency_lineage(),
        }
    }

    pub fn from_execution(
        attempt: worth_ui::facade::intent::UiIntentExecutionAttemptIdentity,
        idempotency: worth_ui::facade::intent::UiIntentExecutionIdempotencyIdentity,
    ) -> Self {
        Self {
            attempt_slot: attempt.slot(),
            attempt_generation: attempt.generation(),
            idempotency_session: idempotency.session(),
            idempotency_lineage: idempotency.lineage(),
        }
    }

    pub fn attempt_slot(self) -> u8 {
        self.attempt_slot
    }

    pub fn attempt_generation(self) -> u64 {
        self.attempt_generation
    }

    pub fn idempotency_session(self) -> u64 {
        self.idempotency_session
    }

    pub fn idempotency_lineage(self) -> u64 {
        self.idempotency_lineage
    }
}

impl PlatformPulseIntentExecutorStartedObservation {
    pub fn from_transition(
        report: &worth_ui::facade::intent::UiIntentExecutionAdvanceReport,
        transition: &worth_ui::facade::intent::UiIntentExecutionTransition,
    ) -> Option<Self> {
        matches!(
            transition.posture(),
            worth_ui::facade::intent::UiIntentExecutionTransitionPosture::Started
        )
        .then(|| Self {
            reference: PlatformPulseIntentAttemptObservationReference::from_execution(
                transition.attempt(),
                transition.idempotency(),
            ),
            transition_count: count(report.transitions().len()),
            active_slots_visited: count(report.active_slots_visited()),
            provider_calls: count(report.provider_calls()),
            provider_polls: count(report.provider_polls()),
            cancellation_calls: count(report.cancellation_calls()),
            settlements: count(report.settlements()),
        })
    }

    pub fn reference(self) -> PlatformPulseIntentAttemptObservationReference {
        self.reference
    }

    pub fn transition_count(self) -> u64 {
        self.transition_count
    }

    pub fn active_slots_visited(self) -> u64 {
        self.active_slots_visited
    }

    pub fn provider_calls(self) -> u64 {
        self.provider_calls
    }

    pub fn provider_polls(self) -> u64 {
        self.provider_polls
    }

    pub fn cancellation_calls(self) -> u64 {
        self.cancellation_calls
    }

    pub fn settlements(self) -> u64 {
        self.settlements
    }
}

impl PlatformPulseIntentPostureObservation {
    pub fn admitted(dispatch: worth_ui::facade::intent::UiIntentExecutionDispatchReceipt) -> Self {
        Self::Admitted {
            reference: PlatformPulseIntentAttemptObservationReference::from_execution(
                dispatch.attempt(),
                dispatch.idempotency(),
            ),
        }
    }

    pub fn confirmation_required(
        pending: &worth_ui::facade::intent::UiPendingIntentConfirmation,
    ) -> Self {
        Self::ConfirmationRequired {
            slot: pending.slot_identity().slot(),
            generation: pending.slot_identity().generation(),
            lineage: pending.lineage().diagnostic_value(),
            expires_at_tick: pending.expires_at_tick(),
        }
    }

    pub fn completed(
        attempt: worth_ui::facade::intent::UiIntentExecutionAttemptIdentity,
        idempotency: worth_ui::facade::intent::UiIntentExecutionIdempotencyIdentity,
    ) -> Self {
        Self::Completed {
            reference: PlatformPulseIntentAttemptObservationReference::from_execution(
                attempt,
                idempotency,
            ),
        }
    }

    pub fn cancelled(
        attempt: worth_ui::facade::intent::UiIntentExecutionAttemptIdentity,
        idempotency: worth_ui::facade::intent::UiIntentExecutionIdempotencyIdentity,
    ) -> Self {
        Self::Cancelled {
            reference: PlatformPulseIntentAttemptObservationReference::from_execution(
                attempt,
                idempotency,
            ),
        }
    }
}

impl PlatformPulseIntentPosturePublished {
    pub(super) fn new(
        posture: PlatformPulseIntentPostureObservation,
        publication: &worth_ui::facade::app::UiMountedFramePublicationReceipt,
    ) -> Self {
        Self {
            posture,
            frame: super::lifecycle::PlatformPulseMountedFrameObservation::from_publication(
                publication,
            ),
        }
    }

    pub fn posture(&self) -> &PlatformPulseIntentPostureObservation {
        &self.posture
    }

    pub fn frame(&self) -> super::lifecycle::PlatformPulseMountedFrameObservation {
        self.frame
    }
}

impl PlatformPulseQueryActionObservation {
    pub fn executed(
        reference: crate::intent::PlatformPulseActionAttemptReference,
        action_input_revision: crate::intent::PlatformPulseActionInputRevision,
        evidence: &worth_ui::facade::query_binding::WorthUiScalarProjectionActionEvidence,
    ) -> Self {
        Self::Executed {
            reference: PlatformPulseIntentAttemptObservationReference::from_product(reference),
            action_input_revision: action_input_revision.value(),
            query_source_revision: evidence.source_revision(),
            status: evidence.status().to_owned(),
            query_receipt_digest: evidence.query_receipt_digest().to_owned(),
            affected_live_view_ids: evidence.affected_live_view_ids().to_vec(),
        }
    }

    pub fn denied(
        reference: crate::intent::PlatformPulseActionAttemptReference,
        action_input_revision: crate::intent::PlatformPulseActionInputRevision,
        active_query_source_revision: u64,
        submitted_query_source_revision: u64,
    ) -> Self {
        Self::Denied {
            reference: PlatformPulseIntentAttemptObservationReference::from_product(reference),
            action_input_revision: action_input_revision.value(),
            active_query_source_revision,
            submitted_query_source_revision,
        }
    }

    pub fn indeterminate(
        reference: crate::intent::PlatformPulseActionAttemptReference,
        action_input_revision: crate::intent::PlatformPulseActionInputRevision,
        detail: String,
    ) -> Self {
        Self::Indeterminate {
            reference: PlatformPulseIntentAttemptObservationReference::from_product(reference),
            action_input_revision: action_input_revision.value(),
            detail,
        }
    }

    pub fn cancelled_before_effect(
        reference: crate::intent::PlatformPulseActionAttemptReference,
        action_input_revision: crate::intent::PlatformPulseActionInputRevision,
    ) -> Self {
        Self::CancelledBeforeEffect {
            reference: PlatformPulseIntentAttemptObservationReference::from_product(reference),
            action_input_revision: action_input_revision.value(),
        }
    }
}

fn count(value: usize) -> u64 {
    u64::try_from(value).unwrap_or(u64::MAX)
}

#[cfg(test)]
mod tests {
    use super::PlatformPulseIntentInputObservation;
    use crate::intent::PlatformPulseIntentInputInstallation;

    #[test]
    fn intent_observation_is_versioned_reporting_not_adapter_authority() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("intent_samples");
        let installation = PlatformPulseIntentInputInstallation::open(&root)
            .expect("checked-in intent source installs");
        let (record, watch) = installation.into_parts();
        let observation = PlatformPulseIntentInputObservation::from_record(&record);
        assert_eq!(observation.revision(), record.revision());
        watch.shutdown().expect("intent watch closes");
    }
}
