mod artifacts;
mod bridge_denial;
mod contract;
mod exploration;
mod performance;
mod policy;
mod proof;
mod receipt;
mod temporal_async;
use super::admission::{
    AdmittedCausalInspection, AdvisoryCausalInspection, DeniedCausalInspection,
};
use super::identity::{
    compose_causal_artifact_causal_identity, compose_causal_artifact_identity,
    compose_causal_denied_artifact_detail_identity, compose_causal_materialized_detail_identity,
};
use artifacts::BuiltBridgeBackedArtifact;
pub use artifacts::DeniedQueryCausalInspectionArtifact;
pub use artifacts::{
    AdmittedQueryCausalInspectionArtifact, AdvisoryQueryCausalInspectionArtifact,
    QueryCausalEvidenceReferenceArtifact, QueryCausalInspectionArtifact,
};
pub(crate) use bridge_denial::{
    materialize_bridge_denied_admitted_causal_inspection,
    materialize_bridge_denied_advisory_causal_inspection,
};
use contract::validate_materialization_contract;
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
use temporal_async::{
    project_admitted_temporal_async_explanation, project_advisory_temporal_async_explanation,
    project_denied_temporal_async_explanation,
};
pub use temporal_async::{
    QueryCausalTemporalAsyncExplanation, QueryCausalTemporalAsyncExplanationKind,
};

pub fn materialize_admitted_causal_inspection(
    inspection: &AdmittedCausalInspection,
    envelope: &BridgeCausalExplanationEnvelope,
    redaction_policy: CausalInspectionRedactionPolicy,
    materialization_policy: CausalInspectionMaterializationPolicy,
) -> Result<QueryCausalInspectionArtifact, CausalInspectionMaterializationError> {
    let readmission_proof = validate_bridge_summary(
        inspection.admitted_inspection_identity(),
        inspection.subject().anchor_identity(),
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
        inspection.admitted_inspection_identity(),
        inspection.subject().query_observation_digest(),
        None,
        envelope,
        &readmission_proof,
        redaction_policy,
        materialization_policy,
    );
    let temporal_async_explanation = project_admitted_temporal_async_explanation(inspection);
    Ok(QueryCausalInspectionArtifact::Admitted(
        AdmittedQueryCausalInspectionArtifact::from_parts(
            inspection.admitted_inspection_identity(),
            inspection.subject().query_observation_identity(),
            inspection.subject().result_shape_context_identity(),
            envelope,
            temporal_async_explanation,
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
        inspection.advisory_inspection_identity(),
        inspection.subject().anchor_identity(),
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
        inspection.advisory_inspection_identity(),
        inspection.subject().query_observation_digest(),
        Some(&advisory_reason),
        envelope,
        &readmission_proof,
        redaction_policy,
        materialization_policy,
    );
    let temporal_async_explanation = project_advisory_temporal_async_explanation(inspection);
    Ok(QueryCausalInspectionArtifact::Advisory(
        AdvisoryQueryCausalInspectionArtifact::from_parts(
            inspection.advisory_inspection_identity(),
            inspection.subject().query_observation_identity(),
            inspection.subject().result_shape_context_identity(),
            advisory_reason,
            envelope,
            temporal_async_explanation,
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
    let detail_digest = compose_causal_denied_artifact_detail_identity(
        inspection.subject().query_observation_digest(),
        inspection.subject().result_shape_context_digest(),
        &denial_reason,
        bridge_denial_digest.as_deref(),
        bridge_denial_kind.as_deref(),
        bridge_denial_family.as_deref(),
    )
    .as_str()
    .to_string();
    let receipt = CausalMaterializationReceipt::new(
        inspection.denied_inspection_identity(),
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
    let temporal_async_explanation =
        project_denied_temporal_async_explanation(inspection, bridge_denial_family.as_deref());
    QueryCausalInspectionArtifact::Denied(DeniedQueryCausalInspectionArtifact::from_parts(
        inspection.denied_inspection_identity(),
        denial_reason,
        inspection.subject().query_observation_identity(),
        inspection.subject().result_shape_context_identity(),
        bridge_denial_digest,
        bridge_denial_kind,
        bridge_denial_family,
        temporal_async_explanation,
        boundary_categories,
        performance,
        receipt,
        artifact_digest,
    ))
}
fn validate_bridge_summary(
    query_admission_identity: &super::identity::CausalInspectionOutcomeIdentity,
    anchor_identity: &super::observation_identity::CausalObservationAnchorDigest,
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
            BridgeCausalInspectionAdmissionSummary::admitted(
                query_admission_identity.as_str(),
                anchor_identity.as_str(),
            )
        }
        BridgeCausalInspectionAdmissionSummaryKind::Advisory => {
            BridgeCausalInspectionAdmissionSummary::advisory(
                query_admission_identity.as_str(),
                anchor_identity.as_str(),
            )
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
            query_admission_identity,
            anchor_identity,
            envelope,
        ),
    )
}

fn build_bridge_backed_artifact(
    kind: CausalInspectionArtifactKind,
    query_admission_identity: &super::identity::CausalInspectionOutcomeIdentity,
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
        query_admission_identity,
        Some(envelope.envelope_digest()),
        Some(envelope.receipt().receipt_digest()),
        redaction_policy,
        materialization_policy,
        &performance,
        &detail_digest,
    );
    let artifact_digest = artifact_digest(
        kind,
        query_admission_identity.as_str(),
        Some(envelope.identity().identity_digest()),
        Some(envelope.envelope_digest()),
        &receipt,
        Some(readmission_proof),
        &detail_digest,
    );
    let causal_identity_digest = causal_identity_digest(
        kind,
        query_admission_identity.as_str(),
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
    compose_causal_artifact_causal_identity(
        kind,
        query_admission_digest,
        query_observation_digest,
        bridge_identity_digest,
        bridge_envelope_digest,
    )
    .as_str()
    .to_string()
}

fn materialized_detail_digest(
    query_observation_digest: &str,
    advisory_reason: Option<&str>,
    readmission_proof: &CausalBridgeReadmissionProof,
    evidence_references: &[QueryCausalEvidenceReferenceArtifact],
    redaction_policy: CausalInspectionRedactionPolicy,
    materialization_policy: CausalInspectionMaterializationPolicy,
) -> String {
    compose_causal_materialized_detail_identity(
        query_observation_digest,
        advisory_reason,
        readmission_proof,
        evidence_references,
        redaction_policy,
        materialization_policy,
    )
    .as_str()
    .to_string()
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
    compose_causal_artifact_identity(
        kind,
        query_admission_digest,
        bridge_identity_digest,
        bridge_envelope_digest,
        receipt.receipt_digest(),
        readmission_proof.map(CausalBridgeReadmissionProof::readmission_proof_digest),
        detail_digest,
    )
    .as_str()
    .to_string()
}
