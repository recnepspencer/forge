mod binding;
mod feature_graph;
mod filesystem;
mod platform;
mod process;
mod rerun;

pub use binding::{PhysicalWorkCourtroomRunBinding, PhysicalWorkRunEnvironmentEvidence};
pub use feature_graph::{PhysicalWorkFeatureGraphEvidence, PhysicalWorkFeatureNodeEvidence};
pub use filesystem::{
    PhysicalWorkFilesystemCapabilityEvidence, PhysicalWorkFilesystemCapabilityObservation,
    PhysicalWorkFilesystemLocationEvidence, PhysicalWorkFilesystemProfileEvidence,
    PhysicalWorkFilesystemProfileParts, PhysicalWorkFilesystemSupportEvidence,
};
pub use platform::PhysicalWorkPlatformEvidence;
pub use process::{
    PhysicalWorkExecutionContext, PhysicalWorkProcessEvidence, PhysicalWorkProcessFateEvidence,
};
pub use rerun::PhysicalWorkRerunEvidence;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkRunProvenanceDenial {
    EmptyProcessRole,
    EmptyProcessSet,
    DuplicateProcessIdentity,
    DuplicateProcessRole,
    EmptyYieldpoint,
    EmptyScheduleBinding,
    EmptyFeatureGraph,
    EmptyFeatureNode,
    EmptyFeatureName,
    EmptyDependencyNode,
    DuplicateFeatureRoot,
    DuplicateFeatureNode,
    DuplicateFeatureName,
    DuplicateDependencyNode,
    MissingFeatureRoot,
    MissingDependencyNode,
    UnqualifiedFilesystemRoot,
    UnqualifiedFilesystemVolume,
    EmptyFilesystemType,
    MissingFilesystemCapability,
    DuplicateFilesystemCapability,
    EmptyPlatformField,
    EmptyRerunProgram,
    EmptyRerunArgument,
}

pub(super) fn require_text(
    value: &str,
    denial: PhysicalWorkRunProvenanceDenial,
) -> Result<(), PhysicalWorkRunProvenanceDenial> {
    if value.trim().is_empty() {
        Err(denial)
    } else {
        Ok(())
    }
}
