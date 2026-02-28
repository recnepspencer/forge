//! Universal test logging for the Forge kernel.
//!
//! DOMAIN: Test infrastructure — structured decision log output for all tests.
//! INVARIANTS: Consistent output format across all crates. Never panics.
//! DEPENDENCIES: `forge-core` (DecisionLog, OperationResult)
//!
//! This module re-exports the logging helpers from `forge-core`.
//! All crates can use either path:
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
//! Set the `RUST_LOG` environment variable (auto-defaulted to `info` in `.cargo/config.toml`):
//! - `info`  — display_interesting() summary (default)
//! - `debug` — every decision, line by line
//! - `off`   — silent (CI-friendly)

pub use forge_core::{log_decision_log, log_error, log_result};
