use super::certification::{
    SupportCertificationCoverageWitness, SupportCertificationEvidenceBundle,
};
use super::classification::{
    SupportTrustClassificationCostSurface, SupportTrustClassificationCounterSnapshot,
};
use super::drift::{
    SupportTrustDriftCause, SupportTrustDriftLocality, SupportTrustDriftReport,
    SupportTrustDriftScanPlan,
};
use super::epochs::{SupportTrustEpoch, SupportTrustFreshnessWitness};
use super::equivalence::{
    SupportTrustEquivalenceEvidence, SupportTrustEquivalenceLane,
    SupportTrustTransformedEquivalenceWitness,
};
use super::failure::{SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture};
use super::performance::{
    SupportTrustDensityClass, SupportTrustEvidenceBudget, SupportTrustPerformancePlan,
};
use super::receipts::{SupportTrustReceiptBundle, SupportTrustReceiptStatus};
use super::reports::{
    CertifiedSupportTrustReport, OperationalSupportTrustReport, SupportTrustCertificationStamp,
};
use super::taxonomy::{SupportTrustProvenance, SupportTrustStrength};
use super::translation::SupportTrustTranslationPlan;
use super::witnesses::{
    DegradedSupportTrustWitness, ExactSupportTrustWitness, RebuildDerivedSupportTrustWitness,
    RejectedSupportTrustWitness,
};
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportArtifactId, SubscriptionSupportFamilyId,
    SubscriptionSupportOperationalVerdict, SubscriptionSupportRole,
};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportTrustRequestedUse {
    StoreLocalResume,
    CertifiedPlatformClaim,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportTrustBatchCardinality {
    SingleSupportArtifact,
    FamilyRoleBatch { artifact_count: u64 },
}

impl SupportTrustBatchCardinality {
    pub fn artifact_count(self) -> u64 {
        match self {
            Self::SingleSupportArtifact => 1,
            Self::FamilyRoleBatch { artifact_count } => artifact_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RawSupportTrustRequest {
    family_id: SubscriptionSupportFamilyId,
    support_role: SubscriptionSupportRole,
    artifact_id: SubscriptionSupportArtifactId,
    requested_strength: SupportTrustStrength,
    provenance: SupportTrustProvenance,
    requested_use: SupportTrustRequestedUse,
    batch_cardinality: SupportTrustBatchCardinality,
    epoch: SupportTrustEpoch,
    performance_plan: SupportTrustPerformancePlan,
    evidence_budget: SupportTrustEvidenceBudget,
}

impl RawSupportTrustRequest {
    pub fn new(
        family_id: SubscriptionSupportFamilyId,
        support_role: SubscriptionSupportRole,
        artifact_id: SubscriptionSupportArtifactId,
        requested_strength: SupportTrustStrength,
        provenance: SupportTrustProvenance,
        requested_use: SupportTrustRequestedUse,
        batch_cardinality: SupportTrustBatchCardinality,
        epoch: SupportTrustEpoch,
        performance_plan: SupportTrustPerformancePlan,
        evidence_budget: SupportTrustEvidenceBudget,
    ) -> Self {
        Self {
            family_id,
            support_role,
            artifact_id,
            requested_strength,
            provenance,
            requested_use,
            batch_cardinality,
            epoch,
            performance_plan,
            evidence_budget,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportTrustRequestAdmitted {
    request: RawSupportTrustRequest,
    receipt_bundle: SupportTrustReceiptBundle,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportTrustTranslatedInputs {
    admitted: SupportTrustRequestAdmitted,
    translation_plan: SupportTrustTranslationPlan,
    receipt_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportTrustDriftChecked {
    translated: SupportTrustTranslatedInputs,
    drift_report: SupportTrustDriftReport,
}

impl SupportTrustDriftChecked {
    pub fn drift_report(&self) -> &SupportTrustDriftReport {
        &self.drift_report
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportTrustEquivalenceChecked {
    drift_checked: SupportTrustDriftChecked,
    transformed_equivalence: Option<SupportTrustTransformedEquivalenceWitness>,
    equivalence_checks_performed: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OperationalSupportTrustClassified {
    report: OperationalSupportTrustReport,
    cost_surface: SupportTrustClassificationCostSurface,
    counter_snapshot: SupportTrustClassificationCounterSnapshot,
}

impl OperationalSupportTrustClassified {
    pub fn report(&self) -> &OperationalSupportTrustReport {
        &self.report
    }

    pub fn cost_surface(&self) -> SupportTrustClassificationCostSurface {
        self.cost_surface
    }

    pub fn counter_snapshot(&self) -> SupportTrustClassificationCounterSnapshot {
        self.counter_snapshot
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportTrustCoverageChecked {
    operational: OperationalSupportTrustClassified,
    coverage_witness: SupportCertificationCoverageWitness,
    covered_row_id: String,
    evidence_bundle_digest: String,
}

impl SupportTrustCoverageChecked {
    pub fn operational(&self) -> &OperationalSupportTrustClassified {
        &self.operational
    }

    pub fn coverage_witness(&self) -> &SupportCertificationCoverageWitness {
        &self.coverage_witness
    }

    pub fn covered_row_id(&self) -> &str {
        &self.covered_row_id
    }

    pub fn evidence_bundle_digest(&self) -> &str {
        &self.evidence_bundle_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CertifiedSupportTrustClassified {
    report: CertifiedSupportTrustReport,
    coverage_witness: SupportCertificationCoverageWitness,
}

impl CertifiedSupportTrustClassified {
    pub fn report(&self) -> &CertifiedSupportTrustReport {
        &self.report
    }

    pub fn coverage_witness(&self) -> &SupportCertificationCoverageWitness {
        &self.coverage_witness
    }
}

pub fn admit_support_trust_request(
    request: RawSupportTrustRequest,
    receipt_bundle: SupportTrustReceiptBundle,
) -> Result<SupportTrustRequestAdmitted, SupportTrustFailure> {
    let family_role_receipt = receipt_bundle.family_role();
    require_proven(family_role_receipt.status(), "family-role")?;
    if request.requested_use == SupportTrustRequestedUse::CertifiedPlatformClaim {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "certified platform trust claims require certification coverage before admission",
        ));
    }
    if request.batch_cardinality.artifact_count() == 0 {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustPayloadBudgetExceeded,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust batch cardinality must include at least one artifact",
        ));
    }
    require_cardinality_density_match(
        request.batch_cardinality,
        request.performance_plan.density_class(),
    )?;
    if !request.evidence_budget.admits(
        receipt_bundle.receipt_bytes(),
        receipt_bundle.receipt_count(),
        request.batch_cardinality.artifact_count(),
    ) {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustPayloadBudgetExceeded,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust receipt bundle exceeds the admitted evidence budget",
        ));
    }
    if request.family_id != *family_role_receipt.family_id() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustFamilyMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust request family must match family-role receipt",
        ));
    }
    if request.support_role != family_role_receipt.support_role() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustRoleMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust request role must match family-role receipt",
        ));
    }
    if request.artifact_id != *family_role_receipt.artifact_id() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustBasisMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust request artifact must match family-role receipt",
        ));
    }
    Ok(SupportTrustRequestAdmitted {
        request,
        receipt_bundle,
    })
}

pub fn translate_support_trust_inputs(
    admitted: SupportTrustRequestAdmitted,
) -> Result<SupportTrustTranslatedInputs, SupportTrustFailure> {
    let resume_receipt = admitted.receipt_bundle.resume();
    let operational_receipt = admitted.receipt_bundle.operational();
    let basis_receipt = admitted.receipt_bundle.basis();
    let cursor_checkpoint_receipt = admitted.receipt_bundle.cursor_checkpoint();
    let compatibility_receipt = admitted.receipt_bundle.compatibility();
    let portability_receipt = admitted.receipt_bundle.portability();
    require_proven(resume_receipt.status(), "resume classification")?;
    require_proven(operational_receipt.status(), "operational verdict")?;
    require_proven(basis_receipt.status(), "basis")?;
    require_proven(cursor_checkpoint_receipt.status(), "cursor/checkpoint")?;
    require_proven(compatibility_receipt.status(), "compatibility")?;
    require_proven(portability_receipt.status(), "portability")?;
    require_contextual_receipts(&admitted)?;
    let artifact_id = &admitted.request.artifact_id;
    for receipt_artifact_id in [
        resume_receipt.artifact_id(),
        operational_receipt.basis().artifact_id(),
        basis_receipt.artifact_id(),
        cursor_checkpoint_receipt.artifact_id(),
        compatibility_receipt.artifact_id(),
        portability_receipt.artifact_id(),
    ] {
        if receipt_artifact_id != artifact_id {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustBasisMismatch,
                SupportTrustRecoveryPosture::RetryWithFresherReceipts,
                "support trust receipts must all be bound to the requested artifact",
            ));
        }
    }
    let translation_plan = SupportTrustTranslationPlan::from_resume_and_operational(
        operational_receipt.basis().clone(),
        resume_receipt.classification(),
        operational_receipt.verdict(),
    )?;
    let receipt_count = admitted.receipt_bundle.receipt_count();
    Ok(SupportTrustTranslatedInputs {
        admitted,
        translation_plan,
        receipt_count,
    })
}

pub fn check_support_trust_drift(
    translated: SupportTrustTranslatedInputs,
    scan_plan: SupportTrustDriftScanPlan,
) -> Result<SupportTrustDriftChecked, SupportTrustFailure> {
    let basis = translated.admitted.receipt_bundle.operational().basis();
    let request = &translated.admitted.request;
    let mut causes = Vec::new();
    if request.family_id != *basis.family_id() {
        causes.push((
            SupportTrustDriftCause::Family,
            SupportTrustDriftLocality::FamilyRole,
        ));
    }
    if request.support_role != basis.support_role() {
        causes.push((
            SupportTrustDriftCause::Role,
            SupportTrustDriftLocality::FamilyRole,
        ));
    }
    if request.artifact_id != *basis.artifact_id() {
        causes.push((
            SupportTrustDriftCause::SupportDigest,
            SupportTrustDriftLocality::SupportIdentity,
        ));
    }
    if translated.admitted.receipt_bundle.basis().basis_digest() != basis.basis_digest() {
        causes.push((
            SupportTrustDriftCause::Basis,
            SupportTrustDriftLocality::BasisLocal,
        ));
    }
    if translated
        .admitted
        .receipt_bundle
        .cursor_checkpoint()
        .cursor_checkpoint_digest()
        != format!("{}:{}", basis.cursor_digest(), basis.checkpoint_digest())
    {
        causes.push((
            SupportTrustDriftCause::CursorCheckpoint,
            SupportTrustDriftLocality::CursorCheckpointLocal,
        ));
    }
    if translated
        .admitted
        .receipt_bundle
        .compatibility()
        .compatibility_digest()
        != basis.compatibility_digest()
    {
        causes.push((
            SupportTrustDriftCause::Compatibility,
            SupportTrustDriftLocality::CompatibilityEpoch,
        ));
    }
    if translated
        .admitted
        .receipt_bundle
        .portability()
        .portability_digest()
        != basis.portability_digest()
    {
        causes.push((
            SupportTrustDriftCause::Portability,
            SupportTrustDriftLocality::SupportIdentity,
        ));
    }
    if !operational_verdict_matches_resume_classification(
        translated.admitted.receipt_bundle.resume().classification(),
        translated.admitted.receipt_bundle.operational().verdict(),
    ) {
        causes.push((
            SupportTrustDriftCause::OperationalVerdict,
            SupportTrustDriftLocality::SupportIdentity,
        ));
    }
    if scan_plan.certification_coverage_is_missing() {
        causes.push((
            SupportTrustDriftCause::CertificationCoverage,
            SupportTrustDriftLocality::CertificationScope,
        ));
    }
    if scan_plan.locality() == SupportTrustDriftLocality::PlacementCostAdvisory {
        causes.push((
            SupportTrustDriftCause::PlacementCost,
            SupportTrustDriftLocality::PlacementCostAdvisory,
        ));
    }
    let drift_report = if causes.is_empty() {
        SupportTrustDriftReport::fresh(&scan_plan)
    } else {
        SupportTrustDriftReport::from_causes(&scan_plan, causes)
    };
    if let Some(cause) = drift_report.blocking_cause() {
        return Err(SupportTrustFailure::new_with_drift_report(
            cause.failure_kind(),
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust drift report rejected stale support evidence",
            drift_report,
        ));
    }
    Ok(SupportTrustDriftChecked {
        translated,
        drift_report,
    })
}

pub fn check_support_trust_equivalence(
    drift_checked: SupportTrustDriftChecked,
    equivalence_evidence: SupportTrustEquivalenceEvidence,
) -> Result<SupportTrustEquivalenceChecked, SupportTrustFailure> {
    let provenance = drift_checked.translated.admitted.request.provenance;
    let requested_strength = drift_checked.translated.admitted.request.requested_strength;
    let transformed_exact_lane = if requested_strength == SupportTrustStrength::Exact {
        equivalence_lane_for(provenance)
    } else {
        None
    };
    if transformed_exact_lane.is_none() && equivalence_evidence.contract_count() > 0 {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustEquivalenceMissing,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust equivalence evidence is only admissible for transformed exact trust",
        ));
    }
    let transformed_equivalence = match transformed_exact_lane {
        Some(lane) => {
            let contract = equivalence_evidence.contract_for(lane).ok_or_else(|| {
                SupportTrustFailure::new(
                    SupportTrustFailureKind::SupportTrustEquivalenceMissing,
                    SupportTrustRecoveryPosture::RetryWithFresherReceipts,
                    "transformed exact support trust requires Phase 3 equivalence proof",
                )
            })?;
            validate_equivalence_contract(&drift_checked, lane, contract)?;
            Some(SupportTrustTransformedEquivalenceWitness::from_contract(
                contract,
            )?)
        }
        None => None,
    };
    if provenance.requires_equivalence_for_exact()
        && requested_strength == SupportTrustStrength::Exact
        && transformed_equivalence.is_none()
    {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustEquivalenceMissing,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "transformed exact support trust requires Phase 3 equivalence proof",
        ));
    }
    Ok(SupportTrustEquivalenceChecked {
        drift_checked,
        transformed_equivalence,
        equivalence_checks_performed: 1 + equivalence_evidence.contract_count(),
    })
}

pub fn classify_operational_support_trust(
    equivalence_checked: SupportTrustEquivalenceChecked,
) -> Result<OperationalSupportTrustClassified, SupportTrustFailure> {
    let translated = equivalence_checked.drift_checked.translated;
    let request = &translated.admitted.request;
    let cost_surface = SupportTrustClassificationCostSurface::new(
        request.batch_cardinality.artifact_count(),
        translated.receipt_count,
        equivalence_checked
            .drift_checked
            .drift_report
            .checks_performed(),
        equivalence_checked.equivalence_checks_performed,
        request.performance_plan.expected_index_probes()
            + equivalence_checked
                .drift_checked
                .drift_report
                .index_probes(),
        request.performance_plan.expected_allocation_count(),
        request.performance_plan.expected_clone_count(),
        equivalence_checked
            .drift_checked
            .drift_report
            .stale_rejection_count(),
        equivalence_checked
            .drift_checked
            .drift_report
            .coverage_drift_count(),
        equivalence_checked
            .drift_checked
            .drift_report
            .placement_advisory_count(),
        equivalence_checked
            .drift_checked
            .drift_report
            .global_scan_debt_count(),
    );
    let freshness = SupportTrustFreshnessWitness::new(request.epoch);
    let report = match translated.translation_plan {
        SupportTrustTranslationPlan::Exact(translation) => {
            let witness = match equivalence_checked.transformed_equivalence {
                Some(equivalence) => ExactSupportTrustWitness::from_equivalent_operational_basis(
                    translation,
                    request.provenance,
                    freshness,
                    equivalence.into_operational_witness(),
                )?,
                None => ExactSupportTrustWitness::from_exact_translation(
                    translation,
                    request.provenance,
                    freshness,
                )?,
            };
            OperationalSupportTrustReport::from_exact_witness_with_cost(witness, cost_surface)
        }
        SupportTrustTranslationPlan::Degraded { basis, .. } => {
            let witness = DegradedSupportTrustWitness::new(basis, freshness);
            OperationalSupportTrustReport::from_degraded_witness(
                witness,
                request.provenance,
                cost_surface,
            )
        }
        SupportTrustTranslationPlan::RebuildDerived { basis, .. } => {
            let witness = RebuildDerivedSupportTrustWitness::new(basis, freshness);
            OperationalSupportTrustReport::from_rebuild_witness(
                witness,
                request.provenance,
                cost_surface,
            )
        }
        SupportTrustTranslationPlan::Rejected { basis, .. } => {
            let witness = RejectedSupportTrustWitness::new(basis, freshness);
            OperationalSupportTrustReport::from_rejected_witness(
                witness,
                request.provenance,
                cost_surface,
            )
        }
    };
    if request.requested_strength == SupportTrustStrength::Exact
        && report.trust_strength() != SupportTrustStrength::Exact
    {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "weaker support posture cannot satisfy an exact trust request",
        ));
    }
    let counter_snapshot = counters_for(&report, cost_surface);
    Ok(OperationalSupportTrustClassified {
        report,
        cost_surface,
        counter_snapshot,
    })
}

pub fn check_support_trust_coverage(
    operational: OperationalSupportTrustClassified,
    evidence_bundle: SupportCertificationEvidenceBundle,
) -> Result<SupportTrustCoverageChecked, SupportTrustFailure> {
    let covered_row_id = evidence_bundle
        .covered_row_id_for_operational_report(operational.report())
        .ok_or_else(|| {
            SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "support trust certification evidence bundle does not cover the operational trust report",
            )
        })?
        .to_string();
    let evidence_bundle_digest = evidence_bundle.evidence_bundle_digest().to_string();
    let coverage_witness = evidence_bundle.into_witness();
    Ok(SupportTrustCoverageChecked {
        operational,
        coverage_witness,
        covered_row_id,
        evidence_bundle_digest,
    })
}

pub fn classify_certified_support_trust(
    coverage_checked: SupportTrustCoverageChecked,
    certification_stamp: SupportTrustCertificationStamp,
) -> Result<CertifiedSupportTrustClassified, SupportTrustFailure> {
    if certification_stamp.row_id() != coverage_checked.covered_row_id.as_str() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "certification stamp row id must match the covered certification row",
        ));
    }
    if certification_stamp.evidence_bundle_digest()
        != coverage_checked.evidence_bundle_digest.as_str()
    {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "certification stamp evidence digest must match the checked certification bundle",
        ));
    }
    let report = CertifiedSupportTrustReport::from_operational_report(
        coverage_checked.operational.report,
        certification_stamp,
    )?;
    Ok(CertifiedSupportTrustClassified {
        report,
        coverage_witness: coverage_checked.coverage_witness,
    })
}

fn operational_verdict_matches_resume_classification(
    resume_classification: SubscriptionResumeClassification,
    operational_verdict: SubscriptionSupportOperationalVerdict,
) -> bool {
    match resume_classification {
        SubscriptionResumeClassification::Exact => {
            operational_verdict == SubscriptionSupportOperationalVerdict::ExactResumePreserved
        }
        SubscriptionResumeClassification::Degraded => {
            operational_verdict == SubscriptionSupportOperationalVerdict::DegradedResumePreserved
        }
        SubscriptionResumeClassification::RebuildRequired => {
            operational_verdict == SubscriptionSupportOperationalVerdict::RebuildRequired
        }
        SubscriptionResumeClassification::NotResumable => matches!(
            operational_verdict,
            SubscriptionSupportOperationalVerdict::NotResumable
                | SubscriptionSupportOperationalVerdict::RejectedByPolicy
        ),
    }
}

fn equivalence_lane_for(provenance: SupportTrustProvenance) -> Option<SupportTrustEquivalenceLane> {
    match provenance {
        SupportTrustProvenance::Rebuilt => Some(SupportTrustEquivalenceLane::Rebuild),
        SupportTrustProvenance::Migrated => Some(SupportTrustEquivalenceLane::Migration),
        SupportTrustProvenance::Replicated => Some(SupportTrustEquivalenceLane::Replication),
        SupportTrustProvenance::Imported => Some(SupportTrustEquivalenceLane::Import),
        _ => None,
    }
}

fn validate_equivalence_contract(
    drift_checked: &SupportTrustDriftChecked,
    lane: SupportTrustEquivalenceLane,
    contract: &super::equivalence::SupportTrustEquivalenceContract,
) -> Result<(), SupportTrustFailure> {
    let request = &drift_checked.translated.admitted.request;
    let basis = drift_checked
        .translated
        .admitted
        .receipt_bundle
        .operational()
        .basis();
    if contract.lane() != lane || contract.source_basis() != basis {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustBasisMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust equivalence contract must be bound to the drift-checked source basis",
        ));
    }
    if contract.target_family_id() != &request.family_id {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustFamilyMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust equivalence target family must match the trust request",
        ));
    }
    if contract.target_support_role() != request.support_role {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustRoleMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust equivalence target role must match the trust request",
        ));
    }
    if contract.target_artifact_id() != &request.artifact_id {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustBasisMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust equivalence target artifact must match the trust request",
        ));
    }
    if contract.target_basis_digest() != basis.basis_digest()
        || contract.target_cursor_digest() != basis.cursor_digest()
        || contract.target_checkpoint_digest() != basis.checkpoint_digest()
    {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustBasisMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust equivalence target basis, cursor, and checkpoint must match source proof",
        ));
    }
    if contract.target_compatibility_digest() != basis.compatibility_digest() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCompatibilityMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust equivalence target compatibility must match source proof",
        ));
    }
    if contract.target_portability_digest() != basis.portability_digest() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustPortabilityMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust equivalence target portability scope must match source proof",
        ));
    }
    if contract.resume_classification() != SubscriptionResumeClassification::Exact {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustResumeClassificationMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust equivalence must prove exact resume classifier equivalence",
        ));
    }
    if contract.operational_verdict() != SubscriptionSupportOperationalVerdict::ExactResumePreserved
    {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustOperationalVerdictMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust equivalence must preserve exact operational verdict",
        ));
    }
    Ok(())
}

fn counters_for(
    report: &OperationalSupportTrustReport,
    cost_surface: SupportTrustClassificationCostSurface,
) -> SupportTrustClassificationCounterSnapshot {
    SupportTrustClassificationCounterSnapshot::new(
        1,
        u64::from(report.trust_strength() == SupportTrustStrength::Exact),
        u64::from(report.trust_strength() == SupportTrustStrength::Degraded),
        u64::from(report.trust_strength() == SupportTrustStrength::RebuildOnly),
        u64::from(report.trust_strength() == SupportTrustStrength::Rejected),
        cost_surface.receipts_consumed(),
        cost_surface.drift_checks_performed(),
        cost_surface.equivalence_checks_performed(),
        0,
        cost_surface.stale_rejection_count(),
        cost_surface.coverage_drift_count(),
        cost_surface.placement_advisory_count(),
        cost_surface.global_scan_debt_count(),
    )
}

fn require_proven(
    status: SupportTrustReceiptStatus,
    label: &'static str,
) -> Result<(), SupportTrustFailure> {
    if !status.is_proven() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            format!("support trust {label} receipt must be proven"),
        ));
    }
    Ok(())
}

fn require_contextual_receipts(
    admitted: &SupportTrustRequestAdmitted,
) -> Result<(), SupportTrustFailure> {
    let operational_verdict = admitted.receipt_bundle.operational().verdict();
    if matches!(
        operational_verdict,
        SubscriptionSupportOperationalVerdict::ExactResumePreserved
            | SubscriptionSupportOperationalVerdict::DegradedResumePreserved
    ) {
        let retention = admitted.receipt_bundle.retention().ok_or_else(|| {
            SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RetryWithFresherReceipts,
                "exact and degraded support trust require retention proof",
            )
        })?;
        require_proven(retention.status(), "retention")?;
        require_receipt_artifact(
            retention.artifact_id(),
            &admitted.request.artifact_id,
            "retention",
        )?;
    }
    if operational_verdict == SubscriptionSupportOperationalVerdict::RebuildRequired {
        let maintenance = admitted.receipt_bundle.maintenance().ok_or_else(|| {
            SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RetryWithFresherReceipts,
                "rebuild-derived support trust requires maintenance admission proof",
            )
        })?;
        require_proven(maintenance.status(), "maintenance")?;
        require_receipt_artifact(
            maintenance.artifact_id(),
            &admitted.request.artifact_id,
            "maintenance",
        )?;
    }
    if admitted.request.provenance == SupportTrustProvenance::Imported {
        let import = admitted.receipt_bundle.import_admission().ok_or_else(|| {
            SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RetryWithFresherReceipts,
                "imported support trust requires target-side import admission proof",
            )
        })?;
        require_proven(import.status(), "import admission")?;
        require_receipt_artifact(
            import.artifact_id(),
            &admitted.request.artifact_id,
            "import admission",
        )?;
        if import.target_family_id() != &admitted.request.family_id {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustFamilyMismatch,
                SupportTrustRecoveryPosture::RetryWithFresherReceipts,
                "import admission target family must match the trust request family",
            ));
        }
    }
    Ok(())
}

fn require_cardinality_density_match(
    cardinality: SupportTrustBatchCardinality,
    density_class: SupportTrustDensityClass,
) -> Result<(), SupportTrustFailure> {
    match (cardinality, density_class) {
        (
            SupportTrustBatchCardinality::SingleSupportArtifact,
            SupportTrustDensityClass::SingleSupportArtifact,
        ) => Ok(()),
        (
            SupportTrustBatchCardinality::FamilyRoleBatch { artifact_count },
            SupportTrustDensityClass::FamilyLocal | SupportTrustDensityClass::RoleLocal,
        ) if artifact_count > 1 => Ok(()),
        (SupportTrustBatchCardinality::FamilyRoleBatch { .. }, _) => Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustAccessStructureDebt,
            SupportTrustRecoveryPosture::RebuildTrustCache,
            "family-role support trust batches require family-local or role-local density",
        )),
        _ => Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustAccessStructureDebt,
            SupportTrustRecoveryPosture::RebuildTrustCache,
            "single support trust requests require single-artifact density",
        )),
    }
}

fn require_receipt_artifact(
    receipt_artifact_id: &SubscriptionSupportArtifactId,
    request_artifact_id: &SubscriptionSupportArtifactId,
    label: &'static str,
) -> Result<(), SupportTrustFailure> {
    if receipt_artifact_id != request_artifact_id {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustBasisMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            format!("support trust {label} receipt must be bound to the requested artifact"),
        ));
    }
    Ok(())
}
