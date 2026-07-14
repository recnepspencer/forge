use serde::Serialize;
use worth_signal::facade::diagnostics::GraphSummary;

use crate::boundary::errors::WorthSignalJsError;
use crate::runtime::summaries::ExecutionHistorySurfaceSummary;

use super::{
    canonical_worker_certification_digest, committed_truth_digest_for_runtime,
    WorkerHostBoundaryPerformanceEnvelope, WorkerRuntimeShell,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerDiagnosticsSummaryReadPacket {
    pub envelope_family: &'static str,
    pub read_mode: &'static str,
    pub runtime_authority: &'static str,
    pub diagnostics_summary_read_count: u64,
    pub diagnostics_rich_read_count: u64,
    pub diagnostics_cold_reconstruction_count: u64,
    pub worker_first_truth_digest: String,
    pub diagnostics_summary_digest: String,
    pub rich_read_availability_digest: String,
    pub boundary_performance: WorkerHostBoundaryPerformanceEnvelope,
    pub packet_digest: String,
    pub summary: GraphSummary,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerDiagnosticsHistoryReadPacket {
    pub envelope_family: &'static str,
    pub read_mode: &'static str,
    pub runtime_authority: &'static str,
    pub diagnostics_summary_read_count: u64,
    pub diagnostics_rich_read_count: u64,
    pub diagnostics_cold_reconstruction_count: u64,
    pub worker_first_truth_digest: String,
    pub diagnostics_history_digest: String,
    pub boundary_performance: WorkerHostBoundaryPerformanceEnvelope,
    pub packet_digest: String,
    pub history: ExecutionHistorySurfaceSummary,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerDiagnosticsSummaryReadCertificationPackage {
    pub certification_family: &'static str,
    pub covered_suite_count: u64,
    pub diagnostics_summary_read_count: u64,
    pub diagnostics_rich_read_count: u64,
    pub diagnostics_cold_reconstruction_count: u64,
    pub worker_first_truth_digest: String,
    pub diagnostics_summary_digest: String,
    pub rich_read_availability_digest: String,
    pub boundary_performance_digest: String,
    pub packet_digest: String,
    pub certification_digest: String,
}

impl WorkerDiagnosticsSummaryReadPacket {
    pub(in crate::runtime::worker_host) fn from_summary(
        summary: GraphSummary,
        worker_first_truth_digest: String,
    ) -> Result<Self, WorthSignalJsError> {
        let diagnostics_summary_digest = canonical_worker_certification_digest(
            &StableDiagnosticsSummaryProjection::from(&summary),
        )?;
        let rich_read_availability_digest =
            diagnostics_rich_read_availability_digest(diagnostics_summary_digest.as_str())?;
        let boundary_performance = WorkerHostBoundaryPerformanceEnvelope::diagnostics_summary_read(
            diagnostics_summary_digest.as_str(),
            rich_read_availability_digest.as_str(),
        )?;
        let packet_digest = canonical_worker_certification_digest(&(
            "diagnosticsHistoryRead",
            "SummaryDiagnosticsRead",
            diagnostics_summary_digest.as_str(),
            rich_read_availability_digest.as_str(),
            boundary_performance.performance_digest.as_str(),
            worker_first_truth_digest.as_str(),
        ))?;

        Ok(Self {
            envelope_family: "diagnosticsHistoryRead",
            read_mode: "SummaryDiagnosticsRead",
            runtime_authority: "workerOwnedRuntime",
            diagnostics_summary_read_count: 1,
            diagnostics_rich_read_count: 0,
            diagnostics_cold_reconstruction_count: 0,
            worker_first_truth_digest,
            diagnostics_summary_digest,
            rich_read_availability_digest,
            boundary_performance,
            packet_digest,
            summary,
        })
    }
}

impl WorkerDiagnosticsHistoryReadPacket {
    pub(in crate::runtime::worker_host) fn from_history(
        history: ExecutionHistorySurfaceSummary,
        worker_first_truth_digest: String,
    ) -> Result<Self, WorthSignalJsError> {
        let diagnostics_history_digest = canonical_worker_certification_digest(&history)?;
        let history_payload_byte_count = serde_json::to_vec(&history)
            .map_err(|error| {
                WorthSignalJsError::internal(format!(
                    "failed to measure diagnostics history payload: {error}"
                ))
            })?
            .len() as u64;
        let diagnostics_cold_reconstruction_count = rich_history_reconstruction_count(&history);
        let boundary_performance =
            WorkerHostBoundaryPerformanceEnvelope::diagnostics_rich_history_read(
                history_payload_byte_count,
                diagnostics_history_digest.as_str(),
                diagnostics_cold_reconstruction_count,
            )?;
        let packet_digest = canonical_worker_certification_digest(&(
            "diagnosticsHistoryRead",
            "RichDiagnosticsHistoryRead",
            diagnostics_history_digest.as_str(),
            diagnostics_cold_reconstruction_count,
            boundary_performance.performance_digest.as_str(),
            worker_first_truth_digest.as_str(),
        ))?;

        Ok(Self {
            envelope_family: "diagnosticsHistoryRead",
            read_mode: "RichDiagnosticsHistoryRead",
            runtime_authority: "workerOwnedRuntime",
            diagnostics_summary_read_count: 0,
            diagnostics_rich_read_count: 1,
            diagnostics_cold_reconstruction_count,
            worker_first_truth_digest,
            diagnostics_history_digest,
            boundary_performance,
            packet_digest,
            history,
        })
    }
}

impl WorkerDiagnosticsSummaryReadCertificationPackage {
    pub(in crate::runtime::worker_host) fn from_worker_retained_packet(
        shell: &WorkerRuntimeShell,
    ) -> Result<Self, WorthSignalJsError> {
        let packet = shell.latest_worker_diagnostics_summary_read_packet()?;
        let worker_first_truth_digest = committed_truth_digest_for_runtime(&shell.core)?;
        if packet.worker_first_truth_digest != worker_first_truth_digest {
            return Err(WorthSignalJsError::invalid_input(
                "worker diagnostics summary certification requires current summary evidence",
            ));
        }
        let certification_digest = canonical_worker_certification_digest(&(
            "workerDiagnosticsSummaryReadCertification",
            packet.diagnostics_summary_digest.as_str(),
            packet.rich_read_availability_digest.as_str(),
            packet.boundary_performance.performance_digest.as_str(),
            packet.packet_digest.as_str(),
            worker_first_truth_digest.as_str(),
        ))?;

        Ok(Self {
            certification_family: "workerDiagnosticsSummaryReadCertification",
            covered_suite_count: 1,
            diagnostics_summary_read_count: packet.diagnostics_summary_read_count,
            diagnostics_rich_read_count: packet.diagnostics_rich_read_count,
            diagnostics_cold_reconstruction_count: packet.diagnostics_cold_reconstruction_count,
            worker_first_truth_digest,
            diagnostics_summary_digest: packet.diagnostics_summary_digest.clone(),
            rich_read_availability_digest: packet.rich_read_availability_digest.clone(),
            boundary_performance_digest: packet.boundary_performance.performance_digest.clone(),
            packet_digest: packet.packet_digest.clone(),
            certification_digest,
        })
    }
}

impl WorkerRuntimeShell {
    pub fn read_diagnostics_summary(
        &mut self,
    ) -> Result<WorkerDiagnosticsSummaryReadPacket, WorthSignalJsError> {
        let packet = WorkerDiagnosticsSummaryReadPacket::from_summary(
            self.core.diagnostics_summary_now()?,
            committed_truth_digest_for_runtime(&self.core)?,
        )?;
        self.latest_worker_diagnostics_summary_read_packet = Some(packet.clone());
        Ok(packet)
    }

    pub fn read_diagnostics_history(
        &mut self,
    ) -> Result<WorkerDiagnosticsHistoryReadPacket, WorthSignalJsError> {
        WorkerDiagnosticsHistoryReadPacket::from_history(
            self.core.execution_history_now()?,
            committed_truth_digest_for_runtime(&self.core)?,
        )
    }

    pub fn certify_worker_diagnostics_summary_read(
        &self,
    ) -> Result<WorkerDiagnosticsSummaryReadCertificationPackage, WorthSignalJsError> {
        WorkerDiagnosticsSummaryReadCertificationPackage::from_worker_retained_packet(self)
    }

    pub(in crate::runtime::worker_host) fn latest_worker_diagnostics_summary_read_packet(
        &self,
    ) -> Result<&WorkerDiagnosticsSummaryReadPacket, WorthSignalJsError> {
        self.latest_worker_diagnostics_summary_read_packet
            .as_ref()
            .ok_or_else(|| {
                WorthSignalJsError::invalid_input(
                    "worker diagnostics summary certification requires summary evidence",
                )
            })
    }
}

fn diagnostics_rich_read_availability_digest(
    diagnostics_summary_digest: &str,
) -> Result<String, WorthSignalJsError> {
    canonical_worker_certification_digest(&(
        "richDiagnosticsHistoryReadAvailable",
        diagnostics_summary_digest,
    ))
}

fn rich_history_reconstruction_count(history: &ExecutionHistorySurfaceSummary) -> u64 {
    if history.callback_nodes.is_empty() {
        1
    } else {
        history.callback_nodes.len() as u64
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StableDiagnosticsSummaryProjection {
    active_node_count: u32,
    clean_node_count: u32,
    maybe_stale_node_count: u32,
    dirty_node_count: u32,
    dependency_edge_count: u32,
    subscriber_edge_count: u32,
    nodes_with_execution_record: u32,
    nodes_with_causality: u32,
}

impl From<&GraphSummary> for StableDiagnosticsSummaryProjection {
    fn from(summary: &GraphSummary) -> Self {
        Self {
            active_node_count: summary.active_node_count,
            clean_node_count: summary.clean_node_count,
            maybe_stale_node_count: summary.maybe_stale_node_count,
            dirty_node_count: summary.dirty_node_count,
            dependency_edge_count: summary.dependency_edge_count,
            subscriber_edge_count: summary.subscriber_edge_count,
            nodes_with_execution_record: summary.nodes_with_execution_record,
            nodes_with_causality: summary.nodes_with_causality,
        }
    }
}
