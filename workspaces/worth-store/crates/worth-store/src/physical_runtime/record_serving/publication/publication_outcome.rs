use worth_store_physical_backend::{ArtifactTreeFailure, MediaCounterSnapshot};
use worth_store_physical_format::store_namespace::StableStoreIdentity;

use super::super::{PublishedRecordBatch, RecordPublicationStage, RecordStreamFailure};

pub type RecordPublicationOutcome = Result<PublishedRecordBatch, super::super::RecordAppendError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnpublishedRecordEffectFate {
    DeniedBeforeEffect,
    EffectPossible,
}

impl UnpublishedRecordEffectFate {
    pub(in crate::physical_runtime::record_serving) const fn combine(self, later: Self) -> Self {
        if matches!(self, Self::EffectPossible) || matches!(later, Self::EffectPossible) {
            Self::EffectPossible
        } else {
            Self::DeniedBeforeEffect
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnpublishedRecordWorldFate {
    Reusable,
    InspectionRequired,
}

impl UnpublishedRecordWorldFate {
    pub const fn requires_inspection(self) -> bool {
        matches!(self, Self::InspectionRequired)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime::record_serving) struct UnpublishedRecordFailurePosture {
    effect_fate: UnpublishedRecordEffectFate,
    world_fate: UnpublishedRecordWorldFate,
}

impl UnpublishedRecordFailurePosture {
    pub(in crate::physical_runtime::record_serving) const fn new(
        effect_fate: UnpublishedRecordEffectFate,
        world_fate: UnpublishedRecordWorldFate,
    ) -> Self {
        Self {
            effect_fate,
            world_fate,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordPublicationRecoveryLocator {
    store: StableStoreIdentity,
    prior_root_generation: u64,
    candidate_root_generation: u64,
    publication: u64,
}

impl RecordPublicationRecoveryLocator {
    pub(in crate::physical_runtime::record_serving) const fn new(
        store: StableStoreIdentity,
        prior_root_generation: u64,
        candidate_root_generation: u64,
        publication: u64,
    ) -> Self {
        Self {
            store,
            prior_root_generation,
            candidate_root_generation,
            publication,
        }
    }
    pub const fn store(self) -> StableStoreIdentity {
        self.store
    }
    pub const fn prior_root_generation(self) -> u64 {
        self.prior_root_generation
    }
    pub const fn candidate_root_generation(self) -> u64 {
        self.candidate_root_generation
    }
    pub const fn publication(self) -> u64 {
        self.publication
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnpublishedRecordBatchCause {
    Backend {
        stage: RecordPublicationStage,
        failure: ArtifactTreeFailure,
    },
    CandidateFrameContract {
        stage: RecordPublicationStage,
        violation: super::super::CandidateFrameContractViolation,
    },
    Residency {
        stage: RecordPublicationStage,
        denial: super::super::RecordAppendDenial,
    },
    Semantic {
        stage: RecordPublicationStage,
        denial: super::super::RecordAppendDenial,
    },
    Stream(RecordStreamFailure),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnpublishedRecordBatchFailure {
    cause: UnpublishedRecordBatchCause,
    effect_fate: UnpublishedRecordEffectFate,
    world_fate: UnpublishedRecordWorldFate,
    recovery: RecordPublicationRecoveryLocator,
    attempted_records: u64,
    counters_before: MediaCounterSnapshot,
    counters_after: MediaCounterSnapshot,
    residue: super::super::RecordPublicationResidueObservation,
}

impl UnpublishedRecordBatchFailure {
    pub(in crate::physical_runtime::record_serving) fn new(
        cause: UnpublishedRecordBatchCause,
        posture: UnpublishedRecordFailurePosture,
        recovery: RecordPublicationRecoveryLocator,
        attempted_records: u64,
        counters_before: MediaCounterSnapshot,
        counters_after: MediaCounterSnapshot,
        residue: super::super::RecordPublicationResidueObservation,
    ) -> Self {
        Self {
            cause,
            effect_fate: posture.effect_fate,
            world_fate: posture.world_fate,
            recovery,
            attempted_records,
            counters_before,
            counters_after,
            residue: if posture.world_fate.requires_inspection() {
                residue
            } else {
                super::super::RecordPublicationResidueObservation::default()
            },
        }
    }
    pub const fn cause(&self) -> &UnpublishedRecordBatchCause {
        &self.cause
    }
    pub const fn recovery_locator(&self) -> RecordPublicationRecoveryLocator {
        self.recovery
    }
    pub const fn effect_fate(&self) -> UnpublishedRecordEffectFate {
        self.effect_fate
    }
    pub const fn world_fate(&self) -> UnpublishedRecordWorldFate {
        self.world_fate
    }
    pub const fn requires_inspection(&self) -> bool {
        self.world_fate.requires_inspection()
    }
    pub const fn attempted_records(&self) -> u64 {
        self.attempted_records
    }
    pub const fn counters_before(&self) -> MediaCounterSnapshot {
        self.counters_before
    }
    pub const fn counters_after(&self) -> MediaCounterSnapshot {
        self.counters_after
    }
    pub const fn residue(&self) -> super::super::RecordPublicationResidueObservation {
        self.residue
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndeterminateRecordPublication {
    stage: RecordPublicationStage,
    failure: ArtifactTreeFailure,
    recovery: RecordPublicationRecoveryLocator,
    attempted_records: u64,
    counters_before: MediaCounterSnapshot,
    counters_after: MediaCounterSnapshot,
    residue: super::super::RecordPublicationResidueObservation,
}

impl IndeterminateRecordPublication {
    pub(in crate::physical_runtime::record_serving) const fn new(
        stage: RecordPublicationStage,
        failure: ArtifactTreeFailure,
        recovery: RecordPublicationRecoveryLocator,
        attempted_records: u64,
        counters_before: MediaCounterSnapshot,
        counters_after: MediaCounterSnapshot,
        residue: super::super::RecordPublicationResidueObservation,
    ) -> Self {
        Self {
            stage,
            failure,
            recovery,
            attempted_records,
            counters_before,
            counters_after,
            residue,
        }
    }
    pub const fn stage(self) -> RecordPublicationStage {
        self.stage
    }
    pub const fn failure(self) -> ArtifactTreeFailure {
        self.failure
    }
    pub const fn recovery_locator(self) -> RecordPublicationRecoveryLocator {
        self.recovery
    }
    pub const fn attempted_records(self) -> u64 {
        self.attempted_records
    }
    pub const fn counters_before(self) -> MediaCounterSnapshot {
        self.counters_before
    }
    pub const fn counters_after(self) -> MediaCounterSnapshot {
        self.counters_after
    }
    pub const fn residue(self) -> super::super::RecordPublicationResidueObservation {
        self.residue
    }
}
