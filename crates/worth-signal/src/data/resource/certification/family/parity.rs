use super::contract::{
    ResourceCertificationBundle, ResourceCertificationBundleMismatchClass,
    ResourceCertificationBundleParityReport, RESOURCE_CERTIFICATION_BUNDLE_PARITY_SCHEMA_VERSION,
};

pub fn resource_certification_bundle_parity_report(
    expected: &ResourceCertificationBundle,
    replayed: &ResourceCertificationBundle,
) -> ResourceCertificationBundleParityReport {
    let mut mismatch_classes = Vec::new();
    if expected.schema_version != replayed.schema_version {
        mismatch_classes.push(ResourceCertificationBundleMismatchClass::SchemaVersionMismatch);
    }
    if expected.bundle_digest != replayed.bundle_digest {
        mismatch_classes.push(ResourceCertificationBundleMismatchClass::BundleDigestMismatch);
    }
    if expected.passed != replayed.passed {
        mismatch_classes.push(ResourceCertificationBundleMismatchClass::PassStatusMismatch);
    }
    if expected.summary != replayed.summary {
        mismatch_classes.push(ResourceCertificationBundleMismatchClass::SummaryMismatch);
    }
    if expected.failures != replayed.failures {
        mismatch_classes.push(ResourceCertificationBundleMismatchClass::FailureSetMismatch);
    }
    if expected.records != replayed.records {
        mismatch_classes.push(ResourceCertificationBundleMismatchClass::RecordSetMismatch);
    }
    ResourceCertificationBundleParityReport {
        proof_schema_version: RESOURCE_CERTIFICATION_BUNDLE_PARITY_SCHEMA_VERSION.to_owned(),
        expected: expected.clone(),
        replayed: replayed.clone(),
        parity: mismatch_classes.is_empty(),
        mismatch_classes,
    }
}
