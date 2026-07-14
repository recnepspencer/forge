use crate::{ChunkEpoch, ExtentEpoch, ManifestEpoch, PageEpoch, RootEpoch, SegmentEpoch};

use super::PhysicalLatchClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalLatchKey {
    kind: PhysicalLatchKeyKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum PhysicalLatchKeyKind {
    Root {
        root_epoch: u64,
    },
    Manifest {
        root_epoch: u64,
        manifest_epoch: u64,
    },
    Segment {
        root_epoch: u64,
        segment_epoch: u64,
    },
    Extent {
        root_epoch: u64,
        extent_epoch: u64,
    },
    Page {
        root_epoch: u64,
        page_epoch: u64,
    },
    FutureChunk {
        root_epoch: u64,
        chunk_epoch: u64,
    },
}

impl PhysicalLatchKey {
    pub const fn root(root_epoch: RootEpoch) -> Self {
        Self {
            kind: PhysicalLatchKeyKind::Root {
                root_epoch: root_epoch.get(),
            },
        }
    }

    pub const fn manifest(root_epoch: RootEpoch, manifest_epoch: ManifestEpoch) -> Self {
        Self {
            kind: PhysicalLatchKeyKind::Manifest {
                root_epoch: root_epoch.get(),
                manifest_epoch: manifest_epoch.get(),
            },
        }
    }

    pub const fn segment(root_epoch: RootEpoch, segment_epoch: SegmentEpoch) -> Self {
        Self {
            kind: PhysicalLatchKeyKind::Segment {
                root_epoch: root_epoch.get(),
                segment_epoch: segment_epoch.get(),
            },
        }
    }

    pub const fn extent(root_epoch: RootEpoch, extent_epoch: ExtentEpoch) -> Self {
        Self {
            kind: PhysicalLatchKeyKind::Extent {
                root_epoch: root_epoch.get(),
                extent_epoch: extent_epoch.get(),
            },
        }
    }

    pub const fn page(root_epoch: RootEpoch, page_epoch: PageEpoch) -> Self {
        Self {
            kind: PhysicalLatchKeyKind::Page {
                root_epoch: root_epoch.get(),
                page_epoch: page_epoch.get(),
            },
        }
    }

    pub const fn future_chunk(root_epoch: RootEpoch, chunk_epoch: ChunkEpoch) -> Self {
        Self {
            kind: PhysicalLatchKeyKind::FutureChunk {
                root_epoch: root_epoch.get(),
                chunk_epoch: chunk_epoch.get(),
            },
        }
    }

    pub const fn class(self) -> PhysicalLatchClass {
        match self.kind {
            PhysicalLatchKeyKind::Root { .. } => PhysicalLatchClass::Root,
            PhysicalLatchKeyKind::Manifest { .. } => PhysicalLatchClass::Manifest,
            PhysicalLatchKeyKind::Segment { .. } => PhysicalLatchClass::Segment,
            PhysicalLatchKeyKind::Extent { .. } => PhysicalLatchClass::Extent,
            PhysicalLatchKeyKind::Page { .. } => PhysicalLatchClass::Page,
            PhysicalLatchKeyKind::FutureChunk { .. } => PhysicalLatchClass::FutureChunk,
        }
    }

    pub const fn root_scope_id(self) -> u64 {
        match self.kind {
            PhysicalLatchKeyKind::Root { root_epoch }
            | PhysicalLatchKeyKind::Manifest { root_epoch, .. }
            | PhysicalLatchKeyKind::Segment { root_epoch, .. }
            | PhysicalLatchKeyKind::Extent { root_epoch, .. }
            | PhysicalLatchKeyKind::Page { root_epoch, .. }
            | PhysicalLatchKeyKind::FutureChunk { root_epoch, .. } => root_epoch,
        }
    }

    pub(crate) const fn canonical_order_tuple(self) -> (u64, u8, u64, u64) {
        match self.kind {
            PhysicalLatchKeyKind::Root { root_epoch } => (root_epoch, 0, 0, 0),
            PhysicalLatchKeyKind::Manifest {
                root_epoch,
                manifest_epoch,
            } => (root_epoch, 1, manifest_epoch, 0),
            PhysicalLatchKeyKind::Segment {
                root_epoch,
                segment_epoch,
            } => (root_epoch, 2, segment_epoch, 0),
            PhysicalLatchKeyKind::Extent {
                root_epoch,
                extent_epoch,
            } => (root_epoch, 3, extent_epoch, 0),
            PhysicalLatchKeyKind::Page {
                root_epoch,
                page_epoch,
            } => (root_epoch, 4, page_epoch, 0),
            PhysicalLatchKeyKind::FutureChunk {
                root_epoch,
                chunk_epoch,
            } => (root_epoch, 5, chunk_epoch, 0),
        }
    }
}
