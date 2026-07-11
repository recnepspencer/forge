#![forbid(unsafe_code)]

mod layout_access;

pub use layout_access::{
    SnapshotLayoutAccessDenial, SnapshotLayoutAccessDenialKind, SnapshotLayoutReport,
    SnapshotLayoutSupportEstimate,
};

use forge_store_contracts::StableArtifactId;
pub use forge_store_layout_indexes::layout_strategy_admission::AdmittedSnapshotLayoutRule;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SnapshotSemanticAuthority;

pub const fn snapshot_semantic_authority() -> SnapshotSemanticAuthority {
    SnapshotSemanticAuthority
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotId(StableArtifactId);

impl SnapshotId {
    pub const fn from_artifact_id(id: StableArtifactId) -> Self {
        Self(id)
    }

    pub fn artifact_id(&self) -> StableArtifactId {
        self.0.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotImageBundle {
    snapshot_id: SnapshotId,
    image_digest: String,
    declared_page_count: u32,
}

impl SnapshotImageBundle {
    pub fn new(snapshot_id: SnapshotId, image_digest: impl Into<String>, declared_page_count: u32) -> Self {
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
    pub(crate) fn new(
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
        layout_access::admit_snapshot_image_support(self, request)
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

pub fn reject_snapshot_bundle_layout_authority(
    bundle: &SnapshotImageBundle,
) -> Result<(), SnapshotLayoutAccessDenial> {
    layout_access::reject_snapshot_bundle_layout_authority(bundle)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotReadRequest {
    snapshot_id: SnapshotId,
    requested_page_count: u32,
}

impl SnapshotReadRequest {
    pub const fn new(snapshot_id: SnapshotId, requested_page_count: u32) -> Self {
        Self {
            snapshot_id,
            requested_page_count,
        }
    }

    pub const fn snapshot_id(&self) -> &SnapshotId {
        &self.snapshot_id
    }

    pub const fn requested_page_count(&self) -> u32 {
        self.requested_page_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotReadResult {
    snapshot_id: SnapshotId,
    returned_page_count: u32,
}

impl SnapshotReadResult {
    pub const fn new(snapshot_id: SnapshotId, returned_page_count: u32) -> Self {
        Self {
            snapshot_id,
            returned_page_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotRestorePlan {
    snapshot_id: SnapshotId,
    restore_frontier_pages: u32,
}

impl SnapshotRestorePlan {
    pub const fn new(snapshot_id: SnapshotId, restore_frontier_pages: u32) -> Self {
        Self {
            snapshot_id,
            restore_frontier_pages,
        }
    }
}
