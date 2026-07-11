#![forbid(unsafe_code)]

mod identity;
mod image;
mod layout_projection;
mod read;
mod restore;

pub use identity::SnapshotId;
pub use image::{
    snapshot_semantic_authority, PublishedSnapshotHandle, SnapshotImageBundle,
    SnapshotSemanticAuthority,
};
pub use layout_projection::{
    SnapshotLayoutAccessDenial, SnapshotLayoutAccessDenialKind, SnapshotLayoutReport,
    SnapshotLayoutSupportEstimate,
};
pub use read::{SnapshotReadRequest, SnapshotReadResult};
pub use restore::SnapshotRestorePlan;

pub fn reject_snapshot_bundle_layout_authority(
    bundle: &SnapshotImageBundle,
) -> Result<(), SnapshotLayoutAccessDenial> {
    layout_projection::reject_snapshot_bundle_layout_authority(bundle)
}
