use std::collections::BTreeMap;

#[cfg(test)]
use std::panic::{catch_unwind, AssertUnwindSafe};

use js_sys::{Array, Function, Promise, Reflect};
use wasm_bindgen::prelude::*;

use crate::boundary::serde::from_js;
#[cfg(target_arch = "wasm32")]
use crate::boundary::serde::to_js;
use crate::expression::model::SignalValue;

use super::registry::{registered_callback, with_registry_mut, RegisteredComputeCallback};
use super::types::{
    ComputeCallbackFailure, ComputeCallbackFailureClass, ComputeCallbackInvocationResult,
    ComputeCallbackToken,
};

#[cfg(target_arch = "wasm32")]
const ACTIVE_RUNTIME_CALLBACK_READS_KEY: &str = "__forgeSignalActiveRuntimeCallbackReads";
#[cfg(target_arch = "wasm32")]
const ACTIVE_RUNTIME_CALLBACK_READER_KEY: &str = "__forgeSignalActiveRuntimeCallbackReader";

pub fn invoke_compute(
    token: ComputeCallbackToken,
) -> Result<ComputeCallbackInvocationResult, ComputeCallbackFailure> {
    invoke_compute_with_reads(token, None, None)
}

pub fn invoke_compute_with_reads(
    token: ComputeCallbackToken,
    read_values: Option<&BTreeMap<String, SignalValue>>,
    read_through: Option<&Function>,
) -> Result<ComputeCallbackInvocationResult, ComputeCallbackFailure> {
    let callback = match registered_callback(token) {
        Ok(callback) => callback,
        Err(failure) => {
            record_invocation_failure(&failure);
            return Err(failure);
        }
    };

    record_collector_installation();
    let mut installed_runtime_reads = if let Some(read_values) = read_values {
        match install_runtime_callback_reads(read_values, read_through) {
            Ok(installed) => Some(installed),
            Err(failure) => {
                record_collector_failure();
                record_invocation_failure(&failure);
                return Err(failure);
            }
        }
    } else {
        None
    };

    match callback {
        RegisteredComputeCallback::Wasm(function) => {
            let js_value = function.call0(&JsValue::NULL).map_err(|error| {
                clear_installed_runtime_reads(&mut installed_runtime_reads);
                let failure = failure_message_from_js(error);
                record_collector_failure();
                record_invocation_failure(&failure);
                failure
            })?;

            clear_installed_runtime_reads(&mut installed_runtime_reads);
            finish_js_invocation(js_value)
        }
        #[cfg(test)]
        RegisteredComputeCallback::Native(callback) => {
            let result = catch_unwind(AssertUnwindSafe(|| callback())).map_err(|_| {
                clear_installed_runtime_reads(&mut installed_runtime_reads);
                let failure = ComputeCallbackFailure {
                    class: ComputeCallbackFailureClass::CallbackThrew,
                    message: "native compute callback panicked".to_owned(),
                    code: Some("computeCallbackPanicked".to_owned()),
                };
                record_collector_failure();
                record_invocation_failure(&failure);
                failure
            })?;
            let invocation = result.map_err(|failure| {
                clear_installed_runtime_reads(&mut installed_runtime_reads);
                record_collector_failure();
                record_invocation_failure(&failure);
                failure
            })?;
            clear_installed_runtime_reads(&mut installed_runtime_reads);
            record_collector_success(invocation.captured_read_ids.len());
            record_invocation_success(invocation.return_serialization_breadth);
            Ok(invocation)
        }
    }
}

fn finish_js_invocation(
    js_value: JsValue,
) -> Result<ComputeCallbackInvocationResult, ComputeCallbackFailure> {
    if js_value.is_instance_of::<Promise>() {
        let failure = ComputeCallbackFailure {
            class: ComputeCallbackFailureClass::PromiseReturnDenied,
            message: "compute callback returned a Promise; sync callback computed nodes must return canonical signal values".to_owned(),
            code: Some("computeCallbackPromiseReturnDenied".to_owned()),
        };
        record_collector_failure();
        record_invocation_failure(&failure);
        return Err(failure);
    }

    match invocation_result_from_js(js_value) {
        Ok(result) => {
            record_collector_success(result.captured_read_ids.len());
            record_invocation_success(result.return_serialization_breadth);
            Ok(result)
        }
        Err(failure) => {
            record_collector_failure();
            record_invocation_failure(&failure);
            Err(failure)
        }
    }
}

fn failure_message_from_js(error: JsValue) -> ComputeCallbackFailure {
    let code = Reflect::get(&error, &JsValue::from_str("code"))
        .ok()
        .and_then(|value| value.as_string());
    let message = Reflect::get(&error, &JsValue::from_str("message"))
        .ok()
        .and_then(|value| value.as_string())
        .or_else(|| error.as_string())
        .unwrap_or_else(|| "compute callback threw".to_owned());

    let class = match code.as_deref() {
        Some("computeCallbackSelfReadDenied") => ComputeCallbackFailureClass::SelfReadDenied,
        Some("computeCallbackDynamicCycleDenied") => {
            ComputeCallbackFailureClass::DynamicCycleDenied
        }
        _ => ComputeCallbackFailureClass::CallbackThrew,
    };

    ComputeCallbackFailure {
        class,
        message,
        code,
    }
}

fn invocation_result_from_js(
    js_value: JsValue,
) -> Result<ComputeCallbackInvocationResult, ComputeCallbackFailure> {
    let envelope_tag = Reflect::get(
        &js_value,
        &JsValue::from_str("__forgeSignalCallbackCapture"),
    )
    .ok()
    .and_then(|value| value.as_bool())
    .unwrap_or(false);

    if !envelope_tag {
        return from_js::<SignalValue>(js_value)
            .map(|value| {
                let breadth = super::types::serialized_breadth(&value);
                ComputeCallbackInvocationResult {
                    return_serialization_breadth: breadth,
                    value,
                    captured_read_ids: Vec::new(),
                    runtime_read_breadth: 0,
                }
            })
            .map_err(|err| ComputeCallbackFailure {
                class: ComputeCallbackFailureClass::InvalidReturnValue,
                message: err.message,
                code: Some("computeCallbackInvalidReturnValue".to_owned()),
            });
    }

    let value =
        Reflect::get(&js_value, &JsValue::from_str("value")).map_err(failure_message_from_js)?;
    if value.is_instance_of::<Promise>() {
        return Err(ComputeCallbackFailure {
            class: ComputeCallbackFailureClass::PromiseReturnDenied,
            message: "compute callback returned a Promise; sync callback computed nodes must return canonical signal values".to_owned(),
            code: Some("computeCallbackPromiseReturnDenied".to_owned()),
        });
    }
    let value = from_js::<SignalValue>(value).map_err(|err| ComputeCallbackFailure {
        class: ComputeCallbackFailureClass::InvalidReturnValue,
        message: err.message,
        code: Some("computeCallbackInvalidReturnValue".to_owned()),
    })?;
    let reads_value =
        Reflect::get(&js_value, &JsValue::from_str("reads")).map_err(failure_message_from_js)?;
    let reads_array = Array::from(&reads_value);
    let mut captured_read_ids = Vec::with_capacity(reads_array.length() as usize);
    for entry in reads_array.iter() {
        let Some(read_id) = entry.as_string() else {
            return Err(ComputeCallbackFailure {
                class: ComputeCallbackFailureClass::InvalidReturnValue,
                message: "compute callback capture envelope must contain only string read ids"
                    .to_owned(),
                code: Some("computeCallbackInvalidCapture".to_owned()),
            });
        };
        captured_read_ids.push(read_id);
    }
    Ok(ComputeCallbackInvocationResult {
        return_serialization_breadth: super::types::serialized_breadth(&value),
        value,
        captured_read_ids,
        runtime_read_breadth: Reflect::get(&js_value, &JsValue::from_str("runtimeReadBreadth"))
            .ok()
            .and_then(|value| value.as_f64())
            .map(|value| value.max(0.0) as u64)
            .unwrap_or(0),
    })
}

fn record_invocation_success(breadth: u64) {
    with_registry_mut(|stats| {
        stats.compute_callback_invocation_count =
            stats.compute_callback_invocation_count.saturating_add(1);
        stats.compute_callback_return_serialization_breadth = stats
            .compute_callback_return_serialization_breadth
            .saturating_add(breadth);
    });
}

fn record_collector_installation() {
    with_registry_mut(|stats| {
        stats.compute_callback_collector_installation_count = stats
            .compute_callback_collector_installation_count
            .saturating_add(1);
        stats.active_compute_collector_count =
            stats.active_compute_collector_count.saturating_add(1);
    });
}

fn record_collector_success(captured_read_count: usize) {
    with_registry_mut(|stats| {
        stats.active_compute_collector_count =
            stats.active_compute_collector_count.saturating_sub(1);
        stats.compute_callback_capture_count =
            stats.compute_callback_capture_count.saturating_add(1);
        stats.compute_callback_captured_read_count = stats
            .compute_callback_captured_read_count
            .saturating_add(captured_read_count as u64);
    });
}

fn record_collector_failure() {
    with_registry_mut(|stats| {
        stats.active_compute_collector_count =
            stats.active_compute_collector_count.saturating_sub(1);
    });
}

fn record_invocation_failure(failure: &ComputeCallbackFailure) {
    with_registry_mut(|stats| {
        stats.compute_callback_invocation_count =
            stats.compute_callback_invocation_count.saturating_add(1);
        stats.compute_callback_failure_count =
            stats.compute_callback_failure_count.saturating_add(1);
        match failure.class {
            ComputeCallbackFailureClass::GenerationMismatch => {
                stats.compute_callback_generation_mismatch_denial_count = stats
                    .compute_callback_generation_mismatch_denial_count
                    .saturating_add(1);
            }
            ComputeCallbackFailureClass::SelfReadDenied => {
                stats.compute_callback_self_read_denial_count = stats
                    .compute_callback_self_read_denial_count
                    .saturating_add(1);
            }
            ComputeCallbackFailureClass::DynamicCycleDenied => {
                stats.compute_callback_dynamic_cycle_denial_count = stats
                    .compute_callback_dynamic_cycle_denial_count
                    .saturating_add(1);
            }
            ComputeCallbackFailureClass::PromiseReturnDenied => {
                stats.compute_callback_promise_return_denial_count = stats
                    .compute_callback_promise_return_denial_count
                    .saturating_add(1);
            }
            ComputeCallbackFailureClass::InvalidReturnValue => {
                stats.compute_callback_invalid_return_denial_count = stats
                    .compute_callback_invalid_return_denial_count
                    .saturating_add(1);
            }
            _ => {}
        }
    });
}

#[cfg(target_arch = "wasm32")]
struct InstalledRuntimeCallbackReads {
    previous: Option<JsValue>,
    previous_reader: Option<JsValue>,
}

#[cfg(not(target_arch = "wasm32"))]
struct InstalledRuntimeCallbackReads;

fn install_runtime_callback_reads(
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

fn clear_installed_runtime_reads(installed: &mut Option<InstalledRuntimeCallbackReads>) {
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
