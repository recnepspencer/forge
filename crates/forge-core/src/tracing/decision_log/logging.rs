//! Logging utilities — structured decision log output via the `tracing` crate.
//!
//! DOMAIN: Output adapter for the kernel's causal reasoning system.
//! All output flows through the standard `tracing` ecosystem so applications,
//! test harnesses, and AI agents receive structured events via `RUST_LOG`.
//!
//! - `info`  → `display_interesting()` summary (default in .cargo/config.toml)
//! - `debug` → full decision log, every decision line by line
//! - `error` → always emitted for `KernelError` failures

use super::decision_log::DecisionLog;
use crate::envelope::OperationResult;

/// Log the decision log from an `OperationResult` at the current verbosity.
///
/// At `info` level, emits the `display_interesting()` compact summary.
/// At `debug` level, emits the full decision log with every decision.
pub fn log_result<T>(label: &str, result: &OperationResult<T>) {
    log_decision_log(label, result.get_decision_log());

    for summary in result.get_extra_summaries() {
        tracing::info!(target: "forge_trace", "[{}] {}", label, summary);
    }
}

/// Log a raw `DecisionLog` at the current verbosity.
///
/// Use this when you have a `DecisionLog` without an `OperationResult` envelope.
pub fn log_decision_log(label: &str, log: &DecisionLog) {
    if log.is_empty() {
        return;
    }

    tracing::info!(target: "forge_trace", "[{}] {}", label, log.display_interesting());
    tracing::debug!(target: "forge_trace", "[{}] {}", label, log);
}

/// Log a `KernelError` via `tracing::error!`.
///
/// Errors are always emitted regardless of verbosity level.
/// Includes full context and remediation hints when available.
pub fn log_error(label: &str, error: &crate::KernelError) {
    tracing::error!(target: "forge_trace", "[{label}] ❌ ERROR: {error}");
    if let Some(ctx) = error.get_context() {
        if !ctx.detail.is_empty() {
            tracing::error!(target: "forge_trace", "[{label}]   detail: {}", ctx.detail);
        }
        for fix in &ctx.suggested_fixes {
            tracing::error!(target: "forge_trace", "[{label}]   fix: {fix}");
        }
    }
}
