use super::super::matrices::{CanonicalCertificationBundle, RejectionCertificationBundle};
use super::super::profiles::CertificationProfile;

pub(super) fn to_bundle(
    profile: CertificationProfile,
    bundle: &crate::facade::foundation::CanonicalQueryBundle,
) -> CanonicalCertificationBundle {
    CanonicalCertificationBundle {
        profile,
        query_digest: bundle.query().digest().as_str().to_string(),
        result_shape_digest: bundle.result_shape().digest().as_str().to_string(),
        canonicalization_report: bundle.report().clone(),
        warning_count: bundle.report().warnings().len(),
        event_count: bundle.report().events().len(),
        counter_snapshot: bundle.counters().clone(),
    }
}

pub(super) fn to_rejection_bundle(
    profile: CertificationProfile,
    error: &crate::facade::foundation::QueryCanonicalizationError,
) -> RejectionCertificationBundle {
    RejectionCertificationBundle {
        profile,
        failure_class: format!("{:?}", error.failure_class()),
        failure_digest: format!("{error:?}"),
    }
}
