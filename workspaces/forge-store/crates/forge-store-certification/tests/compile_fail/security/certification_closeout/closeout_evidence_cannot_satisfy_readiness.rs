use forge_store_certification::S51CertificationCloseoutEvidence;
use forge_store_readiness::S51AdmittedSecurityScopeReadiness;

fn requires_readiness(_: S51AdmittedSecurityScopeReadiness) {}

fn main() {
    let evidence: S51CertificationCloseoutEvidence = todo!();
    requires_readiness(evidence);
}
