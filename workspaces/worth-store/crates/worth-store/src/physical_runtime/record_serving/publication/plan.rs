use worth_store_physical_format::{DurablePhysicalRootManifest, RecordArtifactFile};

use super::super::{
    publication::{
        append_observation::PublicationObservation, extent_publication::ExtentDataPlan,
        segment_publication::SegmentDataPlan,
    },
    residency::frame_ports::{
        CandidateFrameCoordinate, CandidateFrameDeclaration, CandidateFrameRole, CandidateFrameSet,
    },
    RecordAppendDenial,
};

pub(in crate::physical_runtime::record_serving) enum CandidateDataArtifact {
    Segment(SegmentDataPlan),
    Extent(ExtentDataPlan),
}

pub(in crate::physical_runtime::record_serving) struct PublicationPlan {
    pub(in crate::physical_runtime::record_serving) generation: u64,
    pub(in crate::physical_runtime::record_serving) manifests: Vec<(RecordArtifactFile, Vec<u8>)>,
    pub(in crate::physical_runtime::record_serving) root: RecordArtifactFile,
    pub(in crate::physical_runtime::record_serving) candidate: RecordArtifactFile,
    pub(in crate::physical_runtime::record_serving) manifest: DurablePhysicalRootManifest,
    pub(in crate::physical_runtime::record_serving) root_bytes: Vec<u8>,
    pub(in crate::physical_runtime::record_serving) catalog_bytes: Vec<u8>,
    pub(in crate::physical_runtime::record_serving) observation: PublicationObservation,
}

impl PublicationPlan {
    pub(in crate::physical_runtime::record_serving) fn root_candidate_frame_set(
        &self,
    ) -> Result<CandidateFrameSet, RecordAppendDenial> {
        let mut declarations = Vec::new();
        for (artifact, bytes) in &self.manifests {
            push_candidate_declaration(
                &mut declarations,
                CandidateFrameRole::ManifestBlock,
                *artifact,
                bytes.len() as u64,
            )?;
        }
        push_candidate_declaration(
            &mut declarations,
            CandidateFrameRole::RootManifest,
            self.root,
            self.root_bytes.len() as u64,
        )?;
        push_candidate_declaration(
            &mut declarations,
            CandidateFrameRole::CatalogCandidate,
            self.candidate,
            self.catalog_bytes.len() as u64,
        )?;
        CandidateFrameSet::new(self.generation, declarations).ok_or_else(frame_length_denial)
    }
}

fn push_candidate_declaration(
    declarations: &mut Vec<CandidateFrameDeclaration>,
    role: CandidateFrameRole,
    artifact: RecordArtifactFile,
    length: u64,
) -> Result<(), RecordAppendDenial> {
    let length = u32::try_from(length).map_err(|_| frame_length_denial())?;
    let declaration =
        CandidateFrameDeclaration::new(role, CandidateFrameCoordinate::new(artifact, 0), length)
            .ok_or_else(frame_length_denial)?;
    declarations
        .try_reserve(1)
        .map_err(|_| allocation_denial())?;
    declarations.push(declaration);
    Ok(())
}

fn frame_length_denial() -> RecordAppendDenial {
    RecordAppendDenial::from_residency(
        worth_store_buffer_pool::PhysicalResidencyDenial::FrameLengthMismatch,
    )
}

fn allocation_denial() -> RecordAppendDenial {
    RecordAppendDenial::from_residency(
        worth_store_buffer_pool::PhysicalResidencyDenial::AllocationFailed,
    )
}
