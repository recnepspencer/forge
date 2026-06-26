use crate::{
    DerivedManifestOverrideAttempt, ManifestIntegrityCounters, ManifestIntegrityDenial,
    ManifestIntegrityDenialKind, RootManifestIntegrityReport,
};

pub(crate) fn deny_derived_override(
    root: &RootManifestIntegrityReport,
    attempt: DerivedManifestOverrideAttempt,
    counters: ManifestIntegrityCounters,
) -> ManifestIntegrityDenial {
    let authoritative_failure = attempt.authoritative_failure();
    let denial = ManifestIntegrityDenial::new(
        ManifestIntegrityDenialKind::SourcePrecedenceViolation,
        root.posture(),
        counters.with_derived_override_rejection(),
    );
    denial.with_locality(authoritative_failure.locality())
}
