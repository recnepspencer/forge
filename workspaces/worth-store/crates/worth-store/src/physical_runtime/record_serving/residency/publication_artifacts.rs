use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use super::candidate_frame_residency::{
    CandidateFrame, CandidateFramePhysicalWrite, CandidateFrameWriteCompletion,
    CandidateFrameWriteFailure, StoreCandidateFramePublicationSession,
};

pub(in crate::physical_runtime::record_serving) struct PublicationRecordArtifacts<'port> {
    mutation: &'port super::super::CanonicalRecordMutationPort,
}

pub(in crate::physical_runtime::record_serving) struct StagedPublicationRecordArtifacts<
    'port,
    'work,
> {
    mutation: &'port super::super::CanonicalRecordMutationPort,
    stage: super::super::RecordPublicationStage,
    work: &'work mut super::super::RecordPublicationWorkTrace,
}

impl<'port> PublicationRecordArtifacts<'port> {
    pub(in crate::physical_runtime::record_serving) fn new(
        mutation: &'port super::super::CanonicalRecordMutationPort,
    ) -> Self {
        Self { mutation }
    }

    pub(in crate::physical_runtime::record_serving) fn at<'work>(
        &self,
        stage: super::super::RecordPublicationStage,
        work: &'work mut super::super::RecordPublicationWorkTrace,
    ) -> StagedPublicationRecordArtifacts<'port, 'work> {
        StagedPublicationRecordArtifacts {
            mutation: self.mutation,
            stage,
            work,
        }
    }
}

impl StagedPublicationRecordArtifacts<'_, '_> {
    pub(in crate::physical_runtime::record_serving) fn write_new_candidate(
        &mut self,
        residency: &mut StoreCandidateFramePublicationSession,
        frame: CandidateFrame,
        artifact: RecordArtifactFile,
    ) -> Result<
        CandidateFrameWriteCompletion,
        CandidateFrameWriteFailure<super::super::CanonicalRecordMutationFailure>,
    > {
        residency.write_frame(frame, &mut |bytes| self.write_new_frame(artifact, bytes))
    }

    pub(in crate::physical_runtime::record_serving) fn append_candidate(
        &mut self,
        residency: &mut StoreCandidateFramePublicationSession,
        frame: CandidateFrame,
        coordinate: RecordFrameCoordinate,
    ) -> Result<
        CandidateFrameWriteCompletion,
        CandidateFrameWriteFailure<super::super::CanonicalRecordMutationFailure>,
    > {
        residency.write_frame(frame, &mut |bytes| self.append_frame(coordinate, bytes))
    }

    fn write_new_frame(
        &mut self,
        artifact: RecordArtifactFile,
        bytes: &[u8],
    ) -> Result<CandidateFramePhysicalWrite, super::super::CanonicalRecordMutationFailure> {
        let length = u32::try_from(bytes.len()).expect("candidate frame length is u32-bounded");
        let coordinate = RecordFrameCoordinate::new(artifact, 0, length)
            .expect("candidate frames are nonempty and offset-bounded");
        let physical = candidate_frame(
            self.mutation
                .prepare_new_artifact(self.stage, coordinate, bytes)?
                .execute()?,
        )?;
        self.record_candidate(&physical);
        Ok(physical)
    }

    fn append_frame(
        &mut self,
        coordinate: RecordFrameCoordinate,
        bytes: &[u8],
    ) -> Result<CandidateFramePhysicalWrite, super::super::CanonicalRecordMutationFailure> {
        let physical = candidate_frame(
            self.mutation
                .prepare_extent_append(self.stage, coordinate, bytes)?
                .execute()?,
        )?;
        self.record_candidate(&physical);
        Ok(physical)
    }

    pub(in crate::physical_runtime::record_serving) fn synchronize_artifact(
        &mut self,
        artifact: RecordArtifactFile,
    ) -> Result<
        super::super::CanonicalRecordMutationSettlement,
        super::super::CanonicalRecordMutationFailure,
    > {
        self.record_publication(self.mutation.prepare_publication_effect(
            self.stage,
            artifact,
            super::super::CanonicalRecordPublicationEffect::Artifact,
        )?)
    }

    pub(in crate::physical_runtime::record_serving) fn synchronize_artifact_parent(
        &mut self,
        artifact: RecordArtifactFile,
    ) -> Result<
        super::super::CanonicalRecordMutationSettlement,
        super::super::CanonicalRecordMutationFailure,
    > {
        self.record_publication(self.mutation.prepare_publication_effect(
            self.stage,
            artifact,
            super::super::CanonicalRecordPublicationEffect::ArtifactParent,
        )?)
    }

    pub(in crate::physical_runtime::record_serving) fn synchronize_record_family(
        &mut self,
    ) -> Result<
        super::super::CanonicalRecordMutationSettlement,
        super::super::CanonicalRecordMutationFailure,
    > {
        self.record_publication(self.mutation.prepare_publication_effect(
            self.stage,
            RecordArtifactFile::BootstrapCatalog,
            super::super::CanonicalRecordPublicationEffect::RecordFamily,
        )?)
    }

    fn record_candidate(&mut self, physical: &CandidateFramePhysicalWrite) {
        let settlement = physical
            .settlement()
            .expect("canonical candidate writes carry physical settlement");
        self.work
            .record_settled(self.stage, settlement.identity(), settlement.publication());
    }

    fn record_publication(
        &mut self,
        prepared: super::super::PreparedCanonicalRecordMutation,
    ) -> Result<
        super::super::CanonicalRecordMutationSettlement,
        super::super::CanonicalRecordMutationFailure,
    > {
        let settlement = publication_effect(prepared)?;
        self.work
            .record_settled(self.stage, settlement.identity(), settlement.publication());
        Ok(settlement)
    }
}

fn candidate_frame(
    completion: super::super::CanonicalRecordMutationCompletion,
) -> Result<CandidateFramePhysicalWrite, super::super::CanonicalRecordMutationFailure> {
    let settlement = completion.settlement();
    match completion {
        super::super::CanonicalRecordMutationCompletion::CandidateFrame(completed) => {
            Ok(completed.into_physical())
        }
        super::super::CanonicalRecordMutationCompletion::PublicationEffect(_) => {
            Err(super::super::CanonicalRecordMutationFailure::settlement_mismatch(settlement))
        }
    }
}

fn publication_effect(
    prepared: super::super::PreparedCanonicalRecordMutation,
) -> Result<
    super::super::CanonicalRecordMutationSettlement,
    super::super::CanonicalRecordMutationFailure,
> {
    let completion = prepared.execute()?;
    let settlement = completion.settlement();
    match completion {
        super::super::CanonicalRecordMutationCompletion::PublicationEffect(settlement) => {
            Ok(settlement)
        }
        super::super::CanonicalRecordMutationCompletion::CandidateFrame(_) => {
            Err(super::super::CanonicalRecordMutationFailure::settlement_mismatch(settlement))
        }
    }
}
