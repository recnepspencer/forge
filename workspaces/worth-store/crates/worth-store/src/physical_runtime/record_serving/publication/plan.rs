use worth_store_physical_format::{
    DurablePhysicalRootManifest, PersistedRecordIdentity, RecordArtifactFile,
};

use super::super::{
    publication::{
        append_observation::PublicationObservation, extent_publication::ExtentDataPlan,
        segment_publication::SegmentDataPlan,
    },
    residency::frame_ports::{
        CandidateFrameCoordinate, CandidateFrameDeclaration, CandidateFrameRole, CandidateFrameSet,
    },
    AdmittedPhysicalRecordFormat, RecordAppendDenial, RecordPublicationRecoveryBasis,
    RecordPublicationWorkTrace, RecordStreamFailure,
};

pub(in crate::physical_runtime::record_serving) enum CandidateDataArtifact {
    Segment(SegmentDataPlan),
    Extent(ExtentDataPlan),
}

pub(in crate::physical_runtime::record_serving) enum CandidateDataWriteFailure {
    Semantic(RecordAppendDenial),
    Residency(RecordAppendDenial),
    Stream(RecordStreamFailure),
    CandidateFrameContract(super::super::CandidateFrameContractViolation),
    Canonical(Box<super::super::CanonicalRecordMutationFailure>),
}

impl CandidateDataWriteFailure {
    pub(in crate::physical_runtime::record_serving) fn from_frame_write(
        failure: super::super::residency::frame_ports::CandidateFrameWriteFailure<
            super::super::CanonicalRecordMutationFailure,
        >,
    ) -> Self {
        match failure {
            super::super::residency::frame_ports::CandidateFrameWriteFailure::Contract(
                violation,
            ) => Self::CandidateFrameContract(violation),
            super::super::residency::frame_ports::CandidateFrameWriteFailure::Effect(failure) => {
                Self::Canonical(Box::new(failure))
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
    pub(in crate::physical_runtime::record_serving) payload_manifests:
        Vec<(RecordArtifactFile, Vec<u8>)>,
    pub(in crate::physical_runtime::record_serving) manifests: Vec<(RecordArtifactFile, Vec<u8>)>,
    pub(in crate::physical_runtime::record_serving) root: RecordArtifactFile,
    pub(in crate::physical_runtime::record_serving) candidate: RecordArtifactFile,
    pub(in crate::physical_runtime::record_serving) manifest: DurablePhysicalRootManifest,
    pub(in crate::physical_runtime::record_serving) root_bytes: Vec<u8>,
    pub(in crate::physical_runtime::record_serving) catalog_bytes: Vec<u8>,
    pub(in crate::physical_runtime::record_serving) observation: PublicationObservation,
    pub(in crate::physical_runtime::record_serving) work: RecordPublicationWorkTrace,
    pub(in crate::physical_runtime::record_serving) recovery_basis: RecordPublicationRecoveryBasis,
}

impl PublicationPlan {
    pub(in crate::physical_runtime::record_serving) fn payload_candidate_frame_set(
        &self,
        format: AdmittedPhysicalRecordFormat,
    ) -> Result<CandidateFrameSet, RecordAppendDenial> {
        let mut declarations = Vec::new();
        self.push_payload_declarations(format, &mut declarations)?;
        candidate_frame_set(self.generation, declarations)
    }

    pub(in crate::physical_runtime::record_serving) fn root_candidate_frame_set(
        &self,
    ) -> Result<CandidateFrameSet, RecordAppendDenial> {
        let mut declarations = Vec::new();
        self.push_root_declarations(&mut declarations)?;
        candidate_frame_set(self.generation, declarations)
    }

    fn push_payload_declarations(
        &self,
        format: AdmittedPhysicalRecordFormat,
        declarations: &mut Vec<CandidateFrameDeclaration>,
    ) -> Result<(), RecordAppendDenial> {
        let page_bytes = u64::from(format.declaration().page_size().bytes());
        for data in &self.data {
            match data {
                CandidateDataArtifact::Segment(segment) => {
                    for index in 0..segment.pages.len() {
                        push_candidate_declaration(
                            declarations,
                            CandidateFrameRole::InlinePage,
                            segment.artifact,
                            (index as u64).saturating_mul(page_bytes),
                            page_bytes,
                        )?;
                    }
                }
                CandidateDataArtifact::Extent(extent) => {
                    push_extent_declarations(declarations, extent)?;
                }
            }
        }
        for (artifact, bytes) in &self.payload_manifests {
            push_candidate_declaration(
                declarations,
                CandidateFrameRole::ManifestBlock,
                *artifact,
                0,
                bytes.len() as u64,
            )?;
        }
        Ok(())
    }

    fn push_root_declarations(
        &self,
        declarations: &mut Vec<CandidateFrameDeclaration>,
    ) -> Result<(), RecordAppendDenial> {
        for (artifact, bytes) in &self.manifests {
            push_candidate_declaration(
                declarations,
                CandidateFrameRole::ManifestBlock,
                *artifact,
                0,
                bytes.len() as u64,
            )?;
        }
        push_candidate_declaration(
            declarations,
            CandidateFrameRole::RootManifest,
            self.root,
            0,
            self.root_bytes.len() as u64,
        )?;
        push_candidate_declaration(
            declarations,
            CandidateFrameRole::CatalogCandidate,
            self.candidate,
            0,
            self.catalog_bytes.len() as u64,
        )
    }
}

fn push_extent_declarations(
    declarations: &mut Vec<CandidateFrameDeclaration>,
    extent: &ExtentDataPlan,
) -> Result<(), RecordAppendDenial> {
    let mut logical_offset = 0_u64;
    let mut artifact_offset = 0_u64;
    for _ in 0..extent.manifest.chunk_count() {
        let payload = (extent.manifest.logical_bytes() - logical_offset)
            .min(u64::from(extent.manifest.chunk_payload_capacity()));
        let frame_bytes = payload.saturating_add(
            (worth_store_physical_format::DURABLE_EXTENT_FRAME_HEADER_BYTES
                + worth_store_physical_format::EXTENT_CHUNK_METADATA_BYTES) as u64,
        );
        push_candidate_declaration(
            declarations,
            CandidateFrameRole::ExtentChunk,
            extent.artifact,
            artifact_offset,
            frame_bytes,
        )?;
        logical_offset += payload;
        artifact_offset += frame_bytes;
    }
    Ok(())
}

fn candidate_frame_set(
    generation: u64,
    declarations: Vec<CandidateFrameDeclaration>,
) -> Result<CandidateFrameSet, RecordAppendDenial> {
    CandidateFrameSet::new(generation, declarations).ok_or(frame_length_denial())
}

fn push_candidate_declaration(
    declarations: &mut Vec<CandidateFrameDeclaration>,
    role: CandidateFrameRole,
    artifact: RecordArtifactFile,
    offset: u64,
    length: u64,
) -> Result<(), RecordAppendDenial> {
    let length = u32::try_from(length).map_err(|_| frame_length_denial())?;
    let declaration = CandidateFrameDeclaration::new(
        role,
        CandidateFrameCoordinate::new(artifact, offset),
        length,
    )
    .ok_or_else(frame_length_denial)?;
    declarations
        .try_reserve(1)
        .map_err(|_| allocation_denial())?;
    declarations.push(declaration);
    Ok(())
}

fn frame_length_denial() -> RecordAppendDenial {
    RecordAppendDenial::ResidencyUnavailable(
        worth_store_buffer_pool::PhysicalResidencyDenial::FrameLengthMismatch,
    )
}

fn allocation_denial() -> RecordAppendDenial {
    RecordAppendDenial::ResidencyUnavailable(
        worth_store_buffer_pool::PhysicalResidencyDenial::AllocationFailed,
    )
}
