use std::path::Path;

use super::read;

pub(super) fn validate(root: &Path) {
    require_terms(
        &read(&root.join("_docs/worth-store/physical-recovery-and-reopen.md")),
        [
            "physical_store_recover",
            "physical_store_offline_observer c8-recovery-observe",
            "Refused",
            "Blocked",
            "Recovered",
            "PublicationIndeterminate",
            "C8_RECOVERY_BLOCKED",
            "C8_RECOVERY_RUNTIME",
            "ProvenNoEffect",
            "Indeterminate",
            "cleanup-deferred",
            "same stable operation identity",
            "observation_bytes",
            "recovery_memory_bytes",
            "peak_recovery_bytes",
            "outside",
            "C.9",
        ],
        "operator guide",
    );
    require_terms(
        &read(&root.join("workspaces/worth-store/crates/worth-store-recovery-runtime/README.md")),
        [
            "physical_store_recover",
            "outside the observed Store root",
            "Refused",
            "Blocked",
            "Recovered",
            "PublicationIndeterminate",
            "cannot mint authority",
            "C.9",
        ],
        "runtime README",
    );
    require_terms(
        &read(&root.join("workspaces/worth-store/crates/worth-store-offline-verifier/README.md")),
        [
            "c8-recovery-observe",
            "WCP7REC\\0",
            "four-axis",
            "version 1",
            "read-only",
            "no recovery decision",
            "outside the observed root",
            "no redo",
        ],
        "observer README",
    );
    require_terms(
        &read(&root.join("workspaces/worth-store/crates/worth-store-recovery-physics/README.md")),
        [
            "pure meaning only",
            "source precedence",
            "WAL-prefix",
            "pageLSN",
            "observer report",
            "replay",
            "fresh physical reopen",
        ],
        "physics README",
    );
}

fn require_terms<const N: usize>(document: &str, terms: [&str; N], owner: &str) {
    for term in terms {
        assert!(document.contains(term), "{owner} omitted `{term}`");
    }
}
