mod cost_accounting;
mod denial;
mod frame_storage;
mod geometry;
mod hit_test;
mod intent_posture;
mod lowering;
mod mechanical_role;
mod node_receipt;
mod participation;
mod prepared_projection;
mod semantic_text;
mod static_paint;

pub use denial::UiMountedProjectionDenial;
pub(in crate::mounting) use frame_storage::diagnostic_source::UiMountedDiagnosticSource;
pub(crate) use frame_storage::presentation_sources::compile as compile_presentation_sources;
pub use frame_storage::UiMountedProjectionFrame;
pub(in crate::mounting) use frame_storage::UiMountedSemanticProjection;
pub use node_receipt::UiMountedNodeReceipt;
pub use prepared_projection::UiProjectedMountedFrameCandidate;

pub(crate) use intent_posture::{
    UiIntentPostureCommit, UiIntentPostureObservation, UiIntentPostureTable,
};
pub(crate) use lowering::{
    prepare_projection, UiMountedPreviewProjectionInput, UiMountedProjectionInput,
};
pub(crate) use prepared_projection::{
    UiMountedPresentationDeltaSource, UiPreparedMountedProjection,
};
