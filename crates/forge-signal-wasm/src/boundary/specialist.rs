use wasm_bindgen::prelude::*;

use crate::boundary::serde::{from_js, to_js};
use crate::runtime::adapters::RuntimeEnvelope;

use super::types::{SignalAdapters, SignalSpecialist};

#[wasm_bindgen]
impl SignalSpecialist {
    pub fn graph_summary(&self) -> Result<JsValue, JsValue> {
        let summary = self.core.borrow().graph_summary().map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    pub fn evaluate_dirty(&self) -> Result<JsValue, JsValue> {
        let summary = self
            .core
            .borrow_mut()
            .evaluate_dirty()
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    pub fn read_versions(&self, ids: JsValue) -> Result<JsValue, JsValue> {
        let ids: Vec<String> = from_js(ids)?;
        let versions = self
            .core
            .borrow_mut()
            .read_versions(ids)
            .map_err(JsValue::from)?;
        to_js(&versions).map_err(JsValue::from)
    }
}

#[wasm_bindgen]
impl SignalAdapters {
    pub fn export_definitions(&self) -> Result<JsValue, JsValue> {
        let definitions = self
            .core
            .borrow()
            .export_definitions()
            .map_err(JsValue::from)?;
        to_js(&definitions).map_err(JsValue::from)
    }

    pub fn export_runtime_envelope(&self) -> Result<JsValue, JsValue> {
        let envelope = self
            .core
            .borrow_mut()
            .export_runtime_envelope()
            .map_err(JsValue::from)?;
        to_js(&envelope).map_err(JsValue::from)
    }

    pub fn runtime_proof_report(&self) -> Result<JsValue, JsValue> {
        let report = self.core.borrow().runtime_proof_report();
        to_js(&report).map_err(JsValue::from)
    }

    pub fn replace_runtime_envelope(&self, envelope: JsValue) -> Result<(), JsValue> {
        let envelope: RuntimeEnvelope = from_js(envelope)?;
        self.core
            .borrow_mut()
            .replace_runtime_envelope(envelope)
            .map_err(JsValue::from)
    }
}
