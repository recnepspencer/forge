use wasm_bindgen::prelude::*;

use crate::boundary::serde::to_js;
use serde::Serialize;

use super::types::SignalDiagnostics;

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
