use worth_store_physical_backend::{
    CompletedArtifactTreePublicationEffect, CompletedRecoveryStagingWrite,
};

use super::{
    RecoveryPublicationCandidateMaterializationOccurrence, RecoveryPublicationCandidateOccurrence,
    RecoveryPublicationCandidateSynchronizationOccurrence,
};

impl RecoveryPublicationCandidateOccurrence {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        session: [u8; 16],
        plan: [u8; 32],
        staging_generation: u64,
        publication: u64,
        artifact: worth_store_physical_format::RecordArtifactFile,
        ordinal: u64,
        work: crate::physical_runtime::PhysicalWorkIdentity,
        scheduler: crate::physical_runtime::PhysicalWorkSchedulerPosture,
        signal: crate::physical_runtime::PhysicalSignalSettlementOutcome,
    ) -> Self {
        Self {
            session,
            plan,
            staging_generation,
            publication,
            artifact,
            ordinal,
            work,
            scheduler,
            signal,
        }
    }

    pub const fn artifact(&self) -> worth_store_physical_format::RecordArtifactFile {
        self.artifact
    }
    pub const fn ordinal(&self) -> u64 {
        self.ordinal
    }
}

impl RecoveryPublicationCandidateMaterializationOccurrence {
    pub(crate) const fn new(
        publication: RecoveryPublicationCandidateOccurrence,
        physical: CompletedRecoveryStagingWrite,
    ) -> Self {
        Self {
            publication,
            physical,
        }
    }
    pub const fn publication(&self) -> RecoveryPublicationCandidateOccurrence {
        self.publication
    }
    pub const fn physical(&self) -> &CompletedRecoveryStagingWrite {
        &self.physical
    }
}

impl RecoveryPublicationCandidateSynchronizationOccurrence {
    pub(crate) const fn new(
        publication: RecoveryPublicationCandidateOccurrence,
        physical: CompletedArtifactTreePublicationEffect,
    ) -> Self {
        Self {
            publication,
            physical,
        }
    }
    pub const fn publication(&self) -> RecoveryPublicationCandidateOccurrence {
        self.publication
    }
    pub const fn physical(&self) -> &CompletedArtifactTreePublicationEffect {
        &self.physical
    }
}
