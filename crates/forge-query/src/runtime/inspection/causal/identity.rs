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
use super::observation_identity::{
    CausalEvidenceReferenceDigest, CausalObservationAnchorDigest, CausalObservationTargetHandle,
    CausalResultShapeContextHandle,
};
use super::request::{CausalInspectionExplanationFamily, CausalInspectionRichness};
use forge_runtime_bridge::facade::{
    BridgeCausalEnvelopeDenial, BridgeCausalEnvelopeDenialKind, BridgeCausalEnvelopeIdentity,
    BridgeCausalEnvelopeReceipt, BridgeCausalEvidenceFamily, BridgeCausalExplanationEnvelope,
    BridgeCausalInspectionAdmissionSummaryKind,
};

macro_rules! causal_identity_type {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $name(ForgeQueryEvidenceIdentity);

        impl $name {
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }

            #[allow(dead_code)]
            pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
                &self.0
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
causal_identity_type!(CausalInspectionCertificationErrorIdentity);
causal_identity_type!(CausalInspectionCertificationFailureEvidenceIdentity);

pub(super) fn compose_bridge_causal_envelope_identity(
    identity: &BridgeCausalEnvelopeIdentity,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionArtifact)
        .field_shape(ForgeQueryEvidenceTag::new("role"), "bridge-causal-envelope")
        .field_bridge_identity(
            ForgeQueryEvidenceTag::new("request"),
            &identity.request_evidence_identity(),
        )
        .field_bridge_identity(
            ForgeQueryEvidenceTag::new("anchor"),
            &identity.causal_observation_anchor_evidence_identity(),
        )
        .field_bridge_identity(
            ForgeQueryEvidenceTag::new("bindings"),
            &identity.evidence_binding_evidence_identity(),
        )
        .field_bridge_identity(
            ForgeQueryEvidenceTag::new("counters"),
            &identity.counter_evidence_identity(),
        )
        .field_bridge_identity(
            ForgeQueryEvidenceTag::new("identity"),
            &identity.envelope_evidence_identity(),
        )
        .seal()
}

pub(super) fn compose_bridge_causal_explanation_envelope_identity(
    envelope: &BridgeCausalExplanationEnvelope,
) -> ForgeQueryEvidenceIdentity {
    let envelope_identity = compose_bridge_causal_envelope_identity(envelope.identity());
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionArtifact)
        .field_shape(
            ForgeQueryEvidenceTag::new("role"),
            "bridge-causal-explanation-envelope",
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("identity"), &envelope_identity)
        .field_shape(
            ForgeQueryEvidenceTag::new("summary_kind"),
            bridge_causal_admission_summary_kind_label(envelope.admission_summary_kind()),
        )
        .field_bridge_identity(
            ForgeQueryEvidenceTag::new("admission_summary"),
            &envelope.admission_summary_evidence_identity(),
        )
        .field_bridge_identity(
            ForgeQueryEvidenceTag::new("request"),
            &envelope.request_evidence_identity(),
        )
        .field_bridge_identity(
            ForgeQueryEvidenceTag::new("anchor"),
            &envelope.causal_observation_anchor_evidence_identity(),
        )
        .field_bridge_identity(
            ForgeQueryEvidenceTag::new("envelope"),
            &envelope.envelope_evidence_identity(),
        )
        .seal()
}

pub(crate) fn bridge_causal_admission_summary_kind_label(
    kind: BridgeCausalInspectionAdmissionSummaryKind,
) -> &'static str {
    match kind {
        BridgeCausalInspectionAdmissionSummaryKind::Admitted => "admitted",
        BridgeCausalInspectionAdmissionSummaryKind::Advisory => "advisory",
    }
}

pub(super) fn compose_bridge_causal_envelope_receipt_identity(
    receipt: &BridgeCausalEnvelopeReceipt,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionArtifact)
        .field_shape(
            ForgeQueryEvidenceTag::new("role"),
            "bridge-causal-envelope-receipt",
        )
        .field_bridge_identity(
            ForgeQueryEvidenceTag::new("envelope_identity"),
            &receipt.envelope_identity_evidence(),
        )
        .field_bridge_identity(
            ForgeQueryEvidenceTag::new("envelope"),
            &receipt.envelope_evidence_identity(),
        )
        .field_bridge_identity(
            ForgeQueryEvidenceTag::new("counters"),
            &receipt.counter_evidence_identity(),
        )
        .field_bridge_identity(
            ForgeQueryEvidenceTag::new("receipt"),
            &receipt.receipt_evidence_identity(),
        )
        .seal()
}

pub(super) fn compose_bridge_causal_denial_identity(
    denial: &BridgeCausalEnvelopeDenial,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionDeniedArtifactDetail)
        .field_shape(ForgeQueryEvidenceTag::new("role"), "bridge-causal-denial")
        .field_shape(ForgeQueryEvidenceTag::new("kind"), denial.kind().as_str())
        .field_shape(
            ForgeQueryEvidenceTag::new("family"),
            denial.family().as_str(),
        )
        .field_bridge_identity(
            ForgeQueryEvidenceTag::new("failure"),
            &denial.failure_evidence_identity(),
        )
        .seal()
}

pub(super) fn compose_causal_inspection_target_identity(
    observation_target: &CausalObservationTargetHandle,
    result_shape_context: &CausalResultShapeContextHandle,
) -> CausalInspectionTargetIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionTarget)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("observation_target"),
            observation_target.identity().evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("result_shape_context"),
            result_shape_context.identity().evidence_identity(),
        )
        .seal()
        .into()
}

pub(super) fn compose_causal_inspection_request_identity(
    anchor_digest: &CausalObservationAnchorDigest,
    reference_set_digest: &CausalEvidenceReferenceDigest,
    target_identity: &CausalInspectionTargetIdentity,
    explanation_family: CausalInspectionExplanationFamily,
    requested_richness: CausalInspectionRichness,
    requested_evidence_families: &[CausalEvidenceFamily],
) -> CausalInspectionRequestIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionRequest)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("anchor"),
            anchor_digest.evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("reference_set"),
            reference_set_digest.evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("target"),
            target_identity.evidence_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("family"),
            explanation_family.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("richness"),
            requested_richness.as_str(),
        )
        .field_value_sequence(
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
    evidence: &[ForgeQueryEvidenceIdentity],
) -> CausalInspectionRequestFailureIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionRequestFailure)
        .field_shape(ForgeQueryEvidenceTag::new("kind"), kind)
        .field_value(ForgeQueryEvidenceTag::new("message"), message)
        .field_evidence_identity_sequence(ForgeQueryEvidenceTag::new("evidence"), evidence.iter())
        .seal()
        .into()
}

pub(super) fn compose_causal_admission_subject_identity(
    subject: &CausalInspectionAdmissionSubject,
) -> CausalInspectionAdmissionSubjectIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionAdmissionSubject)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("request"),
            subject.request_identity().evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("anchor"),
            subject.anchor_identity().evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("anchor_counters"),
            subject.anchor_counter_identity().evidence_identity(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("anchor_reference_families"),
            subject.anchor_reference_family_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("lower_runtime_families"),
            subject.lower_runtime_evidence_family_count(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("query"),
            subject.query_identity().evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("query_observation"),
            subject.query_observation_identity().evidence_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("inspection_reason"),
            subject.inspection_reason().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("observation_outcome"),
            subject.observation_outcome().as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("reference_set"),
            subject.reference_set_identity().evidence_identity(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("resolved_references"),
            subject.resolved_reference_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("missing_reference_families"),
            subject.missing_reference_family_count(),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("resolved_evidence_families"),
            subject
                .resolved_evidence_families()
                .iter()
                .map(CausalEvidenceFamily::as_str),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("observation_target"),
            subject.observation_target_identity().evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("result_shape_context"),
            subject.result_shape_context_identity().evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("target"),
            subject.target_identity().evidence_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("family"),
            subject.explanation_family().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("richness"),
            subject.requested_richness().as_str(),
        )
        .field_value_sequence(
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
        .field_value_sequence(
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
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("rows"),
            rows.iter().map(CausalDecisionTraceRow::evidence_identity),
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
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("subject"),
            receipt.subject_identity().evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("decision"),
            receipt.decision_identity().evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("trace"),
            receipt.decision_trace_identity().evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("counters"),
            receipt.counter_identity().evidence_identity(),
        )
        .seal()
        .into()
}

pub(super) fn compose_causal_outcome_identity(
    kind: CausalInspectionAdmissionDecisionKind,
    subject_identity: &CausalInspectionAdmissionSubjectIdentity,
    decision_identity: &CausalInspectionAdmissionDecisionIdentity,
    trace_identity: &CausalInspectionDecisionTraceIdentity,
    receipt_identity: &CausalInspectionAdmissionReceiptIdentity,
) -> CausalInspectionOutcomeIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionOutcome)
        .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("subject"),
            subject_identity.evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("decision"),
            decision_identity.evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("trace"),
            trace_identity.evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("receipt"),
            receipt_identity.evidence_identity(),
        )
        .seal()
        .into()
}

pub(super) fn compose_causal_materialized_detail_identity(
    query_observation_identity: &ForgeQueryEvidenceIdentity,
    advisory_reason: Option<&str>,
    readmission_proof: &CausalBridgeReadmissionProof,
    evidence_references: &[QueryCausalEvidenceReferenceArtifact],
    redaction_policy: CausalInspectionRedactionPolicy,
    materialization_policy: CausalInspectionMaterializationPolicy,
) -> CausalInspectionMaterializedDetailIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionMaterializedDetail)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("query_observation"),
            query_observation_identity,
        )
        .optional_shape(ForgeQueryEvidenceTag::new("advisory"), advisory_reason)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("readmission"),
            readmission_proof.readmission_proof_identity(),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("references"),
            evidence_references
                .iter()
                .map(QueryCausalEvidenceReferenceArtifact::reference_receipt_evidence_identity),
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
    query_observation_identity: &ForgeQueryEvidenceIdentity,
    result_shape_context_identity: &ForgeQueryEvidenceIdentity,
    denial_reason: &str,
    bridge_denial_identity: Option<&ForgeQueryEvidenceIdentity>,
    bridge_denial_kind: Option<BridgeCausalEnvelopeDenialKind>,
    bridge_denial_family: Option<BridgeCausalEvidenceFamily>,
) -> CausalInspectionDeniedArtifactDetailIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionDeniedArtifactDetail)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("query_observation"),
            query_observation_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("result_shape_context"),
            result_shape_context_identity,
        )
        .field_shape(ForgeQueryEvidenceTag::new("reason"), denial_reason)
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("bridge_denial"),
            bridge_denial_identity,
        )
        .optional_shape(
            ForgeQueryEvidenceTag::new("bridge_denial_kind"),
            bridge_denial_kind
                .as_ref()
                .map(BridgeCausalEnvelopeDenialKind::as_str),
        )
        .optional_shape(
            ForgeQueryEvidenceTag::new("bridge_denial_family"),
            bridge_denial_family
                .as_ref()
                .map(BridgeCausalEvidenceFamily::as_str),
        )
        .seal()
        .into()
}

pub(super) fn compose_causal_artifact_identity(
    kind: CausalInspectionArtifactKind,
    query_admission_identity: &CausalInspectionOutcomeIdentity,
    bridge_identity: Option<&ForgeQueryEvidenceIdentity>,
    bridge_envelope: Option<&ForgeQueryEvidenceIdentity>,
    receipt: &ForgeQueryEvidenceIdentity,
    readmission_proof: Option<&ForgeQueryEvidenceIdentity>,
    detail_identity: &ForgeQueryEvidenceIdentity,
) -> CausalInspectionArtifactIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionArtifact)
        .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("query_admission"),
            query_admission_identity.evidence_identity(),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("bridge_identity"),
            bridge_identity,
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("bridge_envelope"),
            bridge_envelope,
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("receipt"), receipt)
        .optional_evidence_identity(ForgeQueryEvidenceTag::new("readmission"), readmission_proof)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("detail"), detail_identity)
        .seal()
        .into()
}

pub(super) fn compose_causal_artifact_causal_identity(
    kind: CausalInspectionArtifactKind,
    query_admission_identity: &CausalInspectionOutcomeIdentity,
    query_observation_identity: &ForgeQueryEvidenceIdentity,
    bridge_identity: Option<&ForgeQueryEvidenceIdentity>,
    bridge_envelope: Option<&ForgeQueryEvidenceIdentity>,
) -> CausalInspectionArtifactIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionArtifactIdentity)
        .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("query_admission"),
            query_admission_identity.evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("query_observation"),
            query_observation_identity,
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("bridge_identity"),
            bridge_identity,
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("bridge_envelope"),
            bridge_envelope,
        )
        .seal()
        .into()
}

pub(super) fn compose_causal_performance_snapshot_identity(
    fixture_size: CausalInspectionScaleFixtureSize,
    artifact_identity: &CausalInspectionArtifactIdentity,
    evidence_reference_width: usize,
    anchor_derivation_slope_counter: usize,
    reference_resolution_slope_counter: usize,
    admission_slope_counter: usize,
    bridge_envelope_slope_counter: usize,
    materialization_slope_counter: usize,
    artifact_serialization_slope_counter: usize,
    bridge_unindexed_scan_count: usize,
    bridge_readmission_proof: Option<&ForgeQueryEvidenceIdentity>,
) -> CausalInspectionPerformanceSnapshotIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionPerformanceSnapshot)
        .field_shape(ForgeQueryEvidenceTag::new("size"), fixture_size.as_str())
        .field_evidence_identity(causal_artifact_identity_tag(), artifact_identity.evidence_identity())
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
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("readmission"),
            bridge_readmission_proof,
        )
        .seal()
        .into()
}

fn causal_artifact_identity_tag() -> ForgeQueryEvidenceTag {
    ForgeQueryEvidenceTag::new("artifact")
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
    anchor_derivation_slope_identity: &CausalInspectionPerformanceSlopeIdentity,
    reference_resolution_slope_identity: &CausalInspectionPerformanceSlopeIdentity,
    admission_slope_identity: &CausalInspectionPerformanceSlopeIdentity,
    bridge_envelope_slope_identity: &CausalInspectionPerformanceSlopeIdentity,
    materialization_slope_identity: &CausalInspectionPerformanceSlopeIdentity,
    artifact_serialization_slope_identity: &CausalInspectionPerformanceSlopeIdentity,
) -> CausalInspectionPerformanceScaleSlopeIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::CausalInspectionPerformanceScaleSlope)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("anchor"),
            anchor_derivation_slope_identity.evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("reference"),
            reference_resolution_slope_identity.evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("admission"),
            admission_slope_identity.evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("bridge_envelope"),
            bridge_envelope_slope_identity.evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("materialization"),
            materialization_slope_identity.evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("serialization"),
            artifact_serialization_slope_identity.evidence_identity(),
        )
        .seal()
        .into()
}

pub(super) fn compose_causal_performance_certification_identity(
    small_snapshot_identity: &CausalInspectionPerformanceSnapshotIdentity,
    medium_snapshot_identity: &CausalInspectionPerformanceSnapshotIdentity,
    large_snapshot_identity: &CausalInspectionPerformanceSnapshotIdentity,
    bridge_readmission_proof_identity: &ForgeQueryEvidenceIdentity,
    scale_slope_identity: &CausalInspectionPerformanceScaleSlopeIdentity,
    anchor_derivation_slope_identity: &CausalInspectionPerformanceSlopeIdentity,
    reference_resolution_slope_identity: &CausalInspectionPerformanceSlopeIdentity,
    admission_slope_identity: &CausalInspectionPerformanceSlopeIdentity,
    bridge_envelope_slope_identity: &CausalInspectionPerformanceSlopeIdentity,
    materialization_slope_identity: &CausalInspectionPerformanceSlopeIdentity,
    artifact_serialization_slope_identity: &CausalInspectionPerformanceSlopeIdentity,
    scale_slope_digest_part_count: usize,
) -> CausalInspectionPerformanceCertificationIdentity {
    forge_query_evidence_identity(
        ForgeQueryEvidenceScope::CausalInspectionPerformanceCertificationBundle,
    )
    .field_evidence_identity(
        ForgeQueryEvidenceTag::new("small"),
        small_snapshot_identity.evidence_identity(),
    )
    .field_evidence_identity(
        ForgeQueryEvidenceTag::new("medium"),
        medium_snapshot_identity.evidence_identity(),
    )
    .field_evidence_identity(
        ForgeQueryEvidenceTag::new("large"),
        large_snapshot_identity.evidence_identity(),
    )
    .field_evidence_identity(
        ForgeQueryEvidenceTag::new("readmission"),
        bridge_readmission_proof_identity,
    )
    .field_evidence_identity(
        ForgeQueryEvidenceTag::new("scale_slope"),
        scale_slope_identity.evidence_identity(),
    )
    .field_evidence_identity(
        ForgeQueryEvidenceTag::new("anchor"),
        anchor_derivation_slope_identity.evidence_identity(),
    )
    .field_evidence_identity(
        ForgeQueryEvidenceTag::new("reference"),
        reference_resolution_slope_identity.evidence_identity(),
    )
    .field_evidence_identity(
        ForgeQueryEvidenceTag::new("admission"),
        admission_slope_identity.evidence_identity(),
    )
    .field_evidence_identity(
        ForgeQueryEvidenceTag::new("bridge_envelope"),
        bridge_envelope_slope_identity.evidence_identity(),
    )
    .field_evidence_identity(
        ForgeQueryEvidenceTag::new("materialization"),
        materialization_slope_identity.evidence_identity(),
    )
    .field_evidence_identity(
        ForgeQueryEvidenceTag::new("serialization"),
        artifact_serialization_slope_identity.evidence_identity(),
    )
    .field_usize(
        ForgeQueryEvidenceTag::new("parts"),
        scale_slope_digest_part_count,
    )
    .seal()
    .into()
}

#[cfg(test)]
pub(crate) fn causal_test_compose_bridge_causal_denial_for_reporting(
    denial: &BridgeCausalEnvelopeDenial,
) -> String {
    compose_bridge_causal_denial_identity(denial).to_string()
}

#[cfg(test)]
pub(crate) fn causal_test_compose_bridge_causal_explanation_envelope_identity_for_reporting(
    envelope: &BridgeCausalExplanationEnvelope,
) -> String {
    compose_bridge_causal_explanation_envelope_identity(envelope).to_string()
}

#[cfg(test)]
pub(crate) fn causal_test_compose_bridge_causal_envelope_identity_for_reporting(
    envelope: &BridgeCausalExplanationEnvelope,
) -> String {
    compose_bridge_causal_envelope_identity(envelope.identity()).to_string()
}

#[cfg(test)]
pub(crate) fn causal_test_compose_bridge_causal_envelope_receipt_identity_for_reporting(
    receipt: &BridgeCausalEnvelopeReceipt,
) -> String {
    compose_bridge_causal_envelope_receipt_identity(receipt).to_string()
}

#[cfg(test)]
pub(crate) fn causal_test_bridge_binding_reference_for_reporting(
    owner: &str,
    family: &str,
    bridge_reference: forge_runtime_bridge::facade::BridgeIdentityEvidence,
) -> String {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::CausalEvidenceReferenceReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("role"),
            "bridge-causal-evidence-reference",
        )
        .field_shape(ForgeQueryEvidenceTag::new("owner"), owner)
        .field_shape(ForgeQueryEvidenceTag::new("family"), family)
        .field_bridge_identity(ForgeQueryEvidenceTag::new("reference"), &bridge_reference)
        .seal()
        .to_string()
}
