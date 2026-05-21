use serde::{Deserialize, Serialize};

use crate::boundary::errors::ForgeSignalJsError;
use crate::recipe::model::TransactionOp;

use super::{
    canonical_worker_certification_digest, WorkerCommittedTransactionEnvelope,
    WorkerDiagnosticsHistoryReadPacket, WorkerDiagnosticsSummaryReadPacket,
    WorkerOutputDeliveryPacket, WorkerOutputDeliveryRequest, WorkerRuntimeShell,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerCommittedProjectionRequest {
    pub transaction_ops: Vec<TransactionOp>,
    pub output_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerCommittedProjectionPacket {
    pub envelope_family: &'static str,
    pub deployment_posture: &'static str,
    pub runtime_authority: &'static str,
    pub worker_first_truth_digest: String,
    pub projection_digest: String,
    pub packet_digest: String,
    pub transaction: WorkerCommittedTransactionEnvelope,
    pub outputs: WorkerOutputDeliveryPacket,
    pub diagnostics_summary: WorkerDiagnosticsSummaryReadPacket,
    pub diagnostics_history: WorkerDiagnosticsHistoryReadPacket,
}

impl WorkerCommittedProjectionPacket {
    fn from_projection_parts(
        transaction: WorkerCommittedTransactionEnvelope,
        outputs: WorkerOutputDeliveryPacket,
        diagnostics_summary: WorkerDiagnosticsSummaryReadPacket,
        diagnostics_history: WorkerDiagnosticsHistoryReadPacket,
    ) -> Result<Self, ForgeSignalJsError> {
        let worker_first_truth_digest = transaction.committed_truth_digest.clone();
        reject_projection_truth_mismatch(
            worker_first_truth_digest.as_str(),
            outputs.worker_first_truth_digest.as_str(),
            "output delivery",
        )?;
        reject_projection_truth_mismatch(
            worker_first_truth_digest.as_str(),
            diagnostics_summary.worker_first_truth_digest.as_str(),
            "diagnostics summary",
        )?;
        reject_projection_truth_mismatch(
            worker_first_truth_digest.as_str(),
            diagnostics_history.worker_first_truth_digest.as_str(),
            "diagnostics history",
        )?;

        let projection_digest = canonical_worker_certification_digest(&(
            "workerCommittedProjection",
            transaction.committed_truth_digest.as_str(),
            outputs.output_digest.as_str(),
            diagnostics_summary.diagnostics_summary_digest.as_str(),
            diagnostics_history.diagnostics_history_digest.as_str(),
        ))?;
        let packet_digest = canonical_worker_certification_digest(&(
            "workerCommittedProjectionPacket",
            worker_first_truth_digest.as_str(),
            projection_digest.as_str(),
            outputs.packet_digest.as_str(),
            diagnostics_summary.packet_digest.as_str(),
            diagnostics_history.packet_digest.as_str(),
        ))?;

        Ok(Self {
            envelope_family: "workerCommittedProjection",
            deployment_posture: "workerFirst",
            runtime_authority: "workerOwnedRuntime",
            worker_first_truth_digest,
            projection_digest,
            packet_digest,
            transaction,
            outputs,
            diagnostics_summary,
            diagnostics_history,
        })
    }
}

impl WorkerRuntimeShell {
    pub fn apply_committed_projection(
        &mut self,
        request: WorkerCommittedProjectionRequest,
    ) -> Result<WorkerCommittedProjectionPacket, ForgeSignalJsError> {
        let transaction = self.apply_committed_transaction(request.transaction_ops)?;
        let outputs = self.deliver_outputs(WorkerOutputDeliveryRequest {
            output_ids: request.output_ids,
        })?;
        let diagnostics_summary = self.read_diagnostics_summary()?;
        let diagnostics_history = self.read_diagnostics_history()?;
        WorkerCommittedProjectionPacket::from_projection_parts(
            transaction,
            outputs,
            diagnostics_summary,
            diagnostics_history,
        )
    }
}

fn reject_projection_truth_mismatch(
    expected_truth_digest: &str,
    candidate_truth_digest: &str,
    surface: &str,
) -> Result<(), ForgeSignalJsError> {
    if expected_truth_digest == candidate_truth_digest {
        return Ok(());
    }
    Err(ForgeSignalJsError::internal(format!(
        "worker committed projection requires aligned worker-first truth across {surface}",
    )))
}
