use super::{
    ChunkEpoch, EpochComparisonScope, ExtentEpoch, ManifestEpoch, PageEpoch,
    PhysicalEpochDriftKind, PhysicalEpochFreshness, RootEpoch, SegmentEpoch,
};

#[derive(Debug, Clone, Copy)]
pub struct PhysicalEpochVector {
    scope: EpochComparisonScope,
    root: RootEpoch,
    manifest: ManifestEpoch,
    segment: Option<SegmentEpoch>,
    extent: Option<ExtentEpoch>,
    page: Option<PageEpoch>,
    chunk: Option<ChunkEpoch>,
}

pub struct PhysicalEpochVectorBuilder {
    scope: EpochComparisonScope,
    root: Option<RootEpoch>,
    manifest: Option<ManifestEpoch>,
    segment: Option<SegmentEpoch>,
    extent: Option<ExtentEpoch>,
    page: Option<PageEpoch>,
    chunk: Option<ChunkEpoch>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalEpochVectorDenial {
    MissingRootEpoch,
    MissingManifestEpoch,
    ScopeMismatch,
}

impl PhysicalEpochVector {
    pub const fn for_scope(scope: EpochComparisonScope) -> PhysicalEpochVectorBuilder {
        PhysicalEpochVectorBuilder {
            scope,
            root: None,
            manifest: None,
            segment: None,
            extent: None,
            page: None,
            chunk: None,
        }
    }

    pub const fn scope(self) -> EpochComparisonScope {
        self.scope
    }

    pub const fn root_epoch(self) -> RootEpoch {
        self.root
    }

    pub const fn manifest_epoch(self) -> ManifestEpoch {
        self.manifest
    }

    pub const fn page_epoch(self) -> Option<PageEpoch> {
        self.page
    }

    pub const fn segment_epoch(self) -> Option<SegmentEpoch> {
        self.segment
    }

    pub const fn extent_epoch(self) -> Option<ExtentEpoch> {
        self.extent
    }

    pub const fn chunk_epoch(self) -> Option<ChunkEpoch> {
        self.chunk
    }

    pub fn compare_against(self, observed: Self) -> PhysicalEpochFreshness {
        if self.scope.require_same(observed.scope).is_err() {
            return PhysicalEpochFreshness::rebind_required(PhysicalEpochDriftKind::ScopeMismatch);
        }
        if !self.root.has_same_epoch_value(observed.root) {
            return PhysicalEpochFreshness::retry(PhysicalEpochDriftKind::RootEpoch);
        }
        if !self.manifest.has_same_epoch_value(observed.manifest) {
            return PhysicalEpochFreshness::retry(PhysicalEpochDriftKind::ManifestEpoch);
        }
        if !same_optional_epoch(self.segment, observed.segment) {
            return PhysicalEpochFreshness::retry(PhysicalEpochDriftKind::SegmentEpoch);
        }
        if !same_optional_epoch(self.extent, observed.extent) {
            return PhysicalEpochFreshness::retry(PhysicalEpochDriftKind::ExtentEpoch);
        }
        if !same_optional_epoch(self.page, observed.page) {
            return PhysicalEpochFreshness::retry(PhysicalEpochDriftKind::PageEpoch);
        }
        if !same_optional_epoch(self.chunk, observed.chunk) {
            return PhysicalEpochFreshness::rebind_required(PhysicalEpochDriftKind::ChunkEpoch);
        }
        PhysicalEpochFreshness::current()
    }
}

trait ScopedEpochValue {
    fn has_same_scoped_epoch_value(self, other: Self) -> bool;
}

impl ScopedEpochValue for SegmentEpoch {
    fn has_same_scoped_epoch_value(self, other: Self) -> bool {
        self.has_same_epoch_value(other)
    }
}

impl ScopedEpochValue for ExtentEpoch {
    fn has_same_scoped_epoch_value(self, other: Self) -> bool {
        self.has_same_epoch_value(other)
    }
}

impl ScopedEpochValue for PageEpoch {
    fn has_same_scoped_epoch_value(self, other: Self) -> bool {
        self.has_same_epoch_value(other)
    }
}

impl ScopedEpochValue for ChunkEpoch {
    fn has_same_scoped_epoch_value(self, other: Self) -> bool {
        self.has_same_epoch_value(other)
    }
}

fn same_optional_epoch<T: ScopedEpochValue>(expected: Option<T>, observed: Option<T>) -> bool {
    match (expected, observed) {
        (Some(expected), Some(observed)) => expected.has_same_scoped_epoch_value(observed),
        (None, None) => true,
        _ => false,
    }
}

impl PhysicalEpochVectorBuilder {
    pub const fn with_root(mut self, epoch: RootEpoch) -> Self {
        self.root = Some(epoch);
        self
    }

    pub const fn with_manifest(mut self, epoch: ManifestEpoch) -> Self {
        self.manifest = Some(epoch);
        self
    }

    pub const fn with_segment(mut self, epoch: SegmentEpoch) -> Self {
        self.segment = Some(epoch);
        self
    }

    pub const fn with_extent(mut self, epoch: ExtentEpoch) -> Self {
        self.extent = Some(epoch);
        self
    }

    pub const fn with_page(mut self, epoch: PageEpoch) -> Self {
        self.page = Some(epoch);
        self
    }

    pub const fn with_chunk(mut self, epoch: ChunkEpoch) -> Self {
        self.chunk = Some(epoch);
        self
    }

    pub const fn seal(self) -> Result<PhysicalEpochVector, PhysicalEpochVectorDenial> {
        let Some(root) = self.root else {
            return Err(PhysicalEpochVectorDenial::MissingRootEpoch);
        };
        let Some(manifest) = self.manifest else {
            return Err(PhysicalEpochVectorDenial::MissingManifestEpoch);
        };
        Ok(PhysicalEpochVector {
            scope: self.scope,
            root,
            manifest,
            segment: self.segment,
            extent: self.extent,
            page: self.page,
            chunk: self.chunk,
        })
    }
}
