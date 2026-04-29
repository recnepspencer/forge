use serde::{Deserialize, Serialize};

use crate::expression::model::SignalValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComputeCallbackToken {
    pub slot: u64,
    pub generation: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ComputeCallbackFailureClass {
    Disposed,
    GenerationMismatch,
    CallbackThrew,
    SelfReadDenied,
    DynamicCycleDenied,
    PromiseReturnDenied,
    InvalidReturnValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComputeCallbackFailure {
    pub class: ComputeCallbackFailureClass,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComputeCallbackInvocationResult {
    pub value: SignalValue,
    pub captured_read_ids: Vec<String>,
    pub runtime_read_breadth: u64,
    pub return_serialization_breadth: u64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComputeCallbackStats {
    pub active_compute_callback_count: u64,
    pub active_compute_collector_count: u64,
    pub compute_callback_registration_count: u64,
    pub compute_callback_disposal_count: u64,
    pub compute_callback_invocation_count: u64,
    pub compute_callback_failure_count: u64,
    pub compute_callback_generation_mismatch_denial_count: u64,
    pub compute_callback_self_read_denial_count: u64,
    pub compute_callback_dynamic_cycle_denial_count: u64,
    pub compute_callback_promise_return_denial_count: u64,
    pub compute_callback_invalid_return_denial_count: u64,
    pub compute_callback_collector_installation_count: u64,
    pub compute_callback_capture_count: u64,
    pub compute_callback_captured_read_count: u64,
    pub compute_callback_return_serialization_breadth: u64,
    pub compute_callback_allocation_count: u64,
    pub compute_callback_reuse_count: u64,
}

pub(crate) fn serialized_breadth(value: &SignalValue) -> u64 {
    match value {
        SignalValue::Null
        | SignalValue::Bool(_)
        | SignalValue::Number(_)
        | SignalValue::String(_) => 1,
        SignalValue::Array(items) => 1 + items.iter().map(serialized_breadth).sum::<u64>(),
        SignalValue::Object(fields) => {
            1 + fields
                .iter()
                .map(|(_, value)| serialized_breadth(value))
                .sum::<u64>()
        }
    }
}
