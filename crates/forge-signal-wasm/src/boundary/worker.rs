use std::cell::RefCell;

use wasm_bindgen::prelude::*;

use crate::boundary::serde::{from_js, to_js};
use crate::recipe::model::TransactionOp;
use crate::runtime::policy::RuntimePolicySpec;
use crate::runtime::worker_host::{
    WorkerBrowserHistoryIngress, WorkerBrowserHistoryIngressReport,
    WorkerCommittedTransactionEnvelope, WorkerGraphPublicationSummary,
    WorkerHostCapabilityIngressBatch, WorkerHostCapabilityIngressReport,
    WorkerHostEffectAcknowledgement, WorkerHostEffectAcknowledgementReport,
    WorkerHostEffectRequest, WorkerHostEffectRequestEnvelope,
    WorkerMainThreadHostBridgeCertificationPackage, WorkerPortableGraphPublication,
    WorkerRuntimeBootstrapRecord, WorkerRuntimeShell, WorkerRuntimeShellLock,
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
}
