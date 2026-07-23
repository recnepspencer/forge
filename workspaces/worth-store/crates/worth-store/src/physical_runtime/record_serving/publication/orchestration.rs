use worth_store_physical_backend::{
    ArtifactTreeFailure, MediaCounterSnapshot, QualifiedFilesystemMedia,
};
use worth_store_physical_format::{
    DurablePhysicalRootManifest, PersistedRecordIdentity, RecordArtifactFile,
};

use super::super::publication::append_observation::PublicationObservation;
use super::super::publication::publication_outcome::UnpublishedRecordFailurePosture;
use super::super::residency::frame_ports::{
    CandidateFrameCoordinate, CandidateFrameDeclaration, CandidateFramePublicationPort,
    CandidateFrameRole, CandidateFrameSet,
};
use super::super::{
    publication::extent_publication::ExtentDataPlan,
    publication::segment_publication::SegmentDataPlan,
};
use super::super::{
    residency::publication_artifacts::PublicationRecordArtifacts, AdmittedPhysicalRecordFormat,
    IndeterminateRecordPublication, PublishedRecordBatch, RecordAppendDenial, RecordAppendError,
    RecordPublicationRecoveryLocator, RecordStreamFailure, UnpublishedRecordBatchCause,
    UnpublishedRecordBatchFailure, UnpublishedRecordEffectFate, UnpublishedRecordWorldFate,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordPublicationStage {
    CandidateDataWrite,
    DataSynchronization,
    ManifestSynchronization,
    CatalogCandidateSynchronization,
    CatalogReplacement,
    NamespaceSynchronization,
}

pub(in crate::physical_runtime::record_serving) enum CandidateDataArtifact {
    Segment(SegmentDataPlan),
    Extent(ExtentDataPlan),
}

pub(in crate::physical_runtime::record_serving) enum CandidateDataWriteFailure {
    PreEffectDenied(RecordAppendDenial),
    Semantic(RecordAppendDenial),
    Residency(RecordAppendDenial),
    Stream(RecordStreamFailure),
    Backend {
        failure: ArtifactTreeFailure,
        effect_fate: UnpublishedRecordEffectFate,
    },
    CandidateFrameContract(super::super::CandidateFrameContractViolation),
}

impl CandidateDataWriteFailure {
    pub(in crate::physical_runtime::record_serving) fn from_frame_write(
        failure: super::super::residency::frame_ports::CandidateFrameWriteFailure,
    ) -> Self {
        match failure {
            super::super::residency::frame_ports::CandidateFrameWriteFailure::Contract(
                violation,
            ) => Self::CandidateFrameContract(violation),
            super::super::residency::frame_ports::CandidateFrameWriteFailure::Backend(failure) => {
                Self::Backend {
                    failure,
                    effect_fate: UnpublishedRecordEffectFate::EffectPossible,
                }
            }
            super::super::residency::frame_ports::CandidateFrameWriteFailure::Residency(denial) => {
                Self::Residency(denial)
            }
        }
    }
}

pub(in crate::physical_runtime::record_serving) struct PublicationPlan {
    pub(in crate::physical_runtime::record_serving) records: Vec<PersistedRecordIdentity>,
    pub(in crate::physical_runtime::record_serving) generation: u64,
    pub(in crate::physical_runtime::record_serving) data: Vec<CandidateDataArtifact>,
    pub(in crate::physical_runtime::record_serving) manifests: Vec<(RecordArtifactFile, Vec<u8>)>,
    pub(in crate::physical_runtime::record_serving) root: RecordArtifactFile,
    pub(in crate::physical_runtime::record_serving) candidate: RecordArtifactFile,
    pub(in crate::physical_runtime::record_serving) manifest: DurablePhysicalRootManifest,
    pub(in crate::physical_runtime::record_serving) root_bytes: Vec<u8>,
    pub(in crate::physical_runtime::record_serving) catalog_bytes: Vec<u8>,
    pub(in crate::physical_runtime::record_serving) observation: PublicationObservation,
}

impl PublicationPlan {
    fn candidate_frame_set(
        &self,
        format: AdmittedPhysicalRecordFormat,
    ) -> Result<CandidateFrameSet, RecordAppendDenial> {
        let page_bytes = u64::from(format.declaration().page_size().bytes());
        let mut declarations = Vec::new();
        for data in &self.data {
            match data {
                CandidateDataArtifact::Segment(segment) => {
                    for index in 0..segment.pages.len() {
                        push_candidate_declaration(
                            &mut declarations,
                            CandidateFrameRole::InlinePage,
                            segment.artifact,
                            (index as u64).saturating_mul(page_bytes),
                            page_bytes,
                        )?;
                    }
                }
                CandidateDataArtifact::Extent(extent) => {
                    let mut logical_offset = 0_u64;
                    let mut artifact_offset = 0_u64;
                    for _ in 0..extent.manifest.chunk_count() {
                        let payload = (extent.manifest.logical_bytes() - logical_offset)
                            .min(u64::from(extent.manifest.chunk_payload_capacity()));
                        let frame_bytes = payload.saturating_add(
                            (worth_store_physical_format::DURABLE_EXTENT_FRAME_HEADER_BYTES
                                + worth_store_physical_format::EXTENT_CHUNK_METADATA_BYTES)
                                as u64,
                        );
                        push_candidate_declaration(
                            &mut declarations,
                            CandidateFrameRole::ExtentChunk,
                            extent.artifact,
                            artifact_offset,
                            frame_bytes,
                        )?;
                        logical_offset += payload;
                        artifact_offset += frame_bytes;
                    }
                }
            }
        }
        for (artifact, bytes) in &self.manifests {
            push_candidate_declaration(
                &mut declarations,
                CandidateFrameRole::ManifestBlock,
                *artifact,
                0,
                bytes.len() as u64,
            )?;
        }
        push_candidate_declaration(
            &mut declarations,
            CandidateFrameRole::RootManifest,
            self.root,
            0,
            self.root_bytes.len() as u64,
        )?;
        push_candidate_declaration(
            &mut declarations,
            CandidateFrameRole::CatalogCandidate,
            self.candidate,
            0,
            self.catalog_bytes.len() as u64,
        )?;
        CandidateFrameSet::new(self.generation, declarations).ok_or(
            RecordAppendDenial::ResidencyUnavailable(
                worth_store_buffer_pool::PhysicalResidencyDenial::FrameLengthMismatch,
            ),
        )
    }
}

fn push_candidate_declaration(
    declarations: &mut Vec<CandidateFrameDeclaration>,
    role: CandidateFrameRole,
    artifact: RecordArtifactFile,
    offset: u64,
    length: u64,
) -> Result<(), RecordAppendDenial> {
    let length = u32::try_from(length).map_err(|_| {
        RecordAppendDenial::ResidencyUnavailable(
            worth_store_buffer_pool::PhysicalResidencyDenial::FrameLengthMismatch,
        )
    })?;
    let declaration = CandidateFrameDeclaration::new(
        role,
        CandidateFrameCoordinate::new(artifact, offset),
        length,
    )
    .ok_or(RecordAppendDenial::ResidencyUnavailable(
        worth_store_buffer_pool::PhysicalResidencyDenial::FrameLengthMismatch,
    ))?;
    declarations.try_reserve(1).map_err(|_| {
        RecordAppendDenial::ResidencyUnavailable(
            worth_store_buffer_pool::PhysicalResidencyDenial::AllocationFailed,
        )
    })?;
    declarations.push(declaration);
    Ok(())
}

pub(in crate::physical_runtime::record_serving) fn execute_publication(
    port: &(dyn CandidateFramePublicationPort + Send + Sync),
    media: &QualifiedFilesystemMedia,
    format: AdmittedPhysicalRecordFormat,
    plan: PublicationPlan,
    counters_before: MediaCounterSnapshot,
) -> Result<(PublishedRecordBatch, DurablePhysicalRootManifest), RecordAppendError> {
    let declaration = plan
        .candidate_frame_set(format)
        .map_err(RecordAppendError::Denied)?;
    let mut residency =
        super::super::residency::frame_ports::StoreCandidateFramePublicationSession::begin(
            port,
            declaration,
        )
        .map_err(RecordAppendError::Denied)?;
    super::super::publication::publication_progression::execute(
        media,
        format,
        plan,
        &mut residency,
        counters_before,
    )
}

pub(in crate::physical_runtime::record_serving) fn write_candidate_data(
    artifacts: &PublicationRecordArtifacts<'_>,
    format: AdmittedPhysicalRecordFormat,
    data: &mut CandidateDataArtifact,
    residency: &mut super::super::residency::frame_ports::StoreCandidateFramePublicationSession,
    observation: &mut PublicationObservation,
) -> Result<(), CandidateDataWriteFailure> {
    match data {
        CandidateDataArtifact::Segment(plan) => {
            super::super::publication::segment_publication::write_segment(
                artifacts,
                format,
                plan,
                residency,
                observation,
            )
        }
        CandidateDataArtifact::Extent(plan) => {
            super::super::publication::extent_publication::write_extent(
                artifacts,
                format,
                plan,
                residency,
                observation,
            )
        }
    }
}

pub(in crate::physical_runtime::record_serving) fn unpublished_backend(
    media: &QualifiedFilesystemMedia,
    plan: &PublicationPlan,
    before: MediaCounterSnapshot,
    stage: RecordPublicationStage,
    failure: ArtifactTreeFailure,
    effect_fate: UnpublishedRecordEffectFate,
) -> RecordAppendError {
    RecordAppendError::Unpublished(UnpublishedRecordBatchFailure::new(
        UnpublishedRecordBatchCause::Backend { stage, failure },
        UnpublishedRecordFailurePosture::new(
            effect_fate,
            UnpublishedRecordWorldFate::InspectionRequired,
        ),
        recovery(media, plan),
        plan.records.len() as u64,
        before,
        media.counters(),
        super::super::RecordPublicationResidueObservation::from_failed_plan(plan, stage),
    ))
}

pub(in crate::physical_runtime::record_serving) fn unpublished_residency(
    media: &QualifiedFilesystemMedia,
    plan: &PublicationPlan,
    before: MediaCounterSnapshot,
    stage: RecordPublicationStage,
    denial: RecordAppendDenial,
) -> RecordAppendError {
    RecordAppendError::Unpublished(UnpublishedRecordBatchFailure::new(
        UnpublishedRecordBatchCause::Residency { stage, denial },
        UnpublishedRecordFailurePosture::new(
            UnpublishedRecordEffectFate::EffectPossible,
            UnpublishedRecordWorldFate::InspectionRequired,
        ),
        recovery(media, plan),
        plan.records.len() as u64,
        before,
        media.counters(),
        super::super::RecordPublicationResidueObservation::from_failed_plan(plan, stage),
    ))
}

pub(in crate::physical_runtime::record_serving) fn unpublished_semantic(
    media: &QualifiedFilesystemMedia,
    plan: &PublicationPlan,
    before: MediaCounterSnapshot,
    stage: RecordPublicationStage,
    denial: RecordAppendDenial,
) -> RecordAppendError {
    RecordAppendError::Unpublished(UnpublishedRecordBatchFailure::new(
        UnpublishedRecordBatchCause::Semantic { stage, denial },
        UnpublishedRecordFailurePosture::new(
            UnpublishedRecordEffectFate::EffectPossible,
            UnpublishedRecordWorldFate::InspectionRequired,
        ),
        recovery(media, plan),
        plan.records.len() as u64,
        before,
        media.counters(),
        super::super::RecordPublicationResidueObservation::from_failed_plan(plan, stage),
    ))
}

pub(in crate::physical_runtime::record_serving) fn unpublished_stream(
    media: &QualifiedFilesystemMedia,
    plan: &PublicationPlan,
    before: MediaCounterSnapshot,
    failure: RecordStreamFailure,
) -> RecordAppendError {
    RecordAppendError::Unpublished(UnpublishedRecordBatchFailure::new(
        UnpublishedRecordBatchCause::Stream(failure),
        UnpublishedRecordFailurePosture::new(
            UnpublishedRecordEffectFate::EffectPossible,
            UnpublishedRecordWorldFate::InspectionRequired,
        ),
        recovery(media, plan),
        plan.records.len() as u64,
        before,
        media.counters(),
        super::super::RecordPublicationResidueObservation::from_failed_plan(
            plan,
            RecordPublicationStage::CandidateDataWrite,
        ),
    ))
}

pub(in crate::physical_runtime::record_serving) fn unpublished_candidate_frame_contract(
    media: &QualifiedFilesystemMedia,
    plan: &PublicationPlan,
    before: MediaCounterSnapshot,
    stage: RecordPublicationStage,
    violation: super::super::CandidateFrameContractViolation,
) -> RecordAppendError {
    RecordAppendError::Unpublished(UnpublishedRecordBatchFailure::new(
        UnpublishedRecordBatchCause::CandidateFrameContract { stage, violation },
        UnpublishedRecordFailurePosture::new(
            UnpublishedRecordEffectFate::EffectPossible,
            UnpublishedRecordWorldFate::InspectionRequired,
        ),
        recovery(media, plan),
        plan.records.len() as u64,
        before,
        media.counters(),
        super::super::RecordPublicationResidueObservation::from_failed_plan(plan, stage),
    ))
}

pub(in crate::physical_runtime::record_serving) fn indeterminate(
    media: &QualifiedFilesystemMedia,
    plan: &PublicationPlan,
    before: MediaCounterSnapshot,
    stage: RecordPublicationStage,
    failure: ArtifactTreeFailure,
) -> RecordAppendError {
    RecordAppendError::Indeterminate(IndeterminateRecordPublication::new(
        stage,
        failure,
        recovery(media, plan),
        plan.records.len() as u64,
        before,
        media.counters(),
        super::super::RecordPublicationResidueObservation::from_failed_plan(plan, stage),
    ))
}

fn recovery(
    media: &QualifiedFilesystemMedia,
    plan: &PublicationPlan,
) -> RecordPublicationRecoveryLocator {
    let RecordArtifactFile::CatalogCandidate { publication } = plan.candidate else {
        unreachable!("publication plans always own one catalog candidate")
    };
    RecordPublicationRecoveryLocator::new(
        media.store_identity(),
        plan.generation - 1,
        plan.generation,
        publication,
    )
}
