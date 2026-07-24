use worth_query_installation::facade::{
    WorthQueryArtifactGovernanceContract, WorthQueryArtifactRedactionPosture,
};

use super::WorthQueryAdmittedDomainEvidenceSidecar;

pub(super) fn process_supplied_records(governance: &WorthQueryArtifactGovernanceContract) -> bool {
    matches!(
        governance.redaction(),
        WorthQueryArtifactRedactionPosture::NotRequired
            | WorthQueryArtifactRedactionPosture::CanonicalProjectionOnly
    )
}

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
    match governance.redaction() {
        WorthQueryArtifactRedactionPosture::NotRequired => {
            let digest = digest(&records);
            WorthQueryAdmittedDomainEvidenceSidecar::Materialized { digest, records }
        }
        WorthQueryArtifactRedactionPosture::CanonicalProjectionOnly => {
            let digest = digest(&records);
            WorthQueryAdmittedDomainEvidenceSidecar::DigestOnly { digest }
        }
        WorthQueryArtifactRedactionPosture::DomainRedactorRequired
        | WorthQueryArtifactRedactionPosture::NeverDisclose => {
            WorthQueryAdmittedDomainEvidenceSidecar::Omitted
        }
    }
}
