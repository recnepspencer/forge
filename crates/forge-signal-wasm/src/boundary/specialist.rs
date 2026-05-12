use wasm_bindgen::prelude::*;

use crate::boundary::restore_tokens::{load_runtime_envelope, store_runtime_envelope};
use crate::boundary::serde::{
    from_js, from_portable_wire, to_js, to_js_structured, to_portable_wire,
};
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
            .borrow_mut()
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
        to_js_structured(&envelope).map_err(JsValue::from)
    }

    pub fn export_runtime_envelope_wire(&self) -> Result<String, JsValue> {
        let envelope = self
            .core
            .borrow_mut()
            .export_exact_runtime_restore_artifact()
            .map_err(JsValue::from)?;
        Ok(store_runtime_envelope(envelope))
    }

    pub fn export_runtime_envelope_portable_wire(&self) -> Result<String, JsValue> {
        let envelope = self
            .core
            .borrow_mut()
            .export_runtime_envelope()
            .map_err(JsValue::from)?;
        to_portable_wire(&envelope).map_err(JsValue::from)
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

    pub fn replace_runtime_envelope_wire(&self, envelope: String) -> Result<(), JsValue> {
        let envelope = load_runtime_envelope(&envelope).map_err(JsValue::from)?;
        self.core
            .borrow_mut()
            .replace_runtime_envelope_exact(envelope)
            .map_err(JsValue::from)
    }

    pub fn replace_runtime_envelope_portable_wire(&self, envelope: String) -> Result<(), JsValue> {
        let envelope: RuntimeEnvelope = from_portable_wire(&envelope).map_err(JsValue::from)?;
        self.core
            .borrow_mut()
            .replace_runtime_envelope(envelope)
            .map_err(JsValue::from)
    }
}

#[cfg(test)]
impl SignalAdapters {
    pub(super) fn export_runtime_envelope_for_test(
        &self,
    ) -> Result<RuntimeEnvelope, crate::boundary::errors::ForgeSignalJsError> {
        self.core.borrow_mut().export_runtime_envelope()
    }

    pub(super) fn replace_runtime_envelope_for_test(
        &self,
        envelope: RuntimeEnvelope,
    ) -> Result<(), crate::boundary::errors::ForgeSignalJsError> {
        self.core.borrow_mut().replace_runtime_envelope(envelope)
    }
}
