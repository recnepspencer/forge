use serde::Serialize;

pub const SUBSCRIPTION_SUPPORT_ACCURACY_CERTIFICATION_SUITE_NAME: &str =
    "Subscription-Support Accuracy And Certification Test";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum SubscriptionSupportAccuracyCertificationRowKind {
    ExactSupportTrustedControl,
    DegradedSupportTrusted,
    RebuildDerivedSupportExactEquivalence,
    RebuildDerivedSupportDowngraded,
    ReplicatedSupportIdentityNotEnough,
    ReplicatedSupportExactEquivalence,
    MigratedSupportExactEquivalence,
    ImportedSupportMissingBasisNotResumable,
    StaleSupportRejected,
    PolicyRejectedSupport,
    FamilyRoleMismatchRejected,
    CompatibilityDriftRejectsExactTrust,
    OperationalVerdictDriftRejectsExactTrust,
    PortabilityDriftRejectsExactTrust,
    CoverageDriftRejectsPlatformTrust,
    MultiDriftPrecedenceDeterministic,
    CertificationMatrixComplete,
    CertificationMissingRowRejected,
    CertificationDuplicateRowRejected,
    CertificationMislabeledRowRejected,
    CertificationSelfComparisonRejected,
    GenericCertificationIncludesSupportTrust,
    DomainGeometrySupportTrust,
    DomainWebDataSupportTrust,
    DomainAiDegradedSupportTrust,
    DomainChipRebuildSupportTrust,
    DomainOfflineOmittedSupportTrust,
    ForbiddenExactOverclaimZero,
    GlobalScanDebtForbidden,
    Roadmap2HandoffPhysicalDebtExplicit,
}

impl SubscriptionSupportAccuracyCertificationRowKind {
    pub fn required() -> &'static [Self] {
        &REQUIRED_SUBSCRIPTION_SUPPORT_ACCURACY_ROWS
    }

    pub(super) fn evidence_lane_label(self) -> &'static str {
        match self {
            Self::RebuildDerivedSupportExactEquivalence => "rebuild-exact-equivalence-lane",
            Self::RebuildDerivedSupportDowngraded => "rebuild-downgrade-lane",
            Self::ReplicatedSupportIdentityNotEnough => "replication-identity-hostile-lane",
            Self::ReplicatedSupportExactEquivalence => "replication-exact-equivalence-lane",
            Self::MigratedSupportExactEquivalence => "migration-exact-equivalence-lane",
            Self::ImportedSupportMissingBasisNotResumable => "import-missing-basis-lane",
            Self::StaleSupportRejected => "stale-support-hostile-lane",
            Self::PolicyRejectedSupport => "policy-rejection-lane",
            Self::FamilyRoleMismatchRejected => "family-role-mismatch-lane",
            Self::CompatibilityDriftRejectsExactTrust => "compatibility-drift-lane",
            Self::OperationalVerdictDriftRejectsExactTrust => "operational-drift-lane",
            Self::PortabilityDriftRejectsExactTrust => "portability-drift-lane",
            Self::CoverageDriftRejectsPlatformTrust => "coverage-drift-lane",
            Self::MultiDriftPrecedenceDeterministic => "multi-drift-precedence-lane",
            Self::CertificationMissingRowRejected => "certification-missing-row-lane",
            Self::CertificationDuplicateRowRejected => "certification-duplicate-row-lane",
            Self::CertificationMislabeledRowRejected => "certification-mislabeled-row-lane",
            Self::CertificationSelfComparisonRejected => "certification-self-comparison-lane",
            Self::ForbiddenExactOverclaimZero => "forbidden-exact-overclaim-counter-lane",
            Self::GlobalScanDebtForbidden => "global-scan-debt-counter-lane",
            Self::ExactSupportTrustedControl
            | Self::DegradedSupportTrusted
            | Self::CertificationMatrixComplete
            | Self::GenericCertificationIncludesSupportTrust
            | Self::DomainGeometrySupportTrust
            | Self::DomainWebDataSupportTrust
            | Self::DomainAiDegradedSupportTrust
            | Self::DomainChipRebuildSupportTrust
            | Self::DomainOfflineOmittedSupportTrust
            | Self::Roadmap2HandoffPhysicalDebtExplicit => "artifact-bound-lane",
        }
    }
}

const REQUIRED_SUBSCRIPTION_SUPPORT_ACCURACY_ROWS:
    [SubscriptionSupportAccuracyCertificationRowKind; 30] = [
    SubscriptionSupportAccuracyCertificationRowKind::ExactSupportTrustedControl,
    SubscriptionSupportAccuracyCertificationRowKind::DegradedSupportTrusted,
    SubscriptionSupportAccuracyCertificationRowKind::RebuildDerivedSupportExactEquivalence,
    SubscriptionSupportAccuracyCertificationRowKind::RebuildDerivedSupportDowngraded,
    SubscriptionSupportAccuracyCertificationRowKind::ReplicatedSupportIdentityNotEnough,
    SubscriptionSupportAccuracyCertificationRowKind::ReplicatedSupportExactEquivalence,
    SubscriptionSupportAccuracyCertificationRowKind::MigratedSupportExactEquivalence,
    SubscriptionSupportAccuracyCertificationRowKind::ImportedSupportMissingBasisNotResumable,
    SubscriptionSupportAccuracyCertificationRowKind::StaleSupportRejected,
    SubscriptionSupportAccuracyCertificationRowKind::PolicyRejectedSupport,
    SubscriptionSupportAccuracyCertificationRowKind::FamilyRoleMismatchRejected,
    SubscriptionSupportAccuracyCertificationRowKind::CompatibilityDriftRejectsExactTrust,
    SubscriptionSupportAccuracyCertificationRowKind::OperationalVerdictDriftRejectsExactTrust,
    SubscriptionSupportAccuracyCertificationRowKind::PortabilityDriftRejectsExactTrust,
    SubscriptionSupportAccuracyCertificationRowKind::CoverageDriftRejectsPlatformTrust,
    SubscriptionSupportAccuracyCertificationRowKind::MultiDriftPrecedenceDeterministic,
    SubscriptionSupportAccuracyCertificationRowKind::CertificationMatrixComplete,
    SubscriptionSupportAccuracyCertificationRowKind::CertificationMissingRowRejected,
    SubscriptionSupportAccuracyCertificationRowKind::CertificationDuplicateRowRejected,
    SubscriptionSupportAccuracyCertificationRowKind::CertificationMislabeledRowRejected,
    SubscriptionSupportAccuracyCertificationRowKind::CertificationSelfComparisonRejected,
    SubscriptionSupportAccuracyCertificationRowKind::GenericCertificationIncludesSupportTrust,
    SubscriptionSupportAccuracyCertificationRowKind::DomainGeometrySupportTrust,
    SubscriptionSupportAccuracyCertificationRowKind::DomainWebDataSupportTrust,
    SubscriptionSupportAccuracyCertificationRowKind::DomainAiDegradedSupportTrust,
    SubscriptionSupportAccuracyCertificationRowKind::DomainChipRebuildSupportTrust,
    SubscriptionSupportAccuracyCertificationRowKind::DomainOfflineOmittedSupportTrust,
    SubscriptionSupportAccuracyCertificationRowKind::ForbiddenExactOverclaimZero,
    SubscriptionSupportAccuracyCertificationRowKind::GlobalScanDebtForbidden,
    SubscriptionSupportAccuracyCertificationRowKind::Roadmap2HandoffPhysicalDebtExplicit,
];
