use worth_store_physical_backend::{ArtifactTreeFailure, MediaCounterSnapshot};
use worth_store_physical_format::store_namespace::StableStoreIdentity;

use super::super::{
    PublishedRecordBatch, RecordPublicationStage, RecordPublicationWorkTrace, RecordStreamFailure,
};

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
pub enum RecordPublicationRecoveryBasis {
    Preparation {
        root_generation: u64,
    },
    RootCandidate {
        prior_root_generation: u64,
        candidate_root_generation: u64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordPublicationRecoveryLocator {
    store: StableStoreIdentity,
    basis: RecordPublicationRecoveryBasis,
    publication: u64,
}

impl RecordPublicationRecoveryLocator {
    pub(in crate::physical_runtime::record_serving) const fn new(
        store: StableStoreIdentity,
        basis: RecordPublicationRecoveryBasis,
        publication: u64,
    ) -> Self {
        Self {
            store,
            basis,
            publication,
        }
    }
    pub const fn store(self) -> StableStoreIdentity {
        self.store
    }
    pub const fn basis(self) -> RecordPublicationRecoveryBasis {
        self.basis
    }
    pub const fn preparation_root_generation(self) -> Option<u64> {
        match self.basis {
            RecordPublicationRecoveryBasis::Preparation { root_generation } => {
                Some(root_generation)
            }
            RecordPublicationRecoveryBasis::RootCandidate { .. } => None,
        }
    }
    pub const fn prior_root_generation(self) -> Option<u64> {
        match self.basis {
            RecordPublicationRecoveryBasis::RootCandidate {
                prior_root_generation,
                ..
            } => Some(prior_root_generation),
            RecordPublicationRecoveryBasis::Preparation { .. } => None,
        }
    }
    pub const fn candidate_root_generation(self) -> Option<u64> {
        match self.basis {
            RecordPublicationRecoveryBasis::RootCandidate {
                candidate_root_generation,
                ..
            } => Some(candidate_root_generation),
            RecordPublicationRecoveryBasis::Preparation { .. } => None,
        }
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
    PhysicalWork {
        stage: RecordPublicationStage,
        failure: Box<super::super::PhysicalRecordMutationFailureEvidence>,
    },
    FrameWriteback {
        stage: RecordPublicationStage,
        failure: Box<super::super::PhysicalRecordWritebackFailureEvidence>,
    },
    Stream(RecordStreamFailure),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::physical_runtime::record_serving) struct RecordPublicationFailureEvidence {
    recovery: RecordPublicationRecoveryLocator,
    attempted_records: u64,
    counters_before: MediaCounterSnapshot,
    counters_after: MediaCounterSnapshot,
    residue: super::super::RecordPublicationResidueObservation,
    work: RecordPublicationWorkTrace,
}

impl RecordPublicationFailureEvidence {
    pub(in crate::physical_runtime::record_serving) fn new(
        recovery: RecordPublicationRecoveryLocator,
        attempted_records: u64,
        counters_before: MediaCounterSnapshot,
        counters_after: MediaCounterSnapshot,
        residue: super::super::RecordPublicationResidueObservation,
        work: RecordPublicationWorkTrace,
    ) -> Self {
        Self {
            recovery,
            attempted_records,
            counters_before,
            counters_after,
            residue,
            work,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnpublishedRecordBatchFailure {
    cause: UnpublishedRecordBatchCause,
    effect_fate: UnpublishedRecordEffectFate,
    world_fate: UnpublishedRecordWorldFate,
    evidence: Box<RecordPublicationFailureEvidence>,
}

impl UnpublishedRecordBatchFailure {
    pub(in crate::physical_runtime::record_serving) fn new(
        cause: UnpublishedRecordBatchCause,
        posture: UnpublishedRecordFailurePosture,
        evidence: RecordPublicationFailureEvidence,
    ) -> Self {
        let mut evidence = evidence;
        if !posture.world_fate.requires_inspection() {
            evidence.residue = super::super::RecordPublicationResidueObservation::default();
        }
        Self {
            cause,
            effect_fate: posture.effect_fate,
            world_fate: posture.world_fate,
            evidence: Box::new(evidence),
        }
    }
    pub const fn cause(&self) -> &UnpublishedRecordBatchCause {
        &self.cause
    }
    pub fn recovery_locator(&self) -> RecordPublicationRecoveryLocator {
        self.evidence.recovery
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
    pub fn attempted_records(&self) -> u64 {
        self.evidence.attempted_records
    }
    pub fn counters_before(&self) -> MediaCounterSnapshot {
        self.evidence.counters_before
    }
    pub fn counters_after(&self) -> MediaCounterSnapshot {
        self.evidence.counters_after
    }
    pub fn residue(&self) -> super::super::RecordPublicationResidueObservation {
        self.evidence.residue
    }
    pub fn physical_work(&self) -> &RecordPublicationWorkTrace {
        &self.evidence.work
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndeterminateRecordPublication {
    stage: RecordPublicationStage,
    cause: IndeterminateRecordPublicationCause,
    evidence: Box<RecordPublicationFailureEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndeterminateRecordPublicationCause {
    Backend(ArtifactTreeFailure),
    PhysicalWork(Box<super::super::PhysicalRecordMutationFailureEvidence>),
}

impl IndeterminateRecordPublication {
    pub(in crate::physical_runtime::record_serving) fn new(
        stage: RecordPublicationStage,
        cause: IndeterminateRecordPublicationCause,
        evidence: RecordPublicationFailureEvidence,
    ) -> Self {
        Self {
            stage,
            cause,
            evidence: Box::new(evidence),
        }
    }
    pub const fn stage(&self) -> RecordPublicationStage {
        self.stage
    }
    pub const fn cause(&self) -> &IndeterminateRecordPublicationCause {
        &self.cause
    }
    pub const fn failure(&self) -> Option<ArtifactTreeFailure> {
        match &self.cause {
            IndeterminateRecordPublicationCause::Backend(failure) => Some(*failure),
            IndeterminateRecordPublicationCause::PhysicalWork(_) => None,
        }
    }
    pub fn recovery_locator(&self) -> RecordPublicationRecoveryLocator {
        self.evidence.recovery
    }
    pub fn attempted_records(&self) -> u64 {
        self.evidence.attempted_records
    }
    pub fn counters_before(&self) -> MediaCounterSnapshot {
        self.evidence.counters_before
    }
    pub fn counters_after(&self) -> MediaCounterSnapshot {
        self.evidence.counters_after
    }
    pub fn residue(&self) -> super::super::RecordPublicationResidueObservation {
        self.evidence.residue
    }
    pub fn physical_work(&self) -> &RecordPublicationWorkTrace {
        &self.evidence.work
    }
}
