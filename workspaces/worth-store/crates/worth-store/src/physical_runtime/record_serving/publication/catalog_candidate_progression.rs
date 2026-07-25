use worth_store_physical_backend::{MediaCounterSnapshot, QualifiedFilesystemMedia};
use worth_store_physical_format::RecordArtifactFile;

use super::super::RecordAppendError;
use super::{
    manifest_progression::ManifestsSynchronized, unpublished_candidate_frame_contract,
    unpublished_physical_work, unpublished_residency, PublicationPlan, RecordPublicationStage,
    RecordPublicationWorkTrace,
};
use crate::physical_runtime::record_serving::residency::{
    candidate_frame_residency::CandidateFrameWriteCompletion,
    frame_ports::{
        CandidateFrame, CandidateFrameCoordinate, CandidateFrameRole, CandidateFrameWriteFailure,
        StoreCandidateFramePublicationSession,
    },
    publication_artifacts::PublicationRecordArtifacts,
};

pub(super) struct CatalogCandidateSynchronized(PublicationPlan);

pub(super) struct SettledPublicationArtifacts {
    candidate: RecordArtifactFile,
}

enum CatalogCandidateStageFailure {
    Frame(CandidateFrameWriteFailure<super::super::CanonicalRecordMutationFailure>),
    Synchronization(super::super::CanonicalRecordMutationFailure),
}

impl CatalogCandidateSynchronized {
    pub(super) fn plan(&self) -> &PublicationPlan {
        &self.0
    }

    pub(super) fn into_plan(self) -> PublicationPlan {
        self.0
    }

    pub(super) fn settled_artifacts(&self) -> SettledPublicationArtifacts {
        SettledPublicationArtifacts {
            candidate: self.0.candidate,
        }
    }
}

impl SettledPublicationArtifacts {
    pub(super) const fn candidate(&self) -> RecordArtifactFile {
        self.candidate
    }
}

pub(super) fn synchronize_catalog_candidate(
    media: &QualifiedFilesystemMedia,
    artifacts: &PublicationRecordArtifacts<'_>,
    synchronized: ManifestsSynchronized,
    residency: &mut StoreCandidateFramePublicationSession,
    before: MediaCounterSnapshot,
) -> Result<CatalogCandidateSynchronized, RecordAppendError> {
    let mut plan = synchronized.into_plan();
    let candidate = plan.candidate;
    let mut stage_work = RecordPublicationWorkTrace::default();
    let stage_outcome = execute_catalog_candidate_stage(
        artifacts,
        residency,
        candidate,
        std::mem::take(&mut plan.catalog_bytes),
        &mut stage_work,
    );
    plan.work.extend(stage_work);
    let resident = match stage_outcome {
        Ok(resident) => resident,
        Err(CatalogCandidateStageFailure::Frame(failure)) => {
            return Err(frame_failure(media, &plan, before, failure))
        }
        Err(CatalogCandidateStageFailure::Synchronization(failure)) => {
            return Err(unpublished_physical_work(
                media,
                &plan,
                before,
                RecordPublicationStage::CatalogCandidateSynchronization,
                &failure,
            ))
        }
    };
    plan.observation
        .observe_transfer(resident.frame_bytes() as usize);
    Ok(CatalogCandidateSynchronized(plan))
}

fn execute_catalog_candidate_stage(
    artifacts: &PublicationRecordArtifacts<'_>,
    residency: &mut StoreCandidateFramePublicationSession,
    candidate: RecordArtifactFile,
    bytes: Vec<u8>,
    work: &mut RecordPublicationWorkTrace,
) -> Result<CandidateFrameWriteCompletion, CatalogCandidateStageFailure> {
    let mut stage = artifacts.at(
        RecordPublicationStage::CatalogCandidateSynchronization,
        work,
    );
    let resident = stage
        .write_new_candidate(
            residency,
            CandidateFrame::new(
                CandidateFrameRole::CatalogCandidate,
                CandidateFrameCoordinate::new(candidate, 0),
                bytes,
            ),
            candidate,
        )
        .map_err(CatalogCandidateStageFailure::Frame)?;
    stage
        .synchronize_artifact(candidate)
        .map_err(CatalogCandidateStageFailure::Synchronization)?;
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
            RecordPublicationStage::CatalogCandidateSynchronization,
            &failure,
        ),
        CandidateFrameWriteFailure::Contract(violation) => unpublished_candidate_frame_contract(
            media,
            plan,
            before,
            RecordPublicationStage::CatalogCandidateSynchronization,
            violation,
        ),
        CandidateFrameWriteFailure::Residency(denial) => unpublished_residency(
            media,
            plan,
            before,
            RecordPublicationStage::CatalogCandidateSynchronization,
            denial,
        ),
    }
}
