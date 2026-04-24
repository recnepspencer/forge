use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySubscriptionRuntimeCertificationErrorKind {
    CoverageResolutionDenied,
    CoverageFamilyMissing,
    CoverageFamilyMismatch,
    CertificationSupportClassDenied,
    CertificationSupportPostureDenied,
    ScopeFamilyMismatch,
    ScopeSourceMismatch,
    CoverageScopeMissingAdmittedRow,
    MissingHostileCoverage,
    UncoveredFamily,
}

impl QuerySubscriptionRuntimeCertificationErrorKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CoverageResolutionDenied => "coverage_resolution_denied",
            Self::CoverageFamilyMissing => "coverage_family_missing",
            Self::CoverageFamilyMismatch => "coverage_family_mismatch",
            Self::CertificationSupportClassDenied => "certification_support_class_denied",
            Self::CertificationSupportPostureDenied => "certification_support_posture_denied",
            Self::ScopeFamilyMismatch => "scope_family_mismatch",
            Self::ScopeSourceMismatch => "scope_source_mismatch",
            Self::CoverageScopeMissingAdmittedRow => "coverage_scope_missing_admitted_row",
            Self::MissingHostileCoverage => "missing_hostile_coverage",
            Self::UncoveredFamily => "uncovered_family",
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct QuerySubscriptionRuntimeCertificationCounters {
    certification_scope_emission_count: u64,
    certified_family_count: u64,
    hostile_row_coverage_count: u64,
    uncovered_family_denial_count: u64,
    family_coverage_index_lookup_count: u64,
    family_coverage_matrix_scan_debt_count: u64,
}

impl QuerySubscriptionRuntimeCertificationCounters {
    pub fn digest(&self) -> String {
        hash_parts(&[
            format!(
                "subscription_certification_scope_emission_count:{}",
                self.certification_scope_emission_count
            ),
            format!(
                "subscription_certified_family_count:{}",
                self.certified_family_count
            ),
            format!(
                "subscription_hostile_row_coverage_count:{}",
                self.hostile_row_coverage_count
            ),
            format!(
                "subscription_uncovered_family_denial_count:{}",
                self.uncovered_family_denial_count
            ),
            format!(
                "subscription_family_coverage_index_lookup_count:{}",
                self.family_coverage_index_lookup_count
            ),
            format!(
                "subscription_family_coverage_matrix_scan_debt_count:{}",
                self.family_coverage_matrix_scan_debt_count
            ),
        ])
    }

    pub fn certification_scope_emission_count(&self) -> u64 {
        self.certification_scope_emission_count
    }

    pub fn certified_family_count(&self) -> u64 {
        self.certified_family_count
    }

    pub fn hostile_row_coverage_count(&self) -> u64 {
        self.hostile_row_coverage_count
    }

    pub fn uncovered_family_denial_count(&self) -> u64 {
        self.uncovered_family_denial_count
    }

    pub fn family_coverage_index_lookup_count(&self) -> u64 {
        self.family_coverage_index_lookup_count
    }

    pub fn family_coverage_matrix_scan_debt_count(&self) -> u64 {
        self.family_coverage_matrix_scan_debt_count
    }

    pub(crate) fn scope_emitted() -> Self {
        Self {
            certification_scope_emission_count: 1,
            ..Default::default()
        }
    }

    pub(crate) fn certified(
        hostile_row_count: usize,
        posture: crate::subscription::runtime_certification::CoverageResolutionPosture,
    ) -> Self {
        Self {
            certified_family_count: 1,
            hostile_row_coverage_count: hostile_row_count as u64,
            family_coverage_index_lookup_count: u64::from(
                posture
                    == crate::subscription::runtime_certification::CoverageResolutionPosture::IndexedCoverageSet,
            ),
            family_coverage_matrix_scan_debt_count: u64::from(
                posture
                    == crate::subscription::runtime_certification::CoverageResolutionPosture::MatrixScanDebtExplicit,
            ),
            ..Default::default()
        }
    }

    pub(crate) fn uncovered_family(posture_requires_scan_debt: bool) -> Self {
        Self {
            uncovered_family_denial_count: 1,
            family_coverage_index_lookup_count: 1,
            family_coverage_matrix_scan_debt_count: u64::from(posture_requires_scan_debt),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionRuntimeCertificationError {
    error_kind: QuerySubscriptionRuntimeCertificationErrorKind,
    message: &'static str,
    failure_digest: String,
    counters: QuerySubscriptionRuntimeCertificationCounters,
}

impl QuerySubscriptionRuntimeCertificationError {
    pub(crate) fn new(
        error_kind: QuerySubscriptionRuntimeCertificationErrorKind,
        message: &'static str,
        evidence_parts: &[String],
        counters: QuerySubscriptionRuntimeCertificationCounters,
    ) -> Self {
        let mut parts = vec![
            "query_subscription_runtime_certification_error_v1".to_string(),
            error_kind.as_str().to_string(),
            message.to_string(),
        ];
        parts.extend(evidence_parts.iter().cloned());
        Self {
            error_kind,
            message,
            failure_digest: hash_parts(&parts),
            counters,
        }
    }

    pub fn error_kind(&self) -> &QuerySubscriptionRuntimeCertificationErrorKind {
        &self.error_kind
    }

    pub fn message(&self) -> &str {
        self.message
    }

    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }

    pub fn counters(&self) -> &QuerySubscriptionRuntimeCertificationCounters {
        &self.counters
    }
}
