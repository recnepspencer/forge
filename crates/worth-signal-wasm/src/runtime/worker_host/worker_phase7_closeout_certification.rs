use serde::Serialize;

use crate::boundary::errors::WorthSignalJsError;

use super::{
    canonical_worker_certification_digest, certify_worker_phase7_performance_contracts,
    certify_worker_phase7_product_guidance, certify_worker_phase7_test_requirements,
    committed_truth_digest_for_runtime, WorkerPhase5CloseoutCertificationPackage,
    WorkerPhase6CloseoutCertificationPackage, WorkerPhase7PerformanceContractPackage,
    WorkerPhase7ProductGuidanceCertificationPackage,
    WorkerPhase7TestRequirementsCertificationPackage, WorkerRuntimeShell,
    WorkerUnavailableCompatibilityCertificationPackage,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerPhase7CloseoutCertificationPackage {
    pub certification_family: &'static str,
    pub suite0_status: &'static str,
    pub milestone_closed: bool,
    pub required_proof_family_count: u64,
    pub covered_proof_family_count: u64,
    pub final_closeout_pending_count: u64,
    pub phase5_certification_digest: String,
    pub phase6_certification_digest: String,
    pub performance_contract_certification_digest: String,
    pub product_guidance_certification_digest: String,
    pub test_requirements_certification_digest: String,
    pub proof_family_digest: String,
    pub performance_counter_catalog_digest: String,
    pub performance_complexity_contract_digest: String,
    pub performance_failure_mode_digest: String,
    pub bridge_allocation_posture_digest: String,
    pub worker_first_truth_digest: String,
    pub boundary_performance_digest: String,
    pub capability_parity_digest: String,
    pub product_guidance_digest: String,
    pub acceptance_artifact_digest: String,
    pub suite0_closeout_digest: String,
    pub certification_digest: String,
}

impl WorkerPhase7CloseoutCertificationPackage {
    pub(crate) fn from_certified_phase7_evidence(
        phase5: WorkerPhase5CloseoutCertificationPackage,
        phase6: WorkerPhase6CloseoutCertificationPackage,
        performance: WorkerPhase7PerformanceContractPackage,
        product_guidance: WorkerPhase7ProductGuidanceCertificationPackage,
        test_requirements: WorkerPhase7TestRequirementsCertificationPackage,
        worker_first_truth_digest: String,
    ) -> Result<Self, WorthSignalJsError> {
        reject_weak_phase5_closeout(&phase5)?;
        reject_weak_phase6_closeout(&phase6)?;
        reject_weak_phase7_performance_contracts(&performance)?;
        reject_weak_phase7_product_guidance(&product_guidance)?;
        reject_weak_phase7_test_requirements(&test_requirements)?;

        let suite0_closeout_digest = canonical_worker_certification_digest(&(
            "workerPhase7Suite0FinalCloseout",
            phase5.certification_digest.as_str(),
            phase6.certification_digest.as_str(),
            performance.certification_digest.as_str(),
            product_guidance.certification_digest.as_str(),
            test_requirements.certification_digest.as_str(),
            test_requirements.proof_family_digest.as_str(),
            performance.counter_catalog_digest.as_str(),
            performance.complexity_contract_digest.as_str(),
            performance.failure_mode_digest.as_str(),
            performance.bridge_allocation_posture_digest.as_str(),
            worker_first_truth_digest.as_str(),
            test_requirements.final_closeout_pending_count,
        ))?;
        let certification_digest = canonical_worker_certification_digest(&(
            "workerPhase7CloseoutCertification",
            suite0_closeout_digest.as_str(),
            phase5.boundary_performance_digest.as_str(),
            phase6.capability_parity_digest.as_str(),
            product_guidance.product_guidance_digest.as_str(),
            test_requirements.acceptance_artifact_digest.as_str(),
            test_requirements.proof_family_digest.as_str(),
            performance.bridge_allocation_posture_digest.as_str(),
        ))?;

        Ok(Self {
            certification_family: "workerPhase7CloseoutCertification",
            suite0_status: "Suite0FinalCloseoutCertified",
            milestone_closed: true,
            required_proof_family_count: test_requirements.required_proof_family_count,
            covered_proof_family_count: test_requirements.covered_proof_family_count,
            final_closeout_pending_count: test_requirements.final_closeout_pending_count,
            phase5_certification_digest: phase5.certification_digest,
            phase6_certification_digest: phase6.certification_digest,
            performance_contract_certification_digest: performance.certification_digest,
            product_guidance_certification_digest: product_guidance.certification_digest,
            test_requirements_certification_digest: test_requirements.certification_digest,
            proof_family_digest: test_requirements.proof_family_digest,
            performance_counter_catalog_digest: performance.counter_catalog_digest,
            performance_complexity_contract_digest: performance.complexity_contract_digest,
            performance_failure_mode_digest: performance.failure_mode_digest,
            bridge_allocation_posture_digest: performance.bridge_allocation_posture_digest,
            worker_first_truth_digest,
            boundary_performance_digest: phase5.boundary_performance_digest,
            capability_parity_digest: phase6.capability_parity_digest,
            product_guidance_digest: product_guidance.product_guidance_digest,
            acceptance_artifact_digest: test_requirements.acceptance_artifact_digest,
            suite0_closeout_digest,
            certification_digest,
        })
    }
}

impl WorkerRuntimeShell {
    pub fn certify_worker_phase7_closeout(
        &self,
        worker_unavailable: WorkerUnavailableCompatibilityCertificationPackage,
    ) -> Result<WorkerPhase7CloseoutCertificationPackage, WorthSignalJsError> {
        WorkerPhase7CloseoutCertificationPackage::from_certified_phase7_evidence(
            self.certify_worker_phase5_closeout()?,
            self.certify_worker_phase6_closeout(worker_unavailable)?,
            certify_worker_phase7_performance_contracts()?,
            certify_worker_phase7_product_guidance()?,
            certify_worker_phase7_test_requirements()?,
            committed_truth_digest_for_runtime(&self.core)?,
        )
    }
}

fn reject_weak_phase5_closeout(
    package: &WorkerPhase5CloseoutCertificationPackage,
) -> Result<(), WorthSignalJsError> {
    if package.certification_family != "workerPhase5CloseoutCertification"
        || package.diagnostics_cold_reconstruction_count != 0
        || package.active_lifecycle_subscription_count == 0
    {
        return Err(WorthSignalJsError::invalid_input(
            "worker Phase 7 closeout requires current Phase 5 delivery and diagnostics evidence",
        ));
    }
    Ok(())
}

fn reject_weak_phase6_closeout(
    package: &WorkerPhase6CloseoutCertificationPackage,
) -> Result<(), WorthSignalJsError> {
    if package.certification_family != "workerPhase6CloseoutCertification"
        || package.covered_phase6_artifact_count != 4
        || package.fallback_count != 0
    {
        return Err(WorthSignalJsError::invalid_input(
            "worker Phase 7 closeout requires zero-fallback Phase 6 capability evidence",
        ));
    }
    Ok(())
}

fn reject_weak_phase7_performance_contracts(
    package: &WorkerPhase7PerformanceContractPackage,
) -> Result<(), WorthSignalJsError> {
    if package.certification_family != "workerPhase7PerformanceContractCertification"
        || package.covered_counter_count < 27
        || package.covered_complexity_contract_count < 14
        || package.prohibited_failure_mode_count < 8
        || package.bridge_allocation_posture.hidden_allocation_allowed
    {
        return Err(WorthSignalJsError::invalid_input(
            "worker Phase 7 closeout requires complete performance contract evidence",
        ));
    }
    Ok(())
}

fn reject_weak_phase7_product_guidance(
    package: &WorkerPhase7ProductGuidanceCertificationPackage,
) -> Result<(), WorthSignalJsError> {
    if package.certification_family != "workerPhase7ProductGuidanceCertification"
        || package.recommended_default_posture != "workerFirstRuntimeOwnedGraph"
        || package.hidden_fallback_allowed
    {
        return Err(WorthSignalJsError::invalid_input(
            "worker Phase 7 closeout requires worker-first product guidance without hidden fallback",
        ));
    }
    Ok(())
}

fn reject_weak_phase7_test_requirements(
    package: &WorkerPhase7TestRequirementsCertificationPackage,
) -> Result<(), WorthSignalJsError> {
    if package.certification_family != "workerPhase7TestRequirementsCertification"
        || package.test_requirements_status != "FinalCloseoutCertified"
        || package.required_proof_family_count != 13
        || package.covered_proof_family_count != 13
        || package.final_closeout_pending_count != 0
        || package.acceptance_artifacts.len() != 16
        || package
            .proof_families
            .iter()
            .any(|row| row.readiness != "ClosedByCanonicalCertification")
    {
        return Err(WorthSignalJsError::invalid_input(
            "worker Phase 7 closeout requires complete test requirement tracking",
        ));
    }
    Ok(())
}
