mod artifact;
mod bridge_denial;
mod contract;
mod denied_artifact;
mod exploration;
mod performance;
mod policy;
mod proof;
mod receipt;

use super::admission::{
    AdmittedCausalInspection, AdvisoryCausalInspection, DeniedCausalInspection,
};
use crate::identity::hash_parts;
pub use artifact::{
    AdmittedQueryCausalInspectionArtifact, AdvisoryQueryCausalInspectionArtifact,
    QueryCausalEvidenceReferenceArtifact, QueryCausalInspectionArtifact,
};
pub(crate) use bridge_denial::{
    materialize_bridge_denied_admitted_causal_inspection,
    materialize_bridge_denied_advisory_causal_inspection,
};
use contract::validate_materialization_contract;
pub use denied_artifact::DeniedQueryCausalInspectionArtifact;
pub use exploration::{CausalInspectionArtifactDecisionTrace, CausalInspectionArtifactIntegrity};
use forge_runtime_bridge::facade::{
    BridgeCausalEnvelopeDenial, BridgeCausalExplanationEnvelope,
    BridgeCausalInspectionAdmissionSummary, BridgeCausalInspectionAdmissionSummaryKind,
};
pub use performance::CausalInspectionPerformanceEnvelope;
pub use policy::{
    CausalInspectionArtifactKind, CausalInspectionBoundaryEnvelopeCategory,
    CausalInspectionMaterializationError, CausalInspectionMaterializationErrorKind,
    CausalInspectionMaterializationPolicy, CausalInspectionRedactionPolicy,
};
pub use proof::CausalBridgeReadmissionProof;
pub use receipt::CausalMaterializationReceipt;
pub fn materialize_admitted_causal_inspection(
    inspection: &AdmittedCausalInspection,
    envelope: &BridgeCausalExplanationEnvelope,
    redaction_policy: CausalInspectionRedactionPolicy,
    materialization_policy: CausalInspectionMaterializationPolicy,
) -> Result<QueryCausalInspectionArtifact, CausalInspectionMaterializationError> {
    let readmission_proof = validate_bridge_summary(
        inspection.admitted_inspection_digest(),
        inspection.subject().anchor_digest(),
        BridgeCausalInspectionAdmissionSummaryKind::Admitted,
        envelope,
    )?;
    validate_materialization_contract(
        inspection.subject().query_observation_digest(),
        inspection.subject().requested_evidence_families(),
        envelope,
        materialization_policy,
    )?;
    let built = build_bridge_backed_artifact(
        CausalInspectionArtifactKind::Admitted,
        inspection.admitted_inspection_digest(),
        inspection.subject().query_observation_digest(),
        None,
        envelope,
        &readmission_proof,
        redaction_policy,
        materialization_policy,
    );
    Ok(QueryCausalInspectionArtifact::Admitted(
        AdmittedQueryCausalInspectionArtifact::from_parts(
            inspection.admitted_inspection_digest(),
            inspection.subject().query_observation_digest(),
            inspection.subject().result_shape_context_digest(),
            envelope,
            built,
        ),
    ))
}

pub fn materialize_advisory_causal_inspection(
    inspection: &AdvisoryCausalInspection,
    envelope: &BridgeCausalExplanationEnvelope,
    redaction_policy: CausalInspectionRedactionPolicy,
    materialization_policy: CausalInspectionMaterializationPolicy,
) -> Result<QueryCausalInspectionArtifact, CausalInspectionMaterializationError> {
    let readmission_proof = validate_bridge_summary(
        inspection.advisory_inspection_digest(),
        inspection.subject().anchor_digest(),
        BridgeCausalInspectionAdmissionSummaryKind::Advisory,
        envelope,
    )?;
    validate_materialization_contract(
        inspection.subject().query_observation_digest(),
        inspection.subject().requested_evidence_families(),
        envelope,
        materialization_policy,
    )?;
    let advisory_reason = inspection
        .decision()
        .advisory_kind()
        .map_or("advisory".to_string(), |kind| kind.as_str().to_string());
    let built = build_bridge_backed_artifact(
        CausalInspectionArtifactKind::Advisory,
        inspection.advisory_inspection_digest(),
        inspection.subject().query_observation_digest(),
        Some(&advisory_reason),
        envelope,
        &readmission_proof,
        redaction_policy,
        materialization_policy,
    );
    Ok(QueryCausalInspectionArtifact::Advisory(
        AdvisoryQueryCausalInspectionArtifact::from_parts(
            inspection.advisory_inspection_digest(),
            inspection.subject().query_observation_digest(),
            inspection.subject().result_shape_context_digest(),
            advisory_reason,
            envelope,
            built,
        ),
    ))
}

pub fn materialize_denied_causal_inspection(
    inspection: &DeniedCausalInspection,
    bridge_denial: Option<&BridgeCausalEnvelopeDenial>,
    redaction_policy: CausalInspectionRedactionPolicy,
    materialization_policy: CausalInspectionMaterializationPolicy,
) -> QueryCausalInspectionArtifact {
    let denial_reason = inspection
        .decision()
        .violation_kind()
        .map_or("bridge_envelope_denial".to_string(), |kind| {
            kind.as_str().to_string()
        });
    let performance = bridge_denial.map_or_else(
        CausalInspectionPerformanceEnvelope::for_denied_query,
        CausalInspectionPerformanceEnvelope::for_bridge_denial,
    );
    let bridge_denial_digest = bridge_denial.map(|denial| denial.failure_digest().to_string());
    let bridge_denial_kind = bridge_denial.map(|denial| denial.kind().as_str().to_string());
    let bridge_denial_family = bridge_denial.map(|denial| denial.family().as_str().to_string());
    let boundary_categories = policy::boundary_categories();
    let detail_digest = hash_parts(&[
        "denied_query_causal_inspection_artifact_detail_v1".to_string(),
        format!(
            "query-observation:{}",
            inspection.subject().query_observation_digest()
        ),
        format!(
            "result-shape:{}",
            inspection.subject().result_shape_context_digest()
        ),
        format!("reason:{denial_reason}"),
        format!(
            "bridge-denial:{}",
            bridge_denial_digest.as_deref().unwrap_or("none")
        ),
        format!(
            "bridge-denial-kind:{}",
            bridge_denial_kind.as_deref().unwrap_or("none")
        ),
        format!(
            "bridge-denial-family:{}",
            bridge_denial_family.as_deref().unwrap_or("none")
        ),
    ]);
    let receipt = CausalMaterializationReceipt::new(
        inspection.denied_inspection_digest(),
        None,
        None,
        redaction_policy,
        materialization_policy,
        &performance,
        &detail_digest,
    );
    let artifact_digest = artifact_digest(
        CausalInspectionArtifactKind::Denied,
        inspection.denied_inspection_digest(),
        None,
        None,
        &receipt,
        None,
        &detail_digest,
    );
    QueryCausalInspectionArtifact::Denied(DeniedQueryCausalInspectionArtifact::from_parts(
        inspection.denied_inspection_digest(),
        denial_reason,
        inspection.subject().query_observation_digest(),
        inspection.subject().result_shape_context_digest(),
        bridge_denial_digest,
        bridge_denial_kind,
        bridge_denial_family,
        boundary_categories,
        performance,
        receipt,
        artifact_digest,
    ))
}

pub(super) struct BuiltBridgeBackedArtifact {
    pub(super) boundary_categories: Vec<CausalInspectionBoundaryEnvelopeCategory>,
    pub(super) evidence_references: Vec<QueryCausalEvidenceReferenceArtifact>,
    pub(super) performance: CausalInspectionPerformanceEnvelope,
    pub(super) receipt: CausalMaterializationReceipt,
    pub(super) readmission_proof: CausalBridgeReadmissionProof,
    pub(super) causal_identity_digest: String,
    pub(super) artifact_digest: String,
}

fn validate_bridge_summary(
    query_admission_digest: &str,
    anchor_digest: &str,
    expected_kind: BridgeCausalInspectionAdmissionSummaryKind,
    envelope: &BridgeCausalExplanationEnvelope,
) -> Result<CausalBridgeReadmissionProof, CausalInspectionMaterializationError> {
    if envelope.admission_summary_kind() != expected_kind {
        return Err(CausalInspectionMaterializationError::new(
            CausalInspectionMaterializationErrorKind::AdmissionSummaryKindMismatch,
            &[
                format!("expected:{expected_kind:?}"),
                format!("actual:{:?}", envelope.admission_summary_kind()),
            ],
        ));
    }
    let expected_summary = match expected_kind {
        BridgeCausalInspectionAdmissionSummaryKind::Admitted => {
            BridgeCausalInspectionAdmissionSummary::admitted(query_admission_digest, anchor_digest)
        }
        BridgeCausalInspectionAdmissionSummaryKind::Advisory => {
            BridgeCausalInspectionAdmissionSummary::advisory(query_admission_digest, anchor_digest)
        }
    }
    .expect("existing Query admission and anchor digests should form a bridge summary");
    if expected_summary.summary_digest() != envelope.admission_summary_digest() {
        return Err(CausalInspectionMaterializationError::new(
            CausalInspectionMaterializationErrorKind::AdmissionSummaryDigestMismatch,
            &[
                format!("expected:{}", expected_summary.summary_digest()),
                format!("actual:{}", envelope.admission_summary_digest()),
            ],
        ));
    }
    Ok(
        CausalBridgeReadmissionProof::from_readmitted_bridge_envelope(
            query_admission_digest,
            anchor_digest,
            envelope,
        ),
    )
}

fn build_bridge_backed_artifact(
    kind: CausalInspectionArtifactKind,
    query_admission_digest: &str,
    query_observation_digest: &str,
    advisory_reason: Option<&str>,
    envelope: &BridgeCausalExplanationEnvelope,
    readmission_proof: &CausalBridgeReadmissionProof,
    redaction_policy: CausalInspectionRedactionPolicy,
    materialization_policy: CausalInspectionMaterializationPolicy,
) -> BuiltBridgeBackedArtifact {
    let evidence_references = envelope
        .bindings()
        .iter()
        .map(|binding| {
            QueryCausalEvidenceReferenceArtifact::from_bridge_binding(binding, redaction_policy)
        })
        .collect::<Vec<_>>();
    let redaction_count = evidence_references
        .iter()
        .filter(|reference| reference.detail_redacted())
        .count();
    let performance =
        CausalInspectionPerformanceEnvelope::for_bridge_envelope(envelope, redaction_count);
    let detail_digest = materialized_detail_digest(
        query_observation_digest,
        advisory_reason,
        readmission_proof,
        &evidence_references,
        redaction_policy,
        materialization_policy,
    );
    let receipt = CausalMaterializationReceipt::new(
        query_admission_digest,
        Some(envelope.envelope_digest()),
        Some(envelope.receipt().receipt_digest()),
        redaction_policy,
        materialization_policy,
        &performance,
        &detail_digest,
    );
    let artifact_digest = artifact_digest(
        kind,
        query_admission_digest,
        Some(envelope.identity().identity_digest()),
        Some(envelope.envelope_digest()),
        &receipt,
        Some(readmission_proof),
        &detail_digest,
    );
    let causal_identity_digest = causal_identity_digest(
        kind,
        query_admission_digest,
        query_observation_digest,
        Some(envelope.identity().identity_digest()),
        Some(envelope.envelope_digest()),
    );
    BuiltBridgeBackedArtifact {
        boundary_categories: policy::boundary_categories(),
        evidence_references,
        performance,
        receipt,
        readmission_proof: readmission_proof.clone(),
        causal_identity_digest,
        artifact_digest,
    }
}

pub(super) fn causal_identity_digest(
    kind: CausalInspectionArtifactKind,
    query_admission_digest: &str,
    query_observation_digest: &str,
    bridge_identity_digest: Option<&str>,
    bridge_envelope_digest: Option<&str>,
) -> String {
    hash_parts(&[
        "query_causal_inspection_causal_identity_v1".to_string(),
        format!("kind:{}", kind.as_str()),
        format!("query-admission:{query_admission_digest}"),
        format!("query-observation:{query_observation_digest}"),
        format!(
            "bridge-identity:{}",
            bridge_identity_digest.unwrap_or("none")
        ),
        format!(
            "bridge-envelope:{}",
            bridge_envelope_digest.unwrap_or("none")
        ),
    ])
}

fn materialized_detail_digest(
    query_observation_digest: &str,
    advisory_reason: Option<&str>,
    readmission_proof: &CausalBridgeReadmissionProof,
    evidence_references: &[QueryCausalEvidenceReferenceArtifact],
    redaction_policy: CausalInspectionRedactionPolicy,
    materialization_policy: CausalInspectionMaterializationPolicy,
) -> String {
    let reference_part = evidence_references
        .iter()
        .map(QueryCausalEvidenceReferenceArtifact::reference_digest)
        .collect::<Vec<_>>()
        .join("|");
    hash_parts(&[
        "query_causal_inspection_materialized_detail_v1".to_string(),
        format!("query-observation:{query_observation_digest}"),
        format!("advisory:{}", advisory_reason.unwrap_or("none")),
        format!(
            "readmission:{}",
            readmission_proof.readmission_proof_digest()
        ),
        format!("references:{reference_part}"),
        format!("redaction:{}", redaction_policy.as_str()),
        format!("materialization:{}", materialization_policy.as_str()),
    ])
}

fn artifact_digest(
    kind: CausalInspectionArtifactKind,
    query_admission_digest: &str,
    bridge_identity_digest: Option<&str>,
    bridge_envelope_digest: Option<&str>,
    receipt: &CausalMaterializationReceipt,
    readmission_proof: Option<&CausalBridgeReadmissionProof>,
    detail_digest: &str,
) -> String {
    hash_parts(&[
        "query_causal_inspection_artifact_v1".to_string(),
        format!("kind:{}", kind.as_str()),
        format!("query-admission:{query_admission_digest}"),
        format!(
            "bridge-identity:{}",
            bridge_identity_digest.unwrap_or("none")
        ),
        format!(
            "bridge-envelope:{}",
            bridge_envelope_digest.unwrap_or("none")
        ),
        format!("receipt:{}", receipt.receipt_digest()),
        format!(
            "readmission:{}",
            readmission_proof
                .map(CausalBridgeReadmissionProof::readmission_proof_digest)
                .unwrap_or("none")
        ),
        format!("detail:{detail_digest}"),
    ])
}
