use crate::identity::hash_parts;

use forge_runtime_bridge::facade::BridgeCausalEnvelopeDenial;

use super::super::admission::{AdmittedCausalInspection, AdvisoryCausalInspection};
use super::super::identity::CausalInspectionOutcomeIdentity;
use super::{
    artifact_digest, policy, CausalInspectionArtifactKind, CausalInspectionMaterializationPolicy,
    CausalInspectionPerformanceEnvelope, CausalInspectionRedactionPolicy,
    CausalMaterializationReceipt, DeniedQueryCausalInspectionArtifact,
    QueryCausalInspectionArtifact, QueryCausalTemporalAsyncExplanation,
};

pub(crate) fn materialize_bridge_denied_admitted_causal_inspection(
    inspection: &AdmittedCausalInspection,
    bridge_denial: &BridgeCausalEnvelopeDenial,
    redaction_policy: CausalInspectionRedactionPolicy,
    materialization_policy: CausalInspectionMaterializationPolicy,
) -> QueryCausalInspectionArtifact {
    materialize_bridge_denied_query_inspection(
        inspection,
        inspection.admitted_inspection_identity(),
        bridge_denial,
        redaction_policy,
        materialization_policy,
    )
}

pub(crate) fn materialize_bridge_denied_advisory_causal_inspection(
    inspection: &AdvisoryCausalInspection,
    bridge_denial: &BridgeCausalEnvelopeDenial,
    redaction_policy: CausalInspectionRedactionPolicy,
    materialization_policy: CausalInspectionMaterializationPolicy,
) -> QueryCausalInspectionArtifact {
    materialize_bridge_denied_query_inspection(
        inspection,
        inspection.advisory_inspection_identity(),
        bridge_denial,
        redaction_policy,
        materialization_policy,
    )
}

fn materialize_bridge_denied_query_inspection(
    inspection: impl CausalInspectionBridgeDeniedSubject,
    query_admission_identity: &CausalInspectionOutcomeIdentity,
    bridge_denial: &BridgeCausalEnvelopeDenial,
    redaction_policy: CausalInspectionRedactionPolicy,
    materialization_policy: CausalInspectionMaterializationPolicy,
) -> QueryCausalInspectionArtifact {
    let query_observation_digest = inspection.subject().query_observation_digest();
    let result_shape_context_digest = inspection.subject().result_shape_context_digest();
    let denial_reason = "bridge_envelope_denial".to_string();
    let performance = CausalInspectionPerformanceEnvelope::for_bridge_denial(bridge_denial);
    let detail_digest = hash_parts(&[
        "denied_query_causal_inspection_artifact_detail_v1".to_string(),
        format!("query-observation:{query_observation_digest}"),
        format!("result-shape:{result_shape_context_digest}"),
        format!("reason:{denial_reason}"),
        format!("bridge-denial:{}", bridge_denial.failure_digest()),
        format!("bridge-denial-kind:{}", bridge_denial.kind().as_str()),
        format!("bridge-denial-family:{}", bridge_denial.family().as_str()),
    ]);
    let receipt = CausalMaterializationReceipt::new(
        query_admission_identity,
        None,
        None,
        redaction_policy,
        materialization_policy,
        &performance,
        &detail_digest,
    );
    let artifact_digest = artifact_digest(
        CausalInspectionArtifactKind::Denied,
        query_admission_identity.as_str(),
        None,
        None,
        &receipt,
        None,
        &detail_digest,
    );
    let temporal_async_explanation = QueryCausalTemporalAsyncExplanation::project(
        inspection.subject().inspection_reason(),
        inspection.subject().observation_outcome(),
        inspection.subject().resolved_evidence_families(),
        Some(bridge_denial.family().as_str()),
    );
    QueryCausalInspectionArtifact::Denied(DeniedQueryCausalInspectionArtifact::from_parts(
        query_admission_identity,
        denial_reason,
        inspection.subject().query_observation_identity(),
        inspection.subject().result_shape_context_identity(),
        Some(bridge_denial.failure_digest().to_string()),
        Some(bridge_denial.kind().as_str().to_string()),
        Some(bridge_denial.family().as_str().to_string()),
        temporal_async_explanation,
        policy::boundary_categories(),
        performance,
        receipt,
        artifact_digest,
    ))
}

trait CausalInspectionBridgeDeniedSubject {
    fn subject(&self) -> &super::super::admission_decision::CausalInspectionAdmissionSubject;
}

impl CausalInspectionBridgeDeniedSubject for &AdmittedCausalInspection {
    fn subject(&self) -> &super::super::admission_decision::CausalInspectionAdmissionSubject {
        (*self).subject()
    }
}

impl CausalInspectionBridgeDeniedSubject for &AdvisoryCausalInspection {
    fn subject(&self) -> &super::super::admission_decision::CausalInspectionAdmissionSubject {
        (*self).subject()
    }
}
