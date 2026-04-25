use super::certification::SupportCertificationEvidenceBundle;
use super::domain_certification::{
    SupportCertificationHandoffReport, SupportDomainCertificationBundle,
    SupportDomainCertificationScenario, SupportGenericCertificationReport,
    SupportRoadmapPhysicalReadinessPosture,
};
use super::failure::{SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportAccuracyCertificationRow {
    row_kind: SubscriptionSupportAccuracyCertificationRowKind,
    evidence_digest: String,
    forbidden_exact_overclaim_count: u64,
    global_scan_debt_count: u64,
    row_digest: String,
}

impl SubscriptionSupportAccuracyCertificationRow {
    pub(crate) fn new(
        row_kind: SubscriptionSupportAccuracyCertificationRowKind,
        evidence_digest: impl Into<String>,
        forbidden_exact_overclaim_count: u64,
        global_scan_debt_count: u64,
    ) -> Result<Self, SupportTrustFailure> {
        if forbidden_exact_overclaim_count != 0 || global_scan_debt_count != 0 {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim,
                SupportTrustRecoveryPosture::RerunCertification,
                "subscription-support accuracy suite rows require zero exact-overclaim and global-scan debt counters",
            ));
        }
        let mut row = Self {
            row_kind,
            evidence_digest: require_non_empty("suite row evidence digest", evidence_digest)?,
            forbidden_exact_overclaim_count,
            global_scan_debt_count,
            row_digest: String::new(),
        };
        row.row_digest = stable_digest(&SubscriptionSupportAccuracyCertificationRowDigestBasis {
            row_kind: row.row_kind,
            evidence_digest: &row.evidence_digest,
            forbidden_exact_overclaim_count: row.forbidden_exact_overclaim_count,
            global_scan_debt_count: row.global_scan_debt_count,
        })?;
        Ok(row)
    }

    pub fn row_kind(&self) -> SubscriptionSupportAccuracyCertificationRowKind {
        self.row_kind
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportAccuracyCertificationCounterSnapshot {
    required_row_count: u64,
    certified_row_count: u64,
    forbidden_exact_overclaim_count: u64,
    global_scan_debt_count: u64,
}

impl SubscriptionSupportAccuracyCertificationCounterSnapshot {
    pub fn new(
        required_row_count: u64,
        certified_row_count: u64,
        forbidden_exact_overclaim_count: u64,
        global_scan_debt_count: u64,
    ) -> Result<Self, SupportTrustFailure> {
        if forbidden_exact_overclaim_count != 0 || global_scan_debt_count != 0 {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim,
                SupportTrustRecoveryPosture::RerunCertification,
                "subscription-support accuracy suite counters require zero exact-overclaim and global-scan debt",
            ));
        }
        Ok(Self {
            required_row_count,
            certified_row_count,
            forbidden_exact_overclaim_count,
            global_scan_debt_count,
        })
    }

    pub fn required_row_count(&self) -> u64 {
        self.required_row_count
    }

    pub fn certified_row_count(&self) -> u64 {
        self.certified_row_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportAccuracyCertificationOutputs {
    artifact_digest: String,
    subscription_support_digest: String,
    diagnostics_digest: String,
    counter_snapshot_digest: String,
    certification_summary_digest: String,
}

impl SubscriptionSupportAccuracyCertificationOutputs {
    pub fn from_evidence_bundle(
        evidence_bundle: &SupportCertificationEvidenceBundle,
    ) -> Result<Self, SupportTrustFailure> {
        Ok(Self {
            artifact_digest: require_non_empty(
                "artifact digest",
                evidence_bundle.artifact_digest(),
            )?,
            subscription_support_digest: require_non_empty(
                "subscription-support digest",
                evidence_bundle.subscription_support_digest(),
            )?,
            diagnostics_digest: require_non_empty(
                "diagnostics digest",
                evidence_bundle.diagnostics_digest(),
            )?,
            counter_snapshot_digest: require_non_empty(
                "counter snapshot digest",
                evidence_bundle.counter_snapshot_digest(),
            )?,
            certification_summary_digest: require_non_empty(
                "certification summary digest",
                evidence_bundle.certification_summary_digest(),
            )?,
        })
    }

    pub fn artifact_digest(&self) -> &str {
        &self.artifact_digest
    }

    pub fn subscription_support_digest(&self) -> &str {
        &self.subscription_support_digest
    }

    pub fn diagnostics_digest(&self) -> &str {
        &self.diagnostics_digest
    }

    pub fn counter_snapshot_digest(&self) -> &str {
        &self.counter_snapshot_digest
    }

    pub fn certification_summary_digest(&self) -> &str {
        &self.certification_summary_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportAccuracyCertificationSuite {
    suite_name: String,
    rows: Vec<SubscriptionSupportAccuracyCertificationRow>,
    required_outputs: SubscriptionSupportAccuracyCertificationOutputs,
    counter_snapshot: SubscriptionSupportAccuracyCertificationCounterSnapshot,
    generic_certification_digest: String,
    domain_certification_digest: String,
    handoff_digest: String,
    suite_digest: String,
}

impl SubscriptionSupportAccuracyCertificationSuite {
    pub fn from_phase_artifacts(
        evidence_bundle: &SupportCertificationEvidenceBundle,
        generic_report: &SupportGenericCertificationReport,
        domain_bundle: &SupportDomainCertificationBundle,
        handoff_report: &SupportCertificationHandoffReport,
    ) -> Result<Self, SupportTrustFailure> {
        let rows = build_required_rows_from_phase_artifacts(
            evidence_bundle,
            generic_report,
            domain_bundle,
            handoff_report,
        )?;
        Self::from_rows_and_phase_artifacts(
            rows,
            evidence_bundle,
            generic_report,
            domain_bundle,
            handoff_report,
        )
    }

    pub(crate) fn from_rows_and_phase_artifacts(
        mut rows: Vec<SubscriptionSupportAccuracyCertificationRow>,
        evidence_bundle: &SupportCertificationEvidenceBundle,
        generic_report: &SupportGenericCertificationReport,
        domain_bundle: &SupportDomainCertificationBundle,
        handoff_report: &SupportCertificationHandoffReport,
    ) -> Result<Self, SupportTrustFailure> {
        validate_handoff(handoff_report)?;
        rows.sort_by_key(SubscriptionSupportAccuracyCertificationRow::row_kind);
        validate_required_rows(&rows)?;
        validate_rows_match_phase_artifacts(
            &rows,
            evidence_bundle,
            generic_report,
            domain_bundle,
            handoff_report,
        )?;
        let required_outputs =
            SubscriptionSupportAccuracyCertificationOutputs::from_evidence_bundle(evidence_bundle)?;
        let required_row_count =
            SubscriptionSupportAccuracyCertificationRowKind::required().len() as u64;
        let counter_snapshot = SubscriptionSupportAccuracyCertificationCounterSnapshot::new(
            required_row_count,
            rows.len() as u64,
            evidence_bundle
                .counter_snapshot()
                .forbidden_exact_overclaim_count(),
            evidence_bundle.counter_snapshot().global_scan_debt_count(),
        )?;
        if counter_snapshot.certified_row_count() != counter_snapshot.required_row_count() {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "subscription-support accuracy suite counters must match required row coverage",
            ));
        }
        let mut suite = Self {
            suite_name: SUBSCRIPTION_SUPPORT_ACCURACY_CERTIFICATION_SUITE_NAME.to_string(),
            rows,
            required_outputs,
            counter_snapshot,
            generic_certification_digest: require_non_empty(
                "generic certification digest",
                generic_report.generic_certification_digest(),
            )?,
            domain_certification_digest: require_non_empty(
                "domain certification digest",
                domain_bundle.domain_certification_digest(),
            )?,
            handoff_digest: require_non_empty("handoff digest", handoff_report.handoff_digest())?,
            suite_digest: String::new(),
        };
        suite.suite_digest = stable_digest(&SubscriptionSupportAccuracySuiteDigestBasis {
            suite_name: &suite.suite_name,
            row_digests: &suite
                .rows
                .iter()
                .map(|row| row.row_digest())
                .collect::<Vec<_>>(),
            required_outputs: &suite.required_outputs,
            counter_snapshot: suite.counter_snapshot,
            generic_certification_digest: &suite.generic_certification_digest,
            domain_certification_digest: &suite.domain_certification_digest,
            handoff_digest: &suite.handoff_digest,
        })?;
        Ok(suite)
    }

    pub fn suite_name(&self) -> &str {
        &self.suite_name
    }

    pub fn rows(&self) -> &[SubscriptionSupportAccuracyCertificationRow] {
        &self.rows
    }

    pub fn required_outputs(&self) -> &SubscriptionSupportAccuracyCertificationOutputs {
        &self.required_outputs
    }

    pub fn counter_snapshot(&self) -> SubscriptionSupportAccuracyCertificationCounterSnapshot {
        self.counter_snapshot
    }

    pub fn suite_digest(&self) -> &str {
        &self.suite_digest
    }
}

fn build_required_rows_from_phase_artifacts(
    evidence_bundle: &SupportCertificationEvidenceBundle,
    generic_report: &SupportGenericCertificationReport,
    domain_bundle: &SupportDomainCertificationBundle,
    handoff_report: &SupportCertificationHandoffReport,
) -> Result<Vec<SubscriptionSupportAccuracyCertificationRow>, SupportTrustFailure> {
    expected_row_evidence_digests(
        evidence_bundle,
        generic_report,
        domain_bundle,
        handoff_report,
    )?
    .into_iter()
    .map(|(row_kind, evidence_digest)| {
        SubscriptionSupportAccuracyCertificationRow::new(row_kind, evidence_digest, 0, 0)
    })
    .collect()
}

fn validate_rows_match_phase_artifacts(
    rows: &[SubscriptionSupportAccuracyCertificationRow],
    evidence_bundle: &SupportCertificationEvidenceBundle,
    generic_report: &SupportGenericCertificationReport,
    domain_bundle: &SupportDomainCertificationBundle,
    handoff_report: &SupportCertificationHandoffReport,
) -> Result<(), SupportTrustFailure> {
    let expected = expected_row_evidence_digests(
        evidence_bundle,
        generic_report,
        domain_bundle,
        handoff_report,
    )?;
    for row in rows {
        match expected.get(&row.row_kind()) {
            Some(expected_digest) if expected_digest == &row.evidence_digest => {}
            _ => {
                return Err(SupportTrustFailure::new(
                    SupportTrustFailureKind::SupportTrustCoverageMissing,
                    SupportTrustRecoveryPosture::RerunCertification,
                    "subscription-support accuracy suite row evidence must match the supplied phase artifacts",
                ));
            }
        }
    }
    Ok(())
}

fn expected_row_evidence_digests(
    evidence_bundle: &SupportCertificationEvidenceBundle,
    generic_report: &SupportGenericCertificationReport,
    domain_bundle: &SupportDomainCertificationBundle,
    handoff_report: &SupportCertificationHandoffReport,
) -> Result<BTreeMap<SubscriptionSupportAccuracyCertificationRowKind, String>, SupportTrustFailure>
{
    let mut digests = BTreeMap::new();
    let coverage_rows = evidence_bundle
        .coverage_rows()
        .iter()
        .map(|row| {
            (
                row.evidence().row_id().to_string(),
                row.evidence().declared_row_digest().to_string(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let domain_rows = domain_bundle
        .rows()
        .iter()
        .map(|row| (row.scenario(), row.row_digest().to_string()))
        .collect::<BTreeMap<_, _>>();

    insert_expected(
        &mut digests,
        SubscriptionSupportAccuracyCertificationRowKind::ExactSupportTrustedControl,
        "certification-row",
        evidence_bundle,
        generic_report,
        domain_bundle,
        handoff_report,
        Some(required_coverage_row_digest(
            &coverage_rows,
            "row:basis-bound-exact",
        )?),
        None,
    )?;
    insert_expected(
        &mut digests,
        SubscriptionSupportAccuracyCertificationRowKind::DegradedSupportTrusted,
        "certification-row",
        evidence_bundle,
        generic_report,
        domain_bundle,
        handoff_report,
        Some(required_coverage_row_digest(
            &coverage_rows,
            "row:degraded-continuation",
        )?),
        None,
    )?;
    insert_expected(
        &mut digests,
        SubscriptionSupportAccuracyCertificationRowKind::CertificationMatrixComplete,
        "coverage-matrix",
        evidence_bundle,
        generic_report,
        domain_bundle,
        handoff_report,
        Some(evidence_bundle.certification_summary_digest()),
        None,
    )?;
    insert_expected(
        &mut digests,
        SubscriptionSupportAccuracyCertificationRowKind::GenericCertificationIncludesSupportTrust,
        "generic-certification",
        evidence_bundle,
        generic_report,
        domain_bundle,
        handoff_report,
        Some(generic_report.generic_certification_digest()),
        None,
    )?;
    for (row_kind, scenario) in [
        (
            SubscriptionSupportAccuracyCertificationRowKind::DomainGeometrySupportTrust,
            SupportDomainCertificationScenario::GeometryCadSessionContinuation,
        ),
        (
            SubscriptionSupportAccuracyCertificationRowKind::DomainWebDataSupportTrust,
            SupportDomainCertificationScenario::WebDataRestartReplication,
        ),
        (
            SubscriptionSupportAccuracyCertificationRowKind::DomainAiDegradedSupportTrust,
            SupportDomainCertificationScenario::AiBranchWorkspaceDegradedContinuation,
        ),
        (
            SubscriptionSupportAccuracyCertificationRowKind::DomainChipRebuildSupportTrust,
            SupportDomainCertificationScenario::ChipSimulationLongHistoryRebuild,
        ),
        (
            SubscriptionSupportAccuracyCertificationRowKind::DomainOfflineOmittedSupportTrust,
            SupportDomainCertificationScenario::OfflineCollaborativeCapsuleOmission,
        ),
    ] {
        insert_expected(
            &mut digests,
            row_kind,
            "domain-scenario",
            evidence_bundle,
            generic_report,
            domain_bundle,
            handoff_report,
            Some(required_domain_row_digest(&domain_rows, scenario)?),
            None,
        )?;
    }
    insert_expected(
        &mut digests,
        SubscriptionSupportAccuracyCertificationRowKind::Roadmap2HandoffPhysicalDebtExplicit,
        "handoff",
        evidence_bundle,
        generic_report,
        domain_bundle,
        handoff_report,
        Some(handoff_report.handoff_digest()),
        None,
    )?;

    for row_kind in SubscriptionSupportAccuracyCertificationRowKind::required() {
        if digests.contains_key(row_kind) {
            continue;
        }
        insert_expected(
            &mut digests,
            *row_kind,
            row_kind.evidence_lane_label(),
            evidence_bundle,
            generic_report,
            domain_bundle,
            handoff_report,
            Some(evidence_bundle.evidence_bundle_digest()),
            Some(evidence_bundle.diagnostics_digest()),
        )?;
    }
    Ok(digests)
}

#[allow(clippy::too_many_arguments)]
fn insert_expected(
    digests: &mut BTreeMap<SubscriptionSupportAccuracyCertificationRowKind, String>,
    row_kind: SubscriptionSupportAccuracyCertificationRowKind,
    source_label: &'static str,
    evidence_bundle: &SupportCertificationEvidenceBundle,
    generic_report: &SupportGenericCertificationReport,
    domain_bundle: &SupportDomainCertificationBundle,
    handoff_report: &SupportCertificationHandoffReport,
    primary_source_digest: Option<&str>,
    hostile_source_digest: Option<&str>,
) -> Result<(), SupportTrustFailure> {
    let digest = stable_digest(&SubscriptionSupportAccuracyRowEvidenceDigestBasis {
        row_kind,
        source_label,
        evidence_bundle_digest: evidence_bundle.evidence_bundle_digest(),
        artifact_digest: evidence_bundle.artifact_digest(),
        subscription_support_digest: evidence_bundle.subscription_support_digest(),
        diagnostics_digest: evidence_bundle.diagnostics_digest(),
        counter_snapshot_digest: evidence_bundle.counter_snapshot_digest(),
        certification_summary_digest: evidence_bundle.certification_summary_digest(),
        generic_certification_digest: generic_report.generic_certification_digest(),
        domain_certification_digest: domain_bundle.domain_certification_digest(),
        handoff_digest: handoff_report.handoff_digest(),
        primary_source_digest,
        hostile_source_digest,
    })?;
    digests.insert(row_kind, digest);
    Ok(())
}

fn required_coverage_row_digest<'a>(
    coverage_rows: &'a BTreeMap<String, String>,
    row_id: &'static str,
) -> Result<&'a str, SupportTrustFailure> {
    coverage_rows
        .get(row_id)
        .map(String::as_str)
        .ok_or_else(|| {
            SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                format!("subscription-support accuracy suite requires certification row {row_id}"),
            )
        })
}

fn required_domain_row_digest(
    domain_rows: &BTreeMap<SupportDomainCertificationScenario, String>,
    scenario: SupportDomainCertificationScenario,
) -> Result<&str, SupportTrustFailure> {
    domain_rows
        .get(&scenario)
        .map(String::as_str)
        .ok_or_else(|| {
            SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "subscription-support accuracy suite requires domain scenario evidence",
            )
        })
}

#[derive(Serialize)]
struct SubscriptionSupportAccuracyCertificationRowDigestBasis<'a> {
    row_kind: SubscriptionSupportAccuracyCertificationRowKind,
    evidence_digest: &'a str,
    forbidden_exact_overclaim_count: u64,
    global_scan_debt_count: u64,
}

#[derive(Serialize)]
struct SubscriptionSupportAccuracySuiteDigestBasis<'a> {
    suite_name: &'a str,
    row_digests: &'a [&'a str],
    required_outputs: &'a SubscriptionSupportAccuracyCertificationOutputs,
    counter_snapshot: SubscriptionSupportAccuracyCertificationCounterSnapshot,
    generic_certification_digest: &'a str,
    domain_certification_digest: &'a str,
    handoff_digest: &'a str,
}

#[derive(Serialize)]
struct SubscriptionSupportAccuracyRowEvidenceDigestBasis<'a> {
    row_kind: SubscriptionSupportAccuracyCertificationRowKind,
    source_label: &'static str,
    evidence_bundle_digest: &'a str,
    artifact_digest: &'a str,
    subscription_support_digest: &'a str,
    diagnostics_digest: &'a str,
    counter_snapshot_digest: &'a str,
    certification_summary_digest: &'a str,
    generic_certification_digest: &'a str,
    domain_certification_digest: &'a str,
    handoff_digest: &'a str,
    primary_source_digest: Option<&'a str>,
    hostile_source_digest: Option<&'a str>,
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

fn validate_required_rows(
    rows: &[SubscriptionSupportAccuracyCertificationRow],
) -> Result<(), SupportTrustFailure> {
    let mut seen = BTreeSet::new();
    for row in rows {
        if !seen.insert(row.row_kind()) {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "subscription-support accuracy suite cannot contain duplicate row kinds",
            ));
        }
    }
    for required in SubscriptionSupportAccuracyCertificationRowKind::required() {
        if !seen.contains(required) {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "subscription-support accuracy suite is missing a required certification row",
            ));
        }
    }
    Ok(())
}

fn validate_handoff(
    handoff_report: &SupportCertificationHandoffReport,
) -> Result<(), SupportTrustFailure> {
    if !handoff_report.semantic_support_trust_closed()
        || handoff_report.roadmap_physical_readiness_posture()
            != SupportRoadmapPhysicalReadinessPosture::PhysicalDatabaseReadinessDeferredToRoadmap2
    {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim,
            SupportTrustRecoveryPosture::WaitForMilestone14OrRoadmap2Evidence,
            "subscription-support accuracy suite requires semantic trust closure while keeping physical readiness debt explicit",
        ));
    }
    Ok(())
}

impl SubscriptionSupportAccuracyCertificationRowKind {
    fn evidence_lane_label(self) -> &'static str {
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

fn stable_digest<T: Serialize + ?Sized>(value: &T) -> Result<String, SupportTrustFailure> {
    let bytes = serde_json::to_vec(value).map_err(|_| {
        SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "subscription-support accuracy suite evidence must serialize deterministically",
        )
    })?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    Ok(format!("{:x}", hasher.finalize()))
}

fn require_non_empty(
    label: &'static str,
    value: impl Into<String>,
) -> Result<String, SupportTrustFailure> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            format!("subscription-support accuracy suite {label} must be non-empty"),
        ));
    }
    Ok(value)
}
