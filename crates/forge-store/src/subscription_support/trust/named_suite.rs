use super::certification::{
    SupportCertificationBatchScopeKind, SupportCertificationEvidenceBundle,
};
use super::domain_certification::{
    SupportCertificationHandoffReport, SupportDomainCertificationBundle,
    SupportDomainCertificationScenario, SupportGenericCertificationReport,
    SupportRoadmapPhysicalReadinessPosture,
};
use super::failure::{SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture};
use super::performance::{
    SupportTrustAllocationScope, SupportTrustDensityClass, SupportTrustPathClass,
};
use super::reports::CertifiedSupportTrustReport;
use super::taxonomy::{SupportTrustClass, SupportTrustProvenance, SupportTrustStrength};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SubscriptionSupportAccuracyLaneOutcome {
    CertifiedPass,
    TypedRejection,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportAccuracyLaneEvidence {
    row_kind: SubscriptionSupportAccuracyCertificationRowKind,
    outcome: SubscriptionSupportAccuracyLaneOutcome,
    failure_kind: Option<SupportTrustFailureKind>,
    recovery_posture: Option<SupportTrustRecoveryPosture>,
    source_digest: String,
    diagnostics_digest: String,
    counter_digest: String,
    evidence_digest: String,
}

impl SubscriptionSupportAccuracyLaneEvidence {
    pub fn certified_pass_from_report(
        row_kind: SubscriptionSupportAccuracyCertificationRowKind,
        report: &CertifiedSupportTrustReport,
        diagnostics_digest: impl Into<String>,
        counter_digest: impl Into<String>,
    ) -> Result<Self, SupportTrustFailure> {
        validate_certified_report_lane(row_kind, report)?;
        let source_digest =
            stable_digest(&SubscriptionSupportAccuracyCertifiedReportLaneDigestBasis {
                row_kind,
                trust_class: report.trust_class(),
                trust_strength: report.trust_strength(),
                provenance: report.provenance(),
                suite_version: report.certification_stamp().suite_version(),
                row_id: report.certification_stamp().row_id(),
                evidence_bundle_digest: report.certification_stamp().evidence_bundle_digest(),
            })?;
        Self::certified_pass(row_kind, source_digest, diagnostics_digest, counter_digest)
    }

    pub fn certified_counter_pass_from_evidence_bundle(
        row_kind: SubscriptionSupportAccuracyCertificationRowKind,
        evidence_bundle: &SupportCertificationEvidenceBundle,
    ) -> Result<Self, SupportTrustFailure> {
        validate_zero_counter_lane(row_kind, evidence_bundle)?;
        let source_digest = stable_digest(&SubscriptionSupportAccuracyCounterLaneDigestBasis {
            row_kind,
            evidence_bundle_digest: evidence_bundle.evidence_bundle_digest(),
            forbidden_exact_overclaim_count: evidence_bundle
                .counter_snapshot()
                .forbidden_exact_overclaim_count(),
            global_scan_debt_count: evidence_bundle.counter_snapshot().global_scan_debt_count(),
        })?;
        Self::certified_pass(
            row_kind,
            source_digest,
            evidence_bundle.diagnostics_digest(),
            evidence_bundle.counter_snapshot_digest(),
        )
    }

    pub(crate) fn certified_pass(
        row_kind: SubscriptionSupportAccuracyCertificationRowKind,
        source_digest: impl Into<String>,
        diagnostics_digest: impl Into<String>,
        counter_digest: impl Into<String>,
    ) -> Result<Self, SupportTrustFailure> {
        Self::new(
            row_kind,
            SubscriptionSupportAccuracyLaneOutcome::CertifiedPass,
            None,
            None,
            source_digest,
            diagnostics_digest,
            counter_digest,
        )
    }

    pub fn typed_rejection_from_failure(
        row_kind: SubscriptionSupportAccuracyCertificationRowKind,
        failure: &SupportTrustFailure,
    ) -> Result<Self, SupportTrustFailure> {
        let source_digest = stable_digest(&SubscriptionSupportAccuracyFailureLaneDigestBasis {
            row_kind,
            digest_role: "failure-source",
            failure,
        })?;
        let diagnostics_digest =
            stable_digest(&SubscriptionSupportAccuracyFailureLaneDigestBasis {
                row_kind,
                digest_role: "failure-diagnostics",
                failure,
            })?;
        let counter_digest = stable_digest(&SubscriptionSupportAccuracyFailureLaneDigestBasis {
            row_kind,
            digest_role: "failure-counter",
            failure,
        })?;
        Self::typed_rejection(
            row_kind,
            failure,
            source_digest,
            diagnostics_digest,
            counter_digest,
        )
    }

    pub(crate) fn typed_rejection(
        row_kind: SubscriptionSupportAccuracyCertificationRowKind,
        failure: &SupportTrustFailure,
        source_digest: impl Into<String>,
        diagnostics_digest: impl Into<String>,
        counter_digest: impl Into<String>,
    ) -> Result<Self, SupportTrustFailure> {
        Self::new(
            row_kind,
            SubscriptionSupportAccuracyLaneOutcome::TypedRejection,
            Some(failure.kind()),
            Some(failure.recovery_posture()),
            source_digest,
            diagnostics_digest,
            counter_digest,
        )
    }

    pub fn row_kind(&self) -> SubscriptionSupportAccuracyCertificationRowKind {
        self.row_kind
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    #[allow(clippy::too_many_arguments)]
    fn new(
        row_kind: SubscriptionSupportAccuracyCertificationRowKind,
        outcome: SubscriptionSupportAccuracyLaneOutcome,
        failure_kind: Option<SupportTrustFailureKind>,
        recovery_posture: Option<SupportTrustRecoveryPosture>,
        source_digest: impl Into<String>,
        diagnostics_digest: impl Into<String>,
        counter_digest: impl Into<String>,
    ) -> Result<Self, SupportTrustFailure> {
        validate_lane_outcome(row_kind, outcome, failure_kind)?;
        let mut evidence = Self {
            row_kind,
            outcome,
            failure_kind,
            recovery_posture,
            source_digest: require_non_empty("lane source digest", source_digest)?,
            diagnostics_digest: require_non_empty("lane diagnostics digest", diagnostics_digest)?,
            counter_digest: require_non_empty("lane counter digest", counter_digest)?,
            evidence_digest: String::new(),
        };
        evidence.evidence_digest =
            stable_digest(&SubscriptionSupportAccuracyLaneEvidenceDigestBasis {
                row_kind: evidence.row_kind,
                outcome: evidence.outcome,
                failure_kind: evidence.failure_kind,
                recovery_posture: evidence.recovery_posture,
                source_digest: &evidence.source_digest,
                diagnostics_digest: &evidence.diagnostics_digest,
                counter_digest: &evidence.counter_digest,
            })?;
        Ok(evidence)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportAccuracyLaneEvidenceSet {
    lanes: Vec<SubscriptionSupportAccuracyLaneEvidence>,
    lane_evidence_set_digest: String,
}

impl SubscriptionSupportAccuracyLaneEvidenceSet {
    pub fn new(
        mut lanes: Vec<SubscriptionSupportAccuracyLaneEvidence>,
    ) -> Result<Self, SupportTrustFailure> {
        lanes.sort_by_key(SubscriptionSupportAccuracyLaneEvidence::row_kind);
        validate_required_lane_evidence(&lanes)?;
        let mut evidence_set = Self {
            lanes,
            lane_evidence_set_digest: String::new(),
        };
        evidence_set.lane_evidence_set_digest =
            stable_digest(&SubscriptionSupportAccuracyLaneEvidenceSetDigestBasis {
                lane_digests: &evidence_set
                    .lanes
                    .iter()
                    .map(SubscriptionSupportAccuracyLaneEvidence::evidence_digest)
                    .collect::<Vec<_>>(),
            })?;
        Ok(evidence_set)
    }

    pub fn lanes(&self) -> &[SubscriptionSupportAccuracyLaneEvidence] {
        &self.lanes
    }

    pub fn lane_evidence_set_digest(&self) -> &str {
        &self.lane_evidence_set_digest
    }

    fn evidence_for(
        &self,
        row_kind: SubscriptionSupportAccuracyCertificationRowKind,
    ) -> Option<&SubscriptionSupportAccuracyLaneEvidence> {
        self.lanes.iter().find(|lane| lane.row_kind() == row_kind)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SubscriptionSupportAccuracyPersistencePosture {
    InMemoryCertificationOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportAccuracyPerformanceCloseout {
    certification_row_count: u64,
    certification_index_probe_count: u64,
    certification_receipt_reuse_count: u64,
    certification_allocation_count: u64,
    generic_row_count: u64,
    generic_index_probe_count: u64,
    generic_receipt_reuse_count: u64,
    generic_allocation_count: u64,
    domain_scenario_row_count: u64,
    domain_index_probe_count: u64,
    domain_receipt_reuse_count: u64,
    domain_allocation_count: u64,
    forbidden_exact_overclaim_count: u64,
    global_scan_debt_count: u64,
}

impl SubscriptionSupportAccuracyPerformanceCloseout {
    fn from_phase_artifacts(
        evidence_bundle: &SupportCertificationEvidenceBundle,
        generic_report: &SupportGenericCertificationReport,
        domain_bundle: &SupportDomainCertificationBundle,
    ) -> Result<Self, SupportTrustFailure> {
        validate_certification_performance(evidence_bundle)?;
        validate_generic_performance(generic_report)?;
        validate_domain_performance(domain_bundle)?;
        let certification_counters = evidence_bundle.counter_snapshot();
        let generic_counters = generic_report.counter_snapshot();
        let domain_counters = domain_bundle.counter_snapshot();
        Ok(Self {
            certification_row_count: certification_counters.coverage_row_count(),
            certification_index_probe_count: certification_counters.index_probe_count(),
            certification_receipt_reuse_count: certification_counters.receipt_reuse_count(),
            certification_allocation_count: certification_counters.allocation_count(),
            generic_row_count: generic_counters.generic_row_count(),
            generic_index_probe_count: generic_counters.index_probe_count(),
            generic_receipt_reuse_count: generic_counters.receipt_reuse_count(),
            generic_allocation_count: generic_counters.allocation_count(),
            domain_scenario_row_count: domain_counters.scenario_row_count(),
            domain_index_probe_count: domain_counters.index_probe_count(),
            domain_receipt_reuse_count: domain_counters.receipt_reuse_count(),
            domain_allocation_count: domain_counters.allocation_count(),
            forbidden_exact_overclaim_count: certification_counters
                .forbidden_exact_overclaim_count(),
            global_scan_debt_count: certification_counters.global_scan_debt_count(),
        })
    }

    pub fn certification_row_count(&self) -> u64 {
        self.certification_row_count
    }

    pub fn certification_index_probe_count(&self) -> u64 {
        self.certification_index_probe_count
    }

    pub fn certification_receipt_reuse_count(&self) -> u64 {
        self.certification_receipt_reuse_count
    }

    pub fn certification_allocation_count(&self) -> u64 {
        self.certification_allocation_count
    }

    pub fn generic_row_count(&self) -> u64 {
        self.generic_row_count
    }

    pub fn generic_index_probe_count(&self) -> u64 {
        self.generic_index_probe_count
    }

    pub fn generic_receipt_reuse_count(&self) -> u64 {
        self.generic_receipt_reuse_count
    }

    pub fn generic_allocation_count(&self) -> u64 {
        self.generic_allocation_count
    }

    pub fn domain_scenario_row_count(&self) -> u64 {
        self.domain_scenario_row_count
    }

    pub fn domain_index_probe_count(&self) -> u64 {
        self.domain_index_probe_count
    }

    pub fn domain_receipt_reuse_count(&self) -> u64 {
        self.domain_receipt_reuse_count
    }

    pub fn domain_allocation_count(&self) -> u64 {
        self.domain_allocation_count
    }

    pub fn forbidden_exact_overclaim_count(&self) -> u64 {
        self.forbidden_exact_overclaim_count
    }

    pub fn global_scan_debt_count(&self) -> u64 {
        self.global_scan_debt_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportAccuracyAccessCloseout {
    certified_semantic_domain_row_count: u64,
    explicit_advanced_family_debt_count: u64,
    roadmap2_physical_debt_explicit: bool,
    milestone15_extension_debt_explicit: bool,
    handoff_semantic_trust_closed: bool,
    persistence_posture: SubscriptionSupportAccuracyPersistencePosture,
}

impl SubscriptionSupportAccuracyAccessCloseout {
    fn from_phase_artifacts(
        domain_bundle: &SupportDomainCertificationBundle,
        handoff_report: &SupportCertificationHandoffReport,
        persistence_posture: SubscriptionSupportAccuracyPersistencePosture,
    ) -> Result<Self, SupportTrustFailure> {
        validate_handoff(handoff_report)?;
        let certified_semantic_domain_row_count = domain_bundle
            .counter_snapshot()
            .certified_semantic_row_count();
        let explicit_advanced_family_debt_count =
            domain_bundle.counter_snapshot().explicit_debt_row_count();
        let roadmap2_physical_debt_explicit = domain_bundle.rows().iter().any(|row| {
            row.required_future_milestone()
                == Some(super::domain_certification::SupportDomainCertificationDebtOwner::Roadmap2PhysicalDatabaseFoundation)
        });
        let milestone15_extension_debt_explicit = domain_bundle.rows().iter().any(|row| {
            row.required_future_milestone()
                == Some(super::domain_certification::SupportDomainCertificationDebtOwner::Milestone15ExtensionSupportRegistration)
        });
        if certified_semantic_domain_row_count == 0
            || explicit_advanced_family_debt_count != 2
            || !roadmap2_physical_debt_explicit
            || !milestone15_extension_debt_explicit
        {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "subscription-support accuracy closeout requires certified first-ship domain rows and explicit future-owned advanced-family debt",
            ));
        }
        Ok(Self {
            certified_semantic_domain_row_count,
            explicit_advanced_family_debt_count,
            roadmap2_physical_debt_explicit,
            milestone15_extension_debt_explicit,
            handoff_semantic_trust_closed: handoff_report.semantic_support_trust_closed(),
            persistence_posture,
        })
    }

    pub fn certified_semantic_domain_row_count(&self) -> u64 {
        self.certified_semantic_domain_row_count
    }

    pub fn explicit_advanced_family_debt_count(&self) -> u64 {
        self.explicit_advanced_family_debt_count
    }

    pub fn roadmap2_physical_debt_explicit(&self) -> bool {
        self.roadmap2_physical_debt_explicit
    }

    pub fn milestone15_extension_debt_explicit(&self) -> bool {
        self.milestone15_extension_debt_explicit
    }

    pub fn handoff_semantic_trust_closed(&self) -> bool {
        self.handoff_semantic_trust_closed
    }

    pub fn persistence_posture(&self) -> SubscriptionSupportAccuracyPersistencePosture {
        self.persistence_posture
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportAccuracyCertificationRun {
    suite: SubscriptionSupportAccuracyCertificationSuite,
    performance_closeout: SubscriptionSupportAccuracyPerformanceCloseout,
    access_closeout: SubscriptionSupportAccuracyAccessCloseout,
    persistence_posture: SubscriptionSupportAccuracyPersistencePosture,
    run_digest: String,
}

impl SubscriptionSupportAccuracyCertificationRun {
    pub fn suite(&self) -> &SubscriptionSupportAccuracyCertificationSuite {
        &self.suite
    }

    pub fn performance_closeout(&self) -> &SubscriptionSupportAccuracyPerformanceCloseout {
        &self.performance_closeout
    }

    pub fn access_closeout(&self) -> &SubscriptionSupportAccuracyAccessCloseout {
        &self.access_closeout
    }

    pub fn persistence_posture(&self) -> SubscriptionSupportAccuracyPersistencePosture {
        self.persistence_posture
    }

    pub fn run_digest(&self) -> &str {
        &self.run_digest
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportAccuracyCertificationRunner {
    persistence_posture: SubscriptionSupportAccuracyPersistencePosture,
}

impl Default for SubscriptionSupportAccuracyCertificationRunner {
    fn default() -> Self {
        Self::production()
    }
}

impl SubscriptionSupportAccuracyCertificationRunner {
    pub fn production() -> Self {
        Self {
            persistence_posture:
                SubscriptionSupportAccuracyPersistencePosture::InMemoryCertificationOnly,
        }
    }

    pub fn certify(
        &self,
        evidence_bundle: &SupportCertificationEvidenceBundle,
        generic_report: &SupportGenericCertificationReport,
        domain_bundle: &SupportDomainCertificationBundle,
        handoff_report: &SupportCertificationHandoffReport,
        lane_evidence: &SubscriptionSupportAccuracyLaneEvidenceSet,
    ) -> Result<SubscriptionSupportAccuracyCertificationRun, SupportTrustFailure> {
        let suite =
            SubscriptionSupportAccuracyCertificationSuite::from_phase_artifacts_and_lane_evidence(
                evidence_bundle,
                generic_report,
                domain_bundle,
                handoff_report,
                lane_evidence,
            )?;
        let performance_closeout =
            SubscriptionSupportAccuracyPerformanceCloseout::from_phase_artifacts(
                evidence_bundle,
                generic_report,
                domain_bundle,
            )?;
        let access_closeout = SubscriptionSupportAccuracyAccessCloseout::from_phase_artifacts(
            domain_bundle,
            handoff_report,
            self.persistence_posture,
        )?;
        let run_digest = stable_digest(&SubscriptionSupportAccuracyCertificationRunDigestBasis {
            suite_digest: suite.suite_digest(),
            performance_closeout: &performance_closeout,
            access_closeout: &access_closeout,
            persistence_posture: self.persistence_posture,
        })?;
        Ok(SubscriptionSupportAccuracyCertificationRun {
            suite,
            performance_closeout,
            access_closeout,
            persistence_posture: self.persistence_posture,
            run_digest,
        })
    }
}

impl SubscriptionSupportAccuracyCertificationSuite {
    pub fn from_phase_artifacts_and_lane_evidence(
        evidence_bundle: &SupportCertificationEvidenceBundle,
        generic_report: &SupportGenericCertificationReport,
        domain_bundle: &SupportDomainCertificationBundle,
        handoff_report: &SupportCertificationHandoffReport,
        lane_evidence: &SubscriptionSupportAccuracyLaneEvidenceSet,
    ) -> Result<Self, SupportTrustFailure> {
        let rows = build_required_rows_from_phase_artifacts(
            evidence_bundle,
            generic_report,
            domain_bundle,
            handoff_report,
            lane_evidence,
        )?;
        Self::from_rows_and_phase_artifacts(
            rows,
            evidence_bundle,
            generic_report,
            domain_bundle,
            handoff_report,
            lane_evidence,
        )
    }

    pub(crate) fn from_rows_and_phase_artifacts(
        mut rows: Vec<SubscriptionSupportAccuracyCertificationRow>,
        evidence_bundle: &SupportCertificationEvidenceBundle,
        generic_report: &SupportGenericCertificationReport,
        domain_bundle: &SupportDomainCertificationBundle,
        handoff_report: &SupportCertificationHandoffReport,
        lane_evidence: &SubscriptionSupportAccuracyLaneEvidenceSet,
    ) -> Result<Self, SupportTrustFailure> {
        validate_handoff(handoff_report)?;
        validate_handoff_matches_phase_artifacts(generic_report, domain_bundle, handoff_report)?;
        rows.sort_by_key(SubscriptionSupportAccuracyCertificationRow::row_kind);
        validate_required_rows(&rows)?;
        validate_rows_match_phase_artifacts(
            &rows,
            evidence_bundle,
            generic_report,
            domain_bundle,
            handoff_report,
            lane_evidence,
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
    lane_evidence: &SubscriptionSupportAccuracyLaneEvidenceSet,
) -> Result<Vec<SubscriptionSupportAccuracyCertificationRow>, SupportTrustFailure> {
    expected_row_evidence_digests(
        evidence_bundle,
        generic_report,
        domain_bundle,
        handoff_report,
        lane_evidence,
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
    lane_evidence: &SubscriptionSupportAccuracyLaneEvidenceSet,
) -> Result<(), SupportTrustFailure> {
    let expected = expected_row_evidence_digests(
        evidence_bundle,
        generic_report,
        domain_bundle,
        handoff_report,
        lane_evidence,
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
    lane_evidence: &SubscriptionSupportAccuracyLaneEvidenceSet,
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
        let lane = lane_evidence.evidence_for(*row_kind).ok_or_else(|| {
            SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "subscription-support accuracy suite requires explicit lane evidence for hostile rows",
            )
        })?;
        insert_expected(
            &mut digests,
            *row_kind,
            row_kind.evidence_lane_label(),
            evidence_bundle,
            generic_report,
            domain_bundle,
            handoff_report,
            Some(lane.evidence_digest()),
            Some(lane_evidence.lane_evidence_set_digest()),
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

#[derive(Serialize)]
struct SubscriptionSupportAccuracyLaneEvidenceDigestBasis<'a> {
    row_kind: SubscriptionSupportAccuracyCertificationRowKind,
    outcome: SubscriptionSupportAccuracyLaneOutcome,
    failure_kind: Option<SupportTrustFailureKind>,
    recovery_posture: Option<SupportTrustRecoveryPosture>,
    source_digest: &'a str,
    diagnostics_digest: &'a str,
    counter_digest: &'a str,
}

#[derive(Serialize)]
struct SubscriptionSupportAccuracyCertifiedReportLaneDigestBasis<'a> {
    row_kind: SubscriptionSupportAccuracyCertificationRowKind,
    trust_class: SupportTrustClass,
    trust_strength: SupportTrustStrength,
    provenance: SupportTrustProvenance,
    suite_version: &'a str,
    row_id: &'a str,
    evidence_bundle_digest: &'a str,
}

#[derive(Serialize)]
struct SubscriptionSupportAccuracyCounterLaneDigestBasis<'a> {
    row_kind: SubscriptionSupportAccuracyCertificationRowKind,
    evidence_bundle_digest: &'a str,
    forbidden_exact_overclaim_count: u64,
    global_scan_debt_count: u64,
}

#[derive(Serialize)]
struct SubscriptionSupportAccuracyFailureLaneDigestBasis<'a> {
    row_kind: SubscriptionSupportAccuracyCertificationRowKind,
    digest_role: &'static str,
    failure: &'a SupportTrustFailure,
}

#[derive(Serialize)]
struct SubscriptionSupportAccuracyCertificationRunDigestBasis<'a> {
    suite_digest: &'a str,
    performance_closeout: &'a SubscriptionSupportAccuracyPerformanceCloseout,
    access_closeout: &'a SubscriptionSupportAccuracyAccessCloseout,
    persistence_posture: SubscriptionSupportAccuracyPersistencePosture,
}

#[derive(Serialize)]
struct SubscriptionSupportAccuracyLaneEvidenceSetDigestBasis<'a> {
    lane_digests: &'a [&'a str],
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

fn validate_required_lane_evidence(
    lanes: &[SubscriptionSupportAccuracyLaneEvidence],
) -> Result<(), SupportTrustFailure> {
    let mut seen = BTreeSet::new();
    for lane in lanes {
        if !requires_explicit_lane_evidence(lane.row_kind()) || !seen.insert(lane.row_kind()) {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "subscription-support accuracy lane evidence must be required and unique",
            ));
        }
    }
    for row_kind in SubscriptionSupportAccuracyCertificationRowKind::required()
        .iter()
        .copied()
        .filter(|row_kind| requires_explicit_lane_evidence(*row_kind))
    {
        if !seen.contains(&row_kind) {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "subscription-support accuracy suite is missing required hostile lane evidence",
            ));
        }
    }
    Ok(())
}

fn requires_explicit_lane_evidence(
    row_kind: SubscriptionSupportAccuracyCertificationRowKind,
) -> bool {
    !matches!(
        row_kind,
        SubscriptionSupportAccuracyCertificationRowKind::ExactSupportTrustedControl
            | SubscriptionSupportAccuracyCertificationRowKind::DegradedSupportTrusted
            | SubscriptionSupportAccuracyCertificationRowKind::CertificationMatrixComplete
            | SubscriptionSupportAccuracyCertificationRowKind::GenericCertificationIncludesSupportTrust
            | SubscriptionSupportAccuracyCertificationRowKind::DomainGeometrySupportTrust
            | SubscriptionSupportAccuracyCertificationRowKind::DomainWebDataSupportTrust
            | SubscriptionSupportAccuracyCertificationRowKind::DomainAiDegradedSupportTrust
            | SubscriptionSupportAccuracyCertificationRowKind::DomainChipRebuildSupportTrust
            | SubscriptionSupportAccuracyCertificationRowKind::DomainOfflineOmittedSupportTrust
            | SubscriptionSupportAccuracyCertificationRowKind::Roadmap2HandoffPhysicalDebtExplicit
    )
}

fn validate_lane_outcome(
    row_kind: SubscriptionSupportAccuracyCertificationRowKind,
    outcome: SubscriptionSupportAccuracyLaneOutcome,
    failure_kind: Option<SupportTrustFailureKind>,
) -> Result<(), SupportTrustFailure> {
    if !requires_explicit_lane_evidence(row_kind) {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "artifact-bound suite rows cannot be represented as hostile lane evidence",
        ));
    }
    let expected_outcome = expected_lane_outcome(row_kind);
    let expected_failure_kind = expected_lane_failure_kind(row_kind);
    if outcome != expected_outcome || failure_kind != expected_failure_kind {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "subscription-support accuracy lane evidence outcome does not match the required row kind",
        ));
    }
    Ok(())
}

fn validate_certified_report_lane(
    row_kind: SubscriptionSupportAccuracyCertificationRowKind,
    report: &CertifiedSupportTrustReport,
) -> Result<(), SupportTrustFailure> {
    let matches_row = match row_kind {
        SubscriptionSupportAccuracyCertificationRowKind::RebuildDerivedSupportExactEquivalence => {
            report.trust_strength() == SupportTrustStrength::Exact
                && report.provenance() == SupportTrustProvenance::Rebuilt
        }
        SubscriptionSupportAccuracyCertificationRowKind::ReplicatedSupportExactEquivalence => {
            report.trust_class() == SupportTrustClass::ReplicatedSupportTrusted
                && report.trust_strength() == SupportTrustStrength::Exact
                && report.provenance() == SupportTrustProvenance::Replicated
        }
        SubscriptionSupportAccuracyCertificationRowKind::MigratedSupportExactEquivalence => {
            report.trust_class() == SupportTrustClass::MigratedSupportTrusted
                && report.trust_strength() == SupportTrustStrength::Exact
                && report.provenance() == SupportTrustProvenance::Migrated
        }
        _ => false,
    };
    if !matches_row {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "subscription-support accuracy certified pass lane must match its certified support report posture",
        ));
    }
    Ok(())
}

fn validate_zero_counter_lane(
    row_kind: SubscriptionSupportAccuracyCertificationRowKind,
    evidence_bundle: &SupportCertificationEvidenceBundle,
) -> Result<(), SupportTrustFailure> {
    if !matches!(
        row_kind,
        SubscriptionSupportAccuracyCertificationRowKind::ForbiddenExactOverclaimZero
            | SubscriptionSupportAccuracyCertificationRowKind::GlobalScanDebtForbidden
    ) {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "subscription-support accuracy counter pass lanes must be counter-debt rows",
        ));
    }
    if evidence_bundle
        .counter_snapshot()
        .forbidden_exact_overclaim_count()
        != 0
        || evidence_bundle.counter_snapshot().global_scan_debt_count() != 0
    {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim,
            SupportTrustRecoveryPosture::RerunCertification,
            "subscription-support accuracy counter pass lanes require zero exact-overclaim and global-scan debt",
        ));
    }
    Ok(())
}

fn validate_certification_performance(
    evidence_bundle: &SupportCertificationEvidenceBundle,
) -> Result<(), SupportTrustFailure> {
    let batch_scope = evidence_bundle.batch_scope();
    let counters = evidence_bundle.counter_snapshot();
    let valid_scope = batch_scope.scope_kind()
        == SupportCertificationBatchScopeKind::CertificationScopeLocal
        && batch_scope.density_class() == SupportTrustDensityClass::CertificationScopeLocal
        && batch_scope.path_class() == SupportTrustPathClass::BatchCertificationPath
        && batch_scope.allocation_scope() == SupportTrustAllocationScope::BatchCertification;
    if !valid_scope
        || batch_scope.row_count() != 4
        || batch_scope.expected_index_probes() != 4
        || batch_scope.expected_receipt_reuse_count() != 3
        || batch_scope.expected_allocation_count() != 1
        || counters.coverage_row_count() != batch_scope.row_count()
        || counters.index_probe_count() != batch_scope.expected_index_probes()
        || counters.receipt_reuse_count() != batch_scope.expected_receipt_reuse_count()
        || counters.allocation_count() != batch_scope.expected_allocation_count()
        || counters.forbidden_exact_overclaim_count() != 0
        || counters.global_scan_debt_count() != 0
    {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "subscription-support accuracy closeout requires exact certification performance counters and bounded batch access",
        ));
    }
    Ok(())
}

fn validate_generic_performance(
    generic_report: &SupportGenericCertificationReport,
) -> Result<(), SupportTrustFailure> {
    let counters = generic_report.counter_snapshot();
    if counters.certified_support_report_count() != 1
        || counters.generic_row_count() != 1
        || counters.index_probe_count() != 1
        || counters.receipt_reuse_count() != 1
        || counters.allocation_count() != 1
        || counters.physical_readiness_debt_count() != 1
    {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "subscription-support accuracy closeout requires exact generic certification counters and explicit physical-readiness debt",
        ));
    }
    Ok(())
}

fn validate_domain_performance(
    domain_bundle: &SupportDomainCertificationBundle,
) -> Result<(), SupportTrustFailure> {
    let counters = domain_bundle.counter_snapshot();
    if counters.scenario_row_count() != 5
        || counters.certified_semantic_row_count() != 3
        || counters.explicit_debt_row_count() != 2
        || counters.index_probe_count() != 5
        || counters.receipt_reuse_count() != 4
        || counters.allocation_count() != 1
        || counters.physical_readiness_debt_count() != counters.explicit_debt_row_count()
    {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "subscription-support accuracy closeout requires exact domain scenario counters and future-owned debt rows",
        ));
    }
    Ok(())
}

fn expected_lane_outcome(
    row_kind: SubscriptionSupportAccuracyCertificationRowKind,
) -> SubscriptionSupportAccuracyLaneOutcome {
    match row_kind {
        SubscriptionSupportAccuracyCertificationRowKind::RebuildDerivedSupportExactEquivalence
        | SubscriptionSupportAccuracyCertificationRowKind::ReplicatedSupportExactEquivalence
        | SubscriptionSupportAccuracyCertificationRowKind::MigratedSupportExactEquivalence
        | SubscriptionSupportAccuracyCertificationRowKind::ForbiddenExactOverclaimZero
        | SubscriptionSupportAccuracyCertificationRowKind::GlobalScanDebtForbidden => {
            SubscriptionSupportAccuracyLaneOutcome::CertifiedPass
        }
        _ => SubscriptionSupportAccuracyLaneOutcome::TypedRejection,
    }
}

fn expected_lane_failure_kind(
    row_kind: SubscriptionSupportAccuracyCertificationRowKind,
) -> Option<SupportTrustFailureKind> {
    match row_kind {
        SubscriptionSupportAccuracyCertificationRowKind::RebuildDerivedSupportExactEquivalence
        | SubscriptionSupportAccuracyCertificationRowKind::ReplicatedSupportExactEquivalence
        | SubscriptionSupportAccuracyCertificationRowKind::MigratedSupportExactEquivalence
        | SubscriptionSupportAccuracyCertificationRowKind::ForbiddenExactOverclaimZero
        | SubscriptionSupportAccuracyCertificationRowKind::GlobalScanDebtForbidden => None,
        SubscriptionSupportAccuracyCertificationRowKind::RebuildDerivedSupportDowngraded
        | SubscriptionSupportAccuracyCertificationRowKind::ReplicatedSupportIdentityNotEnough => {
            Some(SupportTrustFailureKind::SupportTrustEquivalenceMissing)
        }
        SubscriptionSupportAccuracyCertificationRowKind::ImportedSupportMissingBasisNotResumable => {
            Some(SupportTrustFailureKind::SupportTrustBasisMismatch)
        }
        SubscriptionSupportAccuracyCertificationRowKind::StaleSupportRejected => {
            Some(SupportTrustFailureKind::SupportTrustEpochExpired)
        }
        SubscriptionSupportAccuracyCertificationRowKind::PolicyRejectedSupport
        | SubscriptionSupportAccuracyCertificationRowKind::OperationalVerdictDriftRejectsExactTrust => {
            Some(SupportTrustFailureKind::SupportTrustOperationalVerdictMismatch)
        }
        SubscriptionSupportAccuracyCertificationRowKind::FamilyRoleMismatchRejected => {
            Some(SupportTrustFailureKind::SupportTrustRoleMismatch)
        }
        SubscriptionSupportAccuracyCertificationRowKind::CompatibilityDriftRejectsExactTrust => {
            Some(SupportTrustFailureKind::SupportTrustCompatibilityMismatch)
        }
        SubscriptionSupportAccuracyCertificationRowKind::PortabilityDriftRejectsExactTrust => {
            Some(SupportTrustFailureKind::SupportTrustPortabilityMismatch)
        }
        SubscriptionSupportAccuracyCertificationRowKind::CoverageDriftRejectsPlatformTrust
        | SubscriptionSupportAccuracyCertificationRowKind::CertificationMissingRowRejected
        | SubscriptionSupportAccuracyCertificationRowKind::CertificationDuplicateRowRejected
        | SubscriptionSupportAccuracyCertificationRowKind::CertificationMislabeledRowRejected
        | SubscriptionSupportAccuracyCertificationRowKind::CertificationSelfComparisonRejected => {
            Some(SupportTrustFailureKind::SupportTrustCoverageMissing)
        }
        SubscriptionSupportAccuracyCertificationRowKind::MultiDriftPrecedenceDeterministic => {
            Some(SupportTrustFailureKind::SupportTrustBasisMismatch)
        }
        _ => None,
    }
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

fn validate_handoff_matches_phase_artifacts(
    generic_report: &SupportGenericCertificationReport,
    domain_bundle: &SupportDomainCertificationBundle,
    handoff_report: &SupportCertificationHandoffReport,
) -> Result<(), SupportTrustFailure> {
    if handoff_report.generic_certification_digest()
        != generic_report.generic_certification_digest()
        || handoff_report.domain_certification_digest()
            != domain_bundle.domain_certification_digest()
    {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "subscription-support accuracy handoff must be bound to the supplied generic and domain certification artifacts",
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
