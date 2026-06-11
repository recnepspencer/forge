use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::admission_decision::{
    CausalInspectionAdmissionDecision, CausalInspectionAdmissionDecisionKind,
    CausalInspectionAdmissionSubject,
};
use super::admission_trace::{
    CausalDecisionTraceRow, CausalInspectionAdmissionCounters, CausalInspectionAdmissionReceipt,
};
use super::certification::CausalInspectionScaleFixtureSize;
use super::inventory::CausalEvidenceFamily;
use super::materialization::{
    CausalBridgeReadmissionProof, CausalInspectionArtifactKind,
    CausalInspectionMaterializationPolicy, CausalInspectionRedactionPolicy,
    QueryCausalEvidenceReferenceArtifact,
};
use super::observation_identity::{CausalObservationTargetHandle, CausalResultShapeContextHandle};
use super::request::{CausalInspectionExplanationFamily, CausalInspectionRichness};

macro_rules! causal_identity_type {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $name(ForgeQueryEvidenceIdentity);

        impl $name {
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl From<ForgeQueryEvidenceIdentity> for $name {
            fn from(value: ForgeQueryEvidenceIdentity) -> Self {
                Self(value)
            }
        }
    };
}

causal_identity_type!(CausalInspectionTargetIdentity);
causal_identity_type!(CausalInspectionRequestIdentity);
causal_identity_type!(CausalInspectionRequestFailureIdentity);
causal_identity_type!(CausalInspectionAdmissionSubjectIdentity);
causal_identity_type!(CausalInspectionAdmissionDecisionIdentity);
causal_identity_type!(CausalInspectionDecisionTraceRowIdentity);
causal_identity_type!(CausalInspectionDecisionTraceIdentity);
causal_identity_type!(CausalInspectionAdmissionCountersIdentity);
causal_identity_type!(CausalInspectionAdmissionReceiptIdentity);
causal_identity_type!(CausalInspectionOutcomeIdentity);
causal_identity_type!(CausalInspectionMaterializedDetailIdentity);
causal_identity_type!(CausalInspectionDeniedArtifactDetailIdentity);
causal_identity_type!(CausalInspectionArtifactIdentity);
causal_identity_type!(CausalInspectionPerformanceSnapshotIdentity);
causal_identity_type!(CausalInspectionPerformanceSlopeIdentity);
causal_identity_type!(CausalInspectionPerformanceScaleSlopeIdentity);
causal_identity_type!(CausalInspectionPerformanceCertificationIdentity);

pub(super) fn compose_causal_inspection_target_identity(
    observation_target: &CausalObservationTargetHandle,
    result_shape_context: &CausalResultShapeContextHandle,
) -> CausalInspectionTargetIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionTarget)
        .field_identity(
            ForgeQueryEvidenceTag::new("observation_target"),
            observation_target.identity().as_str(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("result_shape_context"),
            result_shape_context.identity().as_str(),
        )
        .seal()
        .into()
}

pub(super) fn compose_causal_inspection_request_identity(
    anchor_digest: &str,
    reference_set_digest: &str,
    target_digest: &str,
    explanation_family: CausalInspectionExplanationFamily,
    requested_richness: CausalInspectionRichness,
    requested_evidence_families: &[CausalEvidenceFamily],
) -> CausalInspectionRequestIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionRequest)
        .field_identity(ForgeQueryEvidenceTag::new("anchor"), anchor_digest)
        .field_identity(
            ForgeQueryEvidenceTag::new("reference_set"),
            reference_set_digest,
        )
        .field_identity(ForgeQueryEvidenceTag::new("target"), target_digest)
        .field_shape(
            ForgeQueryEvidenceTag::new("family"),
            explanation_family.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("richness"),
            requested_richness.as_str(),
        )
        .field_identity_sequence(
            ForgeQueryEvidenceTag::new("evidence_families"),
            requested_evidence_families
                .iter()
                .map(CausalEvidenceFamily::as_str),
        )
        .seal()
        .into()
}

pub(super) fn compose_causal_inspection_request_failure_identity(
    kind: &str,
    message: &str,
    evidence: &[String],
) -> CausalInspectionRequestFailureIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionRequestFailure)
        .field_shape(ForgeQueryEvidenceTag::new("kind"), kind)
        .field_value(ForgeQueryEvidenceTag::new("message"), message)
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("evidence"),
            evidence.iter().map(String::as_str),
        )
        .seal()
        .into()
}

pub(super) fn compose_causal_admission_subject_identity(
    subject: &CausalInspectionAdmissionSubject,
) -> CausalInspectionAdmissionSubjectIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionAdmissionSubject)
        .field_identity(
            ForgeQueryEvidenceTag::new("request"),
            subject.request_digest(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("anchor"),
            subject.anchor_digest(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("anchor_counters"),
            subject.anchor_counter_snapshot(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("anchor_reference_families"),
            subject.anchor_reference_family_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("lower_runtime_families"),
            subject.lower_runtime_evidence_family_count(),
        )
        .field_identity(ForgeQueryEvidenceTag::new("query"), subject.query_digest())
        .field_identity(
            ForgeQueryEvidenceTag::new("query_observation"),
            subject.query_observation_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("inspection_reason"),
            subject.inspection_reason().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("observation_outcome"),
            subject.observation_outcome().as_str(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("reference_set"),
            subject.reference_set_digest(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("resolved_references"),
            subject.resolved_reference_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("missing_reference_families"),
            subject.missing_reference_family_count(),
        )
        .field_identity_sequence(
            ForgeQueryEvidenceTag::new("resolved_evidence_families"),
            subject
                .resolved_evidence_families()
                .iter()
                .map(CausalEvidenceFamily::as_str),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("observation_target"),
            subject.observation_target_digest(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("result_shape_context"),
            subject.result_shape_context_digest(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("target"),
            subject.target_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("family"),
            subject.explanation_family().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("richness"),
            subject.requested_richness().as_str(),
        )
        .field_identity_sequence(
            ForgeQueryEvidenceTag::new("requested_evidence_families"),
            subject
                .requested_evidence_families()
                .iter()
                .map(CausalEvidenceFamily::as_str),
        )
        .seal()
        .into()
}

pub(super) fn compose_causal_admission_decision_identity(
    decision: &CausalInspectionAdmissionDecision,
) -> CausalInspectionAdmissionDecisionIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionAdmissionDecision)
        .field_shape(ForgeQueryEvidenceTag::new("kind"), decision.kind().as_str())
        .optional_shape(
            ForgeQueryEvidenceTag::new("advisory"),
            decision.advisory_kind().map(|kind| kind.as_str()),
        )
        .optional_shape(
            ForgeQueryEvidenceTag::new("violation"),
            decision.violation_kind().map(|kind| kind.as_str()),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("richness"),
            decision.admitted_richness().as_str(),
        )
        .field_identity_sequence(
            ForgeQueryEvidenceTag::new("families"),
            decision
                .permitted_evidence_families()
                .iter()
                .map(CausalEvidenceFamily::as_str),
        )
        .seal()
        .into()
}

pub(super) fn compose_causal_decision_trace_row_identity(
    row: &CausalDecisionTraceRow,
) -> CausalInspectionDecisionTraceRowIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionDecisionTraceRow)
        .field_shape(ForgeQueryEvidenceTag::new("key"), row.key())
        .field_shape(ForgeQueryEvidenceTag::new("span"), row.span())
        .field_shape(
            ForgeQueryEvidenceTag::new("decision"),
            row.decision().as_str(),
        )
        .field_shape(ForgeQueryEvidenceTag::new("authority"), row.authority())
        .field_value(ForgeQueryEvidenceTag::new("reason"), row.reason())
        .seal()
        .into()
}

pub(super) fn compose_causal_decision_trace_identity(
    rows: &[CausalDecisionTraceRow],
) -> CausalInspectionDecisionTraceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionDecisionTraceIndex)
        .field_identity_sequence(
            ForgeQueryEvidenceTag::new("rows"),
            rows.iter().map(CausalDecisionTraceRow::row_digest),
        )
        .seal()
        .into()
}

pub(super) fn compose_causal_admission_counters_identity(
    counters: &CausalInspectionAdmissionCounters,
) -> CausalInspectionAdmissionCountersIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionAdmissionCounters)
        .field_usize(
            ForgeQueryEvidenceTag::new("proof_transition_count"),
            counters.causal_inspection_proof_transition_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("proof_outcome_count"),
            counters.causal_inspection_proof_outcome_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("proof_readmission_count"),
            counters.causal_inspection_proof_readmission_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("request_count"),
            counters.causal_inspection_request_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("admission_count"),
            counters.causal_inspection_admission_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("advisory_count"),
            counters.causal_inspection_advisory_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("denial_count"),
            counters.causal_inspection_denial_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("trace_lookup_count"),
            counters.causal_decision_trace_lookup_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("trace_hit_count"),
            counters.causal_decision_trace_index_hit_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("bridge_envelope_request_count"),
            counters.bridge_causal_envelope_request_count(),
        )
        .seal()
        .into()
}

pub(super) fn compose_causal_admission_receipt_identity(
    receipt: &CausalInspectionAdmissionReceipt,
) -> CausalInspectionAdmissionReceiptIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionAdmissionReceipt)
        .field_identity(
            ForgeQueryEvidenceTag::new("subject"),
            receipt.subject_digest(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("decision"),
            receipt.decision_digest(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("trace"),
            receipt.decision_trace_index_digest(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("counters"),
            receipt.counter_snapshot(),
        )
        .seal()
        .into()
}

pub(super) fn compose_causal_outcome_identity(
    kind: CausalInspectionAdmissionDecisionKind,
    subject_digest: &str,
    decision_digest: &str,
    trace_digest: &str,
    receipt_digest: &str,
) -> CausalInspectionOutcomeIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionOutcome)
        .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
        .field_identity(ForgeQueryEvidenceTag::new("subject"), subject_digest)
        .field_identity(ForgeQueryEvidenceTag::new("decision"), decision_digest)
        .field_identity(ForgeQueryEvidenceTag::new("trace"), trace_digest)
        .field_identity(ForgeQueryEvidenceTag::new("receipt"), receipt_digest)
        .seal()
        .into()
}

pub(super) fn compose_causal_materialized_detail_identity(
    query_observation_digest: &str,
    advisory_reason: Option<&str>,
    readmission_proof: &CausalBridgeReadmissionProof,
    evidence_references: &[QueryCausalEvidenceReferenceArtifact],
    redaction_policy: CausalInspectionRedactionPolicy,
    materialization_policy: CausalInspectionMaterializationPolicy,
) -> CausalInspectionMaterializedDetailIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionMaterializedDetail)
        .field_identity(
            ForgeQueryEvidenceTag::new("query_observation"),
            query_observation_digest,
        )
        .optional_shape(ForgeQueryEvidenceTag::new("advisory"), advisory_reason)
        .field_identity(
            ForgeQueryEvidenceTag::new("readmission"),
            readmission_proof.readmission_proof_digest(),
        )
        .field_identity_sequence(
            ForgeQueryEvidenceTag::new("references"),
            evidence_references
                .iter()
                .map(QueryCausalEvidenceReferenceArtifact::reference_digest),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("redaction"),
            redaction_policy.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("materialization"),
            materialization_policy.as_str(),
        )
        .seal()
        .into()
}

pub(super) fn compose_causal_denied_artifact_detail_identity(
    query_observation_digest: &str,
    result_shape_context_digest: &str,
    denial_reason: &str,
    bridge_denial_digest: Option<&str>,
    bridge_denial_kind: Option<&str>,
    bridge_denial_family: Option<&str>,
) -> CausalInspectionDeniedArtifactDetailIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionDeniedArtifactDetail)
        .field_identity(
            ForgeQueryEvidenceTag::new("query_observation"),
            query_observation_digest,
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("result_shape_context"),
            result_shape_context_digest,
        )
        .field_shape(ForgeQueryEvidenceTag::new("reason"), denial_reason)
        .optional_identity(
            ForgeQueryEvidenceTag::new("bridge_denial"),
            bridge_denial_digest,
        )
        .optional_shape(
            ForgeQueryEvidenceTag::new("bridge_denial_kind"),
            bridge_denial_kind,
        )
        .optional_shape(
            ForgeQueryEvidenceTag::new("bridge_denial_family"),
            bridge_denial_family,
        )
        .seal()
        .into()
}

pub(super) fn compose_causal_artifact_identity(
    kind: CausalInspectionArtifactKind,
    query_admission_digest: &str,
    bridge_identity_digest: Option<&str>,
    bridge_envelope_digest: Option<&str>,
    receipt_digest: &str,
    readmission_proof_digest: Option<&str>,
    detail_digest: &str,
) -> CausalInspectionArtifactIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionArtifact)
        .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
        .field_identity(
            ForgeQueryEvidenceTag::new("query_admission"),
            query_admission_digest,
        )
        .optional_identity(
            ForgeQueryEvidenceTag::new("bridge_identity"),
            bridge_identity_digest,
        )
        .optional_identity(
            ForgeQueryEvidenceTag::new("bridge_envelope"),
            bridge_envelope_digest,
        )
        .field_identity(ForgeQueryEvidenceTag::new("receipt"), receipt_digest)
        .optional_identity(
            ForgeQueryEvidenceTag::new("readmission"),
            readmission_proof_digest,
        )
        .field_identity(ForgeQueryEvidenceTag::new("detail"), detail_digest)
        .seal()
        .into()
}

pub(super) fn compose_causal_artifact_causal_identity(
    kind: CausalInspectionArtifactKind,
    query_admission_digest: &str,
    query_observation_digest: &str,
    bridge_identity_digest: Option<&str>,
    bridge_envelope_digest: Option<&str>,
) -> CausalInspectionArtifactIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionArtifactIdentity)
        .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
        .field_identity(
            ForgeQueryEvidenceTag::new("query_admission"),
            query_admission_digest,
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("query_observation"),
            query_observation_digest,
        )
        .optional_identity(
            ForgeQueryEvidenceTag::new("bridge_identity"),
            bridge_identity_digest,
        )
        .optional_identity(
            ForgeQueryEvidenceTag::new("bridge_envelope"),
            bridge_envelope_digest,
        )
        .seal()
        .into()
}

pub(super) fn compose_causal_performance_snapshot_identity(
    fixture_size: CausalInspectionScaleFixtureSize,
    artifact_digest: &str,
    evidence_reference_width: usize,
    anchor_derivation_slope_counter: usize,
    reference_resolution_slope_counter: usize,
    admission_slope_counter: usize,
    bridge_envelope_slope_counter: usize,
    materialization_slope_counter: usize,
    artifact_serialization_slope_counter: usize,
    bridge_unindexed_scan_count: usize,
    bridge_readmission_proof_digest: Option<&str>,
) -> CausalInspectionPerformanceSnapshotIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionPerformanceSnapshot)
        .field_shape(ForgeQueryEvidenceTag::new("size"), fixture_size.as_str())
        .field_identity(ForgeQueryEvidenceTag::new("artifact"), artifact_digest)
        .field_usize(
            ForgeQueryEvidenceTag::new("evidence_width"),
            evidence_reference_width,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("anchor_slope"),
            anchor_derivation_slope_counter,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("reference_slope"),
            reference_resolution_slope_counter,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("admission_slope"),
            admission_slope_counter,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("bridge_envelope_slope"),
            bridge_envelope_slope_counter,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("materialization_slope"),
            materialization_slope_counter,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("serialization_slope"),
            artifact_serialization_slope_counter,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("bridge_unindexed_scan"),
            bridge_unindexed_scan_count,
        )
        .optional_identity(
            ForgeQueryEvidenceTag::new("readmission"),
            bridge_readmission_proof_digest,
        )
        .seal()
        .into()
}

pub(super) fn compose_causal_performance_slope_identity(
    label: &str,
    small: usize,
    medium: usize,
    large: usize,
) -> CausalInspectionPerformanceSlopeIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionPerformanceSlope)
        .field_shape(ForgeQueryEvidenceTag::new("label"), label)
        .field_usize(ForgeQueryEvidenceTag::new("small"), small)
        .field_usize(ForgeQueryEvidenceTag::new("medium"), medium)
        .field_usize(ForgeQueryEvidenceTag::new("large"), large)
        .seal()
        .into()
}

pub(super) fn compose_causal_performance_scale_slope_identity(
    anchor_derivation_slope_digest: &str,
    reference_resolution_slope_digest: &str,
    admission_slope_digest: &str,
    bridge_envelope_slope_digest: &str,
    materialization_slope_digest: &str,
    artifact_serialization_slope_digest: &str,
) -> CausalInspectionPerformanceScaleSlopeIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionPerformanceScaleSlope)
        .field_identity(
            ForgeQueryEvidenceTag::new("anchor"),
            anchor_derivation_slope_digest,
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("reference"),
            reference_resolution_slope_digest,
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("admission"),
            admission_slope_digest,
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("bridge_envelope"),
            bridge_envelope_slope_digest,
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("materialization"),
            materialization_slope_digest,
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("serialization"),
            artifact_serialization_slope_digest,
        )
        .seal()
        .into()
}

pub(super) fn compose_causal_performance_certification_identity(
    small_snapshot_digest: &str,
    medium_snapshot_digest: &str,
    large_snapshot_digest: &str,
    bridge_readmission_proof_digest: &str,
    scale_slope_digest: &str,
    anchor_derivation_slope_digest: &str,
    reference_resolution_slope_digest: &str,
    admission_slope_digest: &str,
    bridge_envelope_slope_digest: &str,
    materialization_slope_digest: &str,
    artifact_serialization_slope_digest: &str,
    scale_slope_digest_part_count: usize,
) -> CausalInspectionPerformanceCertificationIdentity {
    forge_query_evidence_identity(
        ForgeQueryEvidenceScope::CausalInspectionPerformanceCertificationBundle,
    )
    .field_identity(ForgeQueryEvidenceTag::new("small"), small_snapshot_digest)
    .field_identity(ForgeQueryEvidenceTag::new("medium"), medium_snapshot_digest)
    .field_identity(ForgeQueryEvidenceTag::new("large"), large_snapshot_digest)
    .field_identity(
        ForgeQueryEvidenceTag::new("readmission"),
        bridge_readmission_proof_digest,
    )
    .field_identity(
        ForgeQueryEvidenceTag::new("scale_slope"),
        scale_slope_digest,
    )
    .field_identity(
        ForgeQueryEvidenceTag::new("anchor"),
        anchor_derivation_slope_digest,
    )
    .field_identity(
        ForgeQueryEvidenceTag::new("reference"),
        reference_resolution_slope_digest,
    )
    .field_identity(
        ForgeQueryEvidenceTag::new("admission"),
        admission_slope_digest,
    )
    .field_identity(
        ForgeQueryEvidenceTag::new("bridge_envelope"),
        bridge_envelope_slope_digest,
    )
    .field_identity(
        ForgeQueryEvidenceTag::new("materialization"),
        materialization_slope_digest,
    )
    .field_identity(
        ForgeQueryEvidenceTag::new("serialization"),
        artifact_serialization_slope_digest,
    )
    .field_usize(
        ForgeQueryEvidenceTag::new("parts"),
        scale_slope_digest_part_count,
    )
    .seal()
    .into()
}
