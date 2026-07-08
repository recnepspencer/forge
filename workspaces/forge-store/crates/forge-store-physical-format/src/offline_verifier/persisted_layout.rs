use crate::{ExtentGenerationCell, PageGenerationCell, PhysicalReferenceAdmissionWitness};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PersistedPhysicalLayout {
    root_manifest_candidates: Vec<Vec<u8>>,
    segment_manifest: Vec<u8>,
    extent_manifest: Vec<u8>,
    free_space_map: Vec<u8>,
    pages: Vec<PersistedPageBytes>,
    extents: Vec<PersistedExtentBytes>,
    backend_residue: Vec<PhysicalReferenceAdmissionWitness>,
}

impl PersistedPhysicalLayout {
    pub fn builder() -> PersistedPhysicalLayoutBuilder {
        PersistedPhysicalLayoutBuilder::default()
    }

    pub fn root_manifest_candidates(&self) -> &[Vec<u8>] {
        &self.root_manifest_candidates
    }

    pub fn segment_manifest(&self) -> &[u8] {
        &self.segment_manifest
    }

    pub fn extent_manifest(&self) -> &[u8] {
        &self.extent_manifest
    }

    pub fn free_space_map(&self) -> &[u8] {
        &self.free_space_map
    }

    pub fn pages(&self) -> &[PersistedPageBytes] {
        &self.pages
    }

    pub fn extents(&self) -> &[PersistedExtentBytes] {
        &self.extents
    }

    pub fn backend_residue(&self) -> &[PhysicalReferenceAdmissionWitness] {
        &self.backend_residue
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PersistedPhysicalLayoutBuilder {
    layout: PersistedPhysicalLayout,
}

impl PersistedPhysicalLayoutBuilder {
    pub fn root_manifest(mut self, bytes: impl Into<Vec<u8>>) -> Self {
        self.layout.root_manifest_candidates.push(bytes.into());
        self
    }

    pub fn segment_manifest(mut self, bytes: impl Into<Vec<u8>>) -> Self {
        self.layout.segment_manifest = bytes.into();
        self
    }

    pub fn extent_manifest(mut self, bytes: impl Into<Vec<u8>>) -> Self {
        self.layout.extent_manifest = bytes.into();
        self
    }

    pub fn free_space_map(mut self, bytes: impl Into<Vec<u8>>) -> Self {
        self.layout.free_space_map = bytes.into();
        self
    }

    pub fn page(mut self, page: PersistedPageBytes) -> Self {
        self.layout.pages.push(page);
        self
    }

    pub fn extent(mut self, extent: PersistedExtentBytes) -> Self {
        self.layout.extents.push(extent);
        self
    }

    pub fn backend_residue_reference(
        mut self,
        admission: PhysicalReferenceAdmissionWitness,
    ) -> Self {
        self.layout.backend_residue.push(admission);
        self
    }

    pub fn build(self) -> PersistedPhysicalLayout {
        self.layout
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistedPageBytes {
    cell: PageGenerationCell,
    bytes: Vec<u8>,
}

impl PersistedPageBytes {
    pub fn new(cell: PageGenerationCell, bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            cell,
            bytes: bytes.into(),
        }
    }

    pub const fn cell(&self) -> PageGenerationCell {
        self.cell
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistedExtentBytes {
    cell: ExtentGenerationCell,
    bytes: Vec<u8>,
}

impl PersistedExtentBytes {
    pub fn new(cell: ExtentGenerationCell, bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            cell,
            bytes: bytes.into(),
        }
    }

    pub const fn cell(&self) -> ExtentGenerationCell {
        self.cell
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}
