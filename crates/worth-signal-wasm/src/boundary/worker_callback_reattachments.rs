use js_sys::{Array, Function, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::boundary::errors::WorthSignalJsError;
use crate::boundary::serde::{from_js, to_js};
use crate::runtime::adapters::{RuntimeDefinitionEnvelope, RuntimeEnvelope};
use crate::runtime::compute_callbacks;
use crate::runtime::compute_callbacks::{ComputeCallbackInvocationResult, ComputeCallbackToken};
use crate::runtime::worker_host::{
    DefinitionEnvelopeCallbackReattachment, RuntimeEnvelopeCallbackReattachment,
    WorkerDefinitionEnvelopePublicationReport, WorkerRuntimeEnvelopeImportReport,
};

use super::types::SignalWorkerRuntime;

struct WorkerCallbackReattachmentBoundaryEntry {
    callback_id: String,
    token: ComputeCallbackToken,
    invocation: ComputeCallbackInvocationResult,
}

#[wasm_bindgen]
impl SignalWorkerRuntime {
    #[wasm_bindgen(js_name = admitWorkerRuntimeEnvelopeImportWithCallbackReattachment)]
    pub fn admit_worker_runtime_envelope_import_with_callback_reattachment(
        &self,
        envelope: JsValue,
        callback_id: String,
        callback: Function,
    ) -> Result<JsValue, JsValue> {
        let envelope: RuntimeEnvelope = from_js(envelope).map_err(JsValue::from)?;
        let reattachments = vec![register_boundary_callback_reattachment(
            callback_id,
            callback,
        )?];
        let report = self
            .admit_worker_runtime_envelope_import_with_callback_reattachments_for_test(
                envelope,
                runtime_envelope_reattachments(reattachments),
            )?;
        to_js(&report).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = admitWorkerRuntimeEnvelopeImportWithCallbackReattachments)]
    pub fn admit_worker_runtime_envelope_import_with_callback_reattachments(
        &self,
        envelope: JsValue,
        reattachments: JsValue,
    ) -> Result<JsValue, JsValue> {
        let envelope: RuntimeEnvelope = from_js(envelope).map_err(JsValue::from)?;
        let reattachments = boundary_callback_reattachment_batch(reattachments)?;
        let report = self
            .admit_worker_runtime_envelope_import_with_callback_reattachments_for_test(
                envelope,
                runtime_envelope_reattachments(reattachments),
            )?;
        to_js(&report).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = publishDefinitionEnvelopeWithCallbackReattachment)]
    pub fn publish_definition_envelope_with_callback_reattachment(
        &self,
        envelope: JsValue,
        callback_id: String,
        callback: Function,
    ) -> Result<JsValue, JsValue> {
        let envelope: RuntimeDefinitionEnvelope = from_js(envelope).map_err(JsValue::from)?;
        let reattachments = vec![register_boundary_callback_reattachment(
            callback_id,
            callback,
        )?];
        let report = self.publish_definition_envelope_with_callback_reattachments_for_test(
            envelope,
            definition_envelope_reattachments(reattachments),
        )?;
        to_js(&report).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = publishDefinitionEnvelopeWithCallbackReattachments)]
    pub fn publish_definition_envelope_with_callback_reattachments(
        &self,
        envelope: JsValue,
        reattachments: JsValue,
    ) -> Result<JsValue, JsValue> {
        let envelope: RuntimeDefinitionEnvelope = from_js(envelope).map_err(JsValue::from)?;
        let reattachments = boundary_callback_reattachment_batch(reattachments)?;
        let report = self.publish_definition_envelope_with_callback_reattachments_for_test(
            envelope,
            definition_envelope_reattachments(reattachments),
        )?;
        to_js(&report).map_err(JsValue::from)
    }
}

impl SignalWorkerRuntime {
    pub(crate) fn admit_worker_runtime_envelope_import_with_callback_reattachments_for_test(
        &self,
        envelope: RuntimeEnvelope,
        reattachments: Vec<RuntimeEnvelopeCallbackReattachment>,
    ) -> Result<WorkerRuntimeEnvelopeImportReport, JsValue> {
        self.shell
            .borrow_mut()
            .admit_worker_runtime_envelope_import_with_callback_reattachments(
                envelope,
                reattachments,
            )
            .map_err(JsValue::from)
    }

    pub(crate) fn publish_definition_envelope_with_callback_reattachments_for_test(
        &self,
        envelope: RuntimeDefinitionEnvelope,
        reattachments: Vec<DefinitionEnvelopeCallbackReattachment>,
    ) -> Result<WorkerDefinitionEnvelopePublicationReport, JsValue> {
        self.shell
            .borrow_mut()
            .publish_definition_envelope_with_callback_reattachments(envelope, reattachments)
            .map_err(JsValue::from)
    }
}

fn boundary_callback_reattachment_batch(
    reattachments: JsValue,
) -> Result<Vec<WorkerCallbackReattachmentBoundaryEntry>, JsValue> {
    if !Array::is_array(&reattachments) {
        return Err(JsValue::from(WorthSignalJsError::invalid_input(
            "worker callback reattachments must be an array",
        )));
    }
    let reattachment_array = Array::from(&reattachments);
    let mut registered = Vec::with_capacity(reattachment_array.length() as usize);
    for entry in reattachment_array.iter() {
        match boundary_callback_reattachment_entry(entry) {
            Ok(reattachment) => registered.push(reattachment),
            Err(error) => {
                dispose_registered_boundary_reattachments(registered);
                return Err(error);
            }
        }
    }
    Ok(registered)
}

fn boundary_callback_reattachment_entry(
    entry: JsValue,
) -> Result<WorkerCallbackReattachmentBoundaryEntry, JsValue> {
    let callback_id = Reflect::get(&entry, &JsValue::from_str("callbackId"))
        .map_err(|_| invalid_reattachment_entry("callbackId property is unreadable"))?
        .as_string()
        .ok_or_else(|| invalid_reattachment_entry("callbackId must be a string"))?;
    let callback = Reflect::get(&entry, &JsValue::from_str("callback"))
        .map_err(|_| invalid_reattachment_entry("callback property is unreadable"))?
        .dyn_into::<Function>()
        .map_err(|_| invalid_reattachment_entry("callback must be a function"))?;
    register_boundary_callback_reattachment(callback_id, callback)
}

fn register_boundary_callback_reattachment(
    callback_id: String,
    callback: Function,
) -> Result<WorkerCallbackReattachmentBoundaryEntry, JsValue> {
    let token = compute_callbacks::register_wasm_compute(callback);
    let invocation = compute_callbacks::invoke_compute(token).map_err(|failure| {
        let _ = compute_callbacks::dispose_compute(token);
        JsValue::from(WorthSignalJsError::from_compute_callback_failure(failure))
    })?;
    Ok(WorkerCallbackReattachmentBoundaryEntry {
        callback_id,
        token,
        invocation,
    })
}

fn runtime_envelope_reattachments(
    reattachments: Vec<WorkerCallbackReattachmentBoundaryEntry>,
) -> Vec<RuntimeEnvelopeCallbackReattachment> {
    reattachments
        .into_iter()
        .map(|reattachment| RuntimeEnvelopeCallbackReattachment {
            callback_id: reattachment.callback_id,
            token: reattachment.token,
            invocation: reattachment.invocation,
        })
        .collect()
}

fn definition_envelope_reattachments(
    reattachments: Vec<WorkerCallbackReattachmentBoundaryEntry>,
) -> Vec<DefinitionEnvelopeCallbackReattachment> {
    reattachments
        .into_iter()
        .map(|reattachment| DefinitionEnvelopeCallbackReattachment {
            callback_id: reattachment.callback_id,
            token: reattachment.token,
            invocation: reattachment.invocation,
        })
        .collect()
}

fn dispose_registered_boundary_reattachments(
    reattachments: Vec<WorkerCallbackReattachmentBoundaryEntry>,
) {
    for reattachment in reattachments {
        let _ = compute_callbacks::dispose_compute(reattachment.token);
    }
}

fn invalid_reattachment_entry(message: &str) -> JsValue {
    JsValue::from(WorthSignalJsError::invalid_input(format!(
        "worker callback reattachment {message}"
    )))
}
