#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ForgeQuerySharedReadPinningInventoryRow {
    path: &'static str,
    role: &'static str,
}

impl ForgeQuerySharedReadPinningInventoryRow {
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

pub(super) const SHARED_READ_PINNING_INVENTORY: &[ForgeQuerySharedReadPinningInventoryRow] = &[
    ForgeQuerySharedReadPinningInventoryRow::new(
        "crates/forge-query/src/runtime/workspace_shared_read.rs",
        "shared read workspace authority mint facade",
    ),
    ForgeQuerySharedReadPinningInventoryRow::new(
        "crates/forge-query/src/runtime/workspace_queries.rs",
        "workspace read authority sibling path and live artifact consumption feeder",
    ),
    ForgeQuerySharedReadPinningInventoryRow::new(
        "crates/forge-query/src/runtime/shared_read.rs",
        "shared read context mint and artifact resolution",
    ),
    ForgeQuerySharedReadPinningInventoryRow::new(
        "crates/forge-query/src/runtime/shared_read_pins/registry.rs",
        "snapshot generation single-writer publication and explicit retirement",
    ),
    ForgeQuerySharedReadPinningInventoryRow::new(
        "crates/forge-query/src/runtime/shared_read_pins/generation.rs",
        "pin-hot-path generation identity lease and atomic pin count",
    ),
    ForgeQuerySharedReadPinningInventoryRow::new(
        "crates/forge-query/src/runtime/shared_read_pins/current_generation.rs",
        "pin-hot-path current generation publication cell",
    ),
    ForgeQuerySharedReadPinningInventoryRow::new(
        "crates/forge-query/src/runtime/shared_read_pins/hot_path_measurement.rs",
        "pin-hot-path runtime lock measurement counters",
    ),
    ForgeQuerySharedReadPinningInventoryRow::new(
        "crates/forge-query/src/runtime/shared_read_pins/retirement.rs",
        "generation retirement drain policy",
    ),
    ForgeQuerySharedReadPinningInventoryRow::new(
        "crates/forge-query/src/runtime/published_artifacts/registry.rs",
        "published-artifact hot-path generation resolution registry",
    ),
    ForgeQuerySharedReadPinningInventoryRow::new(
        "crates/forge-query/src/runtime/published_artifacts/diagnostics.rs",
        "published artifact diagnostics for pinning boundary closure posture",
    ),
    ForgeQuerySharedReadPinningInventoryRow::new(
        "crates/forge-query/src/runtime/shared_read_pins/diagnostics.rs",
        "shared read pinning diagnostics for boundary closure posture",
    ),
    ForgeQuerySharedReadPinningInventoryRow::new(
        "crates/forge-query/src/runtime/published_artifacts/entry.rs",
        "published artifact entry and binding lease",
    ),
];
