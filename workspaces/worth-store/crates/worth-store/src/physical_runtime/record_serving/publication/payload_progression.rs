use worth_store_physical_backend::{MediaCounterSnapshot, QualifiedFilesystemMedia};

use super::{
    unpublished_candidate_frame_contract, unpublished_frame_writeback, unpublished_physical_work,
    unpublished_residency, unpublished_semantic, unpublished_stream, write_candidate_data,
    CandidateDataArtifact, CandidateDataWriteFailure, PublicationPlan, RecordPublicationStage,
};
use crate::physical_runtime::record_serving::{
    residency::{
        candidate_frame_residency::CandidateFrameWriteCompletion,
        frame_ports::{
            CandidateFrame, CandidateFrameCoordinate, CandidateFrameRole,
            CandidateFrameWriteFailure, StoreCandidateFramePublicationSession,
        },
        publication_artifacts::PublicationRecordArtifacts,
    },
    AdmittedPhysicalRecordFormat, RecordAppendError,
};

enum PayloadManifestStageFailure {
    Frame(CandidateFrameWriteFailure<super::super::CanonicalRecordMutationFailure>),
    Synchronization(super::super::CanonicalRecordMutationFailure),
}

pub(in crate::physical_runtime::record_serving) fn execute(
    mutation: &super::super::CanonicalRecordMutationPort,
    writeback: &super::super::residency::FrameWritebackPort,
    media: &QualifiedFilesystemMedia,
    format: AdmittedPhysicalRecordFormat,
    mut plan: PublicationPlan,
    residency: &mut StoreCandidateFramePublicationSession<'_>,
    before: MediaCounterSnapshot,
) -> Result<PublicationPlan, RecordAppendError> {
    let artifacts = PublicationRecordArtifacts::new(mutation);
    write_data(
        media, &artifacts, writeback, format, &mut plan, residency, before,
    )?;
    synchronize_data(media, &artifacts, &mut plan, before)?;
    write_payload_manifests(media, &artifacts, &mut plan, residency, before)?;
    Ok(plan)
}

fn write_data(
    media: &QualifiedFilesystemMedia,
    artifacts: &PublicationRecordArtifacts<'_>,
    writeback: &super::super::residency::FrameWritebackPort,
    format: AdmittedPhysicalRecordFormat,
    plan: &mut PublicationPlan,
    residency: &mut StoreCandidateFramePublicationSession<'_>,
    before: MediaCounterSnapshot,
) -> Result<(), RecordAppendError> {
    for index in 0..plan.data.len() {
        let (data, observation, work) =
            (&mut plan.data[index], &mut plan.observation, &mut plan.work);
        let failure = write_candidate_data(
            artifacts,
            writeback,
            format,
            data,
            residency,
            observation,
            work,
        )
        .err();
        let Some(failure) = failure else {
            continue;
        };
        return Err(match failure {
            CandidateDataWriteFailure::Semantic(denial) => unpublished_semantic(
                media,
                plan,
                before,
                RecordPublicationStage::CandidateDataWrite,
                denial,
            ),
            CandidateDataWriteFailure::Residency(denial) => unpublished_residency(
                media,
                plan,
                before,
                RecordPublicationStage::CandidateDataWrite,
                denial,
            ),
            CandidateDataWriteFailure::Stream(failure) => {
                unpublished_stream(media, plan, before, failure)
            }
            CandidateDataWriteFailure::CandidateFrameContract(violation) => {
                unpublished_candidate_frame_contract(
                    media,
                    plan,
                    before,
                    RecordPublicationStage::CandidateDataWrite,
                    violation,
                )
            }
            CandidateDataWriteFailure::Canonical(failure) => {
                if plan.work.effect_count() == 0
                    && failure.effect_fate()
                        == crate::physical_runtime::PhysicalWorkEffectFate::ProvenNoEffect
                    && canonical_failure_is_retryable(&failure)
                {
                    RecordAppendError::Denied(
                        crate::physical_runtime::record_serving::RecordAppendDenial::
                            PhysicalWorkUnavailable(Box::new(failure.evidence())),
                    )
                } else {
                    unpublished_physical_work(
                        media,
                        plan,
                        before,
                        RecordPublicationStage::CandidateDataWrite,
                        &failure,
                    )
                }
            }
            CandidateDataWriteFailure::Writeback(failure) => unpublished_frame_writeback(
                media,
                plan,
                before,
                RecordPublicationStage::CandidateDataWrite,
                *failure,
            ),
        });
    }
    Ok(())
}

fn canonical_failure_is_retryable(
    failure: &crate::physical_runtime::record_serving::CanonicalRecordMutationFailure,
) -> bool {
    use crate::physical_runtime::record_serving::PhysicalRecordMutationFailureCause;
    match failure.evidence().cause() {
        PhysicalRecordMutationFailureCause::Backend(failure) => {
            failure.kind()
                == worth_store_physical_backend::ArtifactTreeFailureKind::DeniedBeforeEffect
        }
        PhysicalRecordMutationFailureCause::Terminal(_)
        | PhysicalRecordMutationFailureCause::InvalidCoordinate
        | PhysicalRecordMutationFailureCause::SettlementMismatch => false,
        PhysicalRecordMutationFailureCause::RuntimeReleased
        | PhysicalRecordMutationFailureCause::SubmissionRejected
        | PhysicalRecordMutationFailureCause::PreEffect(_)
        | PhysicalRecordMutationFailureCause::DependencyBlocked
        | PhysicalRecordMutationFailureCause::CatalogReplacementEligibilityMismatch
        | PhysicalRecordMutationFailureCause::SchedulerReservationDenied(_)
        | PhysicalRecordMutationFailureCause::Scheduler(_)
        | PhysicalRecordMutationFailureCause::Command(_) => true,
    }
}

fn synchronize_data(
    media: &QualifiedFilesystemMedia,
    artifacts: &PublicationRecordArtifacts<'_>,
    plan: &mut PublicationPlan,
    before: MediaCounterSnapshot,
) -> Result<(), RecordAppendError> {
    for index in 0..plan.data.len() {
        let data = &plan.data[index];
        let artifact = match data {
            CandidateDataArtifact::Segment(value) => value.artifact,
            CandidateDataArtifact::Extent(value) => value.artifact,
        };
        let artifact_barrier = {
            let mut stage =
                artifacts.at(RecordPublicationStage::DataSynchronization, &mut plan.work);
            stage.synchronize_artifact(artifact)
        };
        artifact_barrier.map_err(|failure| {
            unpublished_physical_work(
                media,
                plan,
                before,
                RecordPublicationStage::DataSynchronization,
                &failure,
            )
        })?;
        let parent_barrier = {
            let mut stage =
                artifacts.at(RecordPublicationStage::DataSynchronization, &mut plan.work);
            stage.synchronize_artifact_parent(artifact)
        };
        parent_barrier.map_err(|failure| {
            unpublished_physical_work(
                media,
                plan,
                before,
                RecordPublicationStage::DataSynchronization,
                &failure,
            )
        })?;
    }
    Ok(())
}

fn write_payload_manifests(
    media: &QualifiedFilesystemMedia,
    artifacts: &PublicationRecordArtifacts<'_>,
    plan: &mut PublicationPlan,
    residency: &mut StoreCandidateFramePublicationSession<'_>,
    before: MediaCounterSnapshot,
) -> Result<(), RecordAppendError> {
    for index in 0..plan.payload_manifests.len() {
        let (artifact, bytes) = &mut plan.payload_manifests[index];
        let artifact = *artifact;
        let mut stage_work = super::RecordPublicationWorkTrace::default();
        let stage_outcome = execute_payload_manifest_stage(
            artifacts,
            residency,
            artifact,
            std::mem::take(bytes),
            &mut stage_work,
        );
        plan.work.extend(stage_work);
        let resident = match stage_outcome {
            Ok(resident) => resident,
            Err(PayloadManifestStageFailure::Frame(CandidateFrameWriteFailure::Effect(
                failure,
            ))) => {
                return Err(unpublished_physical_work(
                    media,
                    plan,
                    before,
                    RecordPublicationStage::PayloadManifestSynchronization,
                    &failure,
                ));
            }
            Err(PayloadManifestStageFailure::Frame(CandidateFrameWriteFailure::Contract(
                violation,
            ))) => {
                return Err(unpublished_candidate_frame_contract(
                    media,
                    plan,
                    before,
                    RecordPublicationStage::PayloadManifestSynchronization,
                    violation,
                ));
            }
            Err(PayloadManifestStageFailure::Frame(CandidateFrameWriteFailure::Residency(
                denial,
            ))) => {
                return Err(unpublished_residency(
                    media,
                    plan,
                    before,
                    RecordPublicationStage::PayloadManifestSynchronization,
                    denial,
                ));
            }
            Err(PayloadManifestStageFailure::Synchronization(failure)) => {
                return Err(unpublished_physical_work(
                    media,
                    plan,
                    before,
                    RecordPublicationStage::PayloadManifestSynchronization,
                    &failure,
                ))
            }
        };
        plan.observation
            .observe_transfer(resident.frame_bytes() as usize);
    }
    Ok(())
}

fn execute_payload_manifest_stage(
    artifacts: &PublicationRecordArtifacts<'_>,
    residency: &mut StoreCandidateFramePublicationSession<'_>,
    artifact: worth_store_physical_format::RecordArtifactFile,
    bytes: Vec<u8>,
    work: &mut super::RecordPublicationWorkTrace,
) -> Result<CandidateFrameWriteCompletion, PayloadManifestStageFailure> {
    let mut stage = artifacts.at(RecordPublicationStage::PayloadManifestSynchronization, work);
    let resident = stage
        .write_new_candidate(
            residency,
            CandidateFrame::new(
                CandidateFrameRole::ManifestBlock,
                CandidateFrameCoordinate::new(artifact, 0),
                bytes,
            ),
            artifact,
        )
        .map_err(PayloadManifestStageFailure::Frame)?;
    stage
        .synchronize_artifact(artifact)
        .and_then(|_| stage.synchronize_artifact_parent(artifact))
        .map_err(PayloadManifestStageFailure::Synchronization)?;
    Ok(resident)
}
