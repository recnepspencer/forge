use js_sys::Function;
use wasm_bindgen::prelude::*;

use crate::boundary::serde::{from_js, to_js};
use crate::recipe::model::TransactionOp;
use crate::runtime::worker_host::{
    certify_worker_compatibility, probe_worker_branch_lifecycle_parity,
    probe_worker_graph_committed_truth_parity, WorkerCompatibilityCertificationScenario,
    WorkerPortableGraphPublication,
};
use serde::Serialize;

use super::types::{DisposableHandle, SignalDiagnostics};

fn to_nullable_js<T>(value: Option<T>) -> Result<JsValue, JsValue>
where
    T: Serialize,
{
    match value {
        Some(value) => to_js(&value).map_err(JsValue::from),
        None => Ok(JsValue::NULL),
    }
}

#[wasm_bindgen]
impl SignalDiagnostics {
    pub fn why(&self, id: String) -> Result<JsValue, JsValue> {
        let summary = self.core.borrow().why(&id).map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    pub fn health(&self) -> Result<JsValue, JsValue> {
        let summary = self.core.borrow().health().map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = summaryNow)]
    pub fn summary_now(&self) -> Result<JsValue, JsValue> {
        let summary = self
            .core
            .borrow()
            .diagnostics_summary_now()
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = historyNow)]
    pub fn history_now(&self) -> Result<JsValue, JsValue> {
        let history = self
            .core
            .borrow()
            .execution_history_now()
            .map_err(JsValue::from)?;
        to_js(&history).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = latestFlow)]
    pub fn latest_flow(&self) -> Result<JsValue, JsValue> {
        let flow = self.core.borrow().latest_flow().map_err(JsValue::from)?;
        to_nullable_js(flow)
    }

    #[wasm_bindgen(js_name = latestObservation)]
    pub fn latest_observation(&self) -> Result<JsValue, JsValue> {
        let summary = self
            .core
            .borrow()
            .latest_observation()
            .map_err(JsValue::from)?;
        to_nullable_js(summary)
    }

    #[wasm_bindgen(js_name = performanceSummary)]
    pub fn performance_summary(&self) -> Result<JsValue, JsValue> {
        let summary = self.core.borrow().web_performance_summary();
        to_js(&summary).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = workerPlacementSummary)]
    pub fn worker_placement_summary(&self) -> Result<JsValue, JsValue> {
        let summary = self.core.borrow().worker_placement_summary()?;
        to_js(&summary).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = workerCallbackPlacementEligibility)]
    pub fn worker_callback_placement_eligibility(&self) -> Result<JsValue, JsValue> {
        let package = self.core.borrow().worker_callback_placement_eligibility()?;
        to_js(&package).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = workerBoundaryArtifactLock)]
    pub fn worker_boundary_artifact_lock(&self) -> Result<JsValue, JsValue> {
        let lock = self.core.borrow().worker_boundary_artifact_lock();
        to_js(&lock).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = workerRuntimeShellLock)]
    pub fn worker_runtime_shell_lock(&self) -> Result<JsValue, JsValue> {
        let lock = self.core.borrow().worker_runtime_shell_lock();
        to_js(&lock).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = workerRuntimeGraphParityProbe)]
    pub fn worker_runtime_graph_parity_probe(
        &self,
        publication: JsValue,
        transaction_ops: JsValue,
    ) -> Result<JsValue, JsValue> {
        let publication: WorkerPortableGraphPublication =
            from_js(publication).map_err(JsValue::from)?;
        let transaction_ops: Vec<TransactionOp> =
            from_js(transaction_ops).map_err(JsValue::from)?;
        let report = probe_worker_graph_committed_truth_parity(publication, transaction_ops)?;
        to_js(&report).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = workerRuntimeBranchLifecycleParityProbe)]
    pub fn worker_runtime_branch_lifecycle_parity_probe(
        &self,
        publication: JsValue,
        feature_transaction_ops: JsValue,
        main_transaction_ops: JsValue,
    ) -> Result<JsValue, JsValue> {
        let publication: WorkerPortableGraphPublication =
            from_js(publication).map_err(JsValue::from)?;
        let feature_transaction_ops: Vec<TransactionOp> =
            from_js(feature_transaction_ops).map_err(JsValue::from)?;
        let main_transaction_ops: Vec<TransactionOp> =
            from_js(main_transaction_ops).map_err(JsValue::from)?;
        let report = probe_worker_branch_lifecycle_parity(
            publication,
            feature_transaction_ops,
            main_transaction_ops,
        )?;
        to_js(&report).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = workerRuntimeCompatibilityCertification)]
    pub fn worker_runtime_compatibility_certification(
        &self,
        scenario: JsValue,
    ) -> Result<JsValue, JsValue> {
        let scenario: WorkerCompatibilityCertificationScenario =
            from_js(scenario).map_err(JsValue::from)?;
        let report = certify_worker_compatibility(scenario)?;
        to_js(&report).map_err(JsValue::from)
    }

    pub fn subscribe(&self, callback: Function) -> Result<DisposableHandle, JsValue> {
        let callback_token = self
            .core
            .borrow_mut()
            .register_wasm_diagnostics_callback(callback);
        Ok(DisposableHandle {
            core: self.core.clone(),
            observation_handle: None,
            callback_token: None,
            diagnostics_callback_token: Some(callback_token),
        })
    }

    #[wasm_bindgen(js_name = latestFailure)]
    pub fn latest_failure(&self) -> Result<JsValue, JsValue> {
        let failure = self.core.borrow().latest_failure().map_err(JsValue::from)?;
        to_nullable_js(failure)
    }

    #[wasm_bindgen(js_name = latestRollback)]
    pub fn latest_rollback(&self) -> Result<JsValue, JsValue> {
        let rollback = self
            .core
            .borrow()
            .latest_rollback()
            .map_err(JsValue::from)?;
        to_nullable_js(rollback)
    }

    #[wasm_bindgen(js_name = latestFrontierExecution)]
    pub fn latest_frontier_execution(&self) -> Result<JsValue, JsValue> {
        let frontier = self
            .core
            .borrow()
            .latest_frontier_execution()
            .map_err(JsValue::from)?;
        to_nullable_js(frontier)
    }

    #[wasm_bindgen(js_name = latestInvalidationTraceRecords)]
    pub fn latest_invalidation_trace_records(&self) -> Result<JsValue, JsValue> {
        let records = self
            .core
            .borrow()
            .latest_invalidation_trace_records()
            .map_err(JsValue::from)?;
        to_js(&records).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = recentHistory)]
    pub fn recent_history(&self) -> Result<JsValue, JsValue> {
        let history = self.core.borrow().recent_history().map_err(JsValue::from)?;
        to_js(&history).map_err(JsValue::from)
    }
}

#[cfg(test)]
impl SignalDiagnostics {
    pub(super) fn latest_observation_for_test(
        &self,
    ) -> Result<
        Option<crate::runtime::summaries::ObservationSurfaceSummary>,
        crate::boundary::errors::ForgeSignalJsError,
    > {
        self.core.borrow().latest_observation()
    }
}
