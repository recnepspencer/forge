use serde::Serialize;

use crate::boundary::errors::WORTHSignalJsError;

use super::{
    canonical_worker_certification_digest, committed_truth_digest_for_runtime, WorkerRuntimeShell,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerPhase5CloseoutCertificationPackage {
    pub certification_family: &'static str,
    pub phase_closeout_mode: &'static str,
    pub covered_suite_count: u64,
    pub observation_delivery_packet_count: u64,
    pub output_delivery_packet_count: u64,
    pub diagnostics_summary_read_count: u64,
    pub diagnostics_rich_read_count: u64,
    pub diagnostics_cold_reconstruction_count: u64,
    pub active_lifecycle_subscription_count: u64,
    pub observation_delivery_breadth: u64,
    pub output_delivery_breadth: u64,
    pub output_payload_byte_count: u64,
    pub rollback_suppressed_delivery_count: u64,
    pub worker_first_truth_digest: String,
    pub observation_digest: String,
    pub output_digest: String,
    pub diagnostics_summary_digest: String,
    pub rich_read_availability_digest: String,
    pub observation_lifecycle_digest: String,
    pub delivery_breadth_digest: String,
    pub boundary_performance_digest: String,
    pub certification_digest: String,
}

impl WorkerPhase5CloseoutCertificationPackage {
    pub(in crate::runtime::worker_host) fn from_worker_retained_evidence(
        shell: &WorkerRuntimeShell,
    ) -> Result<Self, WORTHSignalJsError> {
        let observation = shell.latest_worker_observation_delivery_packet()?;
        let output = shell.latest_worker_output_delivery_packet()?;
        let diagnostics = shell.latest_worker_diagnostics_summary_read_packet()?;
        let worker_first_truth_digest = committed_truth_digest_for_runtime(&shell.core)?;
        reject_stale_phase5_truth(
            observation.worker_first_truth_digest.as_str(),
            output.worker_first_truth_digest.as_str(),
            diagnostics.worker_first_truth_digest.as_str(),
            worker_first_truth_digest.as_str(),
        )?;
        let active_lifecycle_subscription_count =
            shell.active_observation_delivery_subscription_count();
        let observation_lifecycle_digest = shell.active_observation_delivery_lifecycle_digest()?;
        reject_stale_lifecycle_evidence(
            observation.active_lifecycle_subscription_count,
            observation.observation_lifecycle_digest.as_str(),
            active_lifecycle_subscription_count,
            observation_lifecycle_digest.as_str(),
        )?;
        reject_hidden_summary_reconstruction(diagnostics.diagnostics_cold_reconstruction_count)?;

        let delivery_breadth_digest = canonical_worker_certification_digest(&(
            "phase5DeliveryBreadth",
            observation.observation_delivery_breadth,
            output.output_delivery_breadth,
            output.output_payload_byte_count,
            observation.rollback_suppressed_delivery_count,
            diagnostics.diagnostics_cold_reconstruction_count,
        ))?;
        let boundary_performance_digest = canonical_worker_certification_digest(&(
            "phase5BoundaryPerformance",
            observation.boundary_performance.performance_digest.as_str(),
            output.boundary_performance.performance_digest.as_str(),
            diagnostics.boundary_performance.performance_digest.as_str(),
        ))?;
        let certification_digest = canonical_worker_certification_digest(&(
            "workerPhase5CloseoutCertification",
            observation.packet_digest.as_str(),
            output.packet_digest.as_str(),
            diagnostics.packet_digest.as_str(),
            observation_lifecycle_digest.as_str(),
            delivery_breadth_digest.as_str(),
            boundary_performance_digest.as_str(),
            worker_first_truth_digest.as_str(),
        ))?;

        Ok(Self {
            certification_family: "workerPhase5CloseoutCertification",
            phase_closeout_mode: "ObservationOutputDiagnosticsLifecycleBoundary",
            covered_suite_count: 2,
            observation_delivery_packet_count: observation.observation_delivery_packet_count,
            output_delivery_packet_count: output.output_delivery_packet_count,
            diagnostics_summary_read_count: diagnostics.diagnostics_summary_read_count,
            diagnostics_rich_read_count: diagnostics.diagnostics_rich_read_count,
            diagnostics_cold_reconstruction_count: diagnostics
                .diagnostics_cold_reconstruction_count,
            active_lifecycle_subscription_count,
            observation_delivery_breadth: observation.observation_delivery_breadth,
            output_delivery_breadth: output.output_delivery_breadth,
            output_payload_byte_count: output.output_payload_byte_count,
            rollback_suppressed_delivery_count: observation.rollback_suppressed_delivery_count,
            worker_first_truth_digest,
            observation_digest: observation.observation_digest.clone(),
            output_digest: output.output_digest.clone(),
            diagnostics_summary_digest: diagnostics.diagnostics_summary_digest.clone(),
            rich_read_availability_digest: diagnostics.rich_read_availability_digest.clone(),
            observation_lifecycle_digest,
            delivery_breadth_digest,
            boundary_performance_digest,
            certification_digest,
        })
    }
}

impl WorkerRuntimeShell {
    pub fn certify_worker_phase5_closeout(
        &self,
    ) -> Result<WorkerPhase5CloseoutCertificationPackage, WORTHSignalJsError> {
        WorkerPhase5CloseoutCertificationPackage::from_worker_retained_evidence(self)
    }
}

fn reject_stale_phase5_truth(
    observation_truth_digest: &str,
    output_truth_digest: &str,
    diagnostics_truth_digest: &str,
    current_truth_digest: &str,
) -> Result<(), WORTHSignalJsError> {
    if observation_truth_digest != current_truth_digest
        || output_truth_digest != current_truth_digest
        || diagnostics_truth_digest != current_truth_digest
    {
        return Err(WORTHSignalJsError::invalid_input(
            "worker Phase 5 closeout certification requires one current runtime truth",
        ));
    }
    Ok(())
}

fn reject_stale_lifecycle_evidence(
    observation_lifecycle_subscription_count: u64,
    observation_lifecycle_digest: &str,
    active_lifecycle_subscription_count: u64,
    active_lifecycle_digest: &str,
) -> Result<(), WORTHSignalJsError> {
    if active_lifecycle_subscription_count == 0
        || observation_lifecycle_subscription_count != active_lifecycle_subscription_count
        || observation_lifecycle_digest != active_lifecycle_digest
    {
        return Err(WORTHSignalJsError::invalid_input(
            "worker Phase 5 closeout certification requires current lifecycle evidence",
        ));
    }
    Ok(())
}

fn reject_hidden_summary_reconstruction(
    diagnostics_cold_reconstruction_count: u64,
) -> Result<(), WORTHSignalJsError> {
    if diagnostics_cold_reconstruction_count != 0 {
        return Err(WORTHSignalJsError::invalid_input(
            "worker Phase 5 closeout certification requires zero summary cold reconstruction",
        ));
    }
    Ok(())
}
