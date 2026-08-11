use worth_query_host::facade::convergence_epoch::WorthQueryRetainedConvergenceCandidateEvidence;
use worth_query_host::facade::publication::domain_computation::{
    admit_domain_evidence_content, WorthQueryDomainEvidenceContentAdmissionInput,
    WorthQueryDomainEvidenceMaterial,
};

fn publish_candidate_as_domain_evidence(
    candidate: &WorthQueryRetainedConvergenceCandidateEvidence,
) {
    let _ = admit_domain_evidence_content(WorthQueryDomainEvidenceContentAdmissionInput {
        contract: candidate.domain_evidence().contract(),
        material: Some(WorthQueryDomainEvidenceMaterial::new()),
        ledger: None,
    });
}

fn main() {}
