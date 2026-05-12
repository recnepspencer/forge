use crate::identity::hash_parts;

use forge_runtime_bridge::facade::BridgeCausalEnvelopeDenial;

use super::super::admission::{AdmittedCausalInspection, AdvisoryCausalInspection};
use super::{
    artifact_digest, policy, CausalInspectionArtifactKind, CausalInspectionMaterializationPolicy,
    CausalInspectionPerformanceEnvelope, CausalInspectionRedactionPolicy,
    CausalMaterializationReceipt, DeniedQueryCausalInspectionArtifact,
    QueryCausalInspectionArtifact,
};

pub(crate) fn materialize_bridge_denied_admitted_causal_inspection(
    inspection: &AdmittedCausalInspection,
    bridge_denial: &BridgeCausalEnvelopeDenial,
    redaction_policy: CausalInspectionRedactionPolicy,
    materialization_policy: CausalInspectionMaterializationPolicy,
) -> QueryCausalInspectionArtifact {
    materialize_bridge_denied_query_inspection(
        inspection.admitted_inspection_digest(),
        inspection.subject().query_observation_digest(),
        inspection.subject().result_shape_context_digest(),
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
        inspection.advisory_inspection_digest(),
        inspection.subject().query_observation_digest(),
        inspection.subject().result_shape_context_digest(),
        bridge_denial,
        redaction_policy,
        materialization_policy,
    )
}

fn materialize_bridge_denied_query_inspection(
    query_admission_digest: &str,
    query_observation_digest: &str,
    result_shape_context_digest: &str,
    bridge_denial: &BridgeCausalEnvelopeDenial,
    redaction_policy: CausalInspectionRedactionPolicy,
    materialization_policy: CausalInspectionMaterializationPolicy,
) -> QueryCausalInspectionArtifact {
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
        query_admission_digest,
        None,
        None,
        redaction_policy,
        materialization_policy,
        &performance,
        &detail_digest,
    );
    let artifact_digest = artifact_digest(
        CausalInspectionArtifactKind::Denied,
        query_admission_digest,
        None,
        None,
        &receipt,
        None,
        &detail_digest,
    );
    QueryCausalInspectionArtifact::Denied(DeniedQueryCausalInspectionArtifact::from_parts(
        query_admission_digest,
        denial_reason,
        query_observation_digest,
        result_shape_context_digest,
        Some(bridge_denial.failure_digest().to_string()),
        Some(bridge_denial.kind().as_str().to_string()),
        Some(bridge_denial.family().as_str().to_string()),
        policy::boundary_categories(),
        performance,
        receipt,
        artifact_digest,
    ))
}
