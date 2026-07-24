use worth_query_installation::facade::{
    WorthQueryArtifactGovernanceContract, WorthQueryArtifactRedactionPosture,
};

use super::WorthQueryAdmittedDomainEvidenceSidecar;

pub(super) fn materialize_sidecar<T>(
    applicable: bool,
    records: Option<Vec<T>>,
    governance: &WorthQueryArtifactGovernanceContract,
    digest: impl FnOnce(&[T]) -> String,
) -> WorthQueryAdmittedDomainEvidenceSidecar<T> {
    if !applicable {
        return WorthQueryAdmittedDomainEvidenceSidecar::NotApplicable;
    }
    let Some(records) = records else {
        return WorthQueryAdmittedDomainEvidenceSidecar::Omitted;
    };
    let digest = digest(&records);
    match governance.redaction() {
        WorthQueryArtifactRedactionPosture::NotRequired => {
            WorthQueryAdmittedDomainEvidenceSidecar::Materialized { digest, records }
        }
        WorthQueryArtifactRedactionPosture::CanonicalProjectionOnly => {
            WorthQueryAdmittedDomainEvidenceSidecar::DigestOnly { digest }
        }
        WorthQueryArtifactRedactionPosture::DomainRedactorRequired
        | WorthQueryArtifactRedactionPosture::NeverDisclose => {
            WorthQueryAdmittedDomainEvidenceSidecar::Omitted
        }
    }
}
