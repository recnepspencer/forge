use wasm_bindgen::prelude::JsValue;

use worth_signal::facade::adapters::BranchStateProofReport;

use crate::recipe::model::TransactionOp;
use crate::runtime::adapters::RuntimeEnvelope;
use crate::runtime::adapters::{
    MergePlanArtifactSummary, MergePlanProofEnvelope, MergeResultArtifactSummary,
    MergeResultProofEnvelope,
};
use crate::runtime::core::MergePolicyPreviewRequest;
use crate::runtime::summaries::{ReplaySummary, RuntimeSnapshotEnvelope};
use crate::runtime::worker_host::{
    WorkerBrowserHistoryIngress, WorkerBrowserHistoryIngressReport,
    WorkerCallbackCapabilityExportCertificationPackage,
    WorkerCallbackPhase4CloseoutCertificationPackage, WorkerCommittedTransactionEnvelope,
    WorkerGraphPublicationSummary, WorkerHostCapabilityIngressBatch,
    WorkerHostCapabilityIngressReport, WorkerHostEffectAcknowledgement,
    WorkerHostEffectAcknowledgementReport, WorkerHostEffectRequest,
    WorkerHostEffectRequestEnvelope, WorkerImportExportCallbackUnavailabilityCertificationPackage,
    WorkerMainThreadHostBridgeCertificationPackage, WorkerMainThreadHostedCallbackRequestEnvelope,
    WorkerMainThreadHostedCallbackResult, WorkerMainThreadHostedCallbackResultReport,
    WorkerObservationDeliveryCertificationPackage, WorkerObservationDeliveryPacket,
    WorkerOutputDeliveryCertificationPackage, WorkerOutputDeliveryPacket,
    WorkerOutputDeliveryRequest, WorkerPortableGraphPublication, WorkerRuntimeBootstrapRecord,
    WorkerRuntimeEnvelopeImportReport, WorkerRuntimeShellLock,
};
use worth_signal::facade::history::RuntimeSnapshot;

use super::SignalWorkerRuntime;

impl SignalWorkerRuntime {
    pub(crate) fn bootstrap_record_for_test(
        &self,
    ) -> Result<WorkerRuntimeBootstrapRecord, JsValue> {
        Ok(self.shell.borrow().bootstrap_record())
    }

    pub(crate) fn worker_runtime_shell_lock_for_test(
        &self,
    ) -> Result<WorkerRuntimeShellLock, JsValue> {
        Ok(self.shell.borrow().shell_lock())
    }

    pub(crate) fn publish_portable_graph_for_test(
        &self,
        publication: WorkerPortableGraphPublication,
    ) -> Result<WorkerGraphPublicationSummary, JsValue> {
        self.shell
            .borrow_mut()
            .publish_graph(publication)
            .map_err(JsValue::from)
    }

    pub(crate) fn apply_transaction_for_test(
        &self,
        transaction_ops: Vec<TransactionOp>,
    ) -> Result<WorkerCommittedTransactionEnvelope, JsValue> {
        self.shell
            .borrow_mut()
            .apply_committed_transaction(transaction_ops)
            .map_err(JsValue::from)
    }

    pub(crate) fn admit_host_capability_ingress_for_test(
        &self,
        batch: WorkerHostCapabilityIngressBatch,
    ) -> Result<WorkerHostCapabilityIngressReport, JsValue> {
        self.shell
            .borrow_mut()
            .admit_host_capability_ingress(batch)
            .map_err(JsValue::from)
    }

    pub(crate) fn admit_browser_history_ingress_for_test(
        &self,
        ingress: WorkerBrowserHistoryIngress,
    ) -> Result<WorkerBrowserHistoryIngressReport, JsValue> {
        self.shell
            .borrow_mut()
            .admit_browser_history_ingress(ingress)
            .map_err(JsValue::from)
    }

    pub(crate) fn issue_host_effect_request_for_test(
        &self,
        request: WorkerHostEffectRequest,
    ) -> Result<WorkerHostEffectRequestEnvelope, JsValue> {
        self.shell
            .borrow_mut()
            .issue_host_effect_request(request)
            .map_err(JsValue::from)
    }

    pub(crate) fn admit_host_effect_acknowledgement_for_test(
        &self,
        acknowledgement: WorkerHostEffectAcknowledgement,
    ) -> Result<WorkerHostEffectAcknowledgementReport, JsValue> {
        self.shell
            .borrow_mut()
            .admit_host_effect_acknowledgement(acknowledgement)
            .map_err(JsValue::from)
    }

    pub(crate) fn certify_main_thread_host_bridge_for_test(
        &self,
    ) -> Result<WorkerMainThreadHostBridgeCertificationPackage, JsValue> {
        self.shell
            .borrow()
            .certify_main_thread_host_bridge()
            .map_err(JsValue::from)
    }

    pub(crate) fn issue_main_thread_hosted_callback_request_for_test(
        &self,
        callback_id: String,
    ) -> Result<WorkerMainThreadHostedCallbackRequestEnvelope, JsValue> {
        self.shell
            .borrow_mut()
            .issue_main_thread_hosted_callback_request(&callback_id)
            .map_err(JsValue::from)
    }

    pub(crate) fn admit_main_thread_hosted_callback_result_for_test(
        &self,
        request: WorkerMainThreadHostedCallbackRequestEnvelope,
        result: WorkerMainThreadHostedCallbackResult,
    ) -> Result<WorkerMainThreadHostedCallbackResultReport, JsValue> {
        self.shell
            .borrow_mut()
            .admit_main_thread_hosted_callback_result(request, result)
            .map_err(JsValue::from)
    }

    pub(crate) fn certify_main_thread_hosted_callback_execution_for_test(
        &self,
    ) -> Result<
        crate::runtime::worker_host::WorkerMainThreadHostedCallbackExecutionCertificationPackage,
        JsValue,
    > {
        self.shell
            .borrow()
            .certify_main_thread_hosted_callback_execution()
            .map_err(JsValue::from)
    }

    pub(crate) fn export_worker_runtime_envelope_for_test(
        &self,
    ) -> Result<RuntimeEnvelope, JsValue> {
        self.shell
            .borrow_mut()
            .export_worker_runtime_envelope()
            .map_err(JsValue::from)
    }

    pub(crate) fn export_worker_snapshot_envelope_for_test(
        &self,
    ) -> Result<RuntimeSnapshotEnvelope, JsValue> {
        self.shell
            .borrow_mut()
            .export_worker_snapshot_envelope()
            .map_err(JsValue::from)
    }

    pub(crate) fn restore_snapshot_for_test(
        &self,
        snapshot: RuntimeSnapshotEnvelope,
    ) -> Result<crate::runtime::worker_host::WorkerBranchTruthEnvelope, JsValue> {
        self.shell
            .borrow_mut()
            .restore_snapshot(snapshot)
            .map_err(JsValue::from)
    }

    pub(crate) fn current_branch_for_test(
        &self,
    ) -> Result<worth_signal::facade::history::RuntimeBranch, JsValue> {
        Ok(self.shell.borrow().current_branch())
    }

    pub(crate) fn branches_for_test(
        &self,
    ) -> Result<Vec<worth_signal::facade::history::RuntimeBranch>, JsValue> {
        Ok(self.shell.borrow().branches())
    }

    pub(crate) fn replay_for_branch_for_test(
        &self,
        branch_id: u64,
    ) -> Result<ReplaySummary, JsValue> {
        self.shell
            .borrow_mut()
            .replay_for_branch(branch_id)
            .map_err(JsValue::from)
    }

    pub(crate) fn branch_snapshot_id_for_test(&self, branch_id: u64) -> Result<u64, JsValue> {
        self.shell
            .borrow_mut()
            .branch_snapshot_id(branch_id)
            .map_err(JsValue::from)
    }

    pub(crate) fn branch_snapshot_envelope_for_test(
        &self,
        branch_id: u64,
    ) -> Result<RuntimeSnapshotEnvelope, JsValue> {
        self.shell
            .borrow_mut()
            .branch_snapshot_envelope(branch_id)
            .map_err(JsValue::from)
    }

    pub(crate) fn branch_snapshot_for_test(
        &self,
        branch_id: u64,
    ) -> Result<RuntimeSnapshot, JsValue> {
        self.shell
            .borrow_mut()
            .branch_snapshot(branch_id)
            .map_err(JsValue::from)
    }

    pub(crate) fn create_branch_for_test(
        &self,
        name: String,
    ) -> Result<worth_signal::facade::history::RuntimeBranch, JsValue> {
        self.shell
            .borrow_mut()
            .create_branch(name)
            .map_err(JsValue::from)
    }

    pub(crate) fn switch_branch_for_test(
        &self,
        branch_id: u64,
    ) -> Result<crate::runtime::worker_host::WorkerBranchTruthEnvelope, JsValue> {
        self.shell
            .borrow_mut()
            .switch_branch(branch_id)
            .map_err(JsValue::from)
    }

    pub(crate) fn restore_branch_snapshot_for_test(
        &self,
        branch_id: u64,
        snapshot: RuntimeSnapshot,
    ) -> Result<crate::runtime::worker_host::WorkerBranchTruthEnvelope, JsValue> {
        self.shell
            .borrow_mut()
            .restore_branch_snapshot(branch_id, snapshot)
            .map_err(JsValue::from)
    }

    pub(crate) fn restore_branch_snapshot_by_id_for_test(
        &self,
        branch_id: u64,
        snapshot_id: u64,
    ) -> Result<crate::runtime::worker_host::WorkerBranchTruthEnvelope, JsValue> {
        self.shell
            .borrow_mut()
            .restore_branch_snapshot_by_id(branch_id, snapshot_id)
            .map_err(JsValue::from)
    }

    pub(crate) fn branch_state_proof_for_test(
        &self,
        branch_id: u64,
    ) -> Result<BranchStateProofReport, JsValue> {
        self.shell
            .borrow()
            .branch_state_proof(branch_id)
            .map_err(JsValue::from)
    }

    pub(crate) fn plan_merge_branches_for_test(
        &self,
        source_branch_id: u64,
        target_branch_id: u64,
    ) -> Result<MergePlanArtifactSummary, JsValue> {
        self.shell
            .borrow_mut()
            .plan_merge_branches(source_branch_id, target_branch_id)
            .map_err(JsValue::from)
    }

    pub(crate) fn plan_merge_branches_with_proof_for_test(
        &self,
        source_branch_id: u64,
        target_branch_id: u64,
    ) -> Result<MergePlanProofEnvelope, JsValue> {
        self.shell
            .borrow_mut()
            .plan_merge_branches_with_proof(source_branch_id, target_branch_id)
            .map_err(JsValue::from)
    }

    pub(crate) fn merge_branches_for_test(
        &self,
        source_branch_id: u64,
        target_branch_id: u64,
    ) -> Result<MergeResultArtifactSummary, JsValue> {
        self.shell
            .borrow_mut()
            .merge_branches(source_branch_id, target_branch_id)
            .map_err(JsValue::from)
    }

    pub(crate) fn merge_branches_with_proof_for_test(
        &self,
        source_branch_id: u64,
        target_branch_id: u64,
    ) -> Result<MergeResultProofEnvelope, JsValue> {
        self.shell
            .borrow_mut()
            .merge_branches_with_proof(source_branch_id, target_branch_id)
            .map_err(JsValue::from)
    }

    pub(crate) fn plan_merge_policy_preview_for_test(
        &self,
        request: MergePolicyPreviewRequest,
    ) -> Result<MergePlanArtifactSummary, JsValue> {
        self.shell
            .borrow_mut()
            .plan_merge_policy_preview(request)
            .map_err(JsValue::from)
    }

    pub(crate) fn plan_merge_policy_preview_with_proof_for_test(
        &self,
        request: MergePolicyPreviewRequest,
    ) -> Result<MergePlanProofEnvelope, JsValue> {
        self.shell
            .borrow_mut()
            .plan_merge_policy_preview_with_proof(request)
            .map_err(JsValue::from)
    }

    pub(crate) fn merge_branches_policy_preview_for_test(
        &self,
        request: MergePolicyPreviewRequest,
    ) -> Result<MergeResultArtifactSummary, JsValue> {
        self.shell
            .borrow_mut()
            .merge_branches_policy_preview(request)
            .map_err(JsValue::from)
    }

    pub(crate) fn merge_branches_policy_preview_with_proof_for_test(
        &self,
        request: MergePolicyPreviewRequest,
    ) -> Result<MergeResultProofEnvelope, JsValue> {
        self.shell
            .borrow_mut()
            .merge_branches_policy_preview_with_proof(request)
            .map_err(JsValue::from)
    }

    pub(crate) fn export_exact_worker_runtime_restore_artifact_for_test(
        &self,
    ) -> Result<crate::runtime::core::ExactRuntimeRestoreArtifact, JsValue> {
        self.shell
            .borrow_mut()
            .export_exact_worker_runtime_restore_artifact()
            .map_err(JsValue::from)
    }

    pub(crate) fn certify_worker_callback_capability_export_for_test(
        &self,
    ) -> Result<WorkerCallbackCapabilityExportCertificationPackage, JsValue> {
        self.shell
            .borrow_mut()
            .certify_worker_callback_capability_export()
            .map_err(JsValue::from)
    }

    pub(crate) fn certify_worker_callback_phase4_closeout_for_test(
        &self,
    ) -> Result<WorkerCallbackPhase4CloseoutCertificationPackage, JsValue> {
        self.shell
            .borrow()
            .certify_worker_callback_phase4_closeout()
            .map_err(JsValue::from)
    }

    pub(crate) fn certify_worker_import_export_callback_unavailability_for_test(
        &self,
    ) -> Result<WorkerImportExportCallbackUnavailabilityCertificationPackage, JsValue> {
        self.shell
            .borrow_mut()
            .certify_worker_import_export_callback_unavailability()
            .map_err(JsValue::from)
    }

    pub(crate) fn admit_worker_runtime_envelope_import_for_test(
        &self,
        envelope: RuntimeEnvelope,
    ) -> Result<WorkerRuntimeEnvelopeImportReport, JsValue> {
        self.shell
            .borrow_mut()
            .admit_worker_runtime_envelope_import(envelope)
            .map_err(JsValue::from)
    }

    pub(crate) fn admit_exact_worker_runtime_restore_artifact_for_test(
        &self,
        envelope: crate::runtime::core::ExactRuntimeRestoreArtifact,
    ) -> Result<WorkerRuntimeEnvelopeImportReport, JsValue> {
        self.shell
            .borrow_mut()
            .admit_exact_worker_runtime_restore_artifact(envelope)
            .map_err(JsValue::from)
    }

    pub(crate) fn deliver_latest_observation_for_test(
        &self,
    ) -> Result<WorkerObservationDeliveryPacket, JsValue> {
        self.shell
            .borrow_mut()
            .deliver_latest_observation()
            .map_err(JsValue::from)
    }

    pub(crate) fn certify_worker_observation_delivery_for_test(
        &self,
    ) -> Result<WorkerObservationDeliveryCertificationPackage, JsValue> {
        self.shell
            .borrow()
            .certify_worker_observation_delivery()
            .map_err(JsValue::from)
    }

    pub(crate) fn deliver_outputs_for_test(
        &self,
        request: WorkerOutputDeliveryRequest,
    ) -> Result<WorkerOutputDeliveryPacket, JsValue> {
        self.shell
            .borrow_mut()
            .deliver_outputs(request)
            .map_err(JsValue::from)
    }

    pub(crate) fn certify_worker_output_delivery_for_test(
        &self,
    ) -> Result<WorkerOutputDeliveryCertificationPackage, JsValue> {
        self.shell
            .borrow()
            .certify_worker_output_delivery()
            .map_err(JsValue::from)
    }
}
