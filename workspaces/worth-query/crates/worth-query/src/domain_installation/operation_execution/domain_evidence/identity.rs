mod core;
mod sidecar;
mod vocabulary;

use crate::identity::hash_parts;

use super::{
    WorthQueryAdmittedDomainEvidenceSidecar, WorthQueryCandidateRecord, WorthQueryDecisionRecord,
    WorthQueryDomainEvidenceBinding, WorthQueryDomainEvidenceCore,
    WorthQueryDomainEvidenceGovernance, WorthQueryTransformationRecord,
};

pub(super) fn domain_evidence_identity(
    contract_identity: &str,
    binding: &WorthQueryDomainEvidenceBinding,
    core: &WorthQueryDomainEvidenceCore,
    decision_sidecar: &WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryDecisionRecord>,
    candidate_sidecar: &WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryCandidateRecord>,
    transformation_sidecar: &WorthQueryAdmittedDomainEvidenceSidecar<
        WorthQueryTransformationRecord,
    >,
) -> String {
    hash_parts(&[
        "worth_query_admitted_domain_evidence_v1".into(),
        format!("contract:{contract_identity}"),
        format!("binding:{}", core::binding_material(binding)),
        format!("core:{}", core::core_material(core)),
        format!(
            "decision_sidecar:{}",
            sidecar::sidecar_material(decision_sidecar)
        ),
        format!(
            "candidate_sidecar:{}",
            sidecar::sidecar_material(candidate_sidecar)
        ),
        format!(
            "transformation_sidecar:{}",
            sidecar::sidecar_material(transformation_sidecar)
        ),
    ])
}

pub(crate) fn domain_evidence_core_material(core: &WorthQueryDomainEvidenceCore) -> String {
    core::core_material(core)
}

pub(crate) fn domain_evidence_binding_material(
    binding: &WorthQueryDomainEvidenceBinding,
) -> String {
    core::binding_material(binding)
}

pub(crate) fn domain_evidence_governance_material(
    governance: &WorthQueryDomainEvidenceGovernance,
) -> String {
    core::governance_material(governance)
}

pub(super) fn decision_sidecar_digest(records: &[WorthQueryDecisionRecord]) -> String {
    sidecar::decision_sidecar_digest(records)
}

pub(super) fn candidate_sidecar_digest(records: &[WorthQueryCandidateRecord]) -> String {
    sidecar::candidate_sidecar_digest(records)
}

pub(super) fn transformation_sidecar_digest(records: &[WorthQueryTransformationRecord]) -> String {
    sidecar::transformation_sidecar_digest(records)
}
