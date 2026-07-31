use sha2::{Digest, Sha256};
use worth_store_physical_format::{PhysicalPageLsn, RecordArtifactFile, RecordFrameCoordinate};

const ABSENT_PRIOR_IMAGE_DOMAIN: &[u8] = b"store.physical.data.certified-absent-prior-image.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum PhysicalDataFrameKind {
    InlinePage = 1,
    ExtentChunk = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalDataFrameIdentity {
    kind: PhysicalDataFrameKind,
    coordinate: RecordFrameCoordinate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CertifiedPriorPageBasis {
    target: PhysicalDataFrameIdentity,
    page_lsn: PhysicalPageLsn,
    payload_digest: [u8; 32],
}

impl PhysicalDataFrameIdentity {
    pub(in crate::physical_runtime) fn new(
        kind: PhysicalDataFrameKind,
        artifact: RecordArtifactFile,
        offset: u64,
        length: u32,
    ) -> Option<Self> {
        let artifact_matches = matches!(
            (kind, artifact),
            (
                PhysicalDataFrameKind::InlinePage,
                RecordArtifactFile::Segment { .. }
            ) | (
                PhysicalDataFrameKind::ExtentChunk,
                RecordArtifactFile::Extent { .. }
            )
        );
        if !artifact_matches {
            return None;
        }
        Some(Self {
            kind,
            coordinate: RecordFrameCoordinate::new(artifact, offset, length)?,
        })
    }

    pub const fn kind(self) -> PhysicalDataFrameKind {
        self.kind
    }

    pub const fn coordinate(self) -> RecordFrameCoordinate {
        self.coordinate
    }

    pub(in crate::physical_runtime) fn write_canonical(self, target: &mut Vec<u8>) {
        target.push(self.kind as u8);
        write_artifact(self.coordinate.artifact(), target);
        target.extend_from_slice(&self.coordinate.offset().to_le_bytes());
        target.extend_from_slice(&self.coordinate.length().to_le_bytes());
    }
}

impl CertifiedPriorPageBasis {
    pub(in crate::physical_runtime) fn for_unmaterialized_target(
        target: PhysicalDataFrameIdentity,
    ) -> Self {
        let mut digest = Sha256::new();
        digest.update((ABSENT_PRIOR_IMAGE_DOMAIN.len() as u64).to_le_bytes());
        digest.update(ABSENT_PRIOR_IMAGE_DOMAIN);
        let mut identity = Vec::with_capacity(32);
        target.write_canonical(&mut identity);
        digest.update((identity.len() as u64).to_le_bytes());
        digest.update(identity);
        Self {
            target,
            page_lsn: PhysicalPageLsn::GENESIS,
            payload_digest: digest.finalize().into(),
        }
    }

    pub const fn target(self) -> PhysicalDataFrameIdentity {
        self.target
    }

    pub const fn page_lsn(self) -> PhysicalPageLsn {
        self.page_lsn
    }

    pub const fn payload_digest(self) -> [u8; 32] {
        self.payload_digest
    }
}

fn write_artifact(artifact: RecordArtifactFile, target: &mut Vec<u8>) {
    match artifact {
        RecordArtifactFile::BootstrapCatalog => target.push(1),
        RecordArtifactFile::CatalogCandidate { publication } => {
            target.push(2);
            target.extend_from_slice(&publication.to_le_bytes());
        }
        RecordArtifactFile::RootManifest { generation } => {
            target.push(3);
            target.extend_from_slice(&generation.to_le_bytes());
        }
        RecordArtifactFile::RootRoutingBlock { generation, block } => {
            target.push(4);
            target.extend_from_slice(&generation.to_le_bytes());
            target.extend_from_slice(&block.to_le_bytes());
        }
        RecordArtifactFile::Segment {
            segment,
            generation,
        } => {
            target.push(5);
            target.extend_from_slice(&segment.to_le_bytes());
            target.extend_from_slice(&generation.to_le_bytes());
        }
        RecordArtifactFile::SegmentManifest {
            segment,
            generation,
        } => {
            target.push(6);
            target.extend_from_slice(&segment.to_le_bytes());
            target.extend_from_slice(&generation.to_le_bytes());
        }
        RecordArtifactFile::SegmentMembershipBlock { generation, block } => {
            target.push(7);
            target.extend_from_slice(&generation.to_le_bytes());
            target.extend_from_slice(&block.to_le_bytes());
        }
        RecordArtifactFile::Extent { extent, generation } => {
            target.push(8);
            target.extend_from_slice(&extent.to_le_bytes());
            target.extend_from_slice(&generation.to_le_bytes());
        }
        RecordArtifactFile::ExtentManifest { extent, generation } => {
            target.push(9);
            target.extend_from_slice(&extent.to_le_bytes());
            target.extend_from_slice(&generation.to_le_bytes());
        }
        RecordArtifactFile::FreeSpaceManifest { generation } => {
            target.push(10);
            target.extend_from_slice(&generation.to_le_bytes());
        }
        RecordArtifactFile::FreeSpaceMembershipBlock { generation, block } => {
            target.push(11);
            target.extend_from_slice(&generation.to_le_bytes());
            target.extend_from_slice(&block.to_le_bytes());
        }
    }
}
