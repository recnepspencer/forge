mod cost_accounting;
mod denial;
mod frame_storage;
mod geometry;
mod lowering;
mod mechanical_role;
mod node_receipt;
mod participation;
mod static_paint;

pub use denial::UiMountedProjectionDenial;
pub use frame_storage::UiMountedProjectionFrame;
pub use lowering::UiProjectedMountedFrameCandidate;
pub use node_receipt::UiMountedNodeReceipt;

pub(crate) use lowering::{
    prepare_projection, UiMountedPreviewProjectionInput, UiMountedProjectionInput,
    UiPreparedMountedProjection,
};
