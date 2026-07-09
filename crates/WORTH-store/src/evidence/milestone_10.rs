use crate::{
    authority::AuthoritativeExportBundle, evidence::StoreCounterSnapshot,
    media::DurableBackendFamily,
};
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::delta::ComplexityStatus;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone10ComplexityPathStatus {
    pub status: ComplexityStatus,
    pub proof_basis: Option<String>,
    pub debt_reason: Option<String>,
}

impl Milestone10ComplexityPathStatus {
    pub fn verified(proof_basis: impl Into<String>) -> Self {
        Self {
            status: ComplexityStatus::Verified,
            proof_basis: Some(proof_basis.into()),
            debt_reason: None,
        }
    }

    pub fn debt(debt_reason: impl Into<String>) -> Self {
        Self {
            status: ComplexityStatus::Debt,
            proof_basis: None,
            debt_reason: Some(debt_reason.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone10ComplexitySurface {
    pub retention_candidate_planning: Milestone10ComplexityPathStatus,
    pub compaction_publication: Milestone10ComplexityPathStatus,
    pub reclaim_execution: Milestone10ComplexityPathStatus,
    pub retained_range_rebuild: Milestone10ComplexityPathStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone10CounterContract {
    pub retention_policy_evaluation_count: u64,
    pub retained_authoritative_range_count: u64,
    pub expired_authoritative_range_count: u64,
    pub compaction_plan_count: u64,
    pub compacted_delta_layer_count: u64,
    pub compacted_snapshot_family_count: u64,
    pub compacted_layout_family_count: u64,
    pub compaction_cutover_count: u64,
    pub compaction_cutover_rejection_count: u64,
    pub reclaim_candidate_count: u64,
    pub reclaimed_authoritative_artifact_count: u64,
    pub reclaimed_derived_artifact_count: u64,
    pub reclaim_rejected_live_basis_count: u64,
    pub retention_closure_ancestor_count: u64,
    pub retention_closure_failure_count: u64,
    pub retained_range_rebuild_count: u64,
    pub rebuild_debt_count: u64,
    pub compaction_debt_count: u64,
    pub retention_truth_parity_failure_count: u64,
    pub retention_restore_parity_failure_count: u64,
    pub retention_artifact_rebuild_failure_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone10ArtifactReport {
    pub artifact_digest: String,
    pub unverified_compaction_product_count: usize,
    pub uncleared_rebuild_debt_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone10CertificationBundle {
    pub backend_family: DurableBackendFamily,
    pub truth_digest: String,
    pub restore_digest: String,
    pub artifact_digest: String,
    pub certification_summary: Milestone10CertificationSummary,
    pub artifact_report: Milestone10ArtifactReport,
    pub complexity_surface: Milestone10ComplexitySurface,
    pub counter_contract: Milestone10CounterContract,
    pub counter_snapshot: StoreCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone10CertificationSummary {
    pub truth_matches_control_lane: bool,
    pub restore_truth_parity: bool,
    pub restore_matches_control_lane: bool,
    pub no_unverified_compaction_products: bool,
    pub no_uncleared_rebuild_debt: bool,
    pub no_retention_truth_parity_failures: bool,
    pub no_retention_restore_parity_failures: bool,
    pub no_retention_artifact_rebuild_failures: bool,
    pub verified_path_count: usize,
    pub debt_path_count: usize,
}

impl Milestone10CertificationBundle {
    pub fn new(
        primary_export: &AuthoritativeExportBundle,
        control_export: &AuthoritativeExportBundle,
        restored_export: &AuthoritativeExportBundle,
        backend_family: DurableBackendFamily,
        artifact_report: Milestone10ArtifactReport,
        complexity_surface: Milestone10ComplexitySurface,
        counter_contract: Milestone10CounterContract,
        counter_snapshot: StoreCounterSnapshot,
    ) -> Self {
        let primary_canonical = primary_export.clone().into_canonicalized();
        let control_canonical = control_export.clone().into_canonicalized();
        let restored_canonical = restored_export.clone().into_canonicalized();

        let truth_digest = stable_digest(&primary_canonical);
        let control_truth_digest = stable_digest(&control_canonical);
        let restore_digest = stable_digest(&restored_canonical);

        let verified_path_count = [
            &complexity_surface.retention_candidate_planning,
            &complexity_surface.compaction_publication,
            &complexity_surface.reclaim_execution,
            &complexity_surface.retained_range_rebuild,
        ]
        .into_iter()
        .filter(|path| path.status == ComplexityStatus::Verified)
        .count();
        let debt_path_count = [
            &complexity_surface.retention_candidate_planning,
            &complexity_surface.compaction_publication,
            &complexity_surface.reclaim_execution,
            &complexity_surface.retained_range_rebuild,
        ]
        .into_iter()
        .filter(|path| path.status == ComplexityStatus::Debt)
        .count();

        let certification_summary = Milestone10CertificationSummary {
            truth_matches_control_lane: truth_digest == control_truth_digest,
            restore_truth_parity: restore_digest == truth_digest,
            restore_matches_control_lane: restore_digest == control_truth_digest,
            no_unverified_compaction_products: artifact_report.unverified_compaction_product_count
                == 0,
            no_uncleared_rebuild_debt: artifact_report.uncleared_rebuild_debt_count == 0,
            no_retention_truth_parity_failures: counter_contract
                .retention_truth_parity_failure_count
                == 0,
            no_retention_restore_parity_failures: counter_contract
                .retention_restore_parity_failure_count
                == 0,
            no_retention_artifact_rebuild_failures: counter_contract
                .retention_artifact_rebuild_failure_count
                == 0,
            verified_path_count,
            debt_path_count,
        };

        Self {
            backend_family,
            truth_digest,
            restore_digest,
            artifact_digest: artifact_report.artifact_digest.clone(),
            certification_summary,
            artifact_report,
            complexity_surface,
            counter_contract,
            counter_snapshot,
        }
    }
}

fn stable_digest<T: Serialize>(value: &T) -> String {
    let json = serde_json::to_vec(value).expect("milestone 10 certification serialization");
    let mut hasher = Sha256::new();
    hasher.update(json);
    format!("{:x}", hasher.finalize())
}
