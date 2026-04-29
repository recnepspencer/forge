use wasm_bindgen::prelude::*;

use crate::boundary::serde::{from_js, to_js};

use super::types::{SignalAdapters, SignalSpecialist};

const RUNTIME_ENVELOPE_JS_BOUNDARY_DEFERRED: &str = "runtimeEnvelopeJsBoundaryDeferred";

fn runtime_envelope_js_boundary_deferred_error() -> crate::boundary::errors::ForgeSignalJsError {
    crate::boundary::errors::ForgeSignalJsError::deferred(
        RUNTIME_ENVELOPE_JS_BOUNDARY_DEFERRED,
        "runtime envelope export/import is intentionally deferred on the wasm JS boundary until the boundary can produce a self-describing portable snapshot artifact",
        None,
    )
}

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
        Err(JsValue::from(runtime_envelope_js_boundary_deferred_error()))
    }

    pub fn runtime_proof_report(&self) -> Result<JsValue, JsValue> {
        let report = self.core.borrow().runtime_proof_report();
        to_js(&report).map_err(JsValue::from)
    }

    pub fn replace_runtime_envelope(&self, _envelope: JsValue) -> Result<(), JsValue> {
        Err(JsValue::from(runtime_envelope_js_boundary_deferred_error()))
    }
}

#[cfg(test)]
impl SignalAdapters {
    pub(super) fn runtime_envelope_js_boundary_deferred_error_for_test(
        &self,
    ) -> crate::boundary::errors::ForgeSignalJsError {
        runtime_envelope_js_boundary_deferred_error()
    }
}
