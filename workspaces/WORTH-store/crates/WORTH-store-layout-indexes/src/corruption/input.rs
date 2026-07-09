use crate::{LayoutCorruptionClassification, PhysicalArtifactFamily, S8LayoutCoverageWitness};
use worth_store_physical_integrity::QuarantineRecord;
use worth_store_recovery_physics::{
    RecoveryLayoutReadmissionWitness, ReopenedRecoveryArtifactAdmission,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S8LayoutCorruptionInput {
    Materialization(S8LayoutCoverageWitness),
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
