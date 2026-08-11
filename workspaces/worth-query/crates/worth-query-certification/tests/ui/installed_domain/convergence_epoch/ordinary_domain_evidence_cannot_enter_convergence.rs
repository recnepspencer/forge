use worth_query::facade::domain::WorthQueryDomainEvidenceBinding;
use worth_query_host::facade::installed::domain_computation::WorthQueryConvergenceDomainEvidenceBinding;

fn requires_convergence(_: WorthQueryConvergenceDomainEvidenceBinding) {}

fn substitute(ordinary: WorthQueryDomainEvidenceBinding) {
    requires_convergence(ordinary);
}

fn main() {}
