use super::super::certification::SupportCertificationEvidenceBundle;
use super::super::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use super::super::reports::CertifiedSupportTrustReport;
use super::super::taxonomy::{SupportTrustClass, SupportTrustProvenance, SupportTrustStrength};
use super::lane_evidence::SubscriptionSupportAccuracyLaneOutcome;
use super::row_kind::SubscriptionSupportAccuracyCertificationRowKind;
use std::collections::BTreeSet;

pub(super) fn validate_required_lane_evidence(
    lanes: &[super::lane_evidence::SubscriptionSupportAccuracyLaneEvidence],
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

pub(super) fn validate_lane_outcome(
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

pub(super) fn validate_certified_report_lane(
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

pub(super) fn validate_zero_counter_lane(
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
