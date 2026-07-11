#![forbid(unsafe_code)]

mod identity;
mod image;
mod layout_access;
mod read;
mod restore;

pub use forge_store_layout_indexes::layout_strategy_admission::AdmittedSnapshotLayoutRule;
pub use identity::SnapshotId;
pub use image::{
    snapshot_semantic_authority, PublishedSnapshotHandle, SnapshotImageBundle,
    SnapshotSemanticAuthority,
};
pub use layout_access::{
    SnapshotLayoutAccessDenial, SnapshotLayoutAccessDenialKind, SnapshotLayoutReport,
    SnapshotLayoutSupportEstimate,
};
pub use read::{SnapshotReadRequest, SnapshotReadResult};
pub use restore::SnapshotRestorePlan;

pub fn reject_snapshot_bundle_layout_authority(
    bundle: &SnapshotImageBundle,
) -> Result<(), SnapshotLayoutAccessDenial> {
    layout_access::reject_snapshot_bundle_layout_authority(bundle)
}
