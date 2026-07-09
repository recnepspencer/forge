use super::failure::{SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture};
use super::performance::SupportTrustPathClass;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum SupportTrustDriftCause {
    Family,
    Role,
    Basis,
    CursorCheckpoint,
    SupportDigest,
    Compatibility,
    OperationalVerdict,
    Portability,
    CertificationCoverage,
    PlacementCost,
}

impl SupportTrustDriftCause {
    pub(crate) fn failure_kind(self) -> SupportTrustFailureKind {
        match self {
            Self::Family => SupportTrustFailureKind::SupportTrustFamilyMismatch,
            Self::Role => SupportTrustFailureKind::SupportTrustRoleMismatch,
            Self::Basis | Self::CursorCheckpoint | Self::SupportDigest => {
                SupportTrustFailureKind::SupportTrustBasisMismatch
            }
            Self::Compatibility => SupportTrustFailureKind::SupportTrustCompatibilityMismatch,
            Self::OperationalVerdict => {
                SupportTrustFailureKind::SupportTrustOperationalVerdictMismatch
            }
            Self::Portability => SupportTrustFailureKind::SupportTrustPortabilityMismatch,
            Self::CertificationCoverage => SupportTrustFailureKind::SupportTrustCoverageMissing,
            Self::PlacementCost => SupportTrustFailureKind::SupportTrustAccessStructureDebt,
        }
    }

    pub(crate) fn is_blocking(self) -> bool {
        self != Self::PlacementCost
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportTrustDriftLocality {
    SupportIdentity,
    FamilyRole,
    BasisLocal,
    CursorCheckpointLocal,
    CompatibilityEpoch,
    CertificationScope,
    PlacementCostAdvisory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportStalenessVerdict {
    Fresh,
    StaleRejected,
    CoverageIncomplete,
    PlacementCostAdvisory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SupportTrustSuppressedCause {
    cause: SupportTrustDriftCause,
    locality: SupportTrustDriftLocality,
}

impl SupportTrustSuppressedCause {
    pub(crate) fn new(cause: SupportTrustDriftCause, locality: SupportTrustDriftLocality) -> Self {
        Self { cause, locality }
    }

    pub fn cause(&self) -> SupportTrustDriftCause {
        self.cause
    }

    pub fn locality(&self) -> SupportTrustDriftLocality {
        self.locality
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportTrustDriftScanPlan {
    locality: SupportTrustDriftLocality,
    path_class: SupportTrustPathClass,
    expected_checks: u64,
    expected_index_probes: u64,
    certification_coverage_required: bool,
    certification_coverage_present: bool,
}

impl SupportTrustDriftScanPlan {
    pub fn new(
        locality: SupportTrustDriftLocality,
        path_class: SupportTrustPathClass,
        expected_checks: u64,
        expected_index_probes: u64,
    ) -> Result<Self, SupportTrustFailure> {
        if expected_checks == 0 {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustPayloadBudgetExceeded,
                SupportTrustRecoveryPosture::RetryWithFresherReceipts,
                "support trust drift plans must perform at least one bounded check",
            ));
        }
        if matches!(path_class, SupportTrustPathClass::RoadmapHandoffPath)
            || matches!(locality, SupportTrustDriftLocality::CertificationScope)
                && path_class == SupportTrustPathClass::ForegroundResumeTrustPath
        {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustAccessStructureDebt,
                SupportTrustRecoveryPosture::RebuildTrustCache,
                "support trust foreground drift checks cannot use global or certification-scope scans",
            ));
        }
        Ok(Self {
            locality,
            path_class,
            expected_checks,
            expected_index_probes,
            certification_coverage_required: false,
            certification_coverage_present: true,
        })
    }

    pub fn foreground_support_identity() -> Self {
        Self {
            locality: SupportTrustDriftLocality::SupportIdentity,
            path_class: SupportTrustPathClass::ForegroundResumeTrustPath,
            expected_checks: 8,
            expected_index_probes: 1,
            certification_coverage_required: false,
            certification_coverage_present: true,
        }
    }

    pub fn certification_scope(
        path_class: SupportTrustPathClass,
        expected_checks: u64,
        expected_index_probes: u64,
        coverage_present: bool,
    ) -> Result<Self, SupportTrustFailure> {
        let mut plan = Self::new(
            SupportTrustDriftLocality::CertificationScope,
            path_class,
            expected_checks,
            expected_index_probes,
        )?;
        plan.certification_coverage_required = true;
        plan.certification_coverage_present = coverage_present;
        Ok(plan)
    }

    pub fn locality(&self) -> SupportTrustDriftLocality {
        self.locality
    }

    pub fn expected_checks(&self) -> u64 {
        self.expected_checks
    }

    pub fn expected_index_probes(&self) -> u64 {
        self.expected_index_probes
    }

    pub(crate) fn certification_coverage_is_missing(&self) -> bool {
        self.certification_coverage_required && !self.certification_coverage_present
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportTrustDriftReport {
    primary_cause: Option<SupportTrustDriftCause>,
    suppressed_causes: Vec<SupportTrustSuppressedCause>,
    staleness_verdict: SupportStalenessVerdict,
    checks_performed: u64,
    index_probes: u64,
    global_scan_debt_count: u64,
}

impl SupportTrustDriftReport {
    pub(crate) fn fresh(plan: &SupportTrustDriftScanPlan) -> Self {
        Self {
            primary_cause: None,
            suppressed_causes: Vec::new(),
            staleness_verdict: SupportStalenessVerdict::Fresh,
            checks_performed: plan.expected_checks(),
            index_probes: plan.expected_index_probes(),
            global_scan_debt_count: 0,
        }
    }

    pub(crate) fn from_causes(
        plan: &SupportTrustDriftScanPlan,
        mut causes: Vec<(SupportTrustDriftCause, SupportTrustDriftLocality)>,
    ) -> Self {
        causes.sort_by_key(|(cause, _)| *cause);
        causes.dedup_by_key(|(cause, _)| *cause);
        let primary_cause = causes.first().map(|(cause, _)| *cause);
        let staleness_verdict = match primary_cause {
            Some(SupportTrustDriftCause::CertificationCoverage) => {
                SupportStalenessVerdict::CoverageIncomplete
            }
            Some(SupportTrustDriftCause::PlacementCost) => {
                SupportStalenessVerdict::PlacementCostAdvisory
            }
            Some(_) => SupportStalenessVerdict::StaleRejected,
            None => SupportStalenessVerdict::Fresh,
        };
        let suppressed_causes = causes
            .into_iter()
            .skip(1)
            .map(|(cause, locality)| SupportTrustSuppressedCause::new(cause, locality))
            .collect();
        Self {
            primary_cause,
            suppressed_causes,
            staleness_verdict,
            checks_performed: plan.expected_checks(),
            index_probes: plan.expected_index_probes(),
            global_scan_debt_count: 0,
        }
    }

    #[cfg(test)]
    pub(crate) fn from_observed_causes(
        plan: &SupportTrustDriftScanPlan,
        causes: impl IntoIterator<Item = (SupportTrustDriftCause, SupportTrustDriftLocality)>,
    ) -> Self {
        Self::from_causes(plan, causes.into_iter().collect())
    }

    pub fn primary_cause(&self) -> Option<SupportTrustDriftCause> {
        self.primary_cause
    }

    pub fn suppressed_causes(&self) -> &[SupportTrustSuppressedCause] {
        &self.suppressed_causes
    }

    pub fn staleness_verdict(&self) -> SupportStalenessVerdict {
        self.staleness_verdict
    }

    pub fn checks_performed(&self) -> u64 {
        self.checks_performed
    }

    pub fn index_probes(&self) -> u64 {
        self.index_probes
    }

    pub fn global_scan_debt_count(&self) -> u64 {
        self.global_scan_debt_count
    }

    pub fn stale_rejection_count(&self) -> u64 {
        u64::from(
            self.primary_cause.is_some_and(|cause| {
                cause.is_blocking() && cause != SupportTrustDriftCause::CertificationCoverage
            }) || self.suppressed_causes.iter().any(|suppressed| {
                suppressed.cause().is_blocking()
                    && suppressed.cause() != SupportTrustDriftCause::CertificationCoverage
            }),
        )
    }

    pub fn coverage_drift_count(&self) -> u64 {
        u64::from(self.contains_cause(SupportTrustDriftCause::CertificationCoverage))
    }

    pub fn placement_advisory_count(&self) -> u64 {
        u64::from(self.contains_cause(SupportTrustDriftCause::PlacementCost))
    }

    pub(crate) fn blocking_cause(&self) -> Option<SupportTrustDriftCause> {
        self.primary_cause.filter(|cause| cause.is_blocking())
    }

    fn contains_cause(&self, cause: SupportTrustDriftCause) -> bool {
        self.primary_cause == Some(cause)
            || self
                .suppressed_causes
                .iter()
                .any(|suppressed| suppressed.cause() == cause)
    }
}
