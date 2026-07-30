mod cost_accounting;
mod denial;
mod frame_storage;
mod geometry;
mod hit_test;
mod lowering;
mod mechanical_role;
mod node_receipt;
mod participation;
mod prepared_projection;
mod semantic_text;
mod static_paint;

pub use denial::UiMountedProjectionDenial;
pub use frame_storage::UiMountedProjectionFrame;
pub(in crate::mounting) use frame_storage::UiMountedSemanticProjection;
pub use node_receipt::UiMountedNodeReceipt;
pub use prepared_projection::UiProjectedMountedFrameCandidate;

pub(crate) use lowering::{
    prepare_projection, UiMountedPreviewProjectionInput, UiMountedProjectionInput,
};
pub(crate) use prepared_projection::UiPreparedMountedProjection;
