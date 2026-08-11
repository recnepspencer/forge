use js_sys::{Array, Promise, Reflect};
use wasm_bindgen::{JsCast, JsValue};

use crate::boundary::serde::from_js;
use crate::expression::model::SignalValue;

use super::super::types::{
    CapturedHostCapabilityRead, ComputeCallbackFailure, ComputeCallbackFailureClass,
    ComputeCallbackInvocationResult,
};
use super::metrics::{
    record_collector_failure, record_collector_success, record_invocation_failure,
    record_invocation_success,
};

pub(super) fn finish_js_invocation(
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

pub(super) fn failure_message_from_js(error: JsValue) -> ComputeCallbackFailure {
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

pub(super) fn invocation_result_from_js(
    js_value: JsValue,
) -> Result<ComputeCallbackInvocationResult, ComputeCallbackFailure> {
    if !has_capture_envelope_tag(&js_value) {
        return plain_invocation_result(js_value);
    }
    invocation_result_from_capture_envelope(js_value)
}

fn has_capture_envelope_tag(js_value: &JsValue) -> bool {
    Reflect::get(js_value, &JsValue::from_str("__WorthSignalCallbackCapture"))
        .ok()
        .and_then(|value| value.as_bool())
        .unwrap_or(false)
}

fn plain_invocation_result(
    js_value: JsValue,
) -> Result<ComputeCallbackInvocationResult, ComputeCallbackFailure> {
    from_js::<SignalValue>(js_value)
        .map(|value| {
            let breadth = super::super::types::serialized_breadth(&value);
            ComputeCallbackInvocationResult {
                return_serialization_breadth: breadth,
                value,
                captured_read_ids: Vec::new(),
                captured_host_capability_reads: Vec::new(),
                runtime_read_breadth: 0,
            }
        })
        .map_err(|err| ComputeCallbackFailure {
            class: ComputeCallbackFailureClass::InvalidReturnValue,
            message: err.message,
            code: Some("computeCallbackInvalidReturnValue".to_owned()),
        })
}

fn invocation_result_from_capture_envelope(
    js_value: JsValue,
) -> Result<ComputeCallbackInvocationResult, ComputeCallbackFailure> {
    let value = captured_value_from_js(&js_value)?;
    let captured_read_ids = captured_signal_read_ids(&js_value)?;
    let captured_host_capability_reads = captured_host_capability_reads(&js_value)?;
    let return_serialization_breadth = super::super::types::serialized_breadth(&value);
    let runtime_read_breadth = runtime_read_breadth(&js_value);
    Ok(ComputeCallbackInvocationResult {
        return_serialization_breadth,
        value,
        captured_read_ids,
        captured_host_capability_reads,
        runtime_read_breadth,
    })
}

fn captured_value_from_js(js_value: &JsValue) -> Result<SignalValue, ComputeCallbackFailure> {
    let value =
        Reflect::get(js_value, &JsValue::from_str("value")).map_err(failure_message_from_js)?;
    if value.is_instance_of::<Promise>() {
        return Err(ComputeCallbackFailure {
            class: ComputeCallbackFailureClass::PromiseReturnDenied,
            message: "compute callback returned a Promise; sync callback computed nodes must return canonical signal values".to_owned(),
            code: Some("computeCallbackPromiseReturnDenied".to_owned()),
        });
    }
    from_js::<SignalValue>(value).map_err(|err| ComputeCallbackFailure {
        class: ComputeCallbackFailureClass::InvalidReturnValue,
        message: err.message,
        code: Some("computeCallbackInvalidReturnValue".to_owned()),
    })
}

fn captured_signal_read_ids(js_value: &JsValue) -> Result<Vec<String>, ComputeCallbackFailure> {
    let reads_value =
        Reflect::get(js_value, &JsValue::from_str("reads")).map_err(failure_message_from_js)?;
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
    Ok(captured_read_ids)
}

fn captured_host_capability_reads(
    js_value: &JsValue,
) -> Result<Vec<CapturedHostCapabilityRead>, ComputeCallbackFailure> {
    let host_reads_value = Reflect::get(js_value, &JsValue::from_str("hostCapabilityReads"))
        .unwrap_or(JsValue::UNDEFINED);
    let host_reads_array = Array::from(&host_reads_value);
    let mut captured_host_capability_reads = Vec::with_capacity(host_reads_array.length() as usize);
    for entry in host_reads_array.iter() {
        let family = Reflect::get(&entry, &JsValue::from_str("family"))
            .ok()
            .and_then(|value| value.as_string());
        let registration_id = Reflect::get(&entry, &JsValue::from_str("registrationId"))
            .ok()
            .and_then(|value| value.as_string());
        let compatibility = Reflect::get(&entry, &JsValue::from_str("compatibility"))
            .ok()
            .and_then(|value| value.as_string());
        let (Some(family), Some(registration_id), Some(compatibility)) =
            (family, registration_id, compatibility)
        else {
            return Err(ComputeCallbackFailure {
                class: ComputeCallbackFailureClass::InvalidReturnValue,
                message:
                    "compute callback capture envelope must contain only typed host capability read artifacts"
                        .to_owned(),
                code: Some("computeCallbackInvalidCapture".to_owned()),
            });
        };
        captured_host_capability_reads.push(CapturedHostCapabilityRead {
            family,
            registration_id,
            compatibility,
        });
    }
    Ok(captured_host_capability_reads)
}

fn runtime_read_breadth(js_value: &JsValue) -> u64 {
    Reflect::get(js_value, &JsValue::from_str("runtimeReadBreadth"))
        .ok()
        .and_then(|value| value.as_f64())
        .map(|value| value.max(0.0) as u64)
        .unwrap_or(0)
}
