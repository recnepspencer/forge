mod cancellation;
#[cfg(test)]
mod interruption_tests;
#[cfg(test)]
mod publication_tests;
mod session;
#[cfg(test)]
mod session_identity_tests;
mod source;
mod source_observation;
#[cfg(test)]
mod tests;

pub use cancellation::PhysicalBackupMaterializationCancellation;
pub use session::{
    PendingPhysicalBackupMaterializationCleanup, PhysicalBackupArtifactDurabilityProgress,
    PhysicalBackupCopyProgress, PhysicalBackupMaterializationAbandonment,
    PhysicalBackupMaterializationAbandonmentDenial, PhysicalBackupMaterializationCounterScope,
    PhysicalBackupMaterializationCounters, PhysicalBackupMaterializationDenial,
    PhysicalBackupMaterializationProgress, PhysicalBackupMaterializationSession,
    PhysicalBackupPublicationProgress, PhysicalBackupPublicationSession,
    PhysicalMaterializedBackupBundle,
};
pub use source::PhysicalBackupSource;
pub use source_observation::{
    observe_physical_backup_artifact, PhysicalBackupArtifactObservation,
    PhysicalBackupArtifactObservationDenial,
};
