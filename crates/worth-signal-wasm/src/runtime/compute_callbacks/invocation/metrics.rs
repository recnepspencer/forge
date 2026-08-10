use super::super::registry::with_registry_mut;
use super::super::types::{ComputeCallbackFailure, ComputeCallbackFailureClass};

pub(super) fn record_invocation_success(breadth: u64) {
    with_registry_mut(|stats| {
        stats.compute_callback_invocation_count =
            stats.compute_callback_invocation_count.saturating_add(1);
        stats.compute_callback_return_serialization_breadth = stats
            .compute_callback_return_serialization_breadth
            .saturating_add(breadth);
    });
}

pub(super) fn record_collector_installation() {
    with_registry_mut(|stats| {
        stats.compute_callback_collector_installation_count = stats
            .compute_callback_collector_installation_count
            .saturating_add(1);
        stats.active_compute_collector_count =
            stats.active_compute_collector_count.saturating_add(1);
    });
}

pub(super) fn record_collector_success(captured_read_count: usize) {
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

pub(super) fn record_collector_failure() {
    with_registry_mut(|stats| {
        stats.active_compute_collector_count =
            stats.active_compute_collector_count.saturating_sub(1);
    });
}

pub(super) fn record_invocation_failure(failure: &ComputeCallbackFailure) {
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
