use worth_store_physical_backend::{MediaCounterSnapshot, QualifiedFilesystemMedia};
use worth_store_physical_format::RecordArtifactFile;

use super::super::{
    publication::publication_outcome::{
        RecordPublicationFailureEvidence, UnpublishedRecordFailurePosture,
    },
    residency::artifact_tree::PhysicalRecordArtifactTree,
    IndeterminateRecordPublication, RecordAppendDenial, RecordAppendError,
    RecordPublicationRecoveryLocator, RecordPublicationStage, RecordStreamFailure,
    UnpublishedRecordBatchCause, UnpublishedRecordBatchFailure, UnpublishedRecordEffectFate,
    UnpublishedRecordWorldFate,
};
use super::{CandidateDataArtifact, PublicationPlan};

struct UnpublishedFailureInput {
    stage: RecordPublicationStage,
    cause: UnpublishedRecordBatchCause,
    current_effect_fate: UnpublishedRecordEffectFate,
    world_fate: UnpublishedRecordWorldFate,
    work: super::RecordPublicationWorkTrace,
}

pub(in crate::physical_runtime::record_serving) fn unpublished_residency(
    media: &QualifiedFilesystemMedia,
    plan: &PublicationPlan,
    before: MediaCounterSnapshot,
    stage: RecordPublicationStage,
    denial: RecordAppendDenial,
) -> RecordAppendError {
    unpublished(
        media,
        plan,
        before,
        UnpublishedFailureInput {
            stage,
            cause: UnpublishedRecordBatchCause::Residency { stage, denial },
            current_effect_fate: UnpublishedRecordEffectFate::DeniedBeforeEffect,
            world_fate: UnpublishedRecordWorldFate::InspectionRequired,
            work: plan.work.clone(),
        },
    )
}

pub(in crate::physical_runtime::record_serving) fn unpublished_semantic(
    media: &QualifiedFilesystemMedia,
    plan: &PublicationPlan,
    before: MediaCounterSnapshot,
    stage: RecordPublicationStage,
    denial: RecordAppendDenial,
) -> RecordAppendError {
    unpublished(
        media,
        plan,
        before,
        UnpublishedFailureInput {
            stage,
            cause: UnpublishedRecordBatchCause::Semantic { stage, denial },
            current_effect_fate: UnpublishedRecordEffectFate::DeniedBeforeEffect,
            world_fate: UnpublishedRecordWorldFate::InspectionRequired,
            work: plan.work.clone(),
        },
    )
}

pub(in crate::physical_runtime::record_serving) fn unpublished_stream(
    media: &QualifiedFilesystemMedia,
    plan: &PublicationPlan,
    before: MediaCounterSnapshot,
    failure: RecordStreamFailure,
) -> RecordAppendError {
    let current_effect_fate = if failure.requires_inspection() {
        UnpublishedRecordEffectFate::EffectPossible
    } else {
        UnpublishedRecordEffectFate::DeniedBeforeEffect
    };
    let effect_fate = aggregate_effect_fate(plan, current_effect_fate);
    let world_fate = if effect_fate == UnpublishedRecordEffectFate::EffectPossible {
        UnpublishedRecordWorldFate::InspectionRequired
    } else {
        UnpublishedRecordWorldFate::Reusable
    };
    let residue = if effect_fate == UnpublishedRecordEffectFate::EffectPossible {
        super::super::RecordPublicationResidueObservation::from_failed_plan(
            plan,
            RecordPublicationStage::CandidateDataWrite,
        )
    } else {
        super::super::RecordPublicationResidueObservation::default()
    };
    RecordAppendError::Unpublished(UnpublishedRecordBatchFailure::new(
        UnpublishedRecordBatchCause::Stream(failure),
        UnpublishedRecordFailurePosture::new(effect_fate, world_fate),
        failure_evidence(media, plan, before, residue, plan.work.clone()),
    ))
}

pub(in crate::physical_runtime::record_serving) fn unpublished_candidate_frame_contract(
    media: &QualifiedFilesystemMedia,
    plan: &PublicationPlan,
    before: MediaCounterSnapshot,
    stage: RecordPublicationStage,
    violation: super::super::CandidateFrameContractViolation,
) -> RecordAppendError {
    unpublished(
        media,
        plan,
        before,
        UnpublishedFailureInput {
            stage,
            cause: UnpublishedRecordBatchCause::CandidateFrameContract { stage, violation },
            current_effect_fate: UnpublishedRecordEffectFate::DeniedBeforeEffect,
            world_fate: UnpublishedRecordWorldFate::InspectionRequired,
            work: plan.work.clone(),
        },
    )
}

pub(in crate::physical_runtime::record_serving) fn unpublished_physical_work(
    media: &QualifiedFilesystemMedia,
    plan: &PublicationPlan,
    before: MediaCounterSnapshot,
    stage: RecordPublicationStage,
    failure: &super::super::CanonicalRecordMutationFailure,
) -> RecordAppendError {
    let current_effect_fate = if failure.effect_fate()
        == crate::physical_runtime::PhysicalWorkEffectFate::ProvenNoEffect
    {
        UnpublishedRecordEffectFate::DeniedBeforeEffect
    } else {
        UnpublishedRecordEffectFate::EffectPossible
    };
    unpublished(
        media,
        plan,
        before,
        UnpublishedFailureInput {
            stage,
            cause: UnpublishedRecordBatchCause::PhysicalWork {
                stage,
                failure: Box::new(failure.evidence()),
            },
            current_effect_fate,
            world_fate: UnpublishedRecordWorldFate::InspectionRequired,
            work: plan.work.clone().including(stage, failure.evidence()),
        },
    )
}

pub(in crate::physical_runtime::record_serving) fn unpublished_frame_writeback(
    media: &QualifiedFilesystemMedia,
    generation: crate::physical_runtime::LifecycleGeneration,
    plan: &PublicationPlan,
    before: MediaCounterSnapshot,
    stage: RecordPublicationStage,
    failure: super::super::PhysicalRecordWritebackFailureEvidence,
) -> RecordAppendError {
    let pressure = failure.pressure(generation);
    if let Some(evidence) = pressure {
        if cleanup_extent_only_candidate_data(media, plan) {
            return RecordAppendError::PhysicalPressure { evidence };
        }
    }
    let current_effect_fate = if failure.effect_fate()
        == crate::physical_runtime::PhysicalWorkEffectFate::ProvenNoEffect
    {
        UnpublishedRecordEffectFate::DeniedBeforeEffect
    } else {
        UnpublishedRecordEffectFate::EffectPossible
    };
    unpublished(
        media,
        plan,
        before,
        UnpublishedFailureInput {
            stage,
            cause: UnpublishedRecordBatchCause::FrameWriteback {
                stage,
                failure: Box::new(failure),
                pressure,
            },
            current_effect_fate,
            world_fate: UnpublishedRecordWorldFate::InspectionRequired,
            work: plan.work.clone().including_writeback(stage, failure),
        },
    )
}

fn cleanup_extent_only_candidate_data(
    media: &QualifiedFilesystemMedia,
    plan: &PublicationPlan,
) -> bool {
    let tree = PhysicalRecordArtifactTree::new(media);
    let mut artifacts = Vec::with_capacity(plan.data.len());
    for data in &plan.data {
        let CandidateDataArtifact::Extent(extent) = data else {
            return false;
        };
        artifacts.push(extent.artifact);
    }
    for artifact in artifacts.iter().copied() {
        match tree.file_exists(artifact) {
            Ok(true) if tree.remove_file_durably(artifact).is_err() => return false,
            Ok(_) => {}
            Err(_) => return false,
        }
    }
    artifacts
        .into_iter()
        .all(|artifact| matches!(tree.file_exists(artifact), Ok(false)))
}

pub(in crate::physical_runtime::record_serving) fn unpublished_prepared_physical_work(
    media: &QualifiedFilesystemMedia,
    plan: &PublicationPlan,
    before: MediaCounterSnapshot,
    stage: RecordPublicationStage,
    failure: &super::super::CanonicalRecordMutationFailure,
) -> RecordAppendError {
    if failure.effect_fate() != crate::physical_runtime::PhysicalWorkEffectFate::ProvenNoEffect {
        return unpublished_physical_work(media, plan, before, stage, failure);
    }
    let effect_fate = aggregate_effect_fate(plan, UnpublishedRecordEffectFate::DeniedBeforeEffect);
    let world_fate = if effect_fate == UnpublishedRecordEffectFate::EffectPossible {
        UnpublishedRecordWorldFate::InspectionRequired
    } else {
        UnpublishedRecordWorldFate::Reusable
    };
    let residue = if effect_fate == UnpublishedRecordEffectFate::EffectPossible {
        super::super::RecordPublicationResidueObservation::from_failed_plan(plan, stage)
    } else {
        super::super::RecordPublicationResidueObservation::default()
    };
    RecordAppendError::Unpublished(UnpublishedRecordBatchFailure::new(
        UnpublishedRecordBatchCause::PhysicalWork {
            stage,
            failure: Box::new(failure.evidence()),
        },
        UnpublishedRecordFailurePosture::new(effect_fate, world_fate),
        failure_evidence(
            media,
            plan,
            before,
            residue,
            plan.work.clone().including(stage, failure.evidence()),
        ),
    ))
}

pub(in crate::physical_runtime::record_serving) fn indeterminate_physical_work(
    media: &QualifiedFilesystemMedia,
    plan: &PublicationPlan,
    before: MediaCounterSnapshot,
    stage: RecordPublicationStage,
    failure: &super::super::CanonicalRecordMutationFailure,
) -> RecordAppendError {
    RecordAppendError::Indeterminate(IndeterminateRecordPublication::new(
        stage,
        super::super::IndeterminateRecordPublicationCause::PhysicalWork(Box::new(
            failure.evidence(),
        )),
        failure_evidence(
            media,
            plan,
            before,
            super::super::RecordPublicationResidueObservation::from_failed_plan(plan, stage),
            plan.work.clone().including(stage, failure.evidence()),
        ),
    ))
}

fn unpublished(
    media: &QualifiedFilesystemMedia,
    plan: &PublicationPlan,
    before: MediaCounterSnapshot,
    input: UnpublishedFailureInput,
) -> RecordAppendError {
    let effect_fate = aggregate_effect_fate(plan, input.current_effect_fate);
    RecordAppendError::Unpublished(UnpublishedRecordBatchFailure::new(
        input.cause,
        UnpublishedRecordFailurePosture::new(effect_fate, input.world_fate),
        failure_evidence(
            media,
            plan,
            before,
            super::super::RecordPublicationResidueObservation::from_failed_plan(plan, input.stage),
            input.work,
        ),
    ))
}

fn failure_evidence(
    media: &QualifiedFilesystemMedia,
    plan: &PublicationPlan,
    before: MediaCounterSnapshot,
    residue: super::super::RecordPublicationResidueObservation,
    work: super::RecordPublicationWorkTrace,
) -> RecordPublicationFailureEvidence {
    RecordPublicationFailureEvidence::new(
        recovery(media, plan),
        plan.records.len() as u64,
        before,
        media.counters(),
        residue,
        work,
    )
}

fn aggregate_effect_fate(
    plan: &PublicationPlan,
    current: UnpublishedRecordEffectFate,
) -> UnpublishedRecordEffectFate {
    let prior = if plan.work.effect_count() == 0 {
        UnpublishedRecordEffectFate::DeniedBeforeEffect
    } else {
        UnpublishedRecordEffectFate::EffectPossible
    };
    prior.combine(current)
}

fn recovery(
    media: &QualifiedFilesystemMedia,
    plan: &PublicationPlan,
) -> RecordPublicationRecoveryLocator {
    let RecordArtifactFile::CatalogCandidate { publication } = plan.candidate else {
        unreachable!("publication plans always own one catalog candidate")
    };
    RecordPublicationRecoveryLocator::new(media.store_identity(), plan.recovery_basis, publication)
}
