use crate::domain_installation::{
    domain_evidence_binding_material, domain_evidence_core_material,
    domain_evidence_governance_material, WorthQueryAdmittedDomainEvidence,
    WorthQueryAdmittedStructuralCounter, WorthQueryCandidateRecord, WorthQueryDecisionRecord,
    WorthQueryTransformationRecord,
};
use crate::identity::hash_parts;

use super::{CausalInspectionRedactionPolicy, WorthQueryDomainEvidenceInspectionSidecar};

pub(super) fn inspection_copy_identity(
    source: &WorthQueryAdmittedDomainEvidence,
    counter_sidecar: &WorthQueryDomainEvidenceInspectionSidecar<
        WorthQueryAdmittedStructuralCounter,
    >,
    decision_sidecar: &WorthQueryDomainEvidenceInspectionSidecar<WorthQueryDecisionRecord>,
    candidate_sidecar: &WorthQueryDomainEvidenceInspectionSidecar<WorthQueryCandidateRecord>,
    transformation_sidecar: &WorthQueryDomainEvidenceInspectionSidecar<
        WorthQueryTransformationRecord,
    >,
    redaction_policy: CausalInspectionRedactionPolicy,
) -> String {
    hash_parts(&[
        "worth_query_domain_evidence_inspection_copy_v1".into(),
        format!("source:{}", source.identity()),
        format!("contract:{}", source.contract_identity()),
        format!(
            "binding:{}",
            domain_evidence_binding_material(source.binding())
        ),
        format!(
            "governance:{}",
            domain_evidence_governance_material(source.governance())
        ),
        format!("core:{}", domain_evidence_core_material(source.core())),
        format!("redaction:{}", redaction_policy.as_str()),
        format!("counter-sidecar:{}", sidecar_material(counter_sidecar)),
        format!("decision-sidecar:{}", sidecar_material(decision_sidecar)),
        format!("candidate-sidecar:{}", sidecar_material(candidate_sidecar)),
        format!(
            "transformation-sidecar:{}",
            sidecar_material(transformation_sidecar)
        ),
        "authority:descriptive-only".into(),
    ])
}

pub(super) fn sidecar_material<T>(
    sidecar: &WorthQueryDomainEvidenceInspectionSidecar<T>,
) -> String {
    match sidecar {
        WorthQueryDomainEvidenceInspectionSidecar::NotApplicable => "not-applicable".into(),
        WorthQueryDomainEvidenceInspectionSidecar::Omitted => "omitted".into(),
        WorthQueryDomainEvidenceInspectionSidecar::DigestOnly { digest } => {
            format!("digest-only:{digest}")
        }
        WorthQueryDomainEvidenceInspectionSidecar::Materialized { digest, .. } => {
            format!("materialized:{digest}")
        }
    }
}
