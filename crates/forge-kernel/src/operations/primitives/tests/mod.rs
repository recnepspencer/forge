//! Mesh builder test modules.
//!
//! Tests organized by domain concern rather than monolithic files.
//! All tests use `ModelingContext` (real production sink) — no NullSink, no TestSink.

mod structural_invariants_tests;
mod shape_counts_tests;
mod scale_and_position_tests;
mod edge_cases_tests;
mod determinism_tests;
mod geometric_fidelity_tests;

use crate::configuration::facade::{resolve_config, KernelConfig, ResolvedConfig};
pub(super) use crate::context::scope::OperationScope;
pub(super) use crate::context::ModelingContext;

/// Build a default `ResolvedConfig` for tests.
pub(super) fn test_config() -> ResolvedConfig {
    resolve_config(&KernelConfig::default(), None, None, None).unwrap()
}

/// Initialize tracing subscriber for test output (idempotent).
///
/// Respects `RUST_LOG` env var. Default: `forge_trace=info`.
/// Run tests with `--nocapture` to see decision summaries:
///   `cargo test --release -p forge-kernel -- --nocapture`
pub(super) fn init_test_tracing() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let filter = tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("forge_trace=info"));
        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_test_writer()
            .try_init()
            .ok();
    });
}
