mod snapshot;

pub(crate) use snapshot::{admit_snapshot_image_support, reject_snapshot_bundle_layout_authority};
pub use snapshot::{
    SnapshotLayoutAccessDenial, SnapshotLayoutAccessDenialKind, SnapshotLayoutReport,
    SnapshotLayoutSupportEstimate,
};
