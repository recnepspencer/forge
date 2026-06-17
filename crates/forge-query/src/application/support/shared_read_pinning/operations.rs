#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ForgeQuerySharedReadPinningOperationKind {
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
pub struct ForgeQuerySharedReadPinningOperationRow {
    kind: ForgeQuerySharedReadPinningOperationKind,
    path: &'static str,
    function: &'static str,
}

impl ForgeQuerySharedReadPinningOperationRow {
    pub const fn new(
        kind: ForgeQuerySharedReadPinningOperationKind,
        path: &'static str,
        function: &'static str,
    ) -> Self {
        Self {
            kind,
            path,
            function,
        }
    }

    pub fn kind(self) -> ForgeQuerySharedReadPinningOperationKind {
        self.kind
    }

    pub fn path(self) -> &'static str {
        self.path
    }

    pub fn function(self) -> &'static str {
        self.function
    }
}

pub const SHARED_READ_PINNING_OPERATION_INVENTORY: &[ForgeQuerySharedReadPinningOperationRow] = &[
    ForgeQuerySharedReadPinningOperationRow::new(
        ForgeQuerySharedReadPinningOperationKind::MintSharedReadContext,
        "crates/forge-query/src/runtime/workspace_shared_read.rs",
        "shared_read_context",
    ),
    ForgeQuerySharedReadPinningOperationRow::new(
        ForgeQuerySharedReadPinningOperationKind::InspectSharedReadBasis,
        "crates/forge-query/src/runtime/shared_read.rs",
        "inspect_basis",
    ),
    ForgeQuerySharedReadPinningOperationRow::new(
        ForgeQuerySharedReadPinningOperationKind::ConsumePublishedArtifact,
        "crates/forge-query/src/runtime/shared_read.rs",
        "published_derived_artifact",
    ),
    ForgeQuerySharedReadPinningOperationRow::new(
        ForgeQuerySharedReadPinningOperationKind::ClassifyPinningBoundaryClosure,
        "crates/forge-query/src/application/support/shared_read_pinning/closure.rs",
        "derive",
    ),
    ForgeQuerySharedReadPinningOperationRow::new(
        ForgeQuerySharedReadPinningOperationKind::PinCurrentGeneration,
        "crates/forge-query/src/runtime/shared_read_pins/registry.rs",
        "pin_current_generation",
    ),
    ForgeQuerySharedReadPinningOperationRow::new(
        ForgeQuerySharedReadPinningOperationKind::ReleaseGeneration,
        "crates/forge-query/src/runtime/shared_read_pins/registry.rs",
        "release_generation",
    ),
    ForgeQuerySharedReadPinningOperationRow::new(
        ForgeQuerySharedReadPinningOperationKind::DrainRetiredGeneration,
        "crates/forge-query/src/runtime/shared_read_pins/registry.rs",
        "drain_retired_generation",
    ),
    ForgeQuerySharedReadPinningOperationRow::new(
        ForgeQuerySharedReadPinningOperationKind::CaptureCommittedGeneration,
        "crates/forge-query/src/runtime/shared_read_pins/registry.rs",
        "capture_committed_snapshot",
    ),
    ForgeQuerySharedReadPinningOperationRow::new(
        ForgeQuerySharedReadPinningOperationKind::RetainPublishedArtifactGenerations,
        "crates/forge-query/src/runtime/published_artifacts/registry.rs",
        "retain_generations",
    ),
    ForgeQuerySharedReadPinningOperationRow::new(
        ForgeQuerySharedReadPinningOperationKind::ResolvePublishedArtifactGeneration,
        "crates/forge-query/src/runtime/published_artifacts/registry.rs",
        "resolve",
    ),
    ForgeQuerySharedReadPinningOperationRow::new(
        ForgeQuerySharedReadPinningOperationKind::MeasureCommittedReadHotPath,
        "crates/forge-query/src/runtime/shared_read_pins/hot_path_measurement.rs",
        "committed_read_hot_path_lock_count",
    ),
];

pub fn shared_read_pinning_operation_inventory(
) -> &'static [ForgeQuerySharedReadPinningOperationRow] {
    SHARED_READ_PINNING_OPERATION_INVENTORY
}
