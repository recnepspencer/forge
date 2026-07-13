use crate::facade::foundation::CanonicalQueryBundle;
use crate::facade::runtime::ValidatedQueryBundle;
use crate::harness::fixtures::schema_view::detail_schema_view;
use crate::validation::{
    validate_canonical_bundle_with_failure_artifact, ValidationFailureArtifact,
};

use super::super::profiles::CertificationProfile;
use super::super::validation_matrix::{
    ValidationCertificationBundle, ValidationPerturbationClass,
    ValidationRejectionCertificationBundle, ValidationRejectionCertificationRow,
};

pub(super) fn to_bundle(
    profile: CertificationProfile,
    bundle: &ValidatedQueryBundle,
) -> ValidationCertificationBundle {
    ValidationCertificationBundle {
        profile,
        query_digest: bundle.query().canonical_query_digest().as_str().to_string(),
        validated_query_digest: bundle.query().digest().as_str().to_string(),
        validated_result_shape_digest: bundle.result_shape().digest().as_str().to_string(),
        basis_digest: bundle.query().schema_basis().as_str().to_string(),
        validation_report: bundle.report().clone(),
        counter_snapshot: bundle.counters().clone(),
    }
}

pub(super) fn to_rejection_bundle(
    profile: CertificationProfile,
    failure: &ValidationFailureArtifact,
) -> ValidationRejectionCertificationBundle {
    ValidationRejectionCertificationBundle {
        profile,
        failure_class: format!("{:?}", failure.error.failure_class()),
        failure_digest: failure.error.failure_digest(),
        validation_rejection_matrix: failure.rejection_matrix.clone(),
        counter_snapshot: failure.counters.clone(),
    }
}

pub(super) fn rejection_row(
    row_name: &'static str,
    perturbation_class: ValidationPerturbationClass,
    control: &ValidatedQueryBundle,
    hostile_bundle: CanonicalQueryBundle,
) -> ValidationRejectionCertificationRow {
    let hostile =
        validate_canonical_bundle_with_failure_artifact(hostile_bundle, detail_schema_view())
            .unwrap_err();
    ValidationRejectionCertificationRow {
        row_name,
        perturbation_class,
        control_lane: to_bundle(CertificationProfile::DirectConstruction, control),
        hostile_lane: to_rejection_bundle(CertificationProfile::BuilderReordering, &hostile),
        parity_lane: to_bundle(CertificationProfile::ReplayParity, control),
    }
}
