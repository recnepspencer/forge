//! Milestone 9.6 certification gate â€” support `Closed` requires named suite modules
//! registered with embedded sources at compile time.
//!
//! Lib-suite pass/fail cannot be proven at compile time without brittle CI hooks, so
//! greenness is enforced by running `cargo test -p worth-query --lib` in CI and by the
//! certification modules listed below remaining present in the embedded-source inventory.

#[path = "identity_boundary_inventory_sources.rs"]
mod identity_boundary_inventory_sources;

use identity_boundary_inventory_sources::source_for_certification_gate_path;

/// Named Milestone 9.6 lib certification modules that must remain registered.
pub const MILESTONE_9_6_CERTIFICATION_GATE_PATHS: &[&str] = &[
    "runtime/tests/session_label.rs",
    "runtime/tests/evidence_identity/mod.rs",
    "runtime/tests/stop_class/mod.rs",
    "runtime/tests/identity_boundary/mod.rs",
];

pub fn milestone_nine_six_certification_gate_certified() -> bool {
    MILESTONE_9_6_CERTIFICATION_GATE_PATHS
        .iter()
        .all(|path| source_for_certification_gate_path(path).is_some())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn milestone_nine_six_certification_gate_paths_have_embedded_sources() {
        for path in MILESTONE_9_6_CERTIFICATION_GATE_PATHS {
            assert!(
                source_for_certification_gate_path(path).is_some(),
                "missing embedded certification gate source for {path}"
            );
        }
    }

    #[test]
    fn milestone_nine_six_certification_gate_is_automatic_from_embedded_sources() {
        assert!(
            milestone_nine_six_certification_gate_certified(),
            "support Closed requires all named 9.6 certification suites to remain embedded"
        );
    }
}
