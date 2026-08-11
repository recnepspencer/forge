use super::super::certification::SupportCertificationEvidenceBundle;
use super::super::domain_certification::{
    SupportCertificationHandoffReport, SupportDomainCertificationBundle,
    SupportDomainCertificationScenario, SupportGenericCertificationReport,
};
use super::super::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use super::digest::stable_digest;
use super::lane_evidence_set::SubscriptionSupportAccuracyLaneEvidenceSet;
use super::row_kind::SubscriptionSupportAccuracyCertificationRowKind;
use serde::Serialize;
use std::collections::BTreeMap;

pub(super) fn expected_row_evidence_digests(
    evidence_bundle: &SupportCertificationEvidenceBundle,
    generic_report: &SupportGenericCertificationReport,
    domain_bundle: &SupportDomainCertificationBundle,
    handoff_report: &SupportCertificationHandoffReport,
    lane_evidence: &SubscriptionSupportAccuracyLaneEvidenceSet,
) -> Result<BTreeMap<SubscriptionSupportAccuracyCertificationRowKind, String>, SupportTrustFailure>
{
    ExpectedRowEvidenceSources::new(
        evidence_bundle,
        generic_report,
        domain_bundle,
        handoff_report,
        lane_evidence,
    )
    .build()
}

struct ExpectedRowEvidenceSources<'a> {
    evidence_bundle: &'a SupportCertificationEvidenceBundle,
    generic_report: &'a SupportGenericCertificationReport,
    domain_bundle: &'a SupportDomainCertificationBundle,
    handoff_report: &'a SupportCertificationHandoffReport,
    lane_evidence: &'a SubscriptionSupportAccuracyLaneEvidenceSet,
    coverage_rows: BTreeMap<String, String>,
    domain_rows: BTreeMap<SupportDomainCertificationScenario, String>,
}

impl<'a> ExpectedRowEvidenceSources<'a> {
    fn new(
        evidence_bundle: &'a SupportCertificationEvidenceBundle,
        generic_report: &'a SupportGenericCertificationReport,
        domain_bundle: &'a SupportDomainCertificationBundle,
        handoff_report: &'a SupportCertificationHandoffReport,
        lane_evidence: &'a SubscriptionSupportAccuracyLaneEvidenceSet,
    ) -> Self {
        let coverage_rows = evidence_bundle
            .coverage_rows()
            .iter()
            .map(|row| {
                (
                    row.evidence().row_id().to_string(),
                    row.evidence().declared_row_digest().to_string(),
                )
            })
            .collect();
        let domain_rows = domain_bundle
            .rows()
            .iter()
            .map(|row| (row.scenario(), row.row_digest().to_string()))
            .collect();
        Self {
            evidence_bundle,
            generic_report,
            domain_bundle,
            handoff_report,
            lane_evidence,
            coverage_rows,
            domain_rows,
        }
    }

    fn build(
        &self,
    ) -> Result<
        BTreeMap<SubscriptionSupportAccuracyCertificationRowKind, String>,
        SupportTrustFailure,
    > {
        let mut digests = BTreeMap::new();
        self.insert_coverage_control_rows(&mut digests)?;
        self.insert_certification_matrix_row(&mut digests)?;
        self.insert_generic_certification_row(&mut digests)?;
        self.insert_domain_scenario_rows(&mut digests)?;
        self.insert_handoff_row(&mut digests)?;
        self.insert_certified_report_translation_rows(&mut digests)?;
        self.insert_compatibility_retention_maintenance_rows(&mut digests)?;
        self.insert_certification_coverage_rows(&mut digests)?;
        self.insert_counter_rows(&mut digests)?;
        Ok(digests)
    }

    fn insert_coverage_control_rows(
        &self,
        digests: &mut BTreeMap<SubscriptionSupportAccuracyCertificationRowKind, String>,
    ) -> Result<(), SupportTrustFailure> {
        self.insert_expected(
            digests,
            SubscriptionSupportAccuracyCertificationRowKind::ExactSupportTrustedControl,
            "certification-row",
            Some(self.required_coverage_row_digest("row:basis-bound-exact")?),
            None,
        )?;
        self.insert_expected(
            digests,
            SubscriptionSupportAccuracyCertificationRowKind::DegradedSupportTrusted,
            "certification-row",
            Some(self.required_coverage_row_digest("row:degraded-continuation")?),
            None,
        )
    }

    fn insert_certification_matrix_row(
        &self,
        digests: &mut BTreeMap<SubscriptionSupportAccuracyCertificationRowKind, String>,
    ) -> Result<(), SupportTrustFailure> {
        self.insert_expected(
            digests,
            SubscriptionSupportAccuracyCertificationRowKind::CertificationMatrixComplete,
            "coverage-matrix",
            Some(self.evidence_bundle.certification_summary_digest()),
            None,
        )
    }

    fn insert_generic_certification_row(
        &self,
        digests: &mut BTreeMap<SubscriptionSupportAccuracyCertificationRowKind, String>,
    ) -> Result<(), SupportTrustFailure> {
        self.insert_expected(
            digests,
            SubscriptionSupportAccuracyCertificationRowKind::GenericCertificationIncludesSupportTrust,
            "generic-certification",
            Some(self.generic_report.generic_certification_digest()),
            None,
        )
    }

    fn insert_domain_scenario_rows(
        &self,
        digests: &mut BTreeMap<SubscriptionSupportAccuracyCertificationRowKind, String>,
    ) -> Result<(), SupportTrustFailure> {
        for (row_kind, scenario) in domain_scenarios() {
            self.insert_expected(
                digests,
                row_kind,
                "domain-scenario",
                Some(self.required_domain_row_digest(scenario)?),
                None,
            )?;
        }
        Ok(())
    }

    fn insert_handoff_row(
        &self,
        digests: &mut BTreeMap<SubscriptionSupportAccuracyCertificationRowKind, String>,
    ) -> Result<(), SupportTrustFailure> {
        self.insert_expected(
            digests,
            SubscriptionSupportAccuracyCertificationRowKind::Roadmap2HandoffPhysicalDebtExplicit,
            "handoff",
            Some(self.handoff_report.handoff_digest()),
            None,
        )
    }

    fn insert_certified_report_translation_rows(
        &self,
        digests: &mut BTreeMap<SubscriptionSupportAccuracyCertificationRowKind, String>,
    ) -> Result<(), SupportTrustFailure> {
        self.insert_lane_rows(
            digests,
            &[
                SubscriptionSupportAccuracyCertificationRowKind::RebuildDerivedSupportExactEquivalence,
                SubscriptionSupportAccuracyCertificationRowKind::RebuildDerivedSupportDowngraded,
                SubscriptionSupportAccuracyCertificationRowKind::ReplicatedSupportIdentityNotEnough,
                SubscriptionSupportAccuracyCertificationRowKind::ReplicatedSupportExactEquivalence,
                SubscriptionSupportAccuracyCertificationRowKind::MigratedSupportExactEquivalence,
                SubscriptionSupportAccuracyCertificationRowKind::ImportedSupportMissingBasisNotResumable,
            ],
        )
    }

    fn insert_compatibility_retention_maintenance_rows(
        &self,
        digests: &mut BTreeMap<SubscriptionSupportAccuracyCertificationRowKind, String>,
    ) -> Result<(), SupportTrustFailure> {
        self.insert_lane_rows(
            digests,
            &[
                SubscriptionSupportAccuracyCertificationRowKind::StaleSupportRejected,
                SubscriptionSupportAccuracyCertificationRowKind::PolicyRejectedSupport,
                SubscriptionSupportAccuracyCertificationRowKind::FamilyRoleMismatchRejected,
                SubscriptionSupportAccuracyCertificationRowKind::CompatibilityDriftRejectsExactTrust,
                SubscriptionSupportAccuracyCertificationRowKind::OperationalVerdictDriftRejectsExactTrust,
                SubscriptionSupportAccuracyCertificationRowKind::PortabilityDriftRejectsExactTrust,
                SubscriptionSupportAccuracyCertificationRowKind::CoverageDriftRejectsPlatformTrust,
                SubscriptionSupportAccuracyCertificationRowKind::MultiDriftPrecedenceDeterministic,
            ],
        )
    }

    fn insert_certification_coverage_rows(
        &self,
        digests: &mut BTreeMap<SubscriptionSupportAccuracyCertificationRowKind, String>,
    ) -> Result<(), SupportTrustFailure> {
        self.insert_lane_rows(
            digests,
            &[
                SubscriptionSupportAccuracyCertificationRowKind::CertificationMissingRowRejected,
                SubscriptionSupportAccuracyCertificationRowKind::CertificationDuplicateRowRejected,
                SubscriptionSupportAccuracyCertificationRowKind::CertificationMislabeledRowRejected,
                SubscriptionSupportAccuracyCertificationRowKind::CertificationSelfComparisonRejected,
            ],
        )
    }

    fn insert_counter_rows(
        &self,
        digests: &mut BTreeMap<SubscriptionSupportAccuracyCertificationRowKind, String>,
    ) -> Result<(), SupportTrustFailure> {
        self.insert_lane_rows(
            digests,
            &[
                SubscriptionSupportAccuracyCertificationRowKind::ForbiddenExactOverclaimZero,
                SubscriptionSupportAccuracyCertificationRowKind::GlobalScanDebtForbidden,
            ],
        )
    }

    fn insert_lane_rows(
        &self,
        digests: &mut BTreeMap<SubscriptionSupportAccuracyCertificationRowKind, String>,
        row_kinds: &[SubscriptionSupportAccuracyCertificationRowKind],
    ) -> Result<(), SupportTrustFailure> {
        for row_kind in row_kinds {
            self.insert_lane_expected_row(digests, *row_kind)?;
        }
        Ok(())
    }

    fn insert_lane_expected_row(
        &self,
        digests: &mut BTreeMap<SubscriptionSupportAccuracyCertificationRowKind, String>,
        row_kind: SubscriptionSupportAccuracyCertificationRowKind,
    ) -> Result<(), SupportTrustFailure> {
        let lane = self.lane_evidence.evidence_for(row_kind).ok_or_else(|| {
            SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "subscription-support accuracy suite requires explicit lane evidence for hostile rows",
            )
        })?;
        self.insert_expected(
            digests,
            row_kind,
            row_kind.evidence_lane_label(),
            Some(lane.evidence_digest()),
            Some(self.lane_evidence.lane_evidence_set_digest()),
        )
    }

    fn insert_expected(
        &self,
        digests: &mut BTreeMap<SubscriptionSupportAccuracyCertificationRowKind, String>,
        row_kind: SubscriptionSupportAccuracyCertificationRowKind,
        source_label: &'static str,
        primary_source_digest: Option<&str>,
        hostile_source_digest: Option<&str>,
    ) -> Result<(), SupportTrustFailure> {
        let digest = stable_digest(&SubscriptionSupportAccuracyRowEvidenceDigestBasis {
            row_kind,
            source_label,
            evidence_bundle_digest: self.evidence_bundle.evidence_bundle_digest(),
            artifact_digest: self.evidence_bundle.artifact_digest(),
            subscription_support_digest: self.evidence_bundle.subscription_support_digest(),
            diagnostics_digest: self.evidence_bundle.diagnostics_digest(),
            counter_snapshot_digest: self.evidence_bundle.counter_snapshot_digest(),
            certification_summary_digest: self.evidence_bundle.certification_summary_digest(),
            generic_certification_digest: self.generic_report.generic_certification_digest(),
            domain_certification_digest: self.domain_bundle.domain_certification_digest(),
            handoff_digest: self.handoff_report.handoff_digest(),
            primary_source_digest,
            hostile_source_digest,
        })?;
        digests.insert(row_kind, digest);
        Ok(())
    }

    fn required_coverage_row_digest(
        &self,
        row_id: &'static str,
    ) -> Result<&str, SupportTrustFailure> {
        self.coverage_rows
            .get(row_id)
            .map(String::as_str)
            .ok_or_else(|| {
                SupportTrustFailure::new(
                    SupportTrustFailureKind::SupportTrustCoverageMissing,
                    SupportTrustRecoveryPosture::RerunCertification,
                    format!(
                        "subscription-support accuracy suite requires certification row {row_id}"
                    ),
                )
            })
    }

    fn required_domain_row_digest(
        &self,
        scenario: SupportDomainCertificationScenario,
    ) -> Result<&str, SupportTrustFailure> {
        self.domain_rows
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
}

fn domain_scenarios() -> [(
    SubscriptionSupportAccuracyCertificationRowKind,
    SupportDomainCertificationScenario,
); 5] {
    [
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
    ]
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
