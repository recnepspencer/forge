mod artifact_class;
mod cleanup_execution;
mod cleanup_plan;
mod inventory;
mod inventory_observation;
mod retention;
mod root_admission;
#[cfg(test)]
mod tests;

pub use artifact_class::{BuildArtifactClass, BuildArtifactKind};
pub use cleanup_execution::{BuildArtifactCleanupOutcome, BuildArtifactCleanupReceipt};
pub use cleanup_plan::{
    BuildArtifactCleanupPlan, BuildArtifactCleanupTarget, ProtectedDiagnosticArtifact,
};
pub use inventory::{BuildArtifactInventory, BuildArtifactRecord, BuildArtifactReuseBasis};
pub use retention::BuildArtifactRetentionPolicy;
pub use root_admission::{
    mark_disposable_artifact_root, AdmittedArtifactRoot, DISPOSABLE_ARTIFACT_ROOT_MARKER,
};
