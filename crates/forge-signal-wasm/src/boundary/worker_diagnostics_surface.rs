use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::boundary::serde::to_js;

use super::types::SignalWorkerRuntime;

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
impl SignalWorkerRuntime {
    pub fn health(&self) -> Result<JsValue, JsValue> {
        let summary = self.shell.borrow().health().map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = performanceSummary)]
    pub fn performance_summary(&self) -> Result<JsValue, JsValue> {
        let summary = self.shell.borrow().performance_summary();
        to_js(&summary).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = latestFailure)]
    pub fn latest_failure(&self) -> Result<JsValue, JsValue> {
        let failure = self
            .shell
            .borrow()
            .latest_failure()
            .map_err(JsValue::from)?;
        to_nullable_js(failure)
    }

    #[wasm_bindgen(js_name = latestRollback)]
    pub fn latest_rollback(&self) -> Result<JsValue, JsValue> {
        let rollback = self
            .shell
            .borrow()
            .latest_rollback()
            .map_err(JsValue::from)?;
        to_nullable_js(rollback)
    }

    #[wasm_bindgen(js_name = latestFrontierExecution)]
    pub fn latest_frontier_execution(&self) -> Result<JsValue, JsValue> {
        let frontier = self
            .shell
            .borrow()
            .latest_frontier_execution()
            .map_err(JsValue::from)?;
        to_nullable_js(frontier)
    }

    #[wasm_bindgen(js_name = latestInvalidationTraceRecords)]
    pub fn latest_invalidation_trace_records(&self) -> Result<JsValue, JsValue> {
        let records = self
            .shell
            .borrow()
            .latest_invalidation_trace_records()
            .map_err(JsValue::from)?;
        to_js(&records).map_err(JsValue::from)
    }
}
