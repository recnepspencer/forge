use serde::{Deserialize, Serialize};
use worth_ui::facade::inspection::{
    UiIntentCausalTraceAdmissionEvidence, UiIntentCausalTraceAttemptEvidence,
    UiIntentCausalTraceAttemptPosture, UiIntentCausalTraceCompletionEvidence,
    UiIntentCausalTraceEvidence, UiIntentCausalTraceOperabilityEvidence,
    UiIntentCausalTraceOperabilityPosture, UiIntentCausalTracePayloadEvidence,
    UiIntentCausalTraceRouteEvidence, UiIntentEvidenceReference, UiIntentInteractionEvidenceInput,
};

use super::PlatformPulseIntentAttemptObservationReference;
use crate::observation_contract::{
    PlatformPulseMountedFrameObservation, PlatformPulseQueryProjectionEvidence,
};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseIntentCausalTraceObservation {
    evidence_reference: PlatformPulseIntentEvidenceReferenceObservation,
    source: PlatformPulseIntentSourceTrace,
    route: PlatformPulseIntentRouteTrace,
    payload: PlatformPulseIntentPayloadTrace,
    operability: PlatformPulseIntentOperabilityTrace,
    admission: PlatformPulseIntentAdmissionTrace,
    attempt: PlatformPulseIntentAttemptObservationReference,
    outcome: PlatformPulseIntentOutcomeTrace,
    query_projection: PlatformPulseQueryProjectionEvidence,
    mounted_frame: PlatformPulseMountedFrameObservation,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseIntentEvidenceReferenceObservation {
    session: u64,
    slot: u8,
    generation: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlatformPulseIntentInteractionFamily {
    Activate,
    CommandRoute,
    EditCommit,
    SelectionCommit,
    Submit,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseIntentSourceTrace {
    host_sequence: u64,
    presented_frame: u64,
    presentation_epoch: u64,
    mounted_instance: u64,
    semantic_target_digest: u64,
    interaction_family: PlatformPulseIntentInteractionFamily,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseIntentRouteTrace {
    graph_node: u64,
    definition_digest: u64,
    declaration_digest: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseIntentPayloadTrace {
    owner_revision_count: u8,
    primary_owner_revision: Option<u64>,
    owner_revision_digest: u64,
    admitted_utf8_bytes: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseIntentOperabilityTrace {
    operable: bool,
    dependency_count: u64,
    decision_digest: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseIntentAdmissionTrace {
    slot: u8,
    generation: u64,
    lineage: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseIntentOutcomeTrace {
    outcome_schema_digest: u64,
    consequence_pending_at_completion: bool,
    consequence_published: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlatformPulseIntentTraceProjectionDenial {
    Incomplete,
    NonOperableAdmission,
    NonTerminalAttempt,
}

struct CompletedRuntimeTrace {
    reference: UiIntentEvidenceReference,
    interaction: UiIntentInteractionEvidenceInput,
    route: UiIntentCausalTraceRouteEvidence,
    payload: UiIntentCausalTracePayloadEvidence,
    operability: UiIntentCausalTraceOperabilityEvidence,
    admission: UiIntentCausalTraceAdmissionEvidence,
    attempt: UiIntentCausalTraceAttemptEvidence,
    completion: UiIntentCausalTraceCompletionEvidence,
}

impl PlatformPulseIntentCausalTraceObservation {
    #[doc(hidden)]
    pub fn from_completed_publication(
        trace: UiIntentCausalTraceEvidence,
        query_projection: &PlatformPulseQueryProjectionEvidence,
        publication: &worth_ui::facade::app::UiMountedFramePublicationReceipt,
    ) -> Result<Self, PlatformPulseIntentTraceProjectionDenial> {
        let stages = CompletedRuntimeTrace::read(trace)?;
        Ok(Self {
            evidence_reference: PlatformPulseIntentEvidenceReferenceObservation::from_runtime(
                stages.reference,
            ),
            source: PlatformPulseIntentSourceTrace::from_runtime(stages.interaction),
            route: stages.route.into(),
            payload: stages.payload.into(),
            operability: stages.operability.into(),
            admission: stages.admission.into(),
            attempt: attempt_reference(stages.attempt),
            outcome: stages.completion.into(),
            query_projection: query_projection.clone(),
            mounted_frame: PlatformPulseMountedFrameObservation::from_publication(publication),
        })
    }

    pub fn evidence_reference(&self) -> PlatformPulseIntentEvidenceReferenceObservation {
        self.evidence_reference
    }

    pub fn source(&self) -> PlatformPulseIntentSourceTrace {
        self.source
    }

    pub fn route(&self) -> PlatformPulseIntentRouteTrace {
        self.route
    }

    pub fn payload(&self) -> PlatformPulseIntentPayloadTrace {
        self.payload
    }

    pub fn operability(&self) -> PlatformPulseIntentOperabilityTrace {
        self.operability
    }

    pub fn admission(&self) -> PlatformPulseIntentAdmissionTrace {
        self.admission
    }

    pub fn attempt(&self) -> PlatformPulseIntentAttemptObservationReference {
        self.attempt
    }

    pub fn outcome(&self) -> PlatformPulseIntentOutcomeTrace {
        self.outcome
    }

    pub fn query_projection(&self) -> &PlatformPulseQueryProjectionEvidence {
        &self.query_projection
    }

    pub fn mounted_frame(&self) -> PlatformPulseMountedFrameObservation {
        self.mounted_frame
    }
}

impl CompletedRuntimeTrace {
    fn read(
        trace: UiIntentCausalTraceEvidence,
    ) -> Result<Self, PlatformPulseIntentTraceProjectionDenial> {
        let missing = PlatformPulseIntentTraceProjectionDenial::Incomplete;
        let operability = trace.operability().ok_or(missing)?;
        if operability.posture() != UiIntentCausalTraceOperabilityPosture::Operable {
            return Err(PlatformPulseIntentTraceProjectionDenial::NonOperableAdmission);
        }
        let attempt = trace.attempt().ok_or(missing)?;
        if attempt.posture() != UiIntentCausalTraceAttemptPosture::Completed {
            return Err(PlatformPulseIntentTraceProjectionDenial::NonTerminalAttempt);
        }
        Ok(Self {
            reference: trace.reference(),
            interaction: trace.interaction(),
            route: trace.route().ok_or(missing)?,
            payload: trace.payload().ok_or(missing)?,
            operability,
            admission: trace.admission().ok_or(missing)?,
            attempt,
            completion: trace.completion().ok_or(missing)?,
        })
    }
}

impl From<UiIntentCausalTraceRouteEvidence> for PlatformPulseIntentRouteTrace {
    fn from(route: UiIntentCausalTraceRouteEvidence) -> Self {
        Self {
            graph_node: route.graph_node(),
            definition_digest: route.definition_digest(),
            declaration_digest: route.declaration_digest(),
        }
    }
}

impl From<UiIntentCausalTracePayloadEvidence> for PlatformPulseIntentPayloadTrace {
    fn from(payload: UiIntentCausalTracePayloadEvidence) -> Self {
        Self {
            owner_revision_count: payload.owner_revision_count(),
            primary_owner_revision: payload.primary_owner_revision(),
            owner_revision_digest: payload.owner_revision_digest(),
            admitted_utf8_bytes: count(payload.admitted_utf8_bytes()),
        }
    }
}

impl From<UiIntentCausalTraceOperabilityEvidence> for PlatformPulseIntentOperabilityTrace {
    fn from(operability: UiIntentCausalTraceOperabilityEvidence) -> Self {
        Self {
            operable: true,
            dependency_count: count(operability.dependency_count()),
            decision_digest: operability.decision_digest(),
        }
    }
}

impl From<UiIntentCausalTraceAdmissionEvidence> for PlatformPulseIntentAdmissionTrace {
    fn from(admission: UiIntentCausalTraceAdmissionEvidence) -> Self {
        Self {
            slot: admission.slot(),
            generation: admission.generation(),
            lineage: admission.lineage(),
        }
    }
}

impl From<UiIntentCausalTraceCompletionEvidence> for PlatformPulseIntentOutcomeTrace {
    fn from(completion: UiIntentCausalTraceCompletionEvidence) -> Self {
        Self {
            outcome_schema_digest: completion.outcome_schema_digest(),
            consequence_pending_at_completion: completion.consequence_pending_at_completion(),
            consequence_published: true,
        }
    }
}

fn attempt_reference(
    attempt: UiIntentCausalTraceAttemptEvidence,
) -> PlatformPulseIntentAttemptObservationReference {
    PlatformPulseIntentAttemptObservationReference::from_diagnostic_parts(
        attempt.slot(),
        attempt.generation(),
        attempt.idempotency_session(),
        attempt.idempotency_lineage(),
    )
}

impl PlatformPulseIntentEvidenceReferenceObservation {
    fn from_runtime(reference: worth_ui::facade::inspection::UiIntentEvidenceReference) -> Self {
        Self {
            session: reference.session_diagnostic_value(),
            slot: reference.slot(),
            generation: reference.generation(),
        }
    }

    pub fn session(self) -> u64 {
        self.session
    }

    pub fn slot(self) -> u8 {
        self.slot
    }

    pub fn generation(self) -> u64 {
        self.generation
    }
}

impl PlatformPulseIntentSourceTrace {
    fn from_runtime(input: worth_ui::facade::inspection::UiIntentInteractionEvidenceInput) -> Self {
        let interaction_family = match input.family() {
            worth_ui::facade::inspection::UiIntentInteractionEvidenceFamily::Activate => {
                PlatformPulseIntentInteractionFamily::Activate
            }
            worth_ui::facade::inspection::UiIntentInteractionEvidenceFamily::CommandRoute => {
                PlatformPulseIntentInteractionFamily::CommandRoute
            }
            worth_ui::facade::inspection::UiIntentInteractionEvidenceFamily::EditCommit => {
                PlatformPulseIntentInteractionFamily::EditCommit
            }
            worth_ui::facade::inspection::UiIntentInteractionEvidenceFamily::SelectionCommit => {
                PlatformPulseIntentInteractionFamily::SelectionCommit
            }
            worth_ui::facade::inspection::UiIntentInteractionEvidenceFamily::Submit => {
                PlatformPulseIntentInteractionFamily::Submit
            }
        };
        Self {
            host_sequence: input.source_sequence(),
            presented_frame: input.presented_frame(),
            presentation_epoch: input.presentation_epoch(),
            mounted_instance: input.mounted_instance(),
            semantic_target_digest: input.semantic_target_digest(),
            interaction_family,
        }
    }

    pub fn host_sequence(self) -> u64 {
        self.host_sequence
    }
    pub fn presented_frame(self) -> u64 {
        self.presented_frame
    }
    pub fn presentation_epoch(self) -> u64 {
        self.presentation_epoch
    }
    pub fn mounted_instance(self) -> u64 {
        self.mounted_instance
    }
    pub fn semantic_target_digest(self) -> u64 {
        self.semantic_target_digest
    }
    pub fn interaction_family(self) -> PlatformPulseIntentInteractionFamily {
        self.interaction_family
    }
}

impl PlatformPulseIntentRouteTrace {
    pub fn graph_node(self) -> u64 {
        self.graph_node
    }
    pub fn definition_digest(self) -> u64 {
        self.definition_digest
    }
    pub fn declaration_digest(self) -> u64 {
        self.declaration_digest
    }
}

impl PlatformPulseIntentPayloadTrace {
    pub fn owner_revision_count(self) -> u8 {
        self.owner_revision_count
    }
    pub fn primary_owner_revision(self) -> Option<u64> {
        self.primary_owner_revision
    }
    pub fn owner_revision_digest(self) -> u64 {
        self.owner_revision_digest
    }
    pub fn admitted_utf8_bytes(self) -> u64 {
        self.admitted_utf8_bytes
    }
}

impl PlatformPulseIntentOperabilityTrace {
    pub fn operable(self) -> bool {
        self.operable
    }
    pub fn dependency_count(self) -> u64 {
        self.dependency_count
    }
    pub fn decision_digest(self) -> u64 {
        self.decision_digest
    }
}

impl PlatformPulseIntentAdmissionTrace {
    pub fn slot(self) -> u8 {
        self.slot
    }
    pub fn generation(self) -> u64 {
        self.generation
    }
    pub fn lineage(self) -> u64 {
        self.lineage
    }
}

impl PlatformPulseIntentOutcomeTrace {
    pub fn outcome_schema_digest(self) -> u64 {
        self.outcome_schema_digest
    }
    pub fn consequence_pending_at_completion(self) -> bool {
        self.consequence_pending_at_completion
    }
    pub fn consequence_published(self) -> bool {
        self.consequence_published
    }
}

fn count(value: usize) -> u64 {
    u64::try_from(value).unwrap_or(u64::MAX)
}
