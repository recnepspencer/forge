use wasm_bindgen::prelude::JsValue;

use super::SignalWorkerRuntime;
use crate::recipe::model::TransactionOp;
use crate::runtime::adapters::RuntimeEnvelope;
use crate::runtime::summaries::RuntimeSnapshotEnvelope;
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
