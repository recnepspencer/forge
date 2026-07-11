use crate::{SnapshotId, SnapshotLayoutAccessDenial, SnapshotLayoutReport, SnapshotReadRequest};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SnapshotSemanticAuthority;

pub const fn snapshot_semantic_authority() -> SnapshotSemanticAuthority {
    SnapshotSemanticAuthority
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotImageBundle {
    snapshot_id: SnapshotId,
    image_digest: String,
    declared_page_count: u32,
}

impl SnapshotImageBundle {
    pub fn new(
        snapshot_id: SnapshotId,
        image_digest: impl Into<String>,
        declared_page_count: u32,
    ) -> Self {
        Self {
            snapshot_id,
            image_digest: image_digest.into(),
            declared_page_count,
        }
    }

    pub const fn snapshot_id(&self) -> &SnapshotId {
        &self.snapshot_id
    }
    pub fn image_digest(&self) -> &str {
        &self.image_digest
    }
    pub const fn declared_page_count(&self) -> u32 {
        self.declared_page_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedSnapshotHandle {
    snapshot_id: SnapshotId,
    image_digest: String,
    declared_page_count: u32,
}

impl PublishedSnapshotHandle {
    fn new(
        snapshot_id: SnapshotId,
        image_digest: impl Into<String>,
        declared_page_count: u32,
    ) -> Self {
        Self {
            snapshot_id,
            image_digest: image_digest.into(),
            declared_page_count,
        }
    }

    pub const fn snapshot_id(&self) -> &SnapshotId {
        &self.snapshot_id
    }
    pub fn image_digest(&self) -> &str {
        &self.image_digest
    }
    pub const fn declared_page_count(&self) -> u32 {
        self.declared_page_count
    }

    pub fn admit_layout_support(
        &self,
        request: &SnapshotReadRequest,
    ) -> Result<SnapshotLayoutReport, SnapshotLayoutAccessDenial> {
        crate::layout_access::admit_snapshot_image_support(self, request)
    }
}

impl SnapshotSemanticAuthority {
    pub fn publish_snapshot_image(
        self,
        snapshot_id: SnapshotId,
        image_digest: impl Into<String>,
        declared_page_count: u32,
    ) -> PublishedSnapshotHandle {
        PublishedSnapshotHandle::new(snapshot_id, image_digest, declared_page_count)
    }
}
