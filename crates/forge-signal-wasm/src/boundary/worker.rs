use std::cell::RefCell;

use wasm_bindgen::prelude::*;

use crate::boundary::serde::{from_js, to_js};
use crate::recipe::model::TransactionOp;
use crate::runtime::adapters::RuntimeEnvelope;
use crate::runtime::policy::RuntimePolicySpec;
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
    WorkerRuntimeEnvelopeImportReport, WorkerRuntimeShell, WorkerRuntimeShellLock,
};

use super::types::SignalWorkerRuntime;

#[wasm_bindgen]
impl SignalWorkerRuntime {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<SignalWorkerRuntime, JsValue> {
        Ok(Self {
            shell: RefCell::new(WorkerRuntimeShell::new(RuntimePolicySpec::default())?),
        })
    }

    #[wasm_bindgen(js_name = bootstrapRecord)]
    pub fn bootstrap_record(&self) -> Result<JsValue, JsValue> {
        to_js(&self.bootstrap_record_for_test()?).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = workerRuntimeShellLock)]
    pub fn worker_runtime_shell_lock(&self) -> Result<JsValue, JsValue> {
        to_js(&self.worker_runtime_shell_lock_for_test()?).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = publishPortableGraph)]
    pub fn publish_portable_graph(&self, publication: JsValue) -> Result<JsValue, JsValue> {
        let publication: WorkerPortableGraphPublication =
            from_js(publication).map_err(JsValue::from)?;
        let summary = self.publish_portable_graph_for_test(publication)?;
        to_js(&summary).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = applyTransaction)]
    pub fn apply_transaction(&self, transaction_ops: JsValue) -> Result<JsValue, JsValue> {
        let transaction_ops: Vec<TransactionOp> =
            from_js(transaction_ops).map_err(JsValue::from)?;
        let envelope = self.apply_transaction_for_test(transaction_ops)?;
        to_js(&envelope).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = admitHostCapabilityIngress)]
    pub fn admit_host_capability_ingress(&self, batch: JsValue) -> Result<JsValue, JsValue> {
        let batch: WorkerHostCapabilityIngressBatch = from_js(batch).map_err(JsValue::from)?;
        let report = self.admit_host_capability_ingress_for_test(batch)?;
        to_js(&report).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = admitBrowserHistoryIngress)]
    pub fn admit_browser_history_ingress(&self, ingress: JsValue) -> Result<JsValue, JsValue> {
        let ingress: WorkerBrowserHistoryIngress = from_js(ingress).map_err(JsValue::from)?;
        let report = self.admit_browser_history_ingress_for_test(ingress)?;
        to_js(&report).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = issueHostEffectRequest)]
    pub fn issue_host_effect_request(&self, request: JsValue) -> Result<JsValue, JsValue> {
        let request: WorkerHostEffectRequest = from_js(request).map_err(JsValue::from)?;
        let envelope = self.issue_host_effect_request_for_test(request)?;
        to_js(&envelope).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = admitHostEffectAcknowledgement)]
    pub fn admit_host_effect_acknowledgement(
        &self,
        acknowledgement: JsValue,
    ) -> Result<JsValue, JsValue> {
        let acknowledgement: WorkerHostEffectAcknowledgement =
            from_js(acknowledgement).map_err(JsValue::from)?;
        let report = self.admit_host_effect_acknowledgement_for_test(acknowledgement)?;
        to_js(&report).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = certifyMainThreadHostBridge)]
    pub fn certify_main_thread_host_bridge(&self) -> Result<JsValue, JsValue> {
        let package = self.certify_main_thread_host_bridge_for_test()?;
        to_js(&package).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = issueMainThreadHostedCallbackRequest)]
    pub fn issue_main_thread_hosted_callback_request(
        &self,
        callback_id: String,
    ) -> Result<JsValue, JsValue> {
        let envelope = self.issue_main_thread_hosted_callback_request_for_test(callback_id)?;
        to_js(&envelope).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = admitMainThreadHostedCallbackResult)]
    pub fn admit_main_thread_hosted_callback_result(
        &self,
        request: JsValue,
        result: JsValue,
    ) -> Result<JsValue, JsValue> {
        let request: WorkerMainThreadHostedCallbackRequestEnvelope =
            from_js(request).map_err(JsValue::from)?;
        let result: WorkerMainThreadHostedCallbackResult =
            from_js(result).map_err(JsValue::from)?;
        let report = self.admit_main_thread_hosted_callback_result_for_test(request, result)?;
        to_js(&report).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = certifyMainThreadHostedCallbackExecution)]
    pub fn certify_main_thread_hosted_callback_execution(&self) -> Result<JsValue, JsValue> {
        let package = self.certify_main_thread_hosted_callback_execution_for_test()?;
        to_js(&package).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = exportWorkerRuntimeEnvelope)]
    pub fn export_worker_runtime_envelope(&self) -> Result<JsValue, JsValue> {
        let envelope = self.export_worker_runtime_envelope_for_test()?;
        to_js(&envelope).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = certifyWorkerCallbackCapabilityExport)]
    pub fn certify_worker_callback_capability_export(&self) -> Result<JsValue, JsValue> {
        let package = self.certify_worker_callback_capability_export_for_test()?;
        to_js(&package).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = certifyWorkerCallbackPhase4Closeout)]
    pub fn certify_worker_callback_phase4_closeout(&self) -> Result<JsValue, JsValue> {
        let package = self.certify_worker_callback_phase4_closeout_for_test()?;
        to_js(&package).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = certifyWorkerImportExportCallbackUnavailability)]
    pub fn certify_worker_import_export_callback_unavailability(&self) -> Result<JsValue, JsValue> {
        let package = self.certify_worker_import_export_callback_unavailability_for_test()?;
        to_js(&package).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = admitWorkerRuntimeEnvelopeImport)]
    pub fn admit_worker_runtime_envelope_import(
        &self,
        envelope: JsValue,
    ) -> Result<JsValue, JsValue> {
        let envelope: RuntimeEnvelope = from_js(envelope).map_err(JsValue::from)?;
        let report = self.admit_worker_runtime_envelope_import_for_test(envelope)?;
        to_js(&report).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = deliverLatestObservation)]
    pub fn deliver_latest_observation(&self) -> Result<JsValue, JsValue> {
        let packet = self.deliver_latest_observation_for_test()?;
        to_js(&packet).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = certifyWorkerObservationDelivery)]
    pub fn certify_worker_observation_delivery(&self) -> Result<JsValue, JsValue> {
        let package = self.certify_worker_observation_delivery_for_test()?;
        to_js(&package).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = deliverOutputs)]
    pub fn deliver_outputs(&self, request: JsValue) -> Result<JsValue, JsValue> {
        let request: WorkerOutputDeliveryRequest = from_js(request).map_err(JsValue::from)?;
        let packet = self.deliver_outputs_for_test(request)?;
        to_js(&packet).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = certifyWorkerOutputDelivery)]
    pub fn certify_worker_output_delivery(&self) -> Result<JsValue, JsValue> {
        let package = self.certify_worker_output_delivery_for_test()?;
        to_js(&package).map_err(JsValue::from)
    }
}

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
