//! Logging utilities — verbosity-controlled decision log output.

use super::decision_log::DecisionLog;
use crate::envelope::OperationResult;

/// Verbosity level for test decision log output.
///
/// Controlled by the `FORGE_LOG` environment variable:
/// - `off`     → `Off` (silent, CI-friendly)
/// - `compact` → `Compact` (default; summary + high-level decisions)
/// - `full`    → `Full` (everything including Euler ops)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    /// No output.
    Off,
    /// Summary + high-level decisions (no Euler ops).
    Compact,
    /// Full decision log including Euler operator decisions.
    Full,
}

/// Read the current log level from the `FORGE_LOG` env var.
///
/// Returns `Compact` if the variable is unset or has an unrecognized value.
pub fn log_level() -> LogLevel {
    match std::env::var("FORGE_LOG").as_deref() {
        Ok("full") | Ok("FULL") => LogLevel::Full,
        Ok("off") | Ok("OFF") => LogLevel::Off,
        _ => LogLevel::Compact,
    }
}

/// Log the decision log from an `OperationResult` at the current verbosity.
///
/// Writes to stderr so output appears with `--nocapture` but doesn't
/// pollute stdout assertions.
pub fn log_result<T>(label: &str, result: &OperationResult<T>) {
    log_decision_log(label, result.get_decision_log());
}

/// Log a raw `DecisionLog` at the current verbosity.
///
/// Use this when you have a `DecisionLog` without an `OperationResult` envelope.
pub fn log_decision_log(label: &str, log: &DecisionLog) {
    match log_level() {
        LogLevel::Off => {}
        LogLevel::Compact => eprint!("[{}] {}", label, log.display_interesting()),
        LogLevel::Full => eprint!("[{}] {}", label, log),
    }
}
