mod certification;
mod classification;
mod domain_certification;
mod drift;
mod epochs;
mod equivalence;
mod failure;
mod named_suite;
mod performance;
mod pipeline;
mod receipts;
mod reports;
mod taxonomy;
mod translation;
mod witnesses;

pub use certification::{
    SubscriptionSupportCertificationCoveragePlan, SupportCertificationBatchScope,
    SupportCertificationBatchScopeKind, SupportCertificationCounterSnapshot,
    SupportCertificationCoverageMatrix, SupportCertificationCoverageWitness,
    SupportCertificationEvidenceBundle, SupportCertificationGapReport,
    SupportCertificationLaneDigestSet, SupportCertificationRow, SupportCertificationRowEvidence,
    SupportCertificationRowRequirement, SupportCertificationSummary,
};
pub use classification::{
    SupportTrustClassificationCostSurface, SupportTrustClassificationCounterSnapshot,
    SupportTrustClassificationPlan, SupportTrustClassificationReport,
    SupportTrustClassificationWitness,
};
pub use domain_certification::{
    SupportCertificationHandoffReport, SupportDomainCertificationBatchPlan,
    SupportDomainCertificationBundle, SupportDomainCertificationCounterSnapshot,
    SupportDomainCertificationDebtOwner, SupportDomainCertificationDebtReason,
    SupportDomainCertificationRow, SupportDomainCertificationRowStatus,
    SupportDomainCertificationScenario, SupportGenericCertificationCounterSnapshot,
    SupportGenericCertificationReport, SupportRoadmapPhysicalReadinessPosture,
};
pub use drift::{
    SupportStalenessVerdict, SupportTrustDriftCause, SupportTrustDriftLocality,
    SupportTrustDriftReport, SupportTrustDriftScanPlan, SupportTrustSuppressedCause,
};
pub use epochs::{
    SupportCatalogEpoch, SupportCertificationCorpusVersion, SupportCertificationEpoch,
    SupportCompatibilityEpoch, SupportOperationalLedgerEpoch, SupportTrustEpoch,
    SupportTrustExpiredReport, SupportTrustFreshnessWitness,
};
pub use equivalence::{
    SupportImportEquivalenceWitness, SupportMigrationEquivalenceWitness,
    SupportRebuildEquivalenceWitness, SupportReplicationEquivalenceWitness,
    SupportTrustEquivalenceContract, SupportTrustEquivalenceEvidence, SupportTrustEquivalenceLane,
};
pub use failure::{SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture};
pub use named_suite::{
    SubscriptionSupportAccuracyAccessCloseout,
    SubscriptionSupportAccuracyCertificationCounterSnapshot,
    SubscriptionSupportAccuracyCertificationOutputs, SubscriptionSupportAccuracyCertificationRow,
    SubscriptionSupportAccuracyCertificationRowKind, SubscriptionSupportAccuracyCertificationRun,
    SubscriptionSupportAccuracyCertificationRunner, SubscriptionSupportAccuracyCertificationSuite,
    SubscriptionSupportAccuracyLaneEvidence, SubscriptionSupportAccuracyLaneEvidenceSet,
    SubscriptionSupportAccuracyLaneOutcome, SubscriptionSupportAccuracyPerformanceCloseout,
    SubscriptionSupportAccuracyPersistencePosture,
    SUBSCRIPTION_SUPPORT_ACCURACY_CERTIFICATION_SUITE_NAME,
};
pub use performance::{
    SupportTrustAccessIndexKind, SupportTrustAccessPath, SupportTrustAccessStructurePlan,
    SupportTrustAllocationScope, SupportTrustCloneBoundary, SupportTrustComplexityContract,
    SupportTrustComplexityStatus, SupportTrustDensityClass, SupportTrustEvidenceBudget,
    SupportTrustPathClass, SupportTrustPerformancePlan,
};
pub use pipeline::{
    admit_support_trust_request, check_support_trust_coverage, check_support_trust_drift,
    check_support_trust_equivalence, classify_certified_support_trust,
    classify_operational_support_trust, translate_support_trust_inputs,
    CertifiedSupportTrustClassified, OperationalSupportTrustClassified, RawSupportTrustRequest,
    SupportTrustBatchCardinality, SupportTrustCoverageChecked, SupportTrustDriftChecked,
    SupportTrustEquivalenceChecked, SupportTrustRequestAdmitted, SupportTrustRequestedUse,
    SupportTrustTranslatedInputs,
};
pub use receipts::{
    SupportBasisReceipt, SupportCompatibilityReceipt, SupportCursorCheckpointReceipt,
    SupportFamilyRoleReceipt, SupportImportAdmissionReceipt, SupportMaintenanceReceipt,
    SupportOperationalVerdictReceipt, SupportPortabilityReceipt,
    SupportResumeClassificationReceipt, SupportRetentionReceipt, SupportTrustReceiptBundle,
    SupportTrustReceiptStatus,
};
pub use reports::{
    CertifiedSupportTrustReport, OperationalSupportTrustReport, SupportTrustCertificationStamp,
    UncertifiedSupportTrustPosture,
};
pub use taxonomy::{
    SubscriptionSupportTrustClass, SupportRoleTrustPosture, SupportTrustClass,
    SupportTrustDowngradeReason, SupportTrustProvenance, SupportTrustStrength,
    SupportTrustStrengthProvenance, SupportTrustUseBoundary,
};
pub use translation::{SupportExactTrustTranslation, SupportTrustTranslationPlan};
pub use witnesses::{
    CertifiedSupportTrustWitness, DegradedSupportTrustWitness, ExactSupportTrustWitness,
    RebuildDerivedSupportTrustWitness, RejectedSupportTrustWitness, SupportTrustEquivalenceWitness,
    SupportTrustOperationalWitness,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::subscription_support::{
        SubscriptionResumeClassification, SubscriptionSupportActionOrigin,
        SubscriptionSupportArtifactId, SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
        SubscriptionSupportOperationalBasis, SubscriptionSupportOperationalVerdict,
        SubscriptionSupportRole,
    };

    fn basis() -> SubscriptionSupportOperationalBasis {
        basis_for(
            "basis-bound-continuation-support",
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            SubscriptionSupportRole::ExactContinuation,
            "artifact:trust:phase-1",
        )
    }

    fn basis_for(
        family_id: &str,
        family_kind: SubscriptionSupportFamilyKind,
        support_role: SubscriptionSupportRole,
        artifact_id: &str,
    ) -> SubscriptionSupportOperationalBasis {
        SubscriptionSupportOperationalBasis::new(
            SubscriptionSupportFamilyId::new(family_id).unwrap(),
            family_kind,
            support_role,
            SubscriptionSupportArtifactId(artifact_id.into()),
            "basis:trust",
            "cursor:trust",
            "checkpoint:trust",
            "compatibility:trust",
            "portability:trust",
            SubscriptionSupportActionOrigin::Retention,
        )
        .unwrap()
    }

    fn epochs() -> SupportTrustEpoch {
        SupportTrustEpoch::new(
            SupportCatalogEpoch::new(1).unwrap(),
            SupportOperationalLedgerEpoch::new(7).unwrap(),
            SupportCompatibilityEpoch::new(3).unwrap(),
            Some(SupportCertificationEpoch::new(11).unwrap()),
        )
    }

    fn exact_translation() -> SupportExactTrustTranslation {
        SupportExactTrustTranslation::new(
            basis(),
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        )
        .unwrap()
    }

    fn phase2_performance_plan() -> SupportTrustPerformancePlan {
        SupportTrustPerformancePlan::new(
            SupportTrustPathClass::ForegroundResumeTrustPath,
            SupportTrustDensityClass::SingleSupportArtifact,
            SupportTrustAccessPath::PointLookup,
            SupportTrustAllocationScope::ForegroundScratch,
            1,
            1,
            0,
            0,
            SupportTrustCloneBoundary::NoClone,
        )
        .unwrap()
    }

    fn raw_phase2_request(
        requested_strength: SupportTrustStrength,
        provenance: SupportTrustProvenance,
    ) -> RawSupportTrustRequest {
        raw_phase2_request_for(
            "basis-bound-continuation-support",
            SubscriptionSupportRole::ExactContinuation,
            "artifact:trust:phase-1",
            requested_strength,
            provenance,
        )
    }

    fn raw_phase2_request_for(
        family_id: &str,
        support_role: SubscriptionSupportRole,
        artifact_id: &str,
        requested_strength: SupportTrustStrength,
        provenance: SupportTrustProvenance,
    ) -> RawSupportTrustRequest {
        RawSupportTrustRequest::new(
            SubscriptionSupportFamilyId::new(family_id).unwrap(),
            support_role,
            SubscriptionSupportArtifactId(artifact_id.into()),
            requested_strength,
            provenance,
            SupportTrustRequestedUse::StoreLocalResume,
            SupportTrustBatchCardinality::SingleSupportArtifact,
            epochs(),
            phase2_performance_plan(),
            SupportTrustEvidenceBudget::new(4096, 8, 1).unwrap(),
        )
    }

    fn family_role_receipt() -> SupportFamilyRoleReceipt {
        family_role_receipt_for(
            "basis-bound-continuation-support",
            SubscriptionSupportRole::ExactContinuation,
            "artifact:trust:phase-1",
        )
    }

    fn family_role_receipt_for(
        family_id: &str,
        support_role: SubscriptionSupportRole,
        artifact_id: &str,
    ) -> SupportFamilyRoleReceipt {
        SupportFamilyRoleReceipt::new(
            SubscriptionSupportFamilyId::new(family_id).unwrap(),
            support_role,
            SubscriptionSupportArtifactId(artifact_id.into()),
            "family-role:proof",
            SupportTrustReceiptStatus::Proven,
        )
        .unwrap()
    }

    fn phase2_receipts(
        classification: SubscriptionResumeClassification,
        verdict: SubscriptionSupportOperationalVerdict,
    ) -> SupportTrustReceiptBundle {
        phase2_receipts_for_basis(basis(), classification, verdict)
    }

    fn phase2_receipts_for_basis(
        basis: SubscriptionSupportOperationalBasis,
        classification: SubscriptionResumeClassification,
        verdict: SubscriptionSupportOperationalVerdict,
    ) -> SupportTrustReceiptBundle {
        let artifact_id = basis.artifact_id().clone();
        let cursor_checkpoint_digest =
            format!("{}:{}", basis.cursor_digest(), basis.checkpoint_digest());
        let bundle = SupportTrustReceiptBundle::new(
            SupportResumeClassificationReceipt::new(
                artifact_id.clone(),
                classification,
                "resume:proof",
                SupportTrustReceiptStatus::Proven,
            )
            .unwrap(),
            SupportOperationalVerdictReceipt::new(
                basis.clone(),
                verdict,
                "operational:proof",
                SupportTrustReceiptStatus::Proven,
            )
            .unwrap(),
            family_role_receipt_for(
                basis.family_id().as_str(),
                basis.support_role(),
                basis.artifact_id().as_str(),
            ),
            SupportBasisReceipt::new(
                artifact_id.clone(),
                basis.basis_digest(),
                SupportTrustReceiptStatus::Proven,
            )
            .unwrap(),
            SupportCursorCheckpointReceipt::new(
                artifact_id.clone(),
                cursor_checkpoint_digest,
                SupportTrustReceiptStatus::Proven,
            )
            .unwrap(),
            SupportCompatibilityReceipt::new(
                artifact_id.clone(),
                basis.compatibility_digest(),
                SupportTrustReceiptStatus::Proven,
            )
            .unwrap(),
            SupportPortabilityReceipt::new(
                artifact_id.clone(),
                basis.portability_digest(),
                SupportTrustReceiptStatus::Proven,
            )
            .unwrap(),
        );
        match verdict {
            SubscriptionSupportOperationalVerdict::ExactResumePreserved
            | SubscriptionSupportOperationalVerdict::DegradedResumePreserved => bundle
                .with_retention(
                    SupportRetentionReceipt::new(
                        artifact_id,
                        "retention:trust",
                        SupportTrustReceiptStatus::Proven,
                    )
                    .unwrap(),
                ),
            SubscriptionSupportOperationalVerdict::RebuildRequired => bundle.with_maintenance(
                SupportMaintenanceReceipt::new(
                    artifact_id,
                    "maintenance:admission",
                    "maintenance:proof",
                    SupportTrustReceiptStatus::Proven,
                )
                .unwrap(),
            ),
            _ => bundle,
        }
    }

    fn classify_phase2(
        requested_strength: SupportTrustStrength,
        provenance: SupportTrustProvenance,
        classification: SubscriptionResumeClassification,
        verdict: SubscriptionSupportOperationalVerdict,
    ) -> Result<OperationalSupportTrustClassified, SupportTrustFailure> {
        classify_phase2_for_basis(
            basis(),
            requested_strength,
            provenance,
            classification,
            verdict,
        )
    }

    fn classify_phase2_for_basis(
        basis: SubscriptionSupportOperationalBasis,
        requested_strength: SupportTrustStrength,
        provenance: SupportTrustProvenance,
        classification: SubscriptionResumeClassification,
        verdict: SubscriptionSupportOperationalVerdict,
    ) -> Result<OperationalSupportTrustClassified, SupportTrustFailure> {
        let admitted = admit_support_trust_request(
            raw_phase2_request_for(
                basis.family_id().as_str(),
                basis.support_role(),
                basis.artifact_id().as_str(),
                requested_strength,
                provenance,
            ),
            phase2_receipts_for_basis(basis, classification, verdict),
        )?;
        let translated = translate_support_trust_inputs(admitted)?;
        let drift_checked = check_support_trust_drift(
            translated,
            SupportTrustDriftScanPlan::foreground_support_identity(),
        )?;
        let equivalence_checked = check_support_trust_equivalence(
            drift_checked,
            SupportTrustEquivalenceEvidence::none(),
        )?;
        classify_operational_support_trust(equivalence_checked)
    }

    fn certification_lanes() -> SupportCertificationLaneDigestSet {
        SupportCertificationLaneDigestSet::new(
            "lane:control:exact",
            "lane:hostile:stale",
            "lane:replay:retained",
        )
        .unwrap()
    }

    fn exact_certification_requirement(row_id: &str) -> SupportCertificationRowRequirement {
        SupportCertificationRowRequirement::new(
            row_id,
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            SubscriptionSupportRole::ExactContinuation,
            SupportTrustClass::ExactSupportTrusted,
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            SubscriptionResumeClassification::Exact,
            None,
        )
        .unwrap()
    }

    fn certification_requirement_for(
        row_id: &str,
        family_id: &str,
        family_kind: SubscriptionSupportFamilyKind,
        support_role: SubscriptionSupportRole,
        trust_class: SupportTrustClass,
        trust_strength: SupportTrustStrength,
        provenance: SupportTrustProvenance,
        verdict: SubscriptionSupportOperationalVerdict,
        classification: SubscriptionResumeClassification,
    ) -> SupportCertificationRowRequirement {
        SupportCertificationRowRequirement::new(
            row_id,
            SubscriptionSupportFamilyId::new(family_id).unwrap(),
            family_kind,
            support_role,
            trust_class,
            trust_strength,
            provenance,
            verdict,
            classification,
            None,
        )
        .unwrap()
    }

    fn exact_certification_plan(row_id: &str) -> SubscriptionSupportCertificationCoveragePlan {
        SubscriptionSupportCertificationCoveragePlan::new(
            SupportOperationalLedgerEpoch::new(7).unwrap(),
            SupportCertificationEpoch::new(11).unwrap(),
            vec![exact_certification_requirement(row_id)],
        )
        .unwrap()
    }

    fn exact_certification_row(
        row_id: &str,
        classified: &OperationalSupportTrustClassified,
    ) -> SupportCertificationRow {
        let evidence = SupportCertificationRowEvidence::from_operational_report(
            row_id,
            classified.report(),
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            SupportOperationalLedgerEpoch::new(7).unwrap(),
            SupportCertificationEpoch::new(11).unwrap(),
            certification_lanes(),
            "artifact:digest:exact",
            "subscription-support:digest:exact",
            "diagnostics:digest:exact",
            None,
            Vec::new(),
        )
        .unwrap();
        SupportCertificationRow::new(evidence).unwrap()
    }

    fn report_for_certification_row(
        family_id: &str,
        family_kind: SubscriptionSupportFamilyKind,
        support_role: SubscriptionSupportRole,
        artifact_id: &str,
        trust_strength: SupportTrustStrength,
    ) -> OperationalSupportTrustReport {
        let basis = basis_for(family_id, family_kind, support_role, artifact_id);
        let freshness = SupportTrustFreshnessWitness::new(epochs());
        match trust_strength {
            SupportTrustStrength::Exact => {
                let translation = SupportExactTrustTranslation::new(
                    basis,
                    SubscriptionResumeClassification::Exact,
                    SubscriptionSupportOperationalVerdict::ExactResumePreserved,
                )
                .unwrap();
                let witness = ExactSupportTrustWitness::from_exact_translation(
                    translation,
                    SupportTrustProvenance::NativePublished,
                    freshness,
                )
                .unwrap();
                OperationalSupportTrustReport::from_exact_witness(witness)
            }
            SupportTrustStrength::Degraded => {
                let witness = DegradedSupportTrustWitness::new(basis, freshness);
                OperationalSupportTrustReport::from_degraded_witness(
                    witness,
                    SupportTrustProvenance::NativePublished,
                    SupportTrustClassificationCostSurface::phase1_zero(),
                )
            }
            SupportTrustStrength::RebuildOnly => {
                let witness = RebuildDerivedSupportTrustWitness::new(basis, freshness);
                OperationalSupportTrustReport::from_rebuild_witness(
                    witness,
                    SupportTrustProvenance::Rebuilt,
                    SupportTrustClassificationCostSurface::phase1_zero(),
                )
            }
            SupportTrustStrength::Rejected | SupportTrustStrength::Unsupported => {
                let witness = RejectedSupportTrustWitness::new(basis, freshness);
                OperationalSupportTrustReport::from_rejected_witness(
                    witness,
                    SupportTrustProvenance::Omitted,
                    SupportTrustClassificationCostSurface::phase1_zero(),
                )
            }
        }
    }

    fn certification_row_from_report(
        row_id: &str,
        report: &OperationalSupportTrustReport,
        classification: SubscriptionResumeClassification,
        verdict: SubscriptionSupportOperationalVerdict,
    ) -> SupportCertificationRow {
        let evidence = SupportCertificationRowEvidence::from_operational_report(
            row_id,
            report,
            classification,
            verdict,
            SupportOperationalLedgerEpoch::new(7).unwrap(),
            SupportCertificationEpoch::new(11).unwrap(),
            certification_lanes(),
            format!("artifact:digest:{row_id}"),
            format!("subscription-support:digest:{row_id}"),
            format!("diagnostics:digest:{row_id}"),
            None,
            Vec::new(),
        )
        .unwrap();
        SupportCertificationRow::new(evidence).unwrap()
    }

    fn first_ship_certification_matrix() -> SupportCertificationCoverageMatrix {
        first_ship_certification_matrix_for_basis_artifact("artifact:trust:phase-1")
    }

    fn first_ship_certification_matrix_for_basis_artifact(
        basis_bound_artifact_id: &str,
    ) -> SupportCertificationCoverageMatrix {
        first_ship_certification_matrix_for_basis_artifact_and_materialized_family(
            basis_bound_artifact_id,
            "materialized-narrowing-support",
        )
    }

    fn first_ship_certification_matrix_for_basis_artifact_and_materialized_family(
        basis_bound_artifact_id: &str,
        materialized_family_id: &str,
    ) -> SupportCertificationCoverageMatrix {
        let requirements = vec![
            certification_requirement_for(
                "row:basis-bound-exact",
                "basis-bound-continuation-support",
                SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                SubscriptionSupportRole::ExactContinuation,
                SupportTrustClass::ExactSupportTrusted,
                SupportTrustStrength::Exact,
                SupportTrustProvenance::NativePublished,
                SubscriptionSupportOperationalVerdict::ExactResumePreserved,
                SubscriptionResumeClassification::Exact,
            ),
            certification_requirement_for(
                "row:materialized-narrowing-exact",
                materialized_family_id,
                SubscriptionSupportFamilyKind::MaterializedNarrowingSupport,
                SubscriptionSupportRole::NarrowingMaterialization,
                SupportTrustClass::ExactSupportTrusted,
                SupportTrustStrength::Exact,
                SupportTrustProvenance::NativePublished,
                SubscriptionSupportOperationalVerdict::ExactResumePreserved,
                SubscriptionResumeClassification::Exact,
            ),
            certification_requirement_for(
                "row:degraded-continuation",
                "degraded-continuation-support",
                SubscriptionSupportFamilyKind::DegradedContinuationSupport,
                SubscriptionSupportRole::DegradedContinuation,
                SupportTrustClass::DegradedSupportTrusted,
                SupportTrustStrength::Degraded,
                SupportTrustProvenance::NativePublished,
                SubscriptionSupportOperationalVerdict::DegradedResumePreserved,
                SubscriptionResumeClassification::Degraded,
            ),
            certification_requirement_for(
                "row:extension-defined-rejected",
                "extension-defined-support",
                SubscriptionSupportFamilyKind::ExtensionDefinedSupport,
                SubscriptionSupportRole::ExactContinuation,
                SupportTrustClass::StaleSupportRejected,
                SupportTrustStrength::Rejected,
                SupportTrustProvenance::Omitted,
                SubscriptionSupportOperationalVerdict::RejectedByPolicy,
                SubscriptionResumeClassification::NotResumable,
            ),
        ];
        let plan = SubscriptionSupportCertificationCoveragePlan::new(
            SupportOperationalLedgerEpoch::new(7).unwrap(),
            SupportCertificationEpoch::new(11).unwrap(),
            requirements,
        )
        .unwrap();
        let basis_bound = report_for_certification_row(
            "basis-bound-continuation-support",
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            SubscriptionSupportRole::ExactContinuation,
            basis_bound_artifact_id,
            SupportTrustStrength::Exact,
        );
        let materialized = report_for_certification_row(
            materialized_family_id,
            SubscriptionSupportFamilyKind::MaterializedNarrowingSupport,
            SubscriptionSupportRole::NarrowingMaterialization,
            "artifact:first-ship:materialized",
            SupportTrustStrength::Exact,
        );
        let degraded = report_for_certification_row(
            "degraded-continuation-support",
            SubscriptionSupportFamilyKind::DegradedContinuationSupport,
            SubscriptionSupportRole::DegradedContinuation,
            "artifact:first-ship:degraded",
            SupportTrustStrength::Degraded,
        );
        let extension = report_for_certification_row(
            "extension-defined-support",
            SubscriptionSupportFamilyKind::ExtensionDefinedSupport,
            SubscriptionSupportRole::ExactContinuation,
            "artifact:first-ship:extension",
            SupportTrustStrength::Rejected,
        );
        SupportCertificationCoverageMatrix::from_rows(
            &plan,
            vec![
                certification_row_from_report(
                    "row:basis-bound-exact",
                    &basis_bound,
                    SubscriptionResumeClassification::Exact,
                    SubscriptionSupportOperationalVerdict::ExactResumePreserved,
                ),
                certification_row_from_report(
                    "row:materialized-narrowing-exact",
                    &materialized,
                    SubscriptionResumeClassification::Exact,
                    SubscriptionSupportOperationalVerdict::ExactResumePreserved,
                ),
                certification_row_from_report(
                    "row:degraded-continuation",
                    &degraded,
                    SubscriptionResumeClassification::Degraded,
                    SubscriptionSupportOperationalVerdict::DegradedResumePreserved,
                ),
                certification_row_from_report(
                    "row:extension-defined-rejected",
                    &extension,
                    SubscriptionResumeClassification::NotResumable,
                    SubscriptionSupportOperationalVerdict::RejectedByPolicy,
                ),
            ],
        )
        .unwrap()
    }

    fn first_ship_batch_scope() -> SupportCertificationBatchScope {
        SupportCertificationBatchScope::new(
            SupportCertificationBatchScopeKind::CertificationScopeLocal,
            SupportTrustDensityClass::CertificationScopeLocal,
            SupportTrustPathClass::BatchCertificationPath,
            SupportTrustAllocationScope::BatchCertification,
            4,
            4,
            3,
            1,
        )
        .unwrap()
    }

    fn first_ship_counter_snapshot() -> SupportCertificationCounterSnapshot {
        SupportCertificationCounterSnapshot::new(4, 4, 3, 4, 1, 0, 0)
    }

    fn first_ship_certification_bundle() -> SupportCertificationEvidenceBundle {
        SupportCertificationEvidenceBundle::new(
            "run:13.3:first-ship",
            first_ship_certification_matrix(),
            first_ship_batch_scope(),
            first_ship_counter_snapshot(),
        )
        .unwrap()
    }

    fn certified_first_ship_support_trust() -> CertifiedSupportTrustClassified {
        certified_first_ship_support_trust_for(
            basis(),
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            "row:basis-bound-exact",
        )
    }

    fn certified_first_ship_support_trust_for(
        basis: SubscriptionSupportOperationalBasis,
        requested_strength: SupportTrustStrength,
        provenance: SupportTrustProvenance,
        classification: SubscriptionResumeClassification,
        verdict: SubscriptionSupportOperationalVerdict,
        row_id: &str,
    ) -> CertifiedSupportTrustClassified {
        let family_id = basis.family_id().clone();
        let support_role = basis.support_role();
        let classified = classify_phase2_for_basis(
            basis,
            requested_strength,
            provenance,
            classification,
            verdict,
        )
        .unwrap();
        let bundle = first_ship_certification_bundle();
        let evidence_bundle_digest = bundle.evidence_bundle_digest().to_string();
        let coverage_checked = check_support_trust_coverage(classified, bundle).unwrap();
        let stamp = SupportTrustCertificationStamp::new(
            SupportCertificationCorpusVersion::new("corpus:13.3").unwrap(),
            SupportCertificationEpoch::new(11).unwrap(),
            "suite:13.3",
            family_id,
            support_role,
            requested_strength,
            provenance,
            row_id,
            evidence_bundle_digest,
        )
        .unwrap();
        classify_certified_support_trust(coverage_checked, stamp).unwrap()
    }

    fn generic_support_certification_report() -> SupportGenericCertificationReport {
        let certified = certified_first_ship_support_trust();
        generic_support_certification_report_for(
            "generic:subscription-support-trust:first-ship",
            certified,
        )
    }

    fn generic_support_certification_report_for(
        generic_row_id: &str,
        certified: CertifiedSupportTrustClassified,
    ) -> SupportGenericCertificationReport {
        SupportGenericCertificationReport::from_certified_support_trust(
            generic_row_id,
            certified.report().clone(),
            certified.coverage_witness(),
            SupportGenericCertificationCounterSnapshot::new(1, 1, 1, 1, 1, 1).unwrap(),
        )
        .unwrap()
    }

    fn first_ship_domain_batch_scope() -> SupportCertificationBatchScope {
        SupportCertificationBatchScope::new(
            SupportCertificationBatchScopeKind::DomainScenarioLocal,
            SupportTrustDensityClass::DomainScenarioLocal,
            SupportTrustPathClass::DomainCertificationPath,
            SupportTrustAllocationScope::DomainCertification,
            5,
            5,
            4,
            1,
        )
        .unwrap()
    }

    fn first_ship_domain_batch_plan() -> SupportDomainCertificationBatchPlan {
        SupportDomainCertificationBatchPlan::new(5, 5, first_ship_domain_batch_scope(), 5).unwrap()
    }

    fn first_ship_domain_rows(
        generic: &SupportGenericCertificationReport,
    ) -> Vec<SupportDomainCertificationRow> {
        let materialized = generic_support_certification_report_for(
            "generic:subscription-support-trust:materialized-narrowing",
            certified_first_ship_support_trust_for(
                basis_for(
                    "materialized-narrowing-support",
                    SubscriptionSupportFamilyKind::MaterializedNarrowingSupport,
                    SubscriptionSupportRole::NarrowingMaterialization,
                    "artifact:first-ship:materialized",
                ),
                SupportTrustStrength::Exact,
                SupportTrustProvenance::NativePublished,
                SubscriptionResumeClassification::Exact,
                SubscriptionSupportOperationalVerdict::ExactResumePreserved,
                "row:materialized-narrowing-exact",
            ),
        );
        let degraded = generic_support_certification_report_for(
            "generic:subscription-support-trust:degraded-continuation",
            certified_first_ship_support_trust_for(
                basis_for(
                    "degraded-continuation-support",
                    SubscriptionSupportFamilyKind::DegradedContinuationSupport,
                    SubscriptionSupportRole::DegradedContinuation,
                    "artifact:first-ship:degraded",
                ),
                SupportTrustStrength::Degraded,
                SupportTrustProvenance::NativePublished,
                SubscriptionResumeClassification::Degraded,
                SubscriptionSupportOperationalVerdict::DegradedResumePreserved,
                "row:degraded-continuation",
            ),
        );
        vec![
            SupportDomainCertificationRow::certified_from_generic_report(
                SupportDomainCertificationScenario::GeometryCadSessionContinuation,
                generic,
            )
            .unwrap(),
            SupportDomainCertificationRow::certified_from_generic_report(
                SupportDomainCertificationScenario::WebDataRestartReplication,
                &materialized,
            )
            .unwrap(),
            SupportDomainCertificationRow::certified_from_generic_report(
                SupportDomainCertificationScenario::AiBranchWorkspaceDegradedContinuation,
                &degraded,
            )
            .unwrap(),
            SupportDomainCertificationRow::explicit_advanced_family_debt(
                SupportDomainCertificationScenario::ChipSimulationLongHistoryRebuild,
                generic,
            )
            .unwrap(),
            SupportDomainCertificationRow::explicit_advanced_family_debt(
                SupportDomainCertificationScenario::OfflineCollaborativeCapsuleOmission,
                generic,
            )
            .unwrap(),
        ]
    }

    fn phase7_suite_artifacts() -> (
        SupportCertificationEvidenceBundle,
        SupportGenericCertificationReport,
        SupportDomainCertificationBundle,
        SupportCertificationHandoffReport,
    ) {
        let evidence_bundle = first_ship_certification_bundle();
        let generic = generic_support_certification_report();
        let domain = SupportDomainCertificationBundle::new(
            first_ship_domain_rows(&generic),
            first_ship_domain_batch_plan(),
            SupportDomainCertificationCounterSnapshot::new(5, 3, 2, 5, 4, 1, 2),
        )
        .unwrap();
        let handoff = SupportCertificationHandoffReport::from_generic_and_domain_certification(
            &generic, &domain,
        )
        .unwrap();
        (evidence_bundle, generic, domain, handoff)
    }

    fn phase7_required_suite_rows(
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

    fn phase7_lane_evidence() -> SubscriptionSupportAccuracyLaneEvidenceSet {
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

    fn phase7_expected_lane_failure(
        row_kind: SubscriptionSupportAccuracyCertificationRowKind,
    ) -> SupportTrustFailure {
        match row_kind {
            SubscriptionSupportAccuracyCertificationRowKind::RebuildDerivedSupportDowngraded => {
                phase7_missing_equivalence_failure(SupportTrustProvenance::Rebuilt)
            }
            SubscriptionSupportAccuracyCertificationRowKind::ReplicatedSupportIdentityNotEnough => {
                phase7_missing_equivalence_failure(SupportTrustProvenance::Replicated)
            }
            SubscriptionSupportAccuracyCertificationRowKind::ImportedSupportMissingBasisNotResumable => {
                phase7_import_basis_mismatch_failure()
            }
            SubscriptionSupportAccuracyCertificationRowKind::StaleSupportRejected => {
                SupportCatalogEpoch::new(0).expect_err("zero catalog epoch must reject as stale")
            }
            SubscriptionSupportAccuracyCertificationRowKind::PolicyRejectedSupport => {
                phase7_operational_drift_failure(
                    SubscriptionSupportOperationalVerdict::RejectedByPolicy,
                )
            }
            SubscriptionSupportAccuracyCertificationRowKind::FamilyRoleMismatchRejected => {
                phase7_family_role_mismatch_failure()
            }
            SubscriptionSupportAccuracyCertificationRowKind::CompatibilityDriftRejectsExactTrust => {
                phase7_receipt_drift_failure(
                    "basis:trust",
                    "compatibility:wrong",
                    "portability:trust",
                )
            }
            SubscriptionSupportAccuracyCertificationRowKind::OperationalVerdictDriftRejectsExactTrust => {
                phase7_operational_drift_failure(
                    SubscriptionSupportOperationalVerdict::DegradedResumePreserved,
                )
            }
            SubscriptionSupportAccuracyCertificationRowKind::PortabilityDriftRejectsExactTrust => {
                phase7_receipt_drift_failure(
                    "basis:trust",
                    "compatibility:trust",
                    "portability:wrong",
                )
            }
            SubscriptionSupportAccuracyCertificationRowKind::CoverageDriftRejectsPlatformTrust => {
                phase7_coverage_drift_failure()
            }
            SubscriptionSupportAccuracyCertificationRowKind::CertificationMissingRowRejected => {
                phase7_certification_missing_row_failure()
            }
            SubscriptionSupportAccuracyCertificationRowKind::CertificationDuplicateRowRejected => {
                phase7_certification_duplicate_row_failure()
            }
            SubscriptionSupportAccuracyCertificationRowKind::CertificationMislabeledRowRejected => {
                phase7_certification_mislabeled_row_failure()
            }
            SubscriptionSupportAccuracyCertificationRowKind::CertificationSelfComparisonRejected => {
                SupportCertificationLaneDigestSet::new(
                    "phase7:lane:same",
                    "phase7:lane:same",
                    "phase7:lane:replay",
                )
                .expect_err("self-comparison lane digests must reject")
            }
            SubscriptionSupportAccuracyCertificationRowKind::MultiDriftPrecedenceDeterministic => {
                phase7_receipt_drift_failure(
                    "basis:wrong",
                    "compatibility:wrong",
                    "portability:wrong",
                )
            }
            _ => unreachable!("phase7 pass rows do not create rejection failures"),
        }
    }

    fn phase7_missing_equivalence_failure(
        provenance: SupportTrustProvenance,
    ) -> SupportTrustFailure {
        let admitted = admit_support_trust_request(
            raw_phase2_request(SupportTrustStrength::Exact, provenance),
            phase2_receipts(
                SubscriptionResumeClassification::Exact,
                SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            ),
        )
        .unwrap();
        let translated = translate_support_trust_inputs(admitted).unwrap();
        let drift_checked = check_support_trust_drift(
            translated,
            SupportTrustDriftScanPlan::foreground_support_identity(),
        )
        .unwrap();
        check_support_trust_equivalence(drift_checked, SupportTrustEquivalenceEvidence::none())
            .expect_err("missing transformed equivalence must reject exact support trust")
    }

    fn phase7_import_basis_mismatch_failure() -> SupportTrustFailure {
        let bundle = phase2_receipts(
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        )
        .with_import_admission(
            SupportImportAdmissionReceipt::new(
                SubscriptionSupportArtifactId("artifact:trust:wrong-import".into()),
                SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                "import:admission",
                SupportTrustReceiptStatus::Proven,
            )
            .unwrap(),
        );
        let admitted = admit_support_trust_request(
            RawSupportTrustRequest::new(
                SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                SubscriptionSupportRole::ExactContinuation,
                SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
                SupportTrustStrength::Exact,
                SupportTrustProvenance::Imported,
                SupportTrustRequestedUse::StoreLocalResume,
                SupportTrustBatchCardinality::SingleSupportArtifact,
                epochs(),
                phase2_performance_plan(),
                SupportTrustEvidenceBudget::new(4096, 9, 1).unwrap(),
            ),
            bundle,
        )
        .unwrap();
        translate_support_trust_inputs(admitted)
            .expect_err("import admission bound to a different artifact must reject")
    }

    fn phase7_family_role_mismatch_failure() -> SupportTrustFailure {
        let role_mismatched_basis = basis_for(
            "basis-bound-continuation-support",
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            SubscriptionSupportRole::DegradedContinuation,
            "artifact:trust:phase-1",
        );
        admit_support_trust_request(
            raw_phase2_request(
                SupportTrustStrength::Exact,
                SupportTrustProvenance::NativePublished,
            ),
            phase2_receipts_for_basis(
                role_mismatched_basis,
                SubscriptionResumeClassification::Exact,
                SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            ),
        )
        .expect_err("family-role receipt role drift must reject admission")
    }

    fn phase7_operational_drift_failure(
        verdict: SubscriptionSupportOperationalVerdict,
    ) -> SupportTrustFailure {
        let admitted = admit_support_trust_request(
            raw_phase2_request(
                SupportTrustStrength::Exact,
                SupportTrustProvenance::NativePublished,
            ),
            phase2_receipts(SubscriptionResumeClassification::Exact, verdict),
        )
        .unwrap();
        let translated = translate_support_trust_inputs(admitted).unwrap();
        check_support_trust_drift(
            translated,
            SupportTrustDriftScanPlan::foreground_support_identity(),
        )
        .expect_err("operational verdict drift must reject exact trust")
    }

    fn phase7_receipt_drift_failure(
        basis_digest: &str,
        compatibility_digest: &str,
        portability_digest: &str,
    ) -> SupportTrustFailure {
        let artifact_id = SubscriptionSupportArtifactId("artifact:trust:phase-1".into());
        let bundle = SupportTrustReceiptBundle::new(
            SupportResumeClassificationReceipt::new(
                artifact_id.clone(),
                SubscriptionResumeClassification::Exact,
                "resume:proof",
                SupportTrustReceiptStatus::Proven,
            )
            .unwrap(),
            SupportOperationalVerdictReceipt::new(
                basis(),
                SubscriptionSupportOperationalVerdict::ExactResumePreserved,
                "operational:proof",
                SupportTrustReceiptStatus::Proven,
            )
            .unwrap(),
            family_role_receipt(),
            SupportBasisReceipt::new(
                artifact_id.clone(),
                basis_digest,
                SupportTrustReceiptStatus::Proven,
            )
            .unwrap(),
            SupportCursorCheckpointReceipt::new(
                artifact_id.clone(),
                "cursor:trust:checkpoint:trust",
                SupportTrustReceiptStatus::Proven,
            )
            .unwrap(),
            SupportCompatibilityReceipt::new(
                artifact_id.clone(),
                compatibility_digest,
                SupportTrustReceiptStatus::Proven,
            )
            .unwrap(),
            SupportPortabilityReceipt::new(
                artifact_id.clone(),
                portability_digest,
                SupportTrustReceiptStatus::Proven,
            )
            .unwrap(),
        )
        .with_retention(
            SupportRetentionReceipt::new(
                artifact_id,
                "retention:trust",
                SupportTrustReceiptStatus::Proven,
            )
            .unwrap(),
        );
        let admitted = admit_support_trust_request(
            raw_phase2_request(
                SupportTrustStrength::Exact,
                SupportTrustProvenance::NativePublished,
            ),
            bundle,
        )
        .unwrap();
        let translated = translate_support_trust_inputs(admitted).unwrap();
        check_support_trust_drift(
            translated,
            SupportTrustDriftScanPlan::foreground_support_identity(),
        )
        .expect_err("receipt drift must reject exact trust")
    }

    fn phase7_coverage_drift_failure() -> SupportTrustFailure {
        let admitted = admit_support_trust_request(
            raw_phase2_request(
                SupportTrustStrength::Exact,
                SupportTrustProvenance::NativePublished,
            ),
            phase2_receipts(
                SubscriptionResumeClassification::Exact,
                SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            ),
        )
        .unwrap();
        let translated = translate_support_trust_inputs(admitted).unwrap();
        check_support_trust_drift(
            translated,
            SupportTrustDriftScanPlan::certification_scope(
                SupportTrustPathClass::BatchCertificationPath,
                9,
                2,
                false,
            )
            .unwrap(),
        )
        .expect_err("missing coverage must reject platform trust")
    }

    fn phase7_certification_missing_row_failure() -> SupportTrustFailure {
        let classified = classify_phase2(
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        )
        .unwrap();
        let plan = SubscriptionSupportCertificationCoveragePlan::new(
            SupportOperationalLedgerEpoch::new(7).unwrap(),
            SupportCertificationEpoch::new(11).unwrap(),
            vec![
                exact_certification_requirement("row:exact-control"),
                exact_certification_requirement("row:hostile-stale"),
            ],
        )
        .unwrap();
        SupportCertificationCoverageMatrix::from_rows(
            &plan,
            vec![exact_certification_row("row:exact-control", &classified)],
        )
        .expect_err("missing required certification row must reject coverage")
    }

    fn phase7_certification_duplicate_row_failure() -> SupportTrustFailure {
        let classified = classify_phase2(
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        )
        .unwrap();
        SupportCertificationCoverageMatrix::from_rows(
            &exact_certification_plan("row:exact-control"),
            vec![
                exact_certification_row("row:exact-control", &classified),
                exact_certification_row("row:exact-control", &classified),
            ],
        )
        .expect_err("duplicate certification rows must reject coverage")
    }

    fn phase7_certification_mislabeled_row_failure() -> SupportTrustFailure {
        let classified = classify_phase2(
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        )
        .unwrap();
        let plan = SubscriptionSupportCertificationCoveragePlan::new(
            SupportOperationalLedgerEpoch::new(7).unwrap(),
            SupportCertificationEpoch::new(11).unwrap(),
            vec![SupportCertificationRowRequirement::new(
                "row:exact-control",
                SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                SubscriptionSupportRole::ExactContinuation,
                SupportTrustClass::DegradedSupportTrusted,
                SupportTrustStrength::Degraded,
                SupportTrustProvenance::NativePublished,
                SubscriptionSupportOperationalVerdict::ExactResumePreserved,
                SubscriptionResumeClassification::Exact,
                None,
            )
            .unwrap()],
        )
        .unwrap();
        SupportCertificationCoverageMatrix::from_rows(
            &plan,
            vec![exact_certification_row("row:exact-control", &classified)],
        )
        .expect_err("mislabeled certification row must reject coverage")
    }

    fn phase7_certified_transformed_exact_report(
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

    fn exact_equivalence_contract(
        lane: SupportTrustEquivalenceLane,
    ) -> SupportTrustEquivalenceContract {
        let source_basis = basis();
        SupportTrustEquivalenceContract::new(
            lane,
            source_basis,
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportRole::ExactContinuation,
            SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
            "basis:trust",
            "cursor:trust",
            "checkpoint:trust",
            "compatibility:trust",
            "portability:trust",
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            "equivalence:exact",
        )
        .unwrap()
    }

    #[test]
    fn phase6_generic_certification_consumes_certified_support_trust() {
        let generic = generic_support_certification_report();

        assert_eq!(
            generic.certified_report().use_boundary(),
            SupportTrustUseBoundary::CertifiedPlatform
        );
        assert_eq!(
            generic.counter_snapshot().certified_support_report_count(),
            1
        );
        assert!(!generic.generic_certification_digest().is_empty());
        assert_eq!(generic.coverage_summary().row_count(), 4);
    }

    #[test]
    fn phase6_domain_certification_emits_scenarios_and_explicit_physical_debt() {
        let generic = generic_support_certification_report();
        let bundle = SupportDomainCertificationBundle::new(
            first_ship_domain_rows(&generic),
            first_ship_domain_batch_plan(),
            SupportDomainCertificationCounterSnapshot::new(5, 3, 2, 5, 4, 1, 2),
        )
        .unwrap();
        let handoff = SupportCertificationHandoffReport::from_generic_and_domain_certification(
            &generic, &bundle,
        )
        .unwrap();

        assert_eq!(bundle.rows().len(), 5);
        assert_eq!(bundle.counter_snapshot().physical_readiness_debt_count(), 2);
        assert_eq!(
            bundle
                .rows()
                .iter()
                .filter(|row| row.row_status()
                    == SupportDomainCertificationRowStatus::CertifiedSemanticSupport)
                .count(),
            3
        );
        let chip_debt = bundle
            .rows()
            .iter()
            .find(|row| {
                row.scenario()
                    == SupportDomainCertificationScenario::ChipSimulationLongHistoryRebuild
            })
            .expect("chip simulation scenario row must be present");
        assert_eq!(
            chip_debt.debt_reason(),
            Some(SupportDomainCertificationDebtReason::RebuildEquivalenceLaneDeferred)
        );
        assert_eq!(
            chip_debt.required_future_milestone(),
            Some(SupportDomainCertificationDebtOwner::Roadmap2PhysicalDatabaseFoundation)
        );
        let offline_debt = bundle
            .rows()
            .iter()
            .find(|row| {
                row.scenario()
                    == SupportDomainCertificationScenario::OfflineCollaborativeCapsuleOmission
            })
            .expect("offline capsule scenario row must be present");
        assert_eq!(
            offline_debt.debt_reason(),
            Some(SupportDomainCertificationDebtReason::OmittedSupportImportLaneDeferred)
        );
        assert_eq!(
            offline_debt.required_future_milestone(),
            Some(SupportDomainCertificationDebtOwner::Milestone15ExtensionSupportRegistration)
        );
        assert!(handoff.semantic_support_trust_closed());
        assert_eq!(
            handoff.roadmap_physical_readiness_posture(),
            SupportRoadmapPhysicalReadinessPosture::PhysicalDatabaseReadinessDeferredToRoadmap2
        );
        assert!(!handoff.handoff_digest().is_empty());
    }

    #[test]
    fn phase6_domain_plan_rejects_counter_width_drift() {
        let generic = generic_support_certification_report();
        let error = SupportDomainCertificationBundle::new(
            first_ship_domain_rows(&generic),
            first_ship_domain_batch_plan(),
            SupportDomainCertificationCounterSnapshot::new(5, 3, 1, 5, 4, 1, 1),
        )
        .expect_err("explicit debt row count must match domain scenario rows");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustCoverageMissing
        );
    }

    #[test]
    fn phase6_domain_certified_row_rejects_scenario_family_drift() {
        let generic = generic_support_certification_report();
        let error = SupportDomainCertificationRow::certified_from_generic_report(
            SupportDomainCertificationScenario::WebDataRestartReplication,
            &generic,
        )
        .expect_err("basis-bound exact support cannot certify materialized narrowing scenario");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustRoleMismatch
        );
    }

    #[test]
    fn phase6_degraded_domain_certification_cannot_satisfy_exact_scenario() {
        let degraded = generic_support_certification_report_for(
            "generic:subscription-support-trust:degraded-continuation",
            certified_first_ship_support_trust_for(
                basis_for(
                    "degraded-continuation-support",
                    SubscriptionSupportFamilyKind::DegradedContinuationSupport,
                    SubscriptionSupportRole::DegradedContinuation,
                    "artifact:first-ship:degraded",
                ),
                SupportTrustStrength::Degraded,
                SupportTrustProvenance::NativePublished,
                SubscriptionResumeClassification::Degraded,
                SubscriptionSupportOperationalVerdict::DegradedResumePreserved,
                "row:degraded-continuation",
            ),
        );

        let error = SupportDomainCertificationRow::certified_from_generic_report(
            SupportDomainCertificationScenario::GeometryCadSessionContinuation,
            &degraded,
        )
        .expect_err("degraded support cannot certify an exact continuation scenario");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustRoleMismatch
        );
    }

    #[test]
    fn phase6_domain_rows_reject_first_ship_scenarios_as_advanced_family_debt() {
        let generic = generic_support_certification_report();
        let error = SupportDomainCertificationRow::explicit_advanced_family_debt(
            SupportDomainCertificationScenario::WebDataRestartReplication,
            &generic,
        )
        .expect_err("web/data first-ship scenario must be certified, not debt");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim
        );
    }

    #[test]
    fn phase6_handoff_keeps_physical_readiness_debt_explicit() {
        let generic = generic_support_certification_report();
        let bundle = SupportDomainCertificationBundle::new(
            first_ship_domain_rows(&generic),
            first_ship_domain_batch_plan(),
            SupportDomainCertificationCounterSnapshot::new(5, 3, 2, 5, 4, 1, 2),
        )
        .unwrap();
        let honest = SupportCertificationHandoffReport::from_generic_and_domain_certification(
            &generic, &bundle,
        )
        .unwrap();

        assert_eq!(
            honest.roadmap_physical_readiness_posture(),
            SupportRoadmapPhysicalReadinessPosture::PhysicalDatabaseReadinessDeferredToRoadmap2
        );
    }

    #[test]
    fn phase7_named_subscription_support_accuracy_suite_emits_required_outputs() {
        let (evidence_bundle, generic, domain, handoff) = phase7_suite_artifacts();
        let lane_evidence = phase7_lane_evidence();
        let suite =
            SubscriptionSupportAccuracyCertificationSuite::from_phase_artifacts_and_lane_evidence(
                &evidence_bundle,
                &generic,
                &domain,
                &handoff,
                &lane_evidence,
            )
            .unwrap();

        assert_eq!(
            suite.suite_name(),
            SUBSCRIPTION_SUPPORT_ACCURACY_CERTIFICATION_SUITE_NAME
        );
        assert_eq!(
            suite.rows().len(),
            SubscriptionSupportAccuracyCertificationRowKind::required().len()
        );
        assert_eq!(
            suite.counter_snapshot().required_row_count(),
            SubscriptionSupportAccuracyCertificationRowKind::required().len() as u64
        );
        assert_eq!(
            suite.required_outputs().artifact_digest(),
            evidence_bundle.artifact_digest()
        );
        assert_eq!(
            suite.required_outputs().subscription_support_digest(),
            evidence_bundle.subscription_support_digest()
        );
        assert_eq!(
            suite.required_outputs().diagnostics_digest(),
            evidence_bundle.diagnostics_digest()
        );
        assert_eq!(
            suite.required_outputs().counter_snapshot_digest(),
            evidence_bundle.counter_snapshot_digest()
        );
        assert_eq!(
            suite.required_outputs().certification_summary_digest(),
            evidence_bundle.certification_summary_digest()
        );
        assert!(!suite.suite_digest().is_empty());
    }

    #[test]
    fn phase7_production_runner_emits_performance_access_and_persistence_closeout() {
        let (evidence_bundle, generic, domain, handoff) = phase7_suite_artifacts();
        let lane_evidence = phase7_lane_evidence();
        let run = SubscriptionSupportAccuracyCertificationRunner::production()
            .certify(
                &evidence_bundle,
                &generic,
                &domain,
                &handoff,
                &lane_evidence,
            )
            .expect("production runner must emit the named suite with closeout proof");

        assert_eq!(
            run.suite().rows().len(),
            SubscriptionSupportAccuracyCertificationRowKind::required().len()
        );
        assert_eq!(run.performance_closeout().certification_row_count(), 4);
        assert_eq!(
            run.performance_closeout().certification_index_probe_count(),
            4
        );
        assert_eq!(
            run.performance_closeout()
                .certification_receipt_reuse_count(),
            3
        );
        assert_eq!(
            run.performance_closeout().certification_allocation_count(),
            1
        );
        assert_eq!(run.performance_closeout().generic_row_count(), 1);
        assert_eq!(run.performance_closeout().generic_index_probe_count(), 1);
        assert_eq!(run.performance_closeout().generic_receipt_reuse_count(), 1);
        assert_eq!(run.performance_closeout().generic_allocation_count(), 1);
        assert_eq!(run.performance_closeout().domain_scenario_row_count(), 5);
        assert_eq!(run.performance_closeout().domain_index_probe_count(), 5);
        assert_eq!(run.performance_closeout().domain_receipt_reuse_count(), 4);
        assert_eq!(run.performance_closeout().domain_allocation_count(), 1);
        assert_eq!(run.performance_closeout().global_scan_debt_count(), 0);
        assert_eq!(
            run.access_closeout().certified_semantic_domain_row_count(),
            3
        );
        assert_eq!(
            run.access_closeout().explicit_advanced_family_debt_count(),
            2
        );
        assert!(run.access_closeout().handoff_semantic_trust_closed());
        assert!(run.access_closeout().roadmap2_physical_debt_explicit());
        assert!(run.access_closeout().milestone15_extension_debt_explicit());
        assert_eq!(
            run.persistence_posture(),
            SubscriptionSupportAccuracyPersistencePosture::InMemoryCertificationOnly
        );
        assert!(!run.run_digest().is_empty());
    }

    #[test]
    fn phase7_runner_rejects_handoff_not_bound_to_phase_artifacts() {
        let (evidence_bundle, generic, domain, _) = phase7_suite_artifacts();
        let mismatched_generic = generic_support_certification_report_for(
            "generic:subscription-support-trust:mismatched",
            certified_first_ship_support_trust(),
        );
        let mismatched_handoff =
            SupportCertificationHandoffReport::from_generic_and_domain_certification(
                &mismatched_generic,
                &domain,
            )
            .unwrap();
        let error = SubscriptionSupportAccuracyCertificationRunner::production()
            .certify(
                &evidence_bundle,
                &generic,
                &domain,
                &mismatched_handoff,
                &phase7_lane_evidence(),
            )
            .expect_err("runner must reject a handoff digest from a different generic artifact");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustCoverageMissing
        );
    }

    #[test]
    fn phase7_runner_rejects_certification_counter_regime_drift() {
        let drifted_scope = SupportCertificationBatchScope::new(
            SupportCertificationBatchScopeKind::CertificationScopeLocal,
            SupportTrustDensityClass::CertificationScopeLocal,
            SupportTrustPathClass::BatchCertificationPath,
            SupportTrustAllocationScope::BatchCertification,
            4,
            5,
            3,
            1,
        )
        .unwrap();
        let evidence_bundle = SupportCertificationEvidenceBundle::new(
            "run:13.3:first-ship:wrong-index-probes",
            first_ship_certification_matrix(),
            drifted_scope,
            SupportCertificationCounterSnapshot::new(4, 4, 3, 5, 1, 0, 0),
        )
        .unwrap();
        let generic = generic_support_certification_report();
        let domain = SupportDomainCertificationBundle::new(
            first_ship_domain_rows(&generic),
            first_ship_domain_batch_plan(),
            SupportDomainCertificationCounterSnapshot::new(5, 3, 2, 5, 4, 1, 2),
        )
        .unwrap();
        let handoff = SupportCertificationHandoffReport::from_generic_and_domain_certification(
            &generic, &domain,
        )
        .unwrap();
        let error = SubscriptionSupportAccuracyCertificationRunner::production()
            .certify(
                &evidence_bundle,
                &generic,
                &domain,
                &handoff,
                &phase7_lane_evidence(),
            )
            .expect_err("runner closeout must reject a valid bundle whose counter regime drifted");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustCoverageMissing
        );
    }

    #[test]
    fn phase7_runner_rejects_generic_performance_without_physical_debt_counter() {
        let evidence_bundle = first_ship_certification_bundle();
        let certified = certified_first_ship_support_trust();
        let generic = SupportGenericCertificationReport::from_certified_support_trust(
            "generic:subscription-support-trust:missing-physical-debt-counter",
            certified.report().clone(),
            certified.coverage_witness(),
            SupportGenericCertificationCounterSnapshot::new(1, 1, 1, 1, 1, 0).unwrap(),
        )
        .unwrap();
        let domain = SupportDomainCertificationBundle::new(
            first_ship_domain_rows(&generic),
            first_ship_domain_batch_plan(),
            SupportDomainCertificationCounterSnapshot::new(5, 3, 2, 5, 4, 1, 2),
        )
        .unwrap();
        let handoff = SupportCertificationHandoffReport::from_generic_and_domain_certification(
            &generic, &domain,
        )
        .unwrap();
        let error = SubscriptionSupportAccuracyCertificationRunner::production()
            .certify(
                &evidence_bundle,
                &generic,
                &domain,
                &handoff,
                &phase7_lane_evidence(),
            )
            .expect_err(
                "runner closeout must reject generic performance counters without physical debt",
            );

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustCoverageMissing
        );
    }

    #[test]
    fn phase7_runner_rejects_domain_counter_regime_drift() {
        let evidence_bundle = first_ship_certification_bundle();
        let generic = generic_support_certification_report();
        let drifted_scope = SupportCertificationBatchScope::new(
            SupportCertificationBatchScopeKind::DomainScenarioLocal,
            SupportTrustDensityClass::DomainScenarioLocal,
            SupportTrustPathClass::DomainCertificationPath,
            SupportTrustAllocationScope::DomainCertification,
            5,
            6,
            4,
            1,
        )
        .unwrap();
        let drifted_domain_plan =
            SupportDomainCertificationBatchPlan::new(5, 5, drifted_scope, 5).unwrap();
        let domain = SupportDomainCertificationBundle::new(
            first_ship_domain_rows(&generic),
            drifted_domain_plan,
            SupportDomainCertificationCounterSnapshot::new(5, 3, 2, 6, 4, 1, 2),
        )
        .unwrap();
        let handoff = SupportCertificationHandoffReport::from_generic_and_domain_certification(
            &generic, &domain,
        )
        .unwrap();
        let error = SubscriptionSupportAccuracyCertificationRunner::production()
            .certify(
                &evidence_bundle,
                &generic,
                &domain,
                &handoff,
                &phase7_lane_evidence(),
            )
            .expect_err(
                "runner closeout must reject a valid domain bundle whose exact counter regime drifted",
            );

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustCoverageMissing
        );
    }

    #[test]
    fn phase7_named_suite_rejects_missing_required_row() {
        let (evidence_bundle, generic, domain, handoff) = phase7_suite_artifacts();
        let lane_evidence = phase7_lane_evidence();
        let mut rows = phase7_required_suite_rows(
            &evidence_bundle,
            &generic,
            &domain,
            &handoff,
            &lane_evidence,
        );
        rows.retain(|row| {
            row.row_kind()
                != SubscriptionSupportAccuracyCertificationRowKind::DomainAiDegradedSupportTrust
        });

        let error = SubscriptionSupportAccuracyCertificationSuite::from_rows_and_phase_artifacts(
            rows,
            &evidence_bundle,
            &generic,
            &domain,
            &handoff,
            &lane_evidence,
        )
        .expect_err("missing named Phase 7 row must reject suite completion");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustCoverageMissing
        );
    }

    #[test]
    fn phase7_named_suite_rejects_duplicate_required_row() {
        let (evidence_bundle, generic, domain, handoff) = phase7_suite_artifacts();
        let lane_evidence = phase7_lane_evidence();
        let mut rows = phase7_required_suite_rows(
            &evidence_bundle,
            &generic,
            &domain,
            &handoff,
            &lane_evidence,
        );
        rows.push(
            SubscriptionSupportAccuracyCertificationRow::new(
                SubscriptionSupportAccuracyCertificationRowKind::ExactSupportTrustedControl,
                rows.iter()
                    .find(|row| {
                        row.row_kind()
                            == SubscriptionSupportAccuracyCertificationRowKind::ExactSupportTrustedControl
                    })
                    .unwrap()
                    .row_digest(),
                0,
                0,
            )
            .unwrap(),
        );

        let error = SubscriptionSupportAccuracyCertificationSuite::from_rows_and_phase_artifacts(
            rows,
            &evidence_bundle,
            &generic,
            &domain,
            &handoff,
            &lane_evidence,
        )
        .expect_err("duplicate named Phase 7 row must reject suite completion");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustCoverageMissing
        );
    }

    #[test]
    fn phase7_named_suite_rejects_tampered_artifact_row_evidence() {
        let (evidence_bundle, generic, domain, handoff) = phase7_suite_artifacts();
        let lane_evidence = phase7_lane_evidence();
        let mut rows = phase7_required_suite_rows(
            &evidence_bundle,
            &generic,
            &domain,
            &handoff,
            &lane_evidence,
        );
        let exact = rows
            .iter_mut()
            .find(|row| {
                row.row_kind()
                    == SubscriptionSupportAccuracyCertificationRowKind::ExactSupportTrustedControl
            })
            .expect("exact suite row should be present");
        *exact = SubscriptionSupportAccuracyCertificationRow::new(
            SubscriptionSupportAccuracyCertificationRowKind::ExactSupportTrustedControl,
            "tampered:exact-control:not-from-phase-artifacts",
            0,
            0,
        )
        .unwrap();

        let error = SubscriptionSupportAccuracyCertificationSuite::from_rows_and_phase_artifacts(
            rows,
            &evidence_bundle,
            &generic,
            &domain,
            &handoff,
            &lane_evidence,
        )
        .expect_err("suite row evidence must be recomputed from supplied artifacts");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustCoverageMissing
        );
    }

    #[test]
    fn phase7_named_suite_rejects_overclaim_or_global_scan_debt_rows() {
        let exact_overclaim = SubscriptionSupportAccuracyCertificationRow::new(
            SubscriptionSupportAccuracyCertificationRowKind::ForbiddenExactOverclaimZero,
            "phase7:evidence:overclaim",
            1,
            0,
        )
        .expect_err("exact overclaim counter must reject named suite row");
        let global_scan = SubscriptionSupportAccuracyCertificationRow::new(
            SubscriptionSupportAccuracyCertificationRowKind::GlobalScanDebtForbidden,
            "phase7:evidence:global-scan",
            0,
            1,
        )
        .expect_err("global scan debt counter must reject named suite row");

        assert_eq!(
            exact_overclaim.kind(),
            SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim
        );
        assert_eq!(
            global_scan.kind(),
            SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim
        );
    }

    #[test]
    fn phase7_named_suite_rejects_missing_hostile_lane_evidence() {
        let mut lanes = phase7_lane_evidence().lanes().to_vec();
        lanes.retain(|lane| {
            lane.row_kind()
                != SubscriptionSupportAccuracyCertificationRowKind::CompatibilityDriftRejectsExactTrust
        });
        let error = SubscriptionSupportAccuracyLaneEvidenceSet::new(lanes)
            .expect_err("every hostile named suite row requires explicit lane evidence");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustCoverageMissing
        );
    }

    #[test]
    fn phase7_named_suite_rejects_misclassified_hostile_lane_outcome() {
        let wrong_failure = phase7_expected_lane_failure(
            SubscriptionSupportAccuracyCertificationRowKind::ImportedSupportMissingBasisNotResumable,
        );
        let error = SubscriptionSupportAccuracyLaneEvidence::typed_rejection_from_failure(
            SubscriptionSupportAccuracyCertificationRowKind::CompatibilityDriftRejectsExactTrust,
            &wrong_failure,
        )
        .expect_err("compatibility drift lane must carry compatibility mismatch evidence");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustCoverageMissing
        );
    }

    #[test]
    fn phase7_certified_pass_lane_derives_from_matching_certified_report() {
        let replicated = phase7_certified_transformed_exact_report(
            SupportTrustProvenance::Replicated,
            "row:phase7:replicated-exact",
        );
        let lane = SubscriptionSupportAccuracyLaneEvidence::certified_pass_from_report(
            SubscriptionSupportAccuracyCertificationRowKind::ReplicatedSupportExactEquivalence,
            &replicated,
            "phase7:diagnostics:replicated",
            "phase7:counter:replicated",
        )
        .expect("replicated exact suite lane must derive from certified replicated exact report");

        assert_eq!(
            lane.row_kind(),
            SubscriptionSupportAccuracyCertificationRowKind::ReplicatedSupportExactEquivalence
        );
        assert!(!lane.evidence_digest().is_empty());
    }

    #[test]
    fn phase7_certified_pass_lane_rejects_report_posture_drift() {
        let replicated = phase7_certified_transformed_exact_report(
            SupportTrustProvenance::Replicated,
            "row:phase7:replicated-exact",
        );
        let error = SubscriptionSupportAccuracyLaneEvidence::certified_pass_from_report(
            SubscriptionSupportAccuracyCertificationRowKind::MigratedSupportExactEquivalence,
            &replicated,
            "phase7:diagnostics:wrong-posture",
            "phase7:counter:wrong-posture",
        )
        .expect_err("migrated exact suite lane cannot consume replicated exact certification");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustCoverageMissing
        );
    }

    #[test]
    fn phase7_counter_pass_lanes_derive_from_zero_debt_evidence_bundle() {
        let (evidence_bundle, _, _, _) = phase7_suite_artifacts();
        let overclaim =
            SubscriptionSupportAccuracyLaneEvidence::certified_counter_pass_from_evidence_bundle(
                SubscriptionSupportAccuracyCertificationRowKind::ForbiddenExactOverclaimZero,
                &evidence_bundle,
            )
            .expect("zero exact-overclaim counter lane must derive from evidence bundle counters");
        let global_scan =
            SubscriptionSupportAccuracyLaneEvidence::certified_counter_pass_from_evidence_bundle(
                SubscriptionSupportAccuracyCertificationRowKind::GlobalScanDebtForbidden,
                &evidence_bundle,
            )
            .expect("zero global-scan counter lane must derive from evidence bundle counters");
        let wrong_row =
            SubscriptionSupportAccuracyLaneEvidence::certified_counter_pass_from_evidence_bundle(
                SubscriptionSupportAccuracyCertificationRowKind::ReplicatedSupportExactEquivalence,
                &evidence_bundle,
            )
            .expect_err("non-counter suite rows cannot be certified from bundle counters");

        assert_eq!(
            overclaim.row_kind(),
            SubscriptionSupportAccuracyCertificationRowKind::ForbiddenExactOverclaimZero
        );
        assert_eq!(
            global_scan.row_kind(),
            SubscriptionSupportAccuracyCertificationRowKind::GlobalScanDebtForbidden
        );
        assert_eq!(
            wrong_row.kind(),
            SupportTrustFailureKind::SupportTrustCoverageMissing
        );
    }

    #[test]
    fn phase7_rejection_lane_digest_is_bound_to_failure_evidence() {
        let catalog_failure =
            SupportCatalogEpoch::new(0).expect_err("zero catalog epoch must reject");
        let ledger_failure = SupportOperationalLedgerEpoch::new(0)
            .expect_err("zero operational ledger epoch must reject");

        let catalog_lane = SubscriptionSupportAccuracyLaneEvidence::typed_rejection_from_failure(
            SubscriptionSupportAccuracyCertificationRowKind::StaleSupportRejected,
            &catalog_failure,
        )
        .unwrap();
        let ledger_lane = SubscriptionSupportAccuracyLaneEvidence::typed_rejection_from_failure(
            SubscriptionSupportAccuracyCertificationRowKind::StaleSupportRejected,
            &ledger_failure,
        )
        .unwrap();

        assert_ne!(
            catalog_lane.evidence_digest(),
            ledger_lane.evidence_digest()
        );
    }

    #[test]
    fn exact_trust_is_strength_and_provenance_not_one_overloaded_enum() {
        let witness = ExactSupportTrustWitness::from_exact_translation(
            exact_translation(),
            SupportTrustProvenance::NativePublished,
            SupportTrustFreshnessWitness::new(epochs()),
        )
        .unwrap();
        let report = OperationalSupportTrustReport::from_exact_witness(witness);

        assert_eq!(report.trust_strength(), SupportTrustStrength::Exact);
        assert_eq!(report.provenance(), SupportTrustProvenance::NativePublished);
        assert_eq!(report.trust_class(), SupportTrustClass::ExactSupportTrusted);
        assert_eq!(
            report.use_boundary(),
            SupportTrustUseBoundary::StoreLocalOperational
        );
    }

    #[test]
    fn replicated_exact_requires_equivalence_witness() {
        let error = ExactSupportTrustWitness::from_exact_translation(
            exact_translation(),
            SupportTrustProvenance::Replicated,
            SupportTrustFreshnessWitness::new(epochs()),
        )
        .expect_err("replicated exact trust requires an equivalence witness");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustEquivalenceMissing
        );
        assert_eq!(
            error.recovery_posture(),
            SupportTrustRecoveryPosture::RetryWithFresherReceipts
        );
    }

    #[test]
    fn transformed_exact_trust_requires_family_bound_equivalence() {
        let translation = exact_translation();
        let equivalence = SupportTrustEquivalenceWitness::new(
            translation.basis().clone(),
            SubscriptionSupportFamilyId::new("replicated-continuation-support").unwrap(),
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            "equivalence:digest",
        )
        .unwrap();

        let witness = ExactSupportTrustWitness::from_equivalent_operational_basis(
            translation,
            SupportTrustProvenance::Replicated,
            SupportTrustFreshnessWitness::new(epochs()),
            equivalence,
        )
        .unwrap();

        assert_eq!(witness.trust().strength(), SupportTrustStrength::Exact);
        assert_eq!(
            witness.trust().provenance(),
            SupportTrustProvenance::Replicated
        );
    }

    #[test]
    fn transformed_exact_trust_rejects_unbound_equivalence() {
        let error = ExactSupportTrustWitness::from_equivalent_operational_basis(
            exact_translation(),
            SupportTrustProvenance::Replicated,
            SupportTrustFreshnessWitness::new(epochs()),
            SupportTrustEquivalenceWitness::new(
                SubscriptionSupportOperationalBasis::new(
                    SubscriptionSupportFamilyId::new("other-support-family").unwrap(),
                    SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                    SubscriptionSupportRole::ExactContinuation,
                    SubscriptionSupportArtifactId("artifact:trust:other".into()),
                    "basis:other",
                    "cursor:trust",
                    "checkpoint:trust",
                    "compatibility:trust",
                    "portability:trust",
                    SubscriptionSupportActionOrigin::Retention,
                )
                .unwrap(),
                SubscriptionSupportFamilyId::new("replicated-continuation-support").unwrap(),
                SubscriptionSupportOperationalVerdict::ExactResumePreserved,
                "equivalence:digest",
            )
            .unwrap(),
        )
        .expect_err("equivalence proof must be family-bound");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustBasisMismatch
        );
    }

    #[test]
    fn certified_report_requires_certification_epoch_and_stamp() {
        let translation = exact_translation();
        let family_id = translation.basis().family_id().clone();
        let support_role = translation.basis().support_role();
        let witness = ExactSupportTrustWitness::from_exact_translation(
            translation,
            SupportTrustProvenance::NativePublished,
            SupportTrustFreshnessWitness::new(epochs()),
        )
        .unwrap();
        let operational = OperationalSupportTrustReport::from_exact_witness(witness);
        let stamp = SupportTrustCertificationStamp::new(
            SupportCertificationCorpusVersion::new("corpus:13.3").unwrap(),
            SupportCertificationEpoch::new(11).unwrap(),
            "suite:13.3-phase-1",
            family_id,
            support_role,
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            "row:exact-control",
            "bundle:digest",
        )
        .unwrap();

        let certified = CertifiedSupportTrustReport::from_operational_report(operational, stamp)
            .expect("matching certification epoch should certify operational trust");

        assert_eq!(
            certified.use_boundary(),
            SupportTrustUseBoundary::CertifiedPlatform
        );
        assert_eq!(
            certified.certification_stamp().row_id(),
            "row:exact-control"
        );
    }

    #[test]
    fn certification_stamp_must_match_operational_report_scope() {
        let witness = ExactSupportTrustWitness::from_exact_translation(
            exact_translation(),
            SupportTrustProvenance::NativePublished,
            SupportTrustFreshnessWitness::new(epochs()),
        )
        .unwrap();
        let operational = OperationalSupportTrustReport::from_exact_witness(witness);
        let stamp = SupportTrustCertificationStamp::new(
            SupportCertificationCorpusVersion::new("corpus:13.3").unwrap(),
            SupportCertificationEpoch::new(11).unwrap(),
            "suite:13.3-phase-1",
            SubscriptionSupportFamilyId::new("other-support-family").unwrap(),
            SubscriptionSupportRole::ExactContinuation,
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            "row:exact-control",
            "bundle:digest",
        )
        .unwrap();

        let error = CertifiedSupportTrustReport::from_operational_report(operational, stamp)
            .expect_err("certification coverage must be family-scoped");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustFamilyMismatch
        );
    }

    #[test]
    fn phase5_certification_coverage_witness_enables_certified_exact_trust() {
        let classified = classify_phase2(
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        )
        .unwrap();
        let bundle = first_ship_certification_bundle();
        let evidence_bundle_digest = bundle.evidence_bundle_digest().to_string();
        let stamp = SupportTrustCertificationStamp::new(
            SupportCertificationCorpusVersion::new("corpus:13.3").unwrap(),
            SupportCertificationEpoch::new(11).unwrap(),
            "suite:13.3-phase-5",
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportRole::ExactContinuation,
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            "row:basis-bound-exact",
            evidence_bundle_digest,
        )
        .unwrap();

        let coverage_checked = check_support_trust_coverage(classified, bundle).unwrap();
        let certified = classify_certified_support_trust(coverage_checked, stamp).unwrap();

        assert_eq!(
            certified.report().trust_class(),
            SupportTrustClass::ExactSupportTrusted
        );
        assert_eq!(
            certified.report().certification_stamp().row_id(),
            "row:basis-bound-exact"
        );
        assert_eq!(certified.coverage_witness().summary().row_count(), 4);
    }

    #[test]
    fn phase5_certification_rejects_stamp_not_bound_to_checked_bundle() {
        let classified = classify_phase2(
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        )
        .unwrap();
        let coverage_checked =
            check_support_trust_coverage(classified, first_ship_certification_bundle()).unwrap();
        let stamp = SupportTrustCertificationStamp::new(
            SupportCertificationCorpusVersion::new("corpus:13.3").unwrap(),
            SupportCertificationEpoch::new(11).unwrap(),
            "suite:13.3-phase-5",
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportRole::ExactContinuation,
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            "row:basis-bound-exact",
            "bundle:digest:forged",
        )
        .unwrap();

        let error = classify_certified_support_trust(coverage_checked, stamp)
            .expect_err("certification stamp must name the checked evidence bundle");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustCoverageMissing
        );
    }

    #[test]
    fn phase5_certification_rejects_stamp_for_different_covered_row() {
        let classified = classify_phase2(
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        )
        .unwrap();
        let bundle = first_ship_certification_bundle();
        let evidence_bundle_digest = bundle.evidence_bundle_digest().to_string();
        let coverage_checked = check_support_trust_coverage(classified, bundle).unwrap();
        let stamp = SupportTrustCertificationStamp::new(
            SupportCertificationCorpusVersion::new("corpus:13.3").unwrap(),
            SupportCertificationEpoch::new(11).unwrap(),
            "suite:13.3-phase-5",
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportRole::ExactContinuation,
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            "row:materialized-narrowing-exact",
            evidence_bundle_digest,
        )
        .unwrap();

        let error = classify_certified_support_trust(coverage_checked, stamp)
            .expect_err("certification stamp row id must name the row that covered the report");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustCoverageMissing
        );
    }

    #[test]
    fn phase5_certification_coverage_is_artifact_and_basis_bound() {
        let classified = classify_phase2(
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        )
        .unwrap();
        let bundle = SupportCertificationEvidenceBundle::new(
            "run:13.3:wrong-artifact",
            first_ship_certification_matrix_for_basis_artifact("artifact:first-ship:other"),
            first_ship_batch_scope(),
            first_ship_counter_snapshot(),
        )
        .unwrap();

        let error = check_support_trust_coverage(classified, bundle)
            .expect_err("same family and posture cannot cover a different support artifact");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustCoverageMissing
        );
    }

    #[test]
    fn phase5_certification_matrix_rejects_duplicate_rows() {
        let classified = classify_phase2(
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        )
        .unwrap();
        let plan = exact_certification_plan("row:exact-control");

        let error = SupportCertificationCoverageMatrix::from_rows(
            &plan,
            vec![
                exact_certification_row("row:exact-control", &classified),
                exact_certification_row("row:exact-control", &classified),
            ],
        )
        .expect_err("duplicate certification rows cannot complete coverage");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustCoverageMissing
        );
    }

    #[test]
    fn phase5_certification_gap_report_names_missing_required_rows() {
        let classified = classify_phase2(
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        )
        .unwrap();
        let plan = SubscriptionSupportCertificationCoveragePlan::new(
            SupportOperationalLedgerEpoch::new(7).unwrap(),
            SupportCertificationEpoch::new(11).unwrap(),
            vec![
                exact_certification_requirement("row:exact-control"),
                exact_certification_requirement("row:hostile-stale"),
            ],
        )
        .unwrap();
        let rows = vec![exact_certification_row("row:exact-control", &classified)];
        let gap = SupportCertificationGapReport::from_plan_and_rows(&plan, &rows);

        assert_eq!(gap.missing_row_ids(), &["row:hostile-stale".to_string()]);
        let error = SupportCertificationCoverageMatrix::from_rows(&plan, rows)
            .expect_err("missing required rows cannot complete coverage");
        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustCoverageMissing
        );
    }

    #[test]
    fn phase5_certification_rows_reject_self_comparison() {
        let error = SupportCertificationLaneDigestSet::new("lane:same", "lane:same", "lane:replay")
            .expect_err("control and hostile lanes cannot be the same run");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustCoverageMissing
        );
    }

    #[test]
    fn phase5_certification_matrix_rejects_mislabeled_trust_posture() {
        let classified = classify_phase2(
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        )
        .unwrap();
        let plan = SubscriptionSupportCertificationCoveragePlan::new(
            SupportOperationalLedgerEpoch::new(7).unwrap(),
            SupportCertificationEpoch::new(11).unwrap(),
            vec![SupportCertificationRowRequirement::new(
                "row:exact-control",
                SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                SubscriptionSupportRole::ExactContinuation,
                SupportTrustClass::DegradedSupportTrusted,
                SupportTrustStrength::Degraded,
                SupportTrustProvenance::NativePublished,
                SubscriptionSupportOperationalVerdict::ExactResumePreserved,
                SubscriptionResumeClassification::Exact,
                None,
            )
            .unwrap()],
        )
        .unwrap();

        let error = SupportCertificationCoverageMatrix::from_rows(
            &plan,
            vec![exact_certification_row("row:exact-control", &classified)],
        )
        .expect_err("row labels cannot certify a different trust posture");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustCoverageMissing
        );
    }

    #[test]
    fn phase5_certification_row_rejects_digest_mismatch() {
        let classified = classify_phase2(
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        )
        .unwrap();
        let evidence = SupportCertificationRowEvidence::from_operational_report(
            "row:exact-control",
            classified.report(),
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            SupportOperationalLedgerEpoch::new(7).unwrap(),
            SupportCertificationEpoch::new(11).unwrap(),
            certification_lanes(),
            "artifact:digest:exact",
            "subscription-support:digest:exact",
            "diagnostics:digest:exact",
            None,
            Vec::new(),
        )
        .unwrap()
        .with_declared_row_digest("digest:forged")
        .unwrap();

        let error = SupportCertificationRow::new(evidence)
            .expect_err("declared row digest must recompute from structured evidence");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustCoverageMissing
        );
    }

    #[test]
    fn phase5_certification_bundle_emits_first_ship_machine_checkable_outputs() {
        let matrix = first_ship_certification_matrix();
        let summary_digest = matrix.summary().certification_summary_digest().to_string();
        let bundle = SupportCertificationEvidenceBundle::new(
            "run:13.3:first-ship",
            matrix,
            first_ship_batch_scope(),
            first_ship_counter_snapshot(),
        )
        .unwrap();

        assert_eq!(bundle.certification_summary_digest(), summary_digest);
        assert!(!bundle.evidence_bundle_digest().is_empty());
        assert_eq!(bundle.counter_snapshot().coverage_row_count(), 4);
        assert_eq!(bundle.counter_snapshot().first_ship_family_count(), 4);
        assert_eq!(bundle.counter_snapshot().receipt_reuse_count(), 3);
    }

    #[test]
    fn phase5_certification_bundle_rejects_missing_first_ship_family() {
        let classified = classify_phase2(
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        )
        .unwrap();
        let plan = exact_certification_plan("row:exact-control");
        let matrix = SupportCertificationCoverageMatrix::from_rows(
            &plan,
            vec![exact_certification_row("row:exact-control", &classified)],
        )
        .unwrap();
        let error = SupportCertificationEvidenceBundle::new(
            "run:13.3:incomplete",
            matrix,
            SupportCertificationBatchScope::new(
                SupportCertificationBatchScopeKind::CertificationScopeLocal,
                SupportTrustDensityClass::CertificationScopeLocal,
                SupportTrustPathClass::BatchCertificationPath,
                SupportTrustAllocationScope::BatchCertification,
                1,
                1,
                0,
                1,
            )
            .unwrap(),
            SupportCertificationCounterSnapshot::new(1, 1, 0, 1, 1, 0, 0),
        )
        .expect_err("first-ship bundle must cover all required support families");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustCoverageMissing
        );
    }

    #[test]
    fn phase5_certification_bundle_rejects_impostor_first_ship_family_id() {
        let matrix = first_ship_certification_matrix_for_basis_artifact_and_materialized_family(
            "artifact:trust:phase-1",
            "materialized-narrowing-support-impostor",
        );
        let error = SupportCertificationEvidenceBundle::new(
            "run:13.3:impostor-family",
            matrix,
            first_ship_batch_scope(),
            first_ship_counter_snapshot(),
        )
        .expect_err("first-ship coverage must name canonical family ids, not only family kinds");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustCoverageMissing
        );
    }

    #[test]
    fn phase5_certification_bundle_rejects_receipt_reuse_counter_mismatch() {
        let matrix = first_ship_certification_matrix();
        let error = SupportCertificationEvidenceBundle::new(
            "run:13.3:bad-counters",
            matrix,
            first_ship_batch_scope(),
            SupportCertificationCounterSnapshot::new(4, 4, 2, 4, 1, 0, 0),
        )
        .expect_err("counter snapshot must prove declared receipt reuse");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustCoverageMissing
        );
    }

    #[test]
    fn phase5_certification_batch_scope_rejects_foreground_or_mismatched_density() {
        let error = SupportCertificationBatchScope::new(
            SupportCertificationBatchScopeKind::CertificationScopeLocal,
            SupportTrustDensityClass::FamilyLocal,
            SupportTrustPathClass::BatchCertificationPath,
            SupportTrustAllocationScope::BatchCertification,
            4,
            4,
            3,
            1,
        )
        .expect_err("certification scope batches must declare certification-scope density");
        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustAccessStructureDebt
        );

        let error = SupportCertificationBatchScope::new(
            SupportCertificationBatchScopeKind::CertificationScopeLocal,
            SupportTrustDensityClass::CertificationScopeLocal,
            SupportTrustPathClass::ForegroundResumeTrustPath,
            SupportTrustAllocationScope::ForegroundScratch,
            4,
            4,
            3,
            1,
        )
        .expect_err("foreground resume paths cannot build certification bundles");
        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustAccessStructureDebt
        );
    }

    #[test]
    fn uncertified_posture_preserves_operational_boundary() {
        let witness = ExactSupportTrustWitness::from_exact_translation(
            exact_translation(),
            SupportTrustProvenance::NativePublished,
            SupportTrustFreshnessWitness::new(epochs()),
        )
        .unwrap();
        let report = OperationalSupportTrustReport::from_exact_witness(witness);
        let posture = UncertifiedSupportTrustPosture::new(report);

        assert_eq!(
            posture.report().use_boundary(),
            SupportTrustUseBoundary::StoreLocalOperational
        );
    }

    #[test]
    fn mismatched_translation_lowers_to_rejected_for_drift_audit() {
        let plan = SupportTrustTranslationPlan::from_resume_and_operational(
            basis(),
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::DegradedResumePreserved,
        )
        .expect("translation should preserve mismatched receipts for drift localization");

        assert!(matches!(plan, SupportTrustTranslationPlan::Rejected { .. }));
    }

    #[test]
    fn performance_plan_rejects_store_global_and_foreground_certification() {
        let error = SupportTrustPerformancePlan::new(
            SupportTrustPathClass::BatchCertificationPath,
            SupportTrustDensityClass::StoreGlobalRejected,
            SupportTrustAccessPath::Rejected,
            SupportTrustAllocationScope::BatchCertification,
            1,
            1,
            0,
            0,
            SupportTrustCloneBoundary::NoClone,
        )
        .expect_err("store-global trust work is rejected in Phase 1");
        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustAccessStructureDebt
        );

        let error = SupportTrustPerformancePlan::new(
            SupportTrustPathClass::ForegroundResumeTrustPath,
            SupportTrustDensityClass::SingleSupportArtifact,
            SupportTrustAccessPath::PointLookup,
            SupportTrustAllocationScope::BatchCertification,
            1,
            1,
            0,
            0,
            SupportTrustCloneBoundary::NoClone,
        )
        .expect_err("foreground resume trust cannot allocate in batch certification scope");
        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustPayloadBudgetExceeded
        );
    }

    #[test]
    fn performance_plan_requires_clone_boundary_when_clone_count_is_nonzero() {
        let error = SupportTrustPerformancePlan::new(
            SupportTrustPathClass::BatchCertificationPath,
            SupportTrustDensityClass::CertificationScopeLocal,
            SupportTrustAccessPath::BatchLookup,
            SupportTrustAllocationScope::BatchCertification,
            3,
            8,
            2,
            1,
            SupportTrustCloneBoundary::NoClone,
        )
        .expect_err("clone count must name a semantic boundary");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustPayloadBudgetExceeded
        );
    }

    #[test]
    fn classification_plan_carries_role_epoch_and_performance_contract() {
        let posture = SupportRoleTrustPosture::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportRole::ExactContinuation,
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
        );
        let performance_plan = SupportTrustPerformancePlan::new(
            SupportTrustPathClass::ForegroundResumeTrustPath,
            SupportTrustDensityClass::SingleSupportArtifact,
            SupportTrustAccessPath::PointLookup,
            SupportTrustAllocationScope::ForegroundScratch,
            1,
            1,
            0,
            0,
            SupportTrustCloneBoundary::NoClone,
        )
        .unwrap();
        let plan = SupportTrustClassificationPlan::new(posture.clone(), epochs(), performance_plan);
        let report = SupportTrustClassificationReport::from_plan(
            plan,
            SupportTrustClass::ExactSupportTrusted,
            None,
        );

        assert_eq!(report.posture(), &posture);
        assert_eq!(report.epoch(), epochs());
        assert_eq!(report.trust_class(), SupportTrustClass::ExactSupportTrusted);
    }

    #[test]
    fn access_structure_and_evidence_budgets_reject_unbounded_shapes() {
        let access_error = SupportTrustAccessStructurePlan::new(
            SupportTrustAccessIndexKind::CertificationRow,
            SupportTrustAccessPath::Rejected,
            "certification-index-rebuild",
            "certification-epoch",
            "support_trust_certification_row_reads",
        )
        .expect_err("required trust indexes cannot lower to rejected access");

        assert_eq!(
            access_error.kind(),
            SupportTrustFailureKind::SupportTrustAccessStructureDebt
        );

        let budget = SupportTrustEvidenceBudget::new(1024, 4, 2).unwrap();
        assert!(budget.admits(1024, 4, 2));
        assert!(!budget.admits(1025, 4, 2));
    }

    #[test]
    fn phase2_exact_pipeline_classifies_only_after_receipts_and_checks() {
        let classified = classify_phase2(
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        )
        .unwrap();

        assert_eq!(
            classified.report().trust_strength(),
            SupportTrustStrength::Exact
        );
        assert_eq!(classified.counter_snapshot().exact_trust_count(), 1);
        assert_eq!(classified.cost_surface().receipts_consumed(), 8);
        assert_eq!(classified.cost_surface().drift_checks_performed(), 8);
        assert_eq!(classified.cost_surface().index_probes(), 2);
    }

    #[test]
    fn phase2_degraded_pipeline_cannot_satisfy_exact_request() {
        let error = classify_phase2(
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            SubscriptionResumeClassification::Degraded,
            SubscriptionSupportOperationalVerdict::DegradedResumePreserved,
        )
        .expect_err("degraded receipts cannot satisfy exact trust requests");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim
        );
    }

    #[test]
    fn phase2_rebuild_pipeline_reports_rebuild_only_for_rebuild_request() {
        let classified = classify_phase2(
            SupportTrustStrength::RebuildOnly,
            SupportTrustProvenance::Rebuilt,
            SubscriptionResumeClassification::RebuildRequired,
            SubscriptionSupportOperationalVerdict::RebuildRequired,
        )
        .unwrap();

        assert_eq!(
            classified.report().trust_strength(),
            SupportTrustStrength::RebuildOnly
        );
        assert_eq!(
            classified.counter_snapshot().rebuild_derived_trust_count(),
            1
        );
    }

    #[test]
    fn phase2_drift_check_rejects_digest_mismatch_before_classification() {
        let bundle = SupportTrustReceiptBundle::new(
            SupportResumeClassificationReceipt::new(
                SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
                SubscriptionResumeClassification::Exact,
                "resume:proof",
                SupportTrustReceiptStatus::Proven,
            )
            .unwrap(),
            SupportOperationalVerdictReceipt::new(
                basis(),
                SubscriptionSupportOperationalVerdict::ExactResumePreserved,
                "operational:proof",
                SupportTrustReceiptStatus::Proven,
            )
            .unwrap(),
            family_role_receipt(),
            SupportBasisReceipt::new(
                SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
                "basis:wrong",
                SupportTrustReceiptStatus::Proven,
            )
            .unwrap(),
            SupportCursorCheckpointReceipt::new(
                SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
                "cursor:trust:checkpoint:trust",
                SupportTrustReceiptStatus::Proven,
            )
            .unwrap(),
            SupportCompatibilityReceipt::new(
                SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
                "compatibility:trust",
                SupportTrustReceiptStatus::Proven,
            )
            .unwrap(),
            SupportPortabilityReceipt::new(
                SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
                "portability:trust",
                SupportTrustReceiptStatus::Proven,
            )
            .unwrap(),
        )
        .with_retention(
            SupportRetentionReceipt::new(
                SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
                "retention:trust",
                SupportTrustReceiptStatus::Proven,
            )
            .unwrap(),
        );
        let admitted = admit_support_trust_request(
            raw_phase2_request(
                SupportTrustStrength::Exact,
                SupportTrustProvenance::NativePublished,
            ),
            bundle,
        )
        .unwrap();
        let translated = translate_support_trust_inputs(admitted).unwrap();

        let error = check_support_trust_drift(
            translated,
            SupportTrustDriftScanPlan::foreground_support_identity(),
        )
        .expect_err("drift check must reject stale basis receipt");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustBasisMismatch
        );
        let drift_report = error
            .drift_report()
            .expect("basis drift failures must retain the deterministic drift report");
        assert_eq!(
            drift_report.primary_cause(),
            Some(SupportTrustDriftCause::Basis)
        );
        assert_eq!(
            drift_report.staleness_verdict(),
            SupportStalenessVerdict::StaleRejected
        );
    }

    #[test]
    fn phase4_multi_drift_report_orders_primary_and_suppressed_causes() {
        let plan = SupportTrustDriftScanPlan::foreground_support_identity();
        let report = SupportTrustDriftReport::from_observed_causes(
            &plan,
            [
                (
                    SupportTrustDriftCause::Portability,
                    SupportTrustDriftLocality::SupportIdentity,
                ),
                (
                    SupportTrustDriftCause::Basis,
                    SupportTrustDriftLocality::BasisLocal,
                ),
                (
                    SupportTrustDriftCause::Compatibility,
                    SupportTrustDriftLocality::CompatibilityEpoch,
                ),
            ],
        );

        assert_eq!(report.primary_cause(), Some(SupportTrustDriftCause::Basis));
        assert_eq!(
            report
                .suppressed_causes()
                .iter()
                .map(SupportTrustSuppressedCause::cause)
                .collect::<Vec<_>>(),
            vec![
                SupportTrustDriftCause::Compatibility,
                SupportTrustDriftCause::Portability
            ]
        );
        assert_eq!(
            report.staleness_verdict(),
            SupportStalenessVerdict::StaleRejected
        );
    }

    #[test]
    fn phase4_certification_coverage_drift_rejects_platform_scope() {
        let admitted = admit_support_trust_request(
            raw_phase2_request(
                SupportTrustStrength::Exact,
                SupportTrustProvenance::NativePublished,
            ),
            phase2_receipts(
                SubscriptionResumeClassification::Exact,
                SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            ),
        )
        .unwrap();
        let translated = translate_support_trust_inputs(admitted).unwrap();
        let plan = SupportTrustDriftScanPlan::certification_scope(
            SupportTrustPathClass::BatchCertificationPath,
            9,
            2,
            false,
        )
        .unwrap();

        let error = check_support_trust_drift(translated, plan)
            .expect_err("missing certification coverage rejects platform trust");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustCoverageMissing
        );
        let drift_report = error
            .drift_report()
            .expect("coverage drift failures must retain the deterministic drift report");
        assert_eq!(
            drift_report.primary_cause(),
            Some(SupportTrustDriftCause::CertificationCoverage)
        );
        assert_eq!(drift_report.coverage_drift_count(), 1);
    }

    #[test]
    fn phase4_operational_verdict_drift_is_reachable_and_audited() {
        let admitted = admit_support_trust_request(
            raw_phase2_request(
                SupportTrustStrength::Exact,
                SupportTrustProvenance::NativePublished,
            ),
            phase2_receipts(
                SubscriptionResumeClassification::Exact,
                SubscriptionSupportOperationalVerdict::DegradedResumePreserved,
            ),
        )
        .unwrap();
        let translated = translate_support_trust_inputs(admitted).unwrap();

        let error = check_support_trust_drift(
            translated,
            SupportTrustDriftScanPlan::foreground_support_identity(),
        )
        .expect_err("resume/operational disagreement must localize as operational drift");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustOperationalVerdictMismatch
        );
        let drift_report = error
            .drift_report()
            .expect("operational drift failures must retain the deterministic drift report");
        assert_eq!(
            drift_report.primary_cause(),
            Some(SupportTrustDriftCause::OperationalVerdict)
        );
        assert_eq!(drift_report.stale_rejection_count(), 1);
    }

    #[test]
    fn phase4_placement_cost_drift_is_advisory_only() {
        let admitted = admit_support_trust_request(
            raw_phase2_request(
                SupportTrustStrength::Exact,
                SupportTrustProvenance::NativePublished,
            ),
            phase2_receipts(
                SubscriptionResumeClassification::Exact,
                SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            ),
        )
        .unwrap();
        let translated = translate_support_trust_inputs(admitted).unwrap();
        let plan = SupportTrustDriftScanPlan::new(
            SupportTrustDriftLocality::PlacementCostAdvisory,
            SupportTrustPathClass::ForegroundResumeTrustPath,
            8,
            1,
        )
        .unwrap();
        let drift_checked = check_support_trust_drift(translated, plan).unwrap();
        assert_eq!(
            drift_checked.drift_report().staleness_verdict(),
            SupportStalenessVerdict::PlacementCostAdvisory
        );
        let equivalence_checked =
            check_support_trust_equivalence(drift_checked, SupportTrustEquivalenceEvidence::none())
                .unwrap();
        let classified = classify_operational_support_trust(equivalence_checked).unwrap();

        assert_eq!(
            classified.report().trust_strength(),
            SupportTrustStrength::Exact
        );
        assert_eq!(classified.counter_snapshot().placement_advisory_count(), 1);
    }

    #[test]
    fn phase4_store_global_drift_scan_plan_rejects_before_execution() {
        let error = SupportTrustDriftScanPlan::new(
            SupportTrustDriftLocality::SupportIdentity,
            SupportTrustPathClass::RoadmapHandoffPath,
            8,
            1,
        )
        .expect_err("drift checks cannot hide global roadmap handoff scans");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustAccessStructureDebt
        );
    }

    #[test]
    fn phase2_transformed_exact_waits_for_phase3_equivalence() {
        let admitted = admit_support_trust_request(
            raw_phase2_request(
                SupportTrustStrength::Exact,
                SupportTrustProvenance::Replicated,
            ),
            phase2_receipts(
                SubscriptionResumeClassification::Exact,
                SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            ),
        )
        .unwrap();
        let translated = translate_support_trust_inputs(admitted).unwrap();
        let drift_checked = check_support_trust_drift(
            translated,
            SupportTrustDriftScanPlan::foreground_support_identity(),
        )
        .unwrap();

        let error =
            check_support_trust_equivalence(drift_checked, SupportTrustEquivalenceEvidence::none())
                .expect_err("transformed exact trust requires Phase 3 equivalence proof");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustEquivalenceMissing
        );
    }

    #[test]
    fn phase3_replicated_exact_classifies_with_full_equivalence_contract() {
        let admitted = admit_support_trust_request(
            raw_phase2_request(
                SupportTrustStrength::Exact,
                SupportTrustProvenance::Replicated,
            ),
            phase2_receipts(
                SubscriptionResumeClassification::Exact,
                SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            ),
        )
        .unwrap();
        let translated = translate_support_trust_inputs(admitted).unwrap();
        let drift_checked = check_support_trust_drift(
            translated,
            SupportTrustDriftScanPlan::foreground_support_identity(),
        )
        .unwrap();
        let equivalence = SupportTrustEquivalenceEvidence::none()
            .with_contract(exact_equivalence_contract(
                SupportTrustEquivalenceLane::Replication,
            ))
            .unwrap();
        let equivalence_checked =
            check_support_trust_equivalence(drift_checked, equivalence).unwrap();
        let classified = classify_operational_support_trust(equivalence_checked).unwrap();

        assert_eq!(
            classified.report().trust_strength(),
            SupportTrustStrength::Exact
        );
        assert_eq!(
            classified.report().trust_class(),
            SupportTrustClass::ReplicatedSupportTrusted
        );
        assert_eq!(classified.cost_surface().equivalence_checks_performed(), 2);
    }

    #[test]
    fn phase3_migrated_exact_classifies_only_with_migration_equivalence() {
        let admitted = admit_support_trust_request(
            raw_phase2_request(
                SupportTrustStrength::Exact,
                SupportTrustProvenance::Migrated,
            ),
            phase2_receipts(
                SubscriptionResumeClassification::Exact,
                SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            ),
        )
        .unwrap();
        let translated = translate_support_trust_inputs(admitted).unwrap();
        let drift_checked = check_support_trust_drift(
            translated,
            SupportTrustDriftScanPlan::foreground_support_identity(),
        )
        .unwrap();
        let equivalence = SupportTrustEquivalenceEvidence::none()
            .with_contract(exact_equivalence_contract(
                SupportTrustEquivalenceLane::Migration,
            ))
            .unwrap();
        let equivalence_checked =
            check_support_trust_equivalence(drift_checked, equivalence).unwrap();
        let classified = classify_operational_support_trust(equivalence_checked).unwrap();

        assert_eq!(
            classified.report().trust_strength(),
            SupportTrustStrength::Exact
        );
        assert_eq!(
            classified.report().trust_class(),
            SupportTrustClass::MigratedSupportTrusted
        );
    }

    #[test]
    fn phase3_imported_exact_requires_admission_and_import_equivalence() {
        let bundle = phase2_receipts(
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        )
        .with_import_admission(
            SupportImportAdmissionReceipt::new(
                SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
                SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                "import:admission",
                SupportTrustReceiptStatus::Proven,
            )
            .unwrap(),
        );
        let request = RawSupportTrustRequest::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportRole::ExactContinuation,
            SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
            SupportTrustStrength::Exact,
            SupportTrustProvenance::Imported,
            SupportTrustRequestedUse::StoreLocalResume,
            SupportTrustBatchCardinality::SingleSupportArtifact,
            epochs(),
            phase2_performance_plan(),
            SupportTrustEvidenceBudget::new(4096, 9, 1).unwrap(),
        );
        let admitted = admit_support_trust_request(request, bundle).unwrap();
        let translated = translate_support_trust_inputs(admitted).unwrap();
        let drift_checked = check_support_trust_drift(
            translated,
            SupportTrustDriftScanPlan::foreground_support_identity(),
        )
        .unwrap();
        let equivalence = SupportTrustEquivalenceEvidence::none()
            .with_contract(exact_equivalence_contract(
                SupportTrustEquivalenceLane::Import,
            ))
            .unwrap();
        let equivalence_checked =
            check_support_trust_equivalence(drift_checked, equivalence).unwrap();
        let classified = classify_operational_support_trust(equivalence_checked).unwrap();

        assert_eq!(
            classified.report().trust_strength(),
            SupportTrustStrength::Exact
        );
        assert_eq!(
            classified.report().provenance(),
            SupportTrustProvenance::Imported
        );
    }

    #[test]
    fn phase3_equivalence_contract_rejects_role_and_portability_drift() {
        let admitted = admit_support_trust_request(
            raw_phase2_request(
                SupportTrustStrength::Exact,
                SupportTrustProvenance::Replicated,
            ),
            phase2_receipts(
                SubscriptionResumeClassification::Exact,
                SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            ),
        )
        .unwrap();
        let translated = translate_support_trust_inputs(admitted).unwrap();
        let drift_checked = check_support_trust_drift(
            translated,
            SupportTrustDriftScanPlan::foreground_support_identity(),
        )
        .unwrap();
        let bad_contract = SupportTrustEquivalenceContract::new(
            SupportTrustEquivalenceLane::Replication,
            basis(),
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportRole::DegradedContinuation,
            SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
            "basis:trust",
            "cursor:trust",
            "checkpoint:trust",
            "compatibility:trust",
            "portability:wrong",
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            "equivalence:exact",
        )
        .unwrap();
        let equivalence = SupportTrustEquivalenceEvidence::none()
            .with_contract(bad_contract)
            .unwrap();

        let error = check_support_trust_equivalence(drift_checked, equivalence)
            .expect_err("role drift must reject before digest-only equivalence can pass");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustRoleMismatch
        );
    }

    #[test]
    fn phase3_native_exact_rejects_irrelevant_equivalence_evidence() {
        let admitted = admit_support_trust_request(
            raw_phase2_request(
                SupportTrustStrength::Exact,
                SupportTrustProvenance::NativePublished,
            ),
            phase2_receipts(
                SubscriptionResumeClassification::Exact,
                SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            ),
        )
        .unwrap();
        let translated = translate_support_trust_inputs(admitted).unwrap();
        let drift_checked = check_support_trust_drift(
            translated,
            SupportTrustDriftScanPlan::foreground_support_identity(),
        )
        .unwrap();
        let equivalence = SupportTrustEquivalenceEvidence::none()
            .with_contract(exact_equivalence_contract(
                SupportTrustEquivalenceLane::Replication,
            ))
            .unwrap();

        let error = check_support_trust_equivalence(drift_checked, equivalence)
            .expect_err("native exact trust must not accept loose transformed evidence");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustEquivalenceMissing
        );
    }

    #[test]
    fn phase2_rebuild_requires_maintenance_receipt_before_translation() {
        let admitted = admit_support_trust_request(
            raw_phase2_request(
                SupportTrustStrength::RebuildOnly,
                SupportTrustProvenance::Rebuilt,
            ),
            SupportTrustReceiptBundle::new(
                SupportResumeClassificationReceipt::new(
                    SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
                    SubscriptionResumeClassification::RebuildRequired,
                    "resume:proof",
                    SupportTrustReceiptStatus::Proven,
                )
                .unwrap(),
                SupportOperationalVerdictReceipt::new(
                    basis(),
                    SubscriptionSupportOperationalVerdict::RebuildRequired,
                    "operational:proof",
                    SupportTrustReceiptStatus::Proven,
                )
                .unwrap(),
                family_role_receipt(),
                SupportBasisReceipt::new(
                    SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
                    "basis:trust",
                    SupportTrustReceiptStatus::Proven,
                )
                .unwrap(),
                SupportCursorCheckpointReceipt::new(
                    SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
                    "cursor:trust:checkpoint:trust",
                    SupportTrustReceiptStatus::Proven,
                )
                .unwrap(),
                SupportCompatibilityReceipt::new(
                    SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
                    "compatibility:trust",
                    SupportTrustReceiptStatus::Proven,
                )
                .unwrap(),
                SupportPortabilityReceipt::new(
                    SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
                    "portability:trust",
                    SupportTrustReceiptStatus::Proven,
                )
                .unwrap(),
            ),
        )
        .unwrap();

        let error = translate_support_trust_inputs(admitted)
            .expect_err("rebuild-derived support trust requires maintenance proof");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustCoverageMissing
        );
    }

    #[test]
    fn phase2_imported_support_requires_target_admission_receipt() {
        let admitted = admit_support_trust_request(
            raw_phase2_request(
                SupportTrustStrength::RebuildOnly,
                SupportTrustProvenance::Imported,
            ),
            phase2_receipts(
                SubscriptionResumeClassification::RebuildRequired,
                SubscriptionSupportOperationalVerdict::RebuildRequired,
            ),
        )
        .unwrap();

        let error = translate_support_trust_inputs(admitted)
            .expect_err("imported support requires target admission proof");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustCoverageMissing
        );
    }

    #[test]
    fn phase2_receipt_bundle_must_fit_evidence_budget() {
        let request = RawSupportTrustRequest::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportRole::ExactContinuation,
            SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            SupportTrustRequestedUse::StoreLocalResume,
            SupportTrustBatchCardinality::SingleSupportArtifact,
            epochs(),
            phase2_performance_plan(),
            SupportTrustEvidenceBudget::new(4096, 7, 1).unwrap(),
        );

        let error = admit_support_trust_request(
            request,
            phase2_receipts(
                SubscriptionResumeClassification::Exact,
                SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            ),
        )
        .expect_err("exact support proof bundle has eight receipts");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustPayloadBudgetExceeded
        );
    }

    #[test]
    fn phase2_receipt_bundle_byte_budget_rejects_oversized_proofs() {
        let request = RawSupportTrustRequest::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportRole::ExactContinuation,
            SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            SupportTrustRequestedUse::StoreLocalResume,
            SupportTrustBatchCardinality::SingleSupportArtifact,
            epochs(),
            phase2_performance_plan(),
            SupportTrustEvidenceBudget::new(16, 8, 1).unwrap(),
        );

        let error = admit_support_trust_request(
            request,
            phase2_receipts(
                SubscriptionResumeClassification::Exact,
                SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            ),
        )
        .expect_err("proof digest bytes must be accounted before translation");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustPayloadBudgetExceeded
        );
    }

    #[test]
    fn phase2_contextual_receipts_must_match_requested_artifact() {
        let admitted = admit_support_trust_request(
            raw_phase2_request(
                SupportTrustStrength::Exact,
                SupportTrustProvenance::NativePublished,
            ),
            SupportTrustReceiptBundle::new(
                SupportResumeClassificationReceipt::new(
                    SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
                    SubscriptionResumeClassification::Exact,
                    "resume:proof",
                    SupportTrustReceiptStatus::Proven,
                )
                .unwrap(),
                SupportOperationalVerdictReceipt::new(
                    basis(),
                    SubscriptionSupportOperationalVerdict::ExactResumePreserved,
                    "operational:proof",
                    SupportTrustReceiptStatus::Proven,
                )
                .unwrap(),
                family_role_receipt(),
                SupportBasisReceipt::new(
                    SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
                    "basis:trust",
                    SupportTrustReceiptStatus::Proven,
                )
                .unwrap(),
                SupportCursorCheckpointReceipt::new(
                    SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
                    "cursor:trust:checkpoint:trust",
                    SupportTrustReceiptStatus::Proven,
                )
                .unwrap(),
                SupportCompatibilityReceipt::new(
                    SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
                    "compatibility:trust",
                    SupportTrustReceiptStatus::Proven,
                )
                .unwrap(),
                SupportPortabilityReceipt::new(
                    SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
                    "portability:trust",
                    SupportTrustReceiptStatus::Proven,
                )
                .unwrap(),
            )
            .with_retention(
                SupportRetentionReceipt::new(
                    SubscriptionSupportArtifactId("artifact:trust:other".into()),
                    "retention:trust",
                    SupportTrustReceiptStatus::Proven,
                )
                .unwrap(),
            ),
        )
        .unwrap();

        let error = translate_support_trust_inputs(admitted)
            .expect_err("foreign-artifact retention proof must not satisfy exact support trust");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustBasisMismatch
        );
    }

    #[test]
    fn phase2_certified_platform_claim_waits_for_coverage_phase() {
        let request = RawSupportTrustRequest::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportRole::ExactContinuation,
            SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            SupportTrustRequestedUse::CertifiedPlatformClaim,
            SupportTrustBatchCardinality::SingleSupportArtifact,
            epochs(),
            phase2_performance_plan(),
            SupportTrustEvidenceBudget::new(4096, 8, 1).unwrap(),
        );

        let error = admit_support_trust_request(
            request,
            phase2_receipts(
                SubscriptionResumeClassification::Exact,
                SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            ),
        )
        .expect_err("operational trust admission cannot certify platform claims");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustCoverageMissing
        );
        assert_eq!(
            error.recovery_posture(),
            SupportTrustRecoveryPosture::RerunCertification
        );
    }

    #[test]
    fn phase2_batch_cardinality_must_match_declared_density() {
        let request = RawSupportTrustRequest::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportRole::ExactContinuation,
            SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            SupportTrustRequestedUse::StoreLocalResume,
            SupportTrustBatchCardinality::FamilyRoleBatch { artifact_count: 2 },
            epochs(),
            phase2_performance_plan(),
            SupportTrustEvidenceBudget::new(4096, 8, 2).unwrap(),
        );

        let error = admit_support_trust_request(
            request,
            phase2_receipts(
                SubscriptionResumeClassification::Exact,
                SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            ),
        )
        .expect_err("family-role batches cannot pretend to be scalar density");

        assert_eq!(
            error.kind(),
            SupportTrustFailureKind::SupportTrustAccessStructureDebt
        );
    }

    #[test]
    fn complexity_contracts_name_bounds_and_status() {
        let contract = SupportTrustComplexityContract::verified(
            "support_trust_classification",
            "O(index_probes + receipts + drift_checks + equivalence_checks)",
            1,
            4,
            0,
        )
        .unwrap();

        assert_eq!(contract.status(), SupportTrustComplexityStatus::Verified);
        assert_eq!(contract.max_global_scans(), 0);
    }
}
