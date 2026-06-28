use forge_foundational::{
    admit_requested_foundational_profile, materialize_admitted_foundational_profile,
    prepare_canonical_basis_bundle, prepare_counter_backed_performance_receipt_for_canonical_basis,
    prepare_diagnostic_support_report_for_canonical_basis,
    prepare_materialized_boundary_bundle_for_canonical_basis, request_foundational_profile_set,
    AdmissionReadinessProfile, CanonicalDerivedDigest, CanonicalizationFrontDoor,
    CanonicalizationRuleVersion, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, FoundationalProfileCompositionDenial, FoundationalProfileSet,
    FoundationalProfileSetInput, MaterializedFoundationalProfileSet, RetentionDeliveryProfile,
    SupportPostureProfile,
};
use forge_proof::TransitionOutcome;

use super::super::denial::RecoveryEvidenceDenial;
use super::super::diagnostics::RecoverySourceDecisionReport;
use super::super::executed_evidence_source::RecoveryPhysicsEvidenceSource;
use super::super::performance::RecoveryCounterPerformanceReceipt;
use super::bundle_materialization::materialize_bundle;
use super::foundational_bundle::MaterializedFoundationalRecoveryEvidenceBundle;
use super::receipt::RecoveryPhysicsReceipt;
use super::report::RecoveryPhysicsReport;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryEvidenceRichness {
    Full,
    Reduced,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryEvidenceCanonicalBasis {
    digest: CanonicalDerivedDigest,
    profile_set: MaterializedFoundationalProfileSet,
    richness: RecoveryEvidenceRichness,
}

impl RecoveryEvidenceCanonicalBasis {
    pub fn full(source: &RecoveryPhysicsEvidenceSource) -> Result<Self, RecoveryEvidenceDenial> {
        let receipt = RecoveryPhysicsReceipt::from_executed_source(source);
        let report = RecoveryPhysicsReport::from_executed_source(source);
        let performance = RecoveryCounterPerformanceReceipt::from_source(source);
        let source_decisions = RecoverySourceDecisionReport::from_source(source);
        let materialized = materialize_bundle(&receipt, &report, &performance);
        Self::full_from_evidence_surfaces(&materialized, &source_decisions, &performance)
    }

    pub(crate) fn full_from_evidence_surfaces(
        bundle: &MaterializedFoundationalRecoveryEvidenceBundle,
        source_decisions: &RecoverySourceDecisionReport,
        performance: &RecoveryCounterPerformanceReceipt,
    ) -> Result<Self, RecoveryEvidenceDenial> {
        let version = CanonicalizationRuleVersion::new("store.s4.recovery.evidence-bundle")
            .expect("static canonicalization version");
        let boundary_ready =
            match prepare_materialized_boundary_bundle_for_canonical_basis(version.clone(), bundle)
            {
                TransitionOutcome::Success(ready) => ready,
                _ => return Err(RecoveryEvidenceDenial::CanonicalBasisMaterializationDenied),
            };
        let diagnostic_ready = match prepare_diagnostic_support_report_for_canonical_basis(
            version.clone(),
            source_decisions.support_report(),
        ) {
            TransitionOutcome::Success(ready) => ready,
            _ => return Err(RecoveryEvidenceDenial::CanonicalBasisMaterializationDenied),
        };
        let performance_ready = match prepare_counter_backed_performance_receipt_for_canonical_basis(
            version.clone(),
            performance.counter_backed(),
        ) {
            TransitionOutcome::Success(ready) => ready,
            _ => return Err(RecoveryEvidenceDenial::CanonicalBasisMaterializationDenied),
        };
        let ready = match prepare_canonical_basis_bundle(
            version,
            [boundary_ready, diagnostic_ready, performance_ready],
        ) {
            TransitionOutcome::Success(ready) => ready,
            _ => return Err(RecoveryEvidenceDenial::CanonicalBasisMaterializationDenied),
        };
        let derivation = match CanonicalizationFrontDoor.digest().for_bundle(
            ready,
            forge_foundational::CanonicalDigestAlgorithmId::test_stable_fixture(),
        ) {
            TransitionOutcome::Success(ready) => ready,
            _ => return Err(RecoveryEvidenceDenial::CanonicalBasisMaterializationDenied),
        };
        Ok(Self {
            digest: CanonicalizationFrontDoor.digest().derive(derivation),
            profile_set: materialized_profile_set(
                full_profile_set()
                    .map_err(|_| RecoveryEvidenceDenial::ProfileReductionChangedRecoveryTruth)?,
            )?,
            richness: RecoveryEvidenceRichness::Full,
        })
    }

    pub fn reduced_from(
        full: &Self,
        original_root: &str,
        reduced_root: &str,
    ) -> Result<Self, RecoveryEvidenceDenial> {
        if original_root != reduced_root {
            return Err(RecoveryEvidenceDenial::ProfileReductionChangedRecoveryTruth);
        }
        Ok(Self {
            digest: full.digest.clone(),
            profile_set: materialized_profile_set(
                reduced_profile_set()
                    .map_err(|_| RecoveryEvidenceDenial::ProfileReductionChangedRecoveryTruth)?,
            )?,
            richness: RecoveryEvidenceRichness::Reduced,
        })
    }

    pub const fn digest(&self) -> &CanonicalDerivedDigest {
        &self.digest
    }

    pub const fn profile_set(&self) -> &MaterializedFoundationalProfileSet {
        &self.profile_set
    }

    pub const fn richness(&self) -> RecoveryEvidenceRichness {
        self.richness
    }
}

pub(crate) fn materialized_profile_set(
    profile: FoundationalProfileSet,
) -> Result<MaterializedFoundationalProfileSet, RecoveryEvidenceDenial> {
    let requested = request_foundational_profile_set(profile);
    let admitted = match admit_requested_foundational_profile(
        requested,
        profile,
        None,
        forge_foundational::foundational_profile_progression_authority(),
    ) {
        TransitionOutcome::Success(admitted) => admitted,
        _ => return Err(RecoveryEvidenceDenial::ProfileReductionChangedRecoveryTruth),
    };
    match materialize_admitted_foundational_profile(
        admitted,
        profile,
        None,
        forge_foundational::foundational_profile_progression_authority(),
    ) {
        TransitionOutcome::Success(materialized) => Ok(*materialized.payload()),
        _ => Err(RecoveryEvidenceDenial::ProfileReductionChangedRecoveryTruth),
    }
}

pub(crate) fn full_profile_set(
) -> Result<FoundationalProfileSet, FoundationalProfileCompositionDenial> {
    FoundationalProfileSet::new(FoundationalProfileSetInput {
        diagnostic_richness: DiagnosticRichnessProfile::Forensic,
        support_posture: SupportPostureProfile::CertificationReady,
        compatibility_posture: CompatibilityPostureProfile::NativeOnly,
        admission_readiness: AdmissionReadinessProfile::ProductionGateReady,
        retention_delivery: RetentionDeliveryProfile::Retained,
        certification_posture: CertificationPostureProfile::EvidenceBacked,
    })
}

fn reduced_profile_set() -> Result<FoundationalProfileSet, FoundationalProfileCompositionDenial> {
    FoundationalProfileSet::new(FoundationalProfileSetInput {
        diagnostic_richness: DiagnosticRichnessProfile::OperationalMinimal,
        support_posture: SupportPostureProfile::SupportReady,
        compatibility_posture: CompatibilityPostureProfile::NativeOnly,
        admission_readiness: AdmissionReadinessProfile::Admitted,
        retention_delivery: RetentionDeliveryProfile::Retained,
        certification_posture: CertificationPostureProfile::EvidenceBacked,
    })
}
