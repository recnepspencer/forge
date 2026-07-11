mod snapshot_family;

pub use snapshot_family::{
    SnapshotLayoutAccessDenial, SnapshotLayoutAccessDenialKind, SnapshotLayoutReport,
    SnapshotLayoutSupportEstimate,
};
pub(crate) use snapshot_family::{
    admit_snapshot_image_support, reject_snapshot_bundle_layout_authority,
};
