use std::collections::BTreeMap;

use js_sys::Function;
#[cfg(target_arch = "wasm32")]
use js_sys::Reflect;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[cfg(target_arch = "wasm32")]
use crate::boundary::serde::to_js;
use crate::expression::model::SignalValue;

use super::super::types::ComputeCallbackFailure;
#[cfg(target_arch = "wasm32")]
use super::super::types::ComputeCallbackFailureClass;
#[cfg(target_arch = "wasm32")]
use super::result_translation::failure_message_from_js;

#[cfg(target_arch = "wasm32")]
const ACTIVE_RUNTIME_CALLBACK_READS_KEY: &str = "__WorthSignalActiveRuntimeCallbackReads";
#[cfg(target_arch = "wasm32")]
const ACTIVE_RUNTIME_CALLBACK_READER_KEY: &str = "__WorthSignalActiveRuntimeCallbackReader";

#[cfg(target_arch = "wasm32")]
pub(super) struct InstalledRuntimeCallbackReads {
    previous: Option<JsValue>,
    previous_reader: Option<JsValue>,
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) struct InstalledRuntimeCallbackReads;

pub(super) fn install_runtime_callback_reads(
    read_values: &BTreeMap<String, SignalValue>,
    read_through: Option<&Function>,
) -> Result<InstalledRuntimeCallbackReads, ComputeCallbackFailure> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = read_values;
        let _ = read_through;
        Ok(InstalledRuntimeCallbackReads)
    }

    #[cfg(target_arch = "wasm32")]
    {
        let runtime_reads = js_sys::Object::new();
        for (id, value) in read_values {
            let js_value = to_js(value).map_err(|err| ComputeCallbackFailure {
                class: ComputeCallbackFailureClass::InvalidReturnValue,
                message: err.message,
                code: Some("computeCallbackRuntimeReadSerializationFailed".to_owned()),
            })?;
            Reflect::set(&runtime_reads, &JsValue::from_str(id), &js_value)
                .map_err(failure_message_from_js)?;
        }
        let global = js_sys::global();
        let key = JsValue::from_str(ACTIVE_RUNTIME_CALLBACK_READS_KEY);
        let reader_key = JsValue::from_str(ACTIVE_RUNTIME_CALLBACK_READER_KEY);
        let previous = Reflect::get(&global, &key)
            .ok()
            .filter(|value| !value.is_undefined());
        let previous_reader = Reflect::get(&global, &reader_key)
            .ok()
            .filter(|value| !value.is_undefined());
        Reflect::set(&global, &key, &runtime_reads).map_err(failure_message_from_js)?;
        if let Some(read_through) = read_through {
            Reflect::set(&global, &reader_key, read_through).map_err(failure_message_from_js)?;
        } else {
            let _ = Reflect::delete_property(&global, &reader_key);
        }
        Ok(InstalledRuntimeCallbackReads {
            previous,
            previous_reader,
        })
    }
}

pub(super) fn clear_installed_runtime_reads(installed: &mut Option<InstalledRuntimeCallbackReads>) {
    if let Some(installed) = installed.take() {
        clear_runtime_callback_reads(installed);
    }
}

fn clear_runtime_callback_reads(installed: InstalledRuntimeCallbackReads) {
    #[cfg(target_arch = "wasm32")]
    {
        let global = js_sys::global();
        let key = JsValue::from_str(ACTIVE_RUNTIME_CALLBACK_READS_KEY);
        let reader_key = JsValue::from_str(ACTIVE_RUNTIME_CALLBACK_READER_KEY);
        if let Some(previous) = installed.previous {
            let _ = Reflect::set(&global, &key, &previous);
        } else {
            let _ = Reflect::delete_property(&global, &key);
        }
        if let Some(previous_reader) = installed.previous_reader {
            let _ = Reflect::set(&global, &reader_key, &previous_reader);
        } else {
            let _ = Reflect::delete_property(&global, &reader_key);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = installed;
    }
}
