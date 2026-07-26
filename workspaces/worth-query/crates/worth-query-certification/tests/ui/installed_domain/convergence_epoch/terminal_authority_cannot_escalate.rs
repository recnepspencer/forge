use worth_query_host::facade::convergence_epoch::{
    WorthQueryConverged, WorthQueryDirectConvergenceTerminal,
    WorthQueryRetainedConvergenceCandidateEvidence,
};
use worth_query_host::facade::publication::domain_computation::{
    admit_domain_evidence, WorthQueryDomainEvidenceAdmissionInput,
};

fn publish_terminal(terminal: WorthQueryDirectConvergenceTerminal<WorthQueryConverged>) {
    let _ = terminal.publish();
}

fn resolve_terminal(terminal: WorthQueryDirectConvergenceTerminal<WorthQueryConverged>) {
    let _ = terminal.resolve();
}

fn publish_candidate(candidate: &WorthQueryRetainedConvergenceCandidateEvidence) {
    let _ = candidate.publish();
}

fn publish_candidate_as_domain_evidence(
    candidate: &WorthQueryRetainedConvergenceCandidateEvidence,
) {
    let _ = admit_domain_evidence(WorthQueryDomainEvidenceAdmissionInput {
        material: None,
        binding: candidate.domain_evidence().clone(),
        ledger: None,
    });
}

fn main() {}
