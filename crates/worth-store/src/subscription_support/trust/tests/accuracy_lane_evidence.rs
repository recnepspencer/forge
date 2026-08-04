use super::super::{
    CertifiedSupportTrustReport, ExactSupportTrustWitness, OperationalSupportTrustReport,
    SubscriptionSupportAccuracyCertificationRow, SubscriptionSupportAccuracyCertificationRowKind,
    SubscriptionSupportAccuracyCertificationSuite, SubscriptionSupportAccuracyLaneEvidence,
    SubscriptionSupportAccuracyLaneEvidenceSet, SupportCertificationCorpusVersion,
    SupportCertificationEpoch, SupportCertificationEvidenceBundle,
    SupportCertificationHandoffReport, SupportDomainCertificationBundle,
    SupportGenericCertificationReport, SupportTrustCertificationStamp,
    SupportTrustEquivalenceWitness, SupportTrustFreshnessWitness, SupportTrustProvenance,
    SupportTrustStrength,
};
use super::accuracy_failure_evidence::phase7_expected_lane_failure;
use super::domain_handoff::phase7_suite_artifacts;
use super::operational_basis::epochs;
use super::operational_classification::exact_translation;
use crate::subscription_support::{
    SubscriptionSupportFamilyId, SubscriptionSupportOperationalVerdict,
};

pub(super) fn phase7_required_suite_rows(
    evidence_bundle: &SupportCertificationEvidenceBundle,
    generic: &SupportGenericCertificationReport,
    domain: &SupportDomainCertificationBundle,
    handoff: &SupportCertificationHandoffReport,
    lane_evidence: &SubscriptionSupportAccuracyLaneEvidenceSet,
) -> Vec<SubscriptionSupportAccuracyCertificationRow> {
    SubscriptionSupportAccuracyCertificationSuite::from_phase_artifacts_and_lane_evidence(
        evidence_bundle,
        generic,
        domain,
        handoff,
        lane_evidence,
    )
    .unwrap()
    .rows()
    .to_vec()
}

pub(super) fn phase7_lane_evidence() -> SubscriptionSupportAccuracyLaneEvidenceSet {
    let lanes = SubscriptionSupportAccuracyCertificationRowKind::required()
        .iter()
        .copied()
        .filter(|row_kind| {
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
        })
        .map(|row_kind| {
            let diagnostics_digest = format!("phase7:lane:diagnostics:{row_kind:?}");
            let counter_digest = format!("phase7:lane:counter:{row_kind:?}");
            match row_kind {
                SubscriptionSupportAccuracyCertificationRowKind::RebuildDerivedSupportExactEquivalence => {
                    let report = phase7_certified_transformed_exact_report(
                        SupportTrustProvenance::Rebuilt,
                        "row:phase7:rebuild-exact",
                    );
                    SubscriptionSupportAccuracyLaneEvidence::certified_pass_from_report(
                        row_kind,
                        &report,
                        diagnostics_digest,
                        counter_digest,
                    )
                }
                SubscriptionSupportAccuracyCertificationRowKind::ReplicatedSupportExactEquivalence => {
                    let report = phase7_certified_transformed_exact_report(
                        SupportTrustProvenance::Replicated,
                        "row:phase7:replicated-exact",
                    );
                    SubscriptionSupportAccuracyLaneEvidence::certified_pass_from_report(
                        row_kind,
                        &report,
                        diagnostics_digest,
                        counter_digest,
                    )
                }
                SubscriptionSupportAccuracyCertificationRowKind::MigratedSupportExactEquivalence => {
                    let report = phase7_certified_transformed_exact_report(
                        SupportTrustProvenance::Migrated,
                        "row:phase7:migrated-exact",
                    );
                    SubscriptionSupportAccuracyLaneEvidence::certified_pass_from_report(
                        row_kind,
                        &report,
                        diagnostics_digest,
                        counter_digest,
                    )
                }
                SubscriptionSupportAccuracyCertificationRowKind::ForbiddenExactOverclaimZero
                | SubscriptionSupportAccuracyCertificationRowKind::GlobalScanDebtForbidden => {
                    let (evidence_bundle, _, _, _) = phase7_suite_artifacts();
                    SubscriptionSupportAccuracyLaneEvidence::certified_counter_pass_from_evidence_bundle(
                        row_kind,
                        &evidence_bundle,
                    )
                }
                _ => {
                    let failure = phase7_expected_lane_failure(row_kind);
                    SubscriptionSupportAccuracyLaneEvidence::typed_rejection_from_failure(
                        row_kind, &failure,
                    )
                }
            }
            .unwrap()
        })
        .collect();
    SubscriptionSupportAccuracyLaneEvidenceSet::new(lanes).unwrap()
}

pub(super) fn phase7_certified_transformed_exact_report(
    provenance: SupportTrustProvenance,
    row_id: &str,
) -> CertifiedSupportTrustReport {
    let translation = exact_translation();
    let source_basis = translation.basis().clone();
    let family_id = source_basis.family_id().clone();
    let support_role = source_basis.support_role();
    let witness = ExactSupportTrustWitness::from_equivalent_operational_basis(
        translation,
        provenance,
        SupportTrustFreshnessWitness::new(epochs()),
        SupportTrustEquivalenceWitness::new(
            source_basis,
            SubscriptionSupportFamilyId::new("phase7:transformed-target").unwrap(),
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            format!("equivalence:phase7:{provenance:?}"),
        )
        .unwrap(),
    )
    .unwrap();
    let operational = OperationalSupportTrustReport::from_exact_witness(witness);
    let stamp = SupportTrustCertificationStamp::new(
        SupportCertificationCorpusVersion::new("corpus:13.3").unwrap(),
        SupportCertificationEpoch::new(11).unwrap(),
        "suite:13.3-phase-7",
        family_id,
        support_role,
        SupportTrustStrength::Exact,
        provenance,
        row_id,
        "bundle:digest:phase7:lane",
    )
    .unwrap();
    CertifiedSupportTrustReport::from_operational_report(operational, stamp).unwrap()
}
