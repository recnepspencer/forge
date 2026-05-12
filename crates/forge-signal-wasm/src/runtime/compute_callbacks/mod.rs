mod invocation;
mod registry;
#[cfg(test)]
mod tests;
mod types;

pub use invocation::{invoke_compute, invoke_compute_with_reads};
pub use registry::{
    compute_callback_stats, dispose_compute, is_compute_registered, register_wasm_compute,
};
#[cfg(test)]
pub use registry::{register_native_compute, register_native_compute_result};
#[allow(unused_imports)]
pub use types::{
    CapturedHostCapabilityRead, ComputeCallbackFailure, ComputeCallbackFailureClass,
    ComputeCallbackInvocationResult, ComputeCallbackToken,
};
