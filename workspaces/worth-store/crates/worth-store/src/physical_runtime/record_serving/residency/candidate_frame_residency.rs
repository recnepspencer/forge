use worth_store_physical_backend::ArtifactTreeFailure;
use worth_store_physical_format::RecordArtifactFile;

pub use self::write_evidence::CandidateFrameContractViolation;
pub(in crate::physical_runtime::record_serving) use self::write_evidence::{
    CandidateFramePhysicalWrite, CandidateFrameWriteCompletion, CandidateFrameWriteFailure,
};
use super::super::RecordAppendDenial;

mod write_evidence;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime::record_serving) struct CandidateFrameDeclaration {
    role: CandidateFrameRole,
    coordinate: CandidateFrameCoordinate,
    length: u32,
}

impl CandidateFrameDeclaration {
    pub(in crate::physical_runtime::record_serving) fn new(
        role: CandidateFrameRole,
        coordinate: CandidateFrameCoordinate,
        length: u32,
    ) -> Option<Self> {
        if length == 0 {
            return None;
        }
        Some(Self {
            role,
            coordinate,
            length,
        })
    }

    pub(in crate::physical_runtime::record_serving) const fn coordinate(
        self,
    ) -> CandidateFrameCoordinate {
        self.coordinate
    }

    pub(in crate::physical_runtime::record_serving) const fn length(self) -> u32 {
        self.length
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::physical_runtime::record_serving) struct CandidateFrameSet {
    frames: Vec<CandidateFrameDeclaration>,
    total_frame_bytes: u64,
}

impl CandidateFrameSet {
    pub(in crate::physical_runtime::record_serving) fn new(
        root_generation: u64,
        frames: Vec<CandidateFrameDeclaration>,
    ) -> Option<Self> {
        if root_generation == 0 || frames.is_empty() {
            return None;
        }
        let mut total_frame_bytes = 0_u64;
        let mut index = 0;
        while index < frames.len() {
            let frame = frames[index];
            if !coordinate_matches_role(frame.role, frame.coordinate.artifact()) {
                return None;
            }
            if matches!(
                frame.coordinate.artifact(),
                RecordArtifactFile::RootManifest { generation }
                    if generation != root_generation
            ) {
                return None;
            }
            total_frame_bytes = total_frame_bytes.checked_add(frame.length as u64)?;
            index += 1;
        }
        Some(Self {
            frames,
            total_frame_bytes,
        })
    }

    pub(in crate::physical_runtime::record_serving) fn frame_count(&self) -> u64 {
        self.frames.len() as u64
    }

    pub(in crate::physical_runtime::record_serving) const fn total_frame_bytes(&self) -> u64 {
        self.total_frame_bytes
    }

    pub(in crate::physical_runtime::record_serving) fn declarations(
        &self,
    ) -> &[CandidateFrameDeclaration] {
        &self.frames
    }

    fn declaration(&self, index: usize) -> Option<CandidateFrameDeclaration> {
        self.frames.get(index).copied()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime::record_serving) enum CandidateFrameRole {
    InlinePage,
    ExtentChunk,
    ManifestBlock,
    RootManifest,
    CatalogCandidate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime::record_serving) struct CandidateFrameCoordinate {
    artifact: RecordArtifactFile,
    offset: u64,
}

impl CandidateFrameCoordinate {
    pub(in crate::physical_runtime::record_serving) const fn new(
        artifact: RecordArtifactFile,
        offset: u64,
    ) -> Self {
        Self { artifact, offset }
    }

    pub(in crate::physical_runtime::record_serving) const fn artifact(self) -> RecordArtifactFile {
        self.artifact
    }

    pub(in crate::physical_runtime::record_serving) const fn offset(self) -> u64 {
        self.offset
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::physical_runtime::record_serving) struct CandidateFrame {
    role: CandidateFrameRole,
    coordinate: CandidateFrameCoordinate,
    bytes: Vec<u8>,
    checksum: u32,
}

impl CandidateFrame {
    pub(in crate::physical_runtime::record_serving) fn new(
        role: CandidateFrameRole,
        coordinate: CandidateFrameCoordinate,
        bytes: Vec<u8>,
    ) -> Self {
        let checksum = worth_store_physical_format::durable_artifact_checksum(&bytes);
        Self {
            role,
            coordinate,
            bytes,
            checksum,
        }
    }

    pub(in crate::physical_runtime::record_serving) const fn role(&self) -> CandidateFrameRole {
        self.role
    }

    pub(in crate::physical_runtime::record_serving) const fn coordinate(
        &self,
    ) -> CandidateFrameCoordinate {
        self.coordinate
    }

    pub(in crate::physical_runtime::record_serving) fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub(in crate::physical_runtime::record_serving) fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub(in crate::physical_runtime::record_serving) const fn checksum(&self) -> u32 {
        self.checksum
    }
}

pub(in crate::physical_runtime::record_serving) trait CandidateFrameResidencySession {
    fn retain(
        &mut self,
        frame: CandidateFrame,
    ) -> Result<Box<dyn ResidentCandidateFrame>, RecordAppendDenial>;

    fn prepare_catalog_cutover(
        &mut self,
        target: CandidateFrameCoordinate,
        length: u32,
    ) -> Result<(), RecordAppendDenial>;
}

pub(in crate::physical_runtime::record_serving) trait ResidentCandidateFrame {
    fn role(&self) -> CandidateFrameRole;
    fn coordinate(&self) -> CandidateFrameCoordinate;
    fn bytes(&self) -> &[u8];

    fn publish_clean(
        self: Box<Self>,
        physical: &CandidateFramePhysicalWrite,
    ) -> Result<CandidateFrameWriteCompletion, RecordAppendDenial>;
}

pub(in crate::physical_runtime::record_serving) trait CandidateFramePublicationPort {
    fn begin(
        &self,
        candidate: &CandidateFrameSet,
    ) -> Result<Box<dyn CandidateFrameResidencySession>, RecordAppendDenial>;
}

pub(in crate::physical_runtime::record_serving) struct StoreCandidateFramePublicationSession {
    declaration: CandidateFrameSet,
    resident_frames: u64,
    resident_bytes: u64,
    next_declaration: usize,
    residency: Box<dyn CandidateFrameResidencySession>,
}

impl StoreCandidateFramePublicationSession {
    pub(in crate::physical_runtime::record_serving) fn begin(
        port: &(dyn CandidateFramePublicationPort + Send + Sync),
        declaration: CandidateFrameSet,
    ) -> Result<Self, RecordAppendDenial> {
        let residency = port.begin(&declaration)?;
        Ok(Self {
            declaration,
            resident_frames: 0,
            resident_bytes: 0,
            next_declaration: 0,
            residency,
        })
    }

    pub(in crate::physical_runtime::record_serving) fn write_frame(
        &mut self,
        frame: CandidateFrame,
        store_write: &mut dyn FnMut(
            &[u8],
        )
            -> Result<CandidateFramePhysicalWrite, ArtifactTreeFailure>,
    ) -> Result<CandidateFrameWriteCompletion, CandidateFrameWriteFailure> {
        let frame_bytes = frame.bytes().len() as u64;
        let frame_role = frame.role();
        let frame_coordinate = frame.coordinate();
        let frame_checksum = frame.checksum();
        if !coordinate_matches_role(frame.role(), frame.coordinate().artifact()) {
            return Err(CandidateFrameWriteFailure::Contract(
                CandidateFrameContractViolation::CoordinateRoleMismatch,
            ));
        }
        let Some(expected) = self.declaration.declaration(self.next_declaration) else {
            return Err(CandidateFrameWriteFailure::Contract(
                CandidateFrameContractViolation::FrameCountExceedsDeclaration,
            ));
        };
        if expected.role != frame_role
            || expected.coordinate != frame_coordinate
            || u64::from(expected.length) != frame_bytes
        {
            return Err(CandidateFrameWriteFailure::Contract(
                CandidateFrameContractViolation::UnexpectedFrame,
            ));
        }
        let next_frames = self.resident_frames.saturating_add(1);
        let next_bytes = self.resident_bytes.saturating_add(frame_bytes);
        if next_bytes > self.declaration.total_frame_bytes() {
            return Err(CandidateFrameWriteFailure::Contract(
                CandidateFrameContractViolation::FrameBytesExceedDeclaration,
            ));
        }

        let resident = self
            .residency
            .retain(frame)
            .map_err(CandidateFrameWriteFailure::Residency)?;
        if resident.bytes().len() as u64 != frame_bytes
            || resident.role() != frame_role
            || resident.coordinate() != frame_coordinate
        {
            return Err(CandidateFrameWriteFailure::Contract(
                CandidateFrameContractViolation::RetainedFrameMismatch,
            ));
        }
        if worth_store_physical_format::durable_artifact_checksum(resident.bytes())
            != frame_checksum
        {
            return Err(CandidateFrameWriteFailure::Contract(
                CandidateFrameContractViolation::RetainedFrameBytesChanged,
            ));
        }
        let physical =
            store_write(resident.bytes()).map_err(CandidateFrameWriteFailure::Backend)?;
        let completion = resident
            .publish_clean(&physical)
            .map_err(CandidateFrameWriteFailure::Residency)?;
        if completion.frame_bytes() != frame_bytes {
            return Err(CandidateFrameWriteFailure::Contract(
                CandidateFrameContractViolation::FrameCompletionMismatch,
            ));
        }
        self.resident_frames = next_frames;
        self.resident_bytes = next_bytes;
        self.next_declaration += 1;
        Ok(completion)
    }

    pub(in crate::physical_runtime::record_serving) fn require_complete(
        &self,
    ) -> Result<(), CandidateFrameContractViolation> {
        if self.resident_frames != self.declaration.frame_count()
            || self.resident_bytes != self.declaration.total_frame_bytes()
        {
            return Err(CandidateFrameContractViolation::IncompleteFrameSet);
        }
        Ok(())
    }

    pub(in crate::physical_runtime::record_serving) fn prepare_catalog_cutover(
        &mut self,
    ) -> Result<(), CandidateFrameContractViolation> {
        let Some(candidate) = self.declaration.frames.last().copied() else {
            return Err(CandidateFrameContractViolation::IncompleteFrameSet);
        };
        if candidate.role != CandidateFrameRole::CatalogCandidate
            || self.next_declaration != self.declaration.frames.len()
        {
            return Err(CandidateFrameContractViolation::IncompleteFrameSet);
        }
        self.residency
            .prepare_catalog_cutover(
                CandidateFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 0),
                candidate.length,
            )
            .map_err(|_| CandidateFrameContractViolation::CatalogResidencyInvalidationFailed)
    }
}

const fn coordinate_matches_role(role: CandidateFrameRole, artifact: RecordArtifactFile) -> bool {
    matches!(
        (role, artifact),
        (
            CandidateFrameRole::InlinePage,
            RecordArtifactFile::Segment { .. }
        ) | (
            CandidateFrameRole::ExtentChunk,
            RecordArtifactFile::Extent { .. }
        ) | (
            CandidateFrameRole::ManifestBlock,
            RecordArtifactFile::RootRoutingBlock { .. }
                | RecordArtifactFile::SegmentManifest { .. }
                | RecordArtifactFile::SegmentMembershipBlock { .. }
                | RecordArtifactFile::ExtentManifest { .. }
                | RecordArtifactFile::FreeSpaceManifest { .. }
                | RecordArtifactFile::FreeSpaceMembershipBlock { .. }
        ) | (
            CandidateFrameRole::RootManifest,
            RecordArtifactFile::RootManifest { .. }
        ) | (
            CandidateFrameRole::CatalogCandidate,
            RecordArtifactFile::CatalogCandidate { .. }
        )
    )
}

#[cfg(test)]
#[path = "candidate_frame_residency/tests.rs"]
mod tests;
