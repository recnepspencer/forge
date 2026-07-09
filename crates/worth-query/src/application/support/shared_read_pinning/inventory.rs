#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct WorthQuerySharedReadPinningInventoryRow {
    path: &'static str,
    role: &'static str,
}

impl WorthQuerySharedReadPinningInventoryRow {
    pub(super) const fn new(path: &'static str, role: &'static str) -> Self {
        Self { path, role }
    }

    pub(super) fn path(self) -> &'static str {
        self.path
    }

    pub(super) fn role(self) -> &'static str {
        self.role
    }
}

pub(super) const SHARED_READ_PINNING_INVENTORY: &[WorthQuerySharedReadPinningInventoryRow] = &[
    WorthQuerySharedReadPinningInventoryRow::new(
        "crates/worth-query/src/runtime/workspace_shared_read.rs",
        "shared read workspace authority mint facade",
    ),
    WorthQuerySharedReadPinningInventoryRow::new(
        "crates/worth-query/src/runtime/workspace_queries.rs",
        "workspace read authority sibling path and live artifact consumption feeder",
    ),
    WorthQuerySharedReadPinningInventoryRow::new(
        "crates/worth-query/src/runtime/shared_read.rs",
        "shared read context mint and artifact resolution",
    ),
    WorthQuerySharedReadPinningInventoryRow::new(
        "crates/worth-query/src/runtime/shared_read_pins/registry.rs",
        "snapshot generation single-writer publication and explicit retirement",
    ),
    WorthQuerySharedReadPinningInventoryRow::new(
        "crates/worth-query/src/runtime/shared_read_pins/generation.rs",
        "pin-hot-path generation identity lease and atomic pin count",
    ),
    WorthQuerySharedReadPinningInventoryRow::new(
        "crates/worth-query/src/runtime/shared_read_pins/current_generation.rs",
        "pin-hot-path current generation publication cell",
    ),
    WorthQuerySharedReadPinningInventoryRow::new(
        "crates/worth-query/src/runtime/shared_read_pins/hot_path_measurement.rs",
        "pin-hot-path runtime lock measurement counters",
    ),
    WorthQuerySharedReadPinningInventoryRow::new(
        "crates/worth-query/src/runtime/shared_read_pins/retirement.rs",
        "generation retirement drain policy",
    ),
    WorthQuerySharedReadPinningInventoryRow::new(
        "crates/worth-query/src/runtime/published_artifacts/registry.rs",
        "published-artifact hot-path generation resolution registry",
    ),
    WorthQuerySharedReadPinningInventoryRow::new(
        "crates/worth-query/src/runtime/published_artifacts/diagnostics.rs",
        "published artifact diagnostics for pinning boundary closure posture",
    ),
    WorthQuerySharedReadPinningInventoryRow::new(
        "crates/worth-query/src/runtime/shared_read_pins/diagnostics.rs",
        "shared read pinning diagnostics for boundary closure posture",
    ),
    WorthQuerySharedReadPinningInventoryRow::new(
        "crates/worth-query/src/runtime/published_artifacts/entry.rs",
        "published artifact entry and binding lease",
    ),
];
