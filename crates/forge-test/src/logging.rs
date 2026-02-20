//! Universal test logging for the Forge kernel.
//!
//! DOMAIN: Test infrastructure — structured decision log output for all tests.
//! INVARIANTS: Consistent output format across all crates. Never panics.
//! DEPENDENCIES: `forge-core` (DecisionLog, OperationResult)
//!
//! This module re-exports the logging helpers from `forge-core::result`
//! for convenience. All crates can use either path:
//!
//! ```ignore
//! // From forge-core (available everywhere):
//! forge_core::log_result("Union", &envelope);
//!
//! // From forge-test (convenient re-export):
//! forge_test::logging::log_result("Union", &envelope);
//! ```
//!
//! # Verbosity Control
//!
//! Set the `FORGE_LOG` environment variable:
//! - `off`     — no output (CI-friendly)
//! - `compact` — summary + high-level decisions, no Euler ops (default)
//! - `full`    — everything including Euler operator decisions

pub use forge_core::{LogLevel, log_level, log_result, log_decision_log};
