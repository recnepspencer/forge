use crate::{delta::ComplexityStatus, evidence::StoreCounterSnapshot, media::DurableBackendFamily};
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone13ComplexityPathStatus {
    pub status: ComplexityStatus,
    pub proof_basis: Option<String>,
    pub debt_reason: Option<String>,
}

impl Milestone13ComplexityPathStatus {
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
pub struct Milestone13ComplexitySurface {
    pub placement_state_reconstruction: Milestone13ComplexityPathStatus,
    pub working_set_classification: Milestone13ComplexityPathStatus,
    pub tier_move_planning: Milestone13ComplexityPathStatus,
    pub tier_move_cutover: Milestone13ComplexityPathStatus,
    pub tier_move_execution: Milestone13ComplexityPathStatus,
    pub cold_recall_execution: Milestone13ComplexityPathStatus,
    pub recall_coalescing: Milestone13ComplexityPathStatus,
}

impl Milestone13ComplexitySurface {
    pub fn phase_1_default() -> Self {
        let phase_1_debt = "Phase 1 vocabulary shipped; execution/planning not wired yet";
        Self {
            placement_state_reconstruction: Milestone13ComplexityPathStatus::verified(
                "phase 1 publishes typed residency manifests and bounded reconstruction vocabulary",
            ),
            working_set_classification: Milestone13ComplexityPathStatus::verified(
                "phase 1 publishes typed working-set observation and classification witnesses",
            ),
            tier_move_planning: Milestone13ComplexityPathStatus::debt(phase_1_debt),
            tier_move_cutover: Milestone13ComplexityPathStatus::debt(phase_1_debt),
            tier_move_execution: Milestone13ComplexityPathStatus::debt(phase_1_debt),
            cold_recall_execution: Milestone13ComplexityPathStatus::debt(phase_1_debt),
            recall_coalescing: Milestone13ComplexityPathStatus::debt(phase_1_debt),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone13CounterContract {
    pub placement_state_manifest_load_count: u64,
    pub placement_state_recovery_count: u64,
    pub working_set_observation_window_count: u64,
    pub working_set_reclassification_count: u64,
    pub hot_tier_resident_read_count: u64,
    pub warm_tier_resident_read_count: u64,
    pub cold_tier_recall_count: u64,
    pub foreground_cold_recall_count: u64,
    pub background_tier_move_count: u64,
    pub restart_recall_count: u64,
    pub tier_move_plan_count: u64,
    pub tier_move_cutover_count: u64,
    pub tier_move_cutover_rejection_count: u64,
    pub authoritative_tier_move_count: u64,
    pub derived_tier_move_count: u64,
    pub tier_move_rejection_count: u64,
    pub tier_miss_count: u64,
    pub broadened_recall_plan_count: u64,
    pub recall_coalesced_request_count: u64,
    pub recall_duplicate_suppression_count: u64,
    pub tier_interleaved_read_count: u64,
    pub tier_interleaved_continuation_count: u64,
    pub tier_interleaving_recall_count: u64,
    pub tier_interleaving_parity_failure_count: u64,
    pub placement_debt_count: u64,
    pub working_set_debt_count: u64,
    pub tier_truth_parity_failure_count: u64,
    pub tier_restore_parity_failure_count: u64,
    pub tier_recall_failure_count: u64,
}

impl Milestone13CounterContract {
    pub fn from_snapshot(snapshot: &StoreCounterSnapshot) -> Self {
        Self {
            placement_state_manifest_load_count: snapshot.placement_state_manifest_load_count,
            placement_state_recovery_count: snapshot.placement_state_recovery_count,
            working_set_observation_window_count: snapshot.working_set_observation_window_count,
            working_set_reclassification_count: snapshot.working_set_reclassification_count,
            hot_tier_resident_read_count: snapshot.hot_tier_resident_read_count,
            warm_tier_resident_read_count: snapshot.warm_tier_resident_read_count,
            cold_tier_recall_count: snapshot.cold_tier_recall_count,
            foreground_cold_recall_count: snapshot.foreground_cold_recall_count,
            background_tier_move_count: snapshot.background_tier_move_count,
            restart_recall_count: snapshot.restart_recall_count,
            tier_move_plan_count: snapshot.tier_move_plan_count,
            tier_move_cutover_count: snapshot.tier_move_cutover_count,
            tier_move_cutover_rejection_count: snapshot.tier_move_cutover_rejection_count,
            authoritative_tier_move_count: snapshot.authoritative_tier_move_count,
            derived_tier_move_count: snapshot.derived_tier_move_count,
            tier_move_rejection_count: snapshot.tier_move_rejection_count,
            tier_miss_count: snapshot.tier_miss_count,
            broadened_recall_plan_count: snapshot.broadened_recall_plan_count,
            recall_coalesced_request_count: snapshot.recall_coalesced_request_count,
            recall_duplicate_suppression_count: snapshot.recall_duplicate_suppression_count,
            tier_interleaved_read_count: snapshot.tier_interleaved_read_count,
            tier_interleaved_continuation_count: snapshot.tier_interleaved_continuation_count,
            tier_interleaving_recall_count: snapshot.tier_interleaving_recall_count,
            tier_interleaving_parity_failure_count: snapshot.tier_interleaving_parity_failure_count,
            placement_debt_count: snapshot.placement_debt_count,
            working_set_debt_count: snapshot.working_set_debt_count,
            tier_truth_parity_failure_count: snapshot.tier_truth_parity_failure_count,
            tier_restore_parity_failure_count: snapshot.tier_restore_parity_failure_count,
            tier_recall_failure_count: snapshot.tier_recall_failure_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone13ArtifactReport {
    pub artifact_digest: String,
    pub diagnostics_digest: String,
    pub resident_artifact_count: usize,
    pub in_flight_transfer_count: usize,
    pub residual_residency_ambiguity_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone13CertificationBundle {
    pub backend_family: DurableBackendFamily,
    pub truth_digest: String,
    pub artifact_digest: String,
    pub diagnostics_digest: String,
    pub certification_summary: Milestone13CertificationSummary,
    pub artifact_report: Milestone13ArtifactReport,
    pub complexity_surface: Milestone13ComplexitySurface,
    pub counter_contract: Milestone13CounterContract,
    pub counter_snapshot: StoreCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone13CertificationSummary {
    pub truth_matches_control_lane: bool,
    pub no_tier_truth_parity_failures: bool,
    pub no_tier_restore_parity_failures: bool,
    pub no_tier_recall_failures: bool,
    pub no_residual_residency_ambiguity: bool,
    pub verified_path_count: usize,
    pub debt_path_count: usize,
}

impl Milestone13CertificationBundle {
    pub fn new(
        primary_export: &crate::AuthoritativeExportBundle,
        control_export: &crate::AuthoritativeExportBundle,
        backend_family: DurableBackendFamily,
        artifact_report: Milestone13ArtifactReport,
        complexity_surface: Milestone13ComplexitySurface,
        counter_contract: Milestone13CounterContract,
        counter_snapshot: StoreCounterSnapshot,
    ) -> Self {
        let truth_digest = stable_digest(primary_export);
        let control_truth_digest = stable_digest(control_export);
        let verified_path_count = [
            &complexity_surface.placement_state_reconstruction,
            &complexity_surface.working_set_classification,
            &complexity_surface.tier_move_planning,
            &complexity_surface.tier_move_cutover,
            &complexity_surface.tier_move_execution,
            &complexity_surface.cold_recall_execution,
            &complexity_surface.recall_coalescing,
        ]
        .into_iter()
        .filter(|path| path.status == ComplexityStatus::Verified)
        .count();
        let debt_path_count = [
            &complexity_surface.placement_state_reconstruction,
            &complexity_surface.working_set_classification,
            &complexity_surface.tier_move_planning,
            &complexity_surface.tier_move_cutover,
            &complexity_surface.tier_move_execution,
            &complexity_surface.cold_recall_execution,
            &complexity_surface.recall_coalescing,
        ]
        .into_iter()
        .filter(|path| path.status == ComplexityStatus::Debt)
        .count();

        let certification_summary = Milestone13CertificationSummary {
            truth_matches_control_lane: truth_digest == control_truth_digest,
            no_tier_truth_parity_failures: counter_contract.tier_truth_parity_failure_count == 0,
            no_tier_restore_parity_failures: counter_contract.tier_restore_parity_failure_count
                == 0,
            no_tier_recall_failures: counter_contract.tier_recall_failure_count == 0,
            no_residual_residency_ambiguity: artifact_report.residual_residency_ambiguity_count
                == 0,
            verified_path_count,
            debt_path_count,
        };

        Self {
            backend_family,
            truth_digest,
            artifact_digest: artifact_report.artifact_digest.clone(),
            diagnostics_digest: artifact_report.diagnostics_digest.clone(),
            certification_summary,
            artifact_report,
            complexity_surface,
            counter_contract,
            counter_snapshot,
        }
    }
}

fn stable_digest<T: Serialize>(value: &T) -> String {
    let json = serde_json::to_vec(value).expect("milestone 13 certification serialization");
    let mut hasher = Sha256::new();
    hasher.update(json);
    format!("{:x}", hasher.finalize())
}
