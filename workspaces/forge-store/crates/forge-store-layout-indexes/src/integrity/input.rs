use crate::{LayoutCorruptionClassification, LayoutCoverageWitness, PhysicalArtifactFamily};
use forge_store_physical_integrity::QuarantineRecord;
use forge_store_recovery_physics::{
    RecoveryLayoutReadmissionWitness, ReopenedRecoveryArtifactAdmission,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutCorruptionInput {
    Materialization(LayoutCoverageWitness),
    RebuildClassification(LayoutCorruptionClassification),
    AuthoritativeQuarantine {
        family: PhysicalArtifactFamily,
        record: QuarantineRecord,
    },
    OfflineEvidence {
        family: PhysicalArtifactFamily,
        admission: ReopenedRecoveryArtifactAdmission,
    },
    TerminalImport {
        witness: RecoveryLayoutReadmissionWitness,
    },
    MigrationRequired {
        family: PhysicalArtifactFamily,
    },
}
