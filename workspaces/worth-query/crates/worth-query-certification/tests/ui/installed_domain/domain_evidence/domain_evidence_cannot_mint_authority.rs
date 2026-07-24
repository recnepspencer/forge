use worth_query::facade::{certification, domain, foundation, runtime};

fn requires_consumed_projection_authority(
    _authority: &foundation::WorthQueryConsumedProjectionAuthority,
) {
}

fn descriptive_evidence_cannot_authorize(
    admitted: &domain::WorthQueryAdmittedDomainEvidence,
    inspection: &runtime::WorthQueryDomainEvidenceInspectionCopy,
    certification: &certification::WorthQueryDomainEvidenceCertificationBundle,
) {
    requires_consumed_projection_authority(admitted);
    requires_consumed_projection_authority(inspection);
    requires_consumed_projection_authority(certification);
}

fn main() {}
