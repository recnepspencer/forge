#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQuerySharedReadPinningOperationKind {
    PinCurrentGeneration,
    ReleaseGeneration,
    DrainRetiredGeneration,
    CaptureCommittedGeneration,
    RetainPublishedArtifactGenerations,
    ResolvePublishedArtifactGeneration,
    MeasureCommittedReadHotPath,
    MintSharedReadContext,
    InspectSharedReadBasis,
    ConsumePublishedArtifact,
    ClassifyPinningBoundaryClosure,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQuerySharedReadPinningOperationRow {
    kind: WorthQuerySharedReadPinningOperationKind,
    path: &'static str,
    function: &'static str,
}

impl WorthQuerySharedReadPinningOperationRow {
    pub const fn new(
        kind: WorthQuerySharedReadPinningOperationKind,
        path: &'static str,
        function: &'static str,
    ) -> Self {
        Self {
            kind,
            path,
            function,
        }
    }

    pub fn kind(self) -> WorthQuerySharedReadPinningOperationKind {
        self.kind
    }

    pub fn path(self) -> &'static str {
        self.path
    }

    pub fn function(self) -> &'static str {
        self.function
    }
}

pub const SHARED_READ_PINNING_OPERATION_INVENTORY: &[WorthQuerySharedReadPinningOperationRow] = &[
    WorthQuerySharedReadPinningOperationRow::new(
        WorthQuerySharedReadPinningOperationKind::MintSharedReadContext,
        "crates/worth-query/src/runtime/workspace_shared_read.rs",
        "shared_read_context",
    ),
    WorthQuerySharedReadPinningOperationRow::new(
        WorthQuerySharedReadPinningOperationKind::InspectSharedReadBasis,
        "crates/worth-query/src/runtime/shared_read.rs",
        "inspect_basis",
    ),
    WorthQuerySharedReadPinningOperationRow::new(
        WorthQuerySharedReadPinningOperationKind::ConsumePublishedArtifact,
        "crates/worth-query/src/runtime/shared_read.rs",
        "published_derived_artifact",
    ),
    WorthQuerySharedReadPinningOperationRow::new(
        WorthQuerySharedReadPinningOperationKind::ClassifyPinningBoundaryClosure,
        "crates/worth-query/src/application/support/shared_read_pinning/closure.rs",
        "derive",
    ),
    WorthQuerySharedReadPinningOperationRow::new(
        WorthQuerySharedReadPinningOperationKind::PinCurrentGeneration,
        "crates/worth-query/src/runtime/shared_read_pins/registry.rs",
        "pin_current_generation",
    ),
    WorthQuerySharedReadPinningOperationRow::new(
        WorthQuerySharedReadPinningOperationKind::ReleaseGeneration,
        "crates/worth-query/src/runtime/shared_read_pins/registry.rs",
        "release_generation",
    ),
    WorthQuerySharedReadPinningOperationRow::new(
        WorthQuerySharedReadPinningOperationKind::DrainRetiredGeneration,
        "crates/worth-query/src/runtime/shared_read_pins/registry.rs",
        "drain_retired_generation",
    ),
    WorthQuerySharedReadPinningOperationRow::new(
        WorthQuerySharedReadPinningOperationKind::CaptureCommittedGeneration,
        "crates/worth-query/src/runtime/shared_read_pins/registry.rs",
        "capture_committed_snapshot",
    ),
    WorthQuerySharedReadPinningOperationRow::new(
        WorthQuerySharedReadPinningOperationKind::RetainPublishedArtifactGenerations,
        "crates/worth-query/src/runtime/published_artifacts/registry.rs",
        "retain_generations",
    ),
    WorthQuerySharedReadPinningOperationRow::new(
        WorthQuerySharedReadPinningOperationKind::ResolvePublishedArtifactGeneration,
        "crates/worth-query/src/runtime/published_artifacts/registry.rs",
        "resolve",
    ),
    WorthQuerySharedReadPinningOperationRow::new(
        WorthQuerySharedReadPinningOperationKind::MeasureCommittedReadHotPath,
        "crates/worth-query/src/runtime/shared_read_pins/hot_path_measurement.rs",
        "committed_read_hot_path_lock_count",
    ),
];

pub fn shared_read_pinning_operation_inventory(
) -> &'static [WorthQuerySharedReadPinningOperationRow] {
    SHARED_READ_PINNING_OPERATION_INVENTORY
}
