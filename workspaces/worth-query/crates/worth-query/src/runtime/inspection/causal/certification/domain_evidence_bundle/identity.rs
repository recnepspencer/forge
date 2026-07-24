use crate::domain_installation::{
    domain_evidence_binding_material, domain_evidence_core_material,
    domain_evidence_governance_material,
};
use crate::identity::hash_parts;

use super::super::super::super::WorthQueryDomainEvidenceInspectionCopy;
use super::WorthQueryDomainEvidenceCertificationSidecar;

pub(super) fn certification_bundle_identity(
    source: &WorthQueryDomainEvidenceInspectionCopy,
    counter_sidecar: &WorthQueryDomainEvidenceCertificationSidecar,
    decision_sidecar: &WorthQueryDomainEvidenceCertificationSidecar,
    candidate_sidecar: &WorthQueryDomainEvidenceCertificationSidecar,
    transformation_sidecar: &WorthQueryDomainEvidenceCertificationSidecar,
) -> String {
    hash_parts(&[
        "worth_query_domain_evidence_certification_bundle_v1".into(),
        format!("inspection:{}", source.identity()),
        format!("source:{}", source.source_evidence_identity()),
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
        format!("source-redaction:{}", source.redaction_policy().as_str()),
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

fn sidecar_material(sidecar: &WorthQueryDomainEvidenceCertificationSidecar) -> String {
    match sidecar {
        WorthQueryDomainEvidenceCertificationSidecar::NotApplicable => "not-applicable".into(),
        WorthQueryDomainEvidenceCertificationSidecar::Omitted => "omitted".into(),
        WorthQueryDomainEvidenceCertificationSidecar::Digest { digest } => {
            format!("digest:{digest}")
        }
    }
}
