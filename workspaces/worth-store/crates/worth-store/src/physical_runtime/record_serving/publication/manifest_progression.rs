use worth_store_physical_backend::{MediaCounterSnapshot, QualifiedFilesystemMedia};

use super::super::RecordAppendError;
use super::{
    unpublished_candidate_frame_contract, unpublished_physical_work, unpublished_residency,
    PublicationPlan, RecordPublicationStage, RecordPublicationWorkTrace,
};
use crate::physical_runtime::record_serving::residency::{
    candidate_frame_residency::CandidateFrameWriteCompletion,
    frame_ports::{
        CandidateFrame, CandidateFrameCoordinate, CandidateFrameRole, CandidateFrameWriteFailure,
        StoreCandidateFramePublicationSession,
    },
    publication_artifacts::PublicationRecordArtifacts,
};

pub(super) struct DataSynchronized(PublicationPlan);
pub(super) struct ManifestsSynchronized(PublicationPlan);

enum ManifestStageFailure {
    Frame(CandidateFrameWriteFailure<super::super::CanonicalRecordMutationFailure>),
    Synchronization(super::super::CanonicalRecordMutationFailure),
}

impl DataSynchronized {
    pub(super) fn new(plan: PublicationPlan) -> Self {
        Self(plan)
    }
}

impl ManifestsSynchronized {
    pub(super) fn into_plan(self) -> PublicationPlan {
        self.0
    }
}

pub(super) fn synchronize_manifests(
    media: &QualifiedFilesystemMedia,
    artifacts: &PublicationRecordArtifacts<'_>,
    mut synchronized: DataSynchronized,
    residency: &mut StoreCandidateFramePublicationSession,
    before: MediaCounterSnapshot,
) -> Result<ManifestsSynchronized, RecordAppendError> {
    for index in 0..synchronized.0.manifests.len() {
        synchronize_manifest_at(
            media,
            artifacts,
            &mut synchronized.0,
            residency,
            before,
            index,
        )?;
    }
    synchronize_root(media, artifacts, &mut synchronized.0, residency, before)?;
    Ok(ManifestsSynchronized(synchronized.0))
}

fn synchronize_manifest_at(
    media: &QualifiedFilesystemMedia,
    artifacts: &PublicationRecordArtifacts<'_>,
    plan: &mut PublicationPlan,
    residency: &mut StoreCandidateFramePublicationSession,
    before: MediaCounterSnapshot,
    index: usize,
) -> Result<(), RecordAppendError> {
    let (artifact, bytes) = &mut plan.manifests[index];
    let artifact = *artifact;
    let frame = CandidateFrame::new(
        CandidateFrameRole::ManifestBlock,
        CandidateFrameCoordinate::new(artifact, 0),
        std::mem::take(bytes),
    );
    write_and_synchronize(media, artifacts, plan, residency, before, artifact, frame)
}

fn synchronize_root(
    media: &QualifiedFilesystemMedia,
    artifacts: &PublicationRecordArtifacts<'_>,
    plan: &mut PublicationPlan,
    residency: &mut StoreCandidateFramePublicationSession,
    before: MediaCounterSnapshot,
) -> Result<(), RecordAppendError> {
    let artifact = plan.root;
    let frame = CandidateFrame::new(
        CandidateFrameRole::RootManifest,
        CandidateFrameCoordinate::new(artifact, 0),
        std::mem::take(&mut plan.root_bytes),
    );
    write_and_synchronize(media, artifacts, plan, residency, before, artifact, frame)
}

fn write_and_synchronize(
    media: &QualifiedFilesystemMedia,
    artifacts: &PublicationRecordArtifacts<'_>,
    plan: &mut PublicationPlan,
    residency: &mut StoreCandidateFramePublicationSession,
    before: MediaCounterSnapshot,
    artifact: worth_store_physical_format::RecordArtifactFile,
    frame: CandidateFrame,
) -> Result<(), RecordAppendError> {
    let mut stage_work = RecordPublicationWorkTrace::default();
    let stage_outcome =
        execute_manifest_stage(artifacts, residency, artifact, frame, &mut stage_work);
    plan.work.extend(stage_work);
    let resident = match stage_outcome {
        Ok(resident) => resident,
        Err(ManifestStageFailure::Frame(failure)) => {
            return Err(frame_failure(media, plan, before, failure))
        }
        Err(ManifestStageFailure::Synchronization(failure)) => {
            return Err(unpublished_physical_work(
                media,
                plan,
                before,
                RecordPublicationStage::ManifestSynchronization,
                &failure,
            ))
        }
    };
    plan.observation
        .observe_transfer(resident.frame_bytes() as usize);
    Ok(())
}

fn execute_manifest_stage(
    artifacts: &PublicationRecordArtifacts<'_>,
    residency: &mut StoreCandidateFramePublicationSession,
    artifact: worth_store_physical_format::RecordArtifactFile,
    frame: CandidateFrame,
    work: &mut RecordPublicationWorkTrace,
) -> Result<CandidateFrameWriteCompletion, ManifestStageFailure> {
    let mut stage = artifacts.at(RecordPublicationStage::ManifestSynchronization, work);
    let resident = stage
        .write_new_candidate(residency, frame, artifact)
        .map_err(ManifestStageFailure::Frame)?;
    stage
        .synchronize_artifact(artifact)
        .and_then(|_| stage.synchronize_artifact_parent(artifact))
        .map_err(ManifestStageFailure::Synchronization)?;
    Ok(resident)
}

fn frame_failure(
    media: &QualifiedFilesystemMedia,
    plan: &PublicationPlan,
    before: MediaCounterSnapshot,
    failure: CandidateFrameWriteFailure<super::super::CanonicalRecordMutationFailure>,
) -> RecordAppendError {
    match failure {
        CandidateFrameWriteFailure::Effect(failure) => unpublished_physical_work(
            media,
            plan,
            before,
            RecordPublicationStage::ManifestSynchronization,
            &failure,
        ),
        CandidateFrameWriteFailure::Contract(violation) => unpublished_candidate_frame_contract(
            media,
            plan,
            before,
            RecordPublicationStage::ManifestSynchronization,
            violation,
        ),
        CandidateFrameWriteFailure::Residency(denial) => unpublished_residency(
            media,
            plan,
            before,
            RecordPublicationStage::ManifestSynchronization,
            denial,
        ),
    }
}
