use worth_store_physical_format::{
    decode_extent_chunk, inspect_inline_page, ExtentChunkCoordinate, PageGenerationCell,
    PhysicalRecordFormatDeclaration, RecordArtifactFile, RecordFrameCoordinate,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum PhysicalDataFrameKind {
    InlinePage = 1,
    ExtentChunk = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalDataFrameSubject {
    InlinePage(PageGenerationCell),
    ExtentChunk(ExtentChunkCoordinate),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalDataFrameIdentity {
    subject: PhysicalDataFrameSubject,
    coordinate: RecordFrameCoordinate,
}

impl PhysicalDataFrameIdentity {
    pub(in crate::physical_runtime) fn inline_page(
        page: PageGenerationCell,
        artifact: RecordArtifactFile,
        offset: u64,
        length: u32,
    ) -> Option<Self> {
        let RecordArtifactFile::Segment { segment, .. } = artifact else {
            return None;
        };
        (segment == page.segment_id().get()).then_some(Self {
            subject: PhysicalDataFrameSubject::InlinePage(page),
            coordinate: RecordFrameCoordinate::new(artifact, offset, length)?,
        })
    }

    pub(in crate::physical_runtime) fn extent_chunk(
        chunk: ExtentChunkCoordinate,
        artifact: RecordArtifactFile,
        offset: u64,
        length: u32,
    ) -> Option<Self> {
        let RecordArtifactFile::Extent { extent, generation } = artifact else {
            return None;
        };
        let cell = chunk.extent_cell();
        (extent == cell.extent_id().get() && generation == cell.generation().get()).then_some(
            Self {
                subject: PhysicalDataFrameSubject::ExtentChunk(chunk),
                coordinate: RecordFrameCoordinate::new(artifact, offset, length)?,
            },
        )
    }

    pub const fn kind(self) -> PhysicalDataFrameKind {
        match self.subject {
            PhysicalDataFrameSubject::InlinePage(_) => PhysicalDataFrameKind::InlinePage,
            PhysicalDataFrameSubject::ExtentChunk(_) => PhysicalDataFrameKind::ExtentChunk,
        }
    }

    pub const fn subject(self) -> PhysicalDataFrameSubject {
        self.subject
    }

    pub const fn coordinate(self) -> RecordFrameCoordinate {
        self.coordinate
    }

    pub(in crate::physical_runtime) fn admits_bytes(
        self,
        format: PhysicalRecordFormatDeclaration,
        bytes: &[u8],
    ) -> bool {
        if bytes.len() != self.coordinate.length() as usize {
            return false;
        }
        match self.subject {
            PhysicalDataFrameSubject::InlinePage(page) => inspect_inline_page(format, bytes)
                .is_ok_and(|geometry| geometry.page_cell() == page),
            PhysicalDataFrameSubject::ExtentChunk(chunk) => decode_extent_chunk(bytes, chunk)
                .is_ok_and(|(_, found_format)| found_format == format),
        }
    }

    pub(in crate::physical_runtime) fn is_exact_successor_of(self, source: Self) -> bool {
        match (source.subject, self.subject) {
            (
                PhysicalDataFrameSubject::InlinePage(source_page),
                PhysicalDataFrameSubject::InlinePage(target_page),
            ) => {
                source_page.segment_id() == target_page.segment_id()
                    && source_page.page_id() == target_page.page_id()
                    && source_page.generation().get().checked_add(1)
                        == Some(target_page.generation().get())
                    && self.coordinate.length() == source.coordinate.length()
                    && artifact_is_exact_successor(
                        source.coordinate.artifact(),
                        self.coordinate.artifact(),
                    )
            }
            _ => false,
        }
    }

    pub(in crate::physical_runtime) fn write_canonical(self, target: &mut Vec<u8>) {
        write_subject(self.subject, target);
        write_artifact(self.coordinate.artifact(), target);
        target.extend_from_slice(&self.coordinate.offset().to_le_bytes());
        target.extend_from_slice(&self.coordinate.length().to_le_bytes());
    }
}

fn artifact_is_exact_successor(source: RecordArtifactFile, target: RecordArtifactFile) -> bool {
    match (source, target) {
        (
            RecordArtifactFile::Segment {
                segment: source_segment,
                generation: source_generation,
            },
            RecordArtifactFile::Segment {
                segment: target_segment,
                generation: target_generation,
            },
        ) => {
            source_segment == target_segment
                && source_generation.checked_add(1) == Some(target_generation)
        }
        _ => false,
    }
}

fn write_subject(subject: PhysicalDataFrameSubject, target: &mut Vec<u8>) {
    match subject {
        PhysicalDataFrameSubject::InlinePage(page) => {
            target.push(PhysicalDataFrameKind::InlinePage as u8);
            target.extend_from_slice(&page.segment_id().get().to_le_bytes());
            target.extend_from_slice(&page.page_id().get().to_le_bytes());
            target.extend_from_slice(&page.generation().get().to_le_bytes());
        }
        PhysicalDataFrameSubject::ExtentChunk(chunk) => {
            target.push(PhysicalDataFrameKind::ExtentChunk as u8);
            let record = chunk.record();
            target.extend_from_slice(&record.allocation_epoch());
            target.extend_from_slice(&record.ordinal().to_le_bytes());
            target.extend_from_slice(&chunk.extent_cell().extent_id().get().to_le_bytes());
            target.extend_from_slice(&chunk.extent_cell().generation().get().to_le_bytes());
            target.extend_from_slice(&chunk.logical_bytes().to_le_bytes());
            target.extend_from_slice(&chunk.logical_offset().to_le_bytes());
            target.extend_from_slice(&chunk.ordinal().to_le_bytes());
        }
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
