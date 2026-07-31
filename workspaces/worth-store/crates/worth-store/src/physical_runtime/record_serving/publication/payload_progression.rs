use worth_store_physical_backend::{MediaCounterSnapshot, QualifiedFilesystemMedia};

use super::{
    unpublished_candidate_frame_contract_with_posture, unpublished_candidate_frame_residency,
    unpublished_frame_writeback, unpublished_physical_work, unpublished_semantic,
    unpublished_stream, write_candidate_data, CandidateDataArtifact, CandidateDataWriteFailure,
    PublicationPlan, RecordPublicationStage,
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

pub(super) struct PayloadPublicationBasis {
    generation: crate::physical_runtime::LifecycleGeneration,
    format: AdmittedPhysicalRecordFormat,
    before: MediaCounterSnapshot,
}

impl PayloadPublicationBasis {
    pub(super) const fn new(
        generation: crate::physical_runtime::LifecycleGeneration,
        format: AdmittedPhysicalRecordFormat,
        before: MediaCounterSnapshot,
    ) -> Self {
        Self {
            generation,
            format,
            before,
        }
    }
}

pub(super) struct PayloadPublicationProgression<'authority> {
    artifacts: PublicationRecordArtifacts<'authority>,
    writeback: &'authority super::super::residency::FrameWritebackPort,
    media: &'authority QualifiedFilesystemMedia,
    basis: PayloadPublicationBasis,
}

impl<'authority> PayloadPublicationProgression<'authority> {
    pub(super) fn new(
        mutation: &'authority super::super::CanonicalRecordMutationPort,
        writeback: &'authority super::super::residency::FrameWritebackPort,
        media: &'authority QualifiedFilesystemMedia,
        basis: PayloadPublicationBasis,
    ) -> Self {
        Self {
            artifacts: PublicationRecordArtifacts::new(mutation),
            writeback,
            media,
            basis,
        }
    }

    pub(super) fn execute(
        self,
        mut plan: PublicationPlan,
        residency: &mut StoreCandidateFramePublicationSession<'_>,
    ) -> Result<PublicationPlan, RecordAppendError> {
        self.write_data(&mut plan, residency)?;
        self.synchronize_data(&mut plan)?;
        self.write_payload_manifests(&mut plan, residency)?;
        Ok(plan)
    }

    fn write_data(
        &self,
        plan: &mut PublicationPlan,
        residency: &mut StoreCandidateFramePublicationSession<'_>,
    ) -> Result<(), RecordAppendError> {
        for index in 0..plan.data.len() {
            let (data, observation, work) =
                (&mut plan.data[index], &mut plan.observation, &mut plan.work);
            let failure = write_candidate_data(
                &self.artifacts,
                self.writeback,
                self.basis.format,
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
                    self.media,
                    plan,
                    self.basis.before,
                    RecordPublicationStage::CandidateDataWrite,
                    denial,
                ),
                CandidateDataWriteFailure::Residency { denial, posture } => {
                    unpublished_candidate_frame_residency(
                        self.media,
                        plan,
                        self.basis.before,
                        RecordPublicationStage::CandidateDataWrite,
                        denial,
                        posture,
                    )
                }
                CandidateDataWriteFailure::Stream(failure) => {
                    unpublished_stream(self.media, plan, self.basis.before, failure)
                }
                CandidateDataWriteFailure::CandidateFrameContract { violation, posture } => {
                    unpublished_candidate_frame_contract_with_posture(
                        self.media,
                        plan,
                        self.basis.before,
                        RecordPublicationStage::CandidateDataWrite,
                        violation,
                        posture,
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
                            self.media,
                            plan,
                            self.basis.before,
                            RecordPublicationStage::CandidateDataWrite,
                            &failure,
                        )
                    }
                }
                CandidateDataWriteFailure::Writeback(failure) => unpublished_frame_writeback(
                    self.media,
                    self.basis.generation,
                    plan,
                    self.basis.before,
                    RecordPublicationStage::CandidateDataWrite,
                    *failure,
                ),
            });
        }
        Ok(())
    }

    fn synchronize_data(&self, plan: &mut PublicationPlan) -> Result<(), RecordAppendError> {
        for index in 0..plan.data.len() {
            let data = &plan.data[index];
            let artifact = match data {
                CandidateDataArtifact::Segment(value) => value.artifact,
                CandidateDataArtifact::Extent(value) => value.artifact,
            };
            let artifact_barrier = {
                let mut stage = self
                    .artifacts
                    .at(RecordPublicationStage::DataSynchronization, &mut plan.work);
                stage.synchronize_artifact(artifact)
            };
            artifact_barrier.map_err(|failure| {
                unpublished_physical_work(
                    self.media,
                    plan,
                    self.basis.before,
                    RecordPublicationStage::DataSynchronization,
                    &failure,
                )
            })?;
            let parent_barrier = {
                let mut stage = self
                    .artifacts
                    .at(RecordPublicationStage::DataSynchronization, &mut plan.work);
                stage.synchronize_artifact_parent(artifact)
            };
            parent_barrier.map_err(|failure| {
                unpublished_physical_work(
                    self.media,
                    plan,
                    self.basis.before,
                    RecordPublicationStage::DataSynchronization,
                    &failure,
                )
            })?;
        }
        Ok(())
    }

    fn write_payload_manifests(
        &self,
        plan: &mut PublicationPlan,
        residency: &mut StoreCandidateFramePublicationSession<'_>,
    ) -> Result<(), RecordAppendError> {
        for index in 0..plan.payload_manifests.len() {
            let (artifact, bytes) = &mut plan.payload_manifests[index];
            let artifact = *artifact;
            let mut stage_work = super::RecordPublicationWorkTrace::default();
            let stage_outcome = self.execute_payload_manifest_stage(
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
                        self.media,
                        plan,
                        self.basis.before,
                        RecordPublicationStage::PayloadManifestSynchronization,
                        &failure,
                    ));
                }
                Err(PayloadManifestStageFailure::Frame(CandidateFrameWriteFailure::Contract {
                    violation,
                    posture,
                })) => {
                    return Err(unpublished_candidate_frame_contract_with_posture(
                        self.media,
                        plan,
                        self.basis.before,
                        RecordPublicationStage::PayloadManifestSynchronization,
                        violation,
                        posture,
                    ));
                }
                Err(PayloadManifestStageFailure::Frame(
                    CandidateFrameWriteFailure::Residency { denial, posture },
                )) => {
                    return Err(unpublished_candidate_frame_residency(
                        self.media,
                        plan,
                        self.basis.before,
                        RecordPublicationStage::PayloadManifestSynchronization,
                        denial,
                        posture,
                    ));
                }
                Err(PayloadManifestStageFailure::Synchronization(failure)) => {
                    return Err(unpublished_physical_work(
                        self.media,
                        plan,
                        self.basis.before,
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
        &self,
        residency: &mut StoreCandidateFramePublicationSession<'_>,
        artifact: worth_store_physical_format::RecordArtifactFile,
        bytes: Vec<u8>,
        work: &mut super::RecordPublicationWorkTrace,
    ) -> Result<CandidateFrameWriteCompletion, PayloadManifestStageFailure> {
        let mut stage = self
            .artifacts
            .at(RecordPublicationStage::PayloadManifestSynchronization, work);
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
