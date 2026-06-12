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
    compose_bridge_causal_denial_identity, compose_bridge_causal_envelope_identity,
    compose_bridge_causal_envelope_receipt_identity,
    compose_bridge_causal_explanation_envelope_identity, compose_causal_artifact_causal_identity,
    compose_causal_artifact_identity, compose_causal_denied_artifact_detail_identity,
    compose_causal_materialized_detail_identity, CausalInspectionArtifactIdentity,
};
use crate::evidence_identity::ForgeQueryEvidenceIdentity;
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
    BridgeIdentityEvidence,
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
        inspection.subject().query_observation_evidence_identity(),
        inspection.subject().requested_evidence_families(),
        envelope,
        materialization_policy,
    )?;
    let built = build_bridge_backed_artifact(
        CausalInspectionArtifactKind::Admitted,
        inspection.admitted_inspection_identity(),
        inspection.subject().query_observation_evidence_identity(),
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
        inspection.subject().query_observation_evidence_identity(),
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
        inspection.subject().query_observation_evidence_identity(),
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
    let bridge_denial_identity = bridge_denial.map(compose_bridge_causal_denial_identity);
    let bridge_denial_kind = bridge_denial.map(BridgeCausalEnvelopeDenial::kind);
    let bridge_denial_family = bridge_denial.map(BridgeCausalEnvelopeDenial::family);
    let boundary_categories = policy::boundary_categories();
    let detail_identity = compose_causal_denied_artifact_detail_identity(
        inspection.subject().query_observation_evidence_identity(),
        inspection.subject().result_shape_context_digest(),
        &denial_reason,
        bridge_denial_identity.as_ref(),
        bridge_denial_kind,
        bridge_denial_family,
    );
    let receipt = CausalMaterializationReceipt::new(
        inspection.denied_inspection_identity(),
        None,
        None,
        redaction_policy,
        materialization_policy,
        &performance,
        detail_identity.evidence_identity(),
    );
    let artifact_identity = artifact_identity(
        CausalInspectionArtifactKind::Denied,
        inspection.denied_inspection_identity(),
        None,
        None,
        &receipt,
        None,
        detail_identity.evidence_identity(),
    );
    let temporal_async_explanation =
        project_denied_temporal_async_explanation(inspection, bridge_denial_family);
    QueryCausalInspectionArtifact::Denied(DeniedQueryCausalInspectionArtifact::from_parts(
        inspection.denied_inspection_identity(),
        denial_reason,
        inspection.subject().query_observation_identity(),
        inspection.subject().result_shape_context_identity(),
        bridge_denial_identity,
        bridge_denial_kind,
        bridge_denial_family,
        temporal_async_explanation,
        boundary_categories,
        performance,
        receipt,
        artifact_identity,
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
                BridgeIdentityEvidence::from_external_authority(
                    query_admission_identity.evidence_identity(),
                ),
                BridgeIdentityEvidence::from_external_authority(
                    anchor_identity.evidence_identity(),
                ),
            )
        }
        BridgeCausalInspectionAdmissionSummaryKind::Advisory => {
            BridgeCausalInspectionAdmissionSummary::advisory(
                BridgeIdentityEvidence::from_external_authority(
                    query_admission_identity.evidence_identity(),
                ),
                BridgeIdentityEvidence::from_external_authority(
                    anchor_identity.evidence_identity(),
                ),
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
    query_observation_identity: &ForgeQueryEvidenceIdentity,
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
    let detail_identity = materialized_detail_identity(
        query_observation_identity,
        advisory_reason,
        readmission_proof,
        &evidence_references,
        redaction_policy,
        materialization_policy,
    );
    let bridge_identity = compose_bridge_causal_envelope_identity(envelope.identity());
    let bridge_envelope_identity = compose_bridge_causal_explanation_envelope_identity(envelope);
    let bridge_receipt_identity =
        compose_bridge_causal_envelope_receipt_identity(envelope.receipt());
    let receipt = CausalMaterializationReceipt::new(
        query_admission_identity,
        Some(&bridge_envelope_identity),
        Some(&bridge_receipt_identity),
        redaction_policy,
        materialization_policy,
        &performance,
        detail_identity.evidence_identity(),
    );
    let artifact_identity = artifact_identity(
        kind,
        query_admission_identity,
        Some(&bridge_identity),
        Some(&bridge_envelope_identity),
        &receipt,
        Some(readmission_proof),
        detail_identity.evidence_identity(),
    );
    let causal_identity = causal_identity_digest(
        kind,
        query_admission_identity,
        query_observation_identity,
        Some(&bridge_identity),
        Some(&bridge_envelope_identity),
    );
    BuiltBridgeBackedArtifact {
        boundary_categories: policy::boundary_categories(),
        evidence_references,
        performance,
        receipt,
        readmission_proof: readmission_proof.clone(),
        causal_identity,
        artifact_identity,
    }
}

pub(super) fn causal_identity_digest(
    kind: CausalInspectionArtifactKind,
    query_admission_identity: &super::identity::CausalInspectionOutcomeIdentity,
    query_observation_identity: &ForgeQueryEvidenceIdentity,
    bridge_identity: Option<&ForgeQueryEvidenceIdentity>,
    bridge_envelope: Option<&ForgeQueryEvidenceIdentity>,
) -> CausalInspectionArtifactIdentity {
    compose_causal_artifact_causal_identity(
        kind,
        query_admission_identity,
        query_observation_identity,
        bridge_identity,
        bridge_envelope,
    )
}

fn materialized_detail_identity(
    query_observation_identity: &ForgeQueryEvidenceIdentity,
    advisory_reason: Option<&str>,
    readmission_proof: &CausalBridgeReadmissionProof,
    evidence_references: &[QueryCausalEvidenceReferenceArtifact],
    redaction_policy: CausalInspectionRedactionPolicy,
    materialization_policy: CausalInspectionMaterializationPolicy,
) -> super::identity::CausalInspectionMaterializedDetailIdentity {
    compose_causal_materialized_detail_identity(
        query_observation_identity,
        advisory_reason,
        readmission_proof,
        evidence_references,
        redaction_policy,
        materialization_policy,
    )
}

fn artifact_identity(
    kind: CausalInspectionArtifactKind,
    query_admission_identity: &super::identity::CausalInspectionOutcomeIdentity,
    bridge_identity: Option<&ForgeQueryEvidenceIdentity>,
    bridge_envelope: Option<&ForgeQueryEvidenceIdentity>,
    receipt: &CausalMaterializationReceipt,
    readmission_proof: Option<&CausalBridgeReadmissionProof>,
    detail_identity: &ForgeQueryEvidenceIdentity,
) -> CausalInspectionArtifactIdentity {
    compose_causal_artifact_identity(
        kind,
        query_admission_identity,
        bridge_identity,
        bridge_envelope,
        receipt.receipt_identity(),
        readmission_proof.map(CausalBridgeReadmissionProof::readmission_proof_identity),
        detail_identity,
    )
}
