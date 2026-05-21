use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::boundary::serde::{from_js, to_js};

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
    pub fn why(&self, id: String) -> Result<JsValue, JsValue> {
        let summary = self.shell.borrow().why(&id).map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = latestFlow)]
    pub fn latest_flow(&self) -> Result<JsValue, JsValue> {
        let flow = self.shell.borrow().latest_flow().map_err(JsValue::from)?;
        to_nullable_js(flow)
    }

    #[wasm_bindgen(js_name = latestObservation)]
    pub fn latest_observation(&self) -> Result<JsValue, JsValue> {
        let observation = self
            .shell
            .borrow()
            .latest_observation()
            .map_err(JsValue::from)?;
        to_nullable_js(observation)
    }

    #[wasm_bindgen(js_name = recentHistory)]
    pub fn recent_history(&self) -> Result<JsValue, JsValue> {
        let history = self
            .shell
            .borrow()
            .recent_history()
            .map_err(JsValue::from)?;
        to_js(&history).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = replayFor)]
    pub fn replay_for(&self, id: String) -> Result<JsValue, JsValue> {
        let replay = self
            .shell
            .borrow_mut()
            .replay_for_id(&id)
            .map_err(JsValue::from)?;
        to_js(&replay).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = lineageFor)]
    pub fn lineage_for(&self, id: String) -> Result<JsValue, JsValue> {
        let lineage = self
            .shell
            .borrow_mut()
            .lineage_for_id(&id)
            .map_err(JsValue::from)?;
        to_js(&lineage).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = readVersions)]
    pub fn read_versions(&self, ids: JsValue) -> Result<JsValue, JsValue> {
        let ids: Vec<String> = from_js(ids)?;
        let versions = self
            .shell
            .borrow_mut()
            .read_versions(ids)
            .map_err(JsValue::from)?;
        to_js(&versions).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = exportDefinitions)]
    pub fn export_definitions(&self) -> Result<JsValue, JsValue> {
        let definitions = self
            .shell
            .borrow_mut()
            .export_definitions()
            .map_err(JsValue::from)?;
        to_js(&definitions).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = runtimeProofReport)]
    pub fn runtime_proof_report(&self) -> Result<JsValue, JsValue> {
        let report = self.shell.borrow().runtime_proof_report();
        to_js(&report).map_err(JsValue::from)
    }
}
