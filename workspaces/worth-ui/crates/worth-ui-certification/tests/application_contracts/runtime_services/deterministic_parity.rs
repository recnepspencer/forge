//! `RS-07` determinism and headless/native convergence.
//!
//! This file makes direct assertions about the production runs and deliberately
//! claims no independent semantic model. The milestone requires independent
//! model oracles for `RS-10` (see `scale_amplification.rs`), not for the
//! `RS-07` determinism and host-parity evidence. The earlier constant event
//! schedule here was not such an oracle in any case: it never consumed the
//! production observation stream, so it restated the expected answer rather
//! than deriving one.
//!
//! The evidence `RS-07` requires is unchanged. Repetition equality proves
//! determinism, cross-host equality proves headless/native convergence, and
//! every named semantic outcome is asserted on its own rather than hidden
//! inside one struct comparison. The fault, ordering, settlement, and
//! resource-zero outcomes `RS-07` names are proven in `protocol_faults.rs`.

use crate::intent::{
    run_native_runtime_service_scenario,
    runtime_services_kit::run_headless_runtime_service_scenario,
};

#[test]
fn repeated_runs_are_deterministic_and_headless_matches_native_semantics() {
    let headless = run_headless_runtime_service_scenario();
    let headless_repeat = run_headless_runtime_service_scenario();
    let native = run_native_runtime_service_scenario();
    let native_repeat = run_native_runtime_service_scenario();

    assert_eq!(
        headless, headless_repeat,
        "headless evidence must be deterministic"
    );
    assert_eq!(
        native, native_repeat,
        "native evidence must be deterministic"
    );
    assert_eq!(
        headless.semantic, native.semantic,
        "headless and native hosts must converge on the same service semantics"
    );

    // Each named semantic outcome, asserted directly.
    assert!(headless.semantic.portal_was_visible);
    assert!(headless.semantic.focus_was_placed);
    assert!(headless.semantic.dismissal_closed_only_top);
    assert!(headless.semantic.focus_restored_to_previous);
    assert!(headless.semantic.duplicate_was_idempotent);
    assert!(headless.semantic.proposals_are_zero);
    assert!(headless.semantic.terminal_resources_are_zero);

    // Host-specific evidence that does not converge by construction.
    assert!(headless.hot_rebind_preserved_portal);
    assert!(headless.focus_retargeted_to_successor);
    assert!(headless.inspection_was_bounded);
    assert!(native.indeterminate_effect_retained);
    assert!(native.reconciled_from_exact_host_truth);
    assert!(native.predecessor_was_reconstructed);
}
