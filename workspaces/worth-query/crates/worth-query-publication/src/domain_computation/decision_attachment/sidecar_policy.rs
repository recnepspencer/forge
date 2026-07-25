use worth_foundational::facade::RetentionDeliveryProfile;
use worth_query_installation::facade::{
    WorthQueryArtifactDeletionPosture, WorthQueryArtifactGovernanceContract,
    WorthQueryArtifactRedactionPosture,
};

use super::WorthQueryAdmittedDomainEvidenceSidecar;

pub(super) struct WorthQuerySidecarMaterializationPolicy {
    pub(super) applicable: bool,
    pub(super) retention_allows_materialization: bool,
}

pub(super) fn process_supplied_records(governance: &WorthQueryArtifactGovernanceContract) -> bool {
    matches!(
        governance.redaction(),
        WorthQueryArtifactRedactionPosture::NotRequired
            | WorthQueryArtifactRedactionPosture::CanonicalProjectionOnly
    )
}

pub(super) fn materialize_sidecar<T>(
    policy: WorthQuerySidecarMaterializationPolicy,
    records: Option<Vec<T>>,
    governance: &WorthQueryArtifactGovernanceContract,
    digest: impl FnOnce(&[T]) -> String,
) -> WorthQueryAdmittedDomainEvidenceSidecar<T> {
    materialize_sidecar_by_record(policy.applicable, records, governance, digest, |_| {
        policy.retention_allows_materialization
    })
}

pub(super) fn materialize_sidecar_by_record<T>(
    applicable: bool,
    records: Option<Vec<T>>,
    governance: &WorthQueryArtifactGovernanceContract,
    digest: impl FnOnce(&[T]) -> String,
    mut retention_allows_materialization: impl FnMut(&T) -> bool,
) -> WorthQueryAdmittedDomainEvidenceSidecar<T> {
    if !applicable {
        return WorthQueryAdmittedDomainEvidenceSidecar::NotApplicable;
    }
    let Some(records) = records else {
        return WorthQueryAdmittedDomainEvidenceSidecar::Omitted;
    };
    match governance.redaction() {
        WorthQueryArtifactRedactionPosture::NotRequired
            if governance.retention() != RetentionDeliveryProfile::Ephemeral
                && governance.deletion() != WorthQueryArtifactDeletionPosture::DeleteWithRun =>
        {
            let digest = digest(&records);
            let supplied_count = records.len();
            let records = records
                .into_iter()
                .filter(|record| retention_allows_materialization(record))
                .collect::<Vec<_>>();
            if records.is_empty() {
                WorthQueryAdmittedDomainEvidenceSidecar::DigestOnly { digest }
            } else if records.len() == supplied_count {
                WorthQueryAdmittedDomainEvidenceSidecar::Materialized { digest, records }
            } else {
                WorthQueryAdmittedDomainEvidenceSidecar::PartiallyMaterialized { digest, records }
            }
        }
        WorthQueryArtifactRedactionPosture::NotRequired
        | WorthQueryArtifactRedactionPosture::CanonicalProjectionOnly => {
            let digest = digest(&records);
            WorthQueryAdmittedDomainEvidenceSidecar::DigestOnly { digest }
        }
        WorthQueryArtifactRedactionPosture::DomainRedactorRequired
        | WorthQueryArtifactRedactionPosture::NeverDisclose => {
            WorthQueryAdmittedDomainEvidenceSidecar::Omitted
        }
    }
}
