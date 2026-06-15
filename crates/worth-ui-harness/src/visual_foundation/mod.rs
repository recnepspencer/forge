mod app_registration;
mod command_visual_projections;
mod icon_catalog;
mod runtime_outcome_visuals;
mod visual_foundation_bundle;
mod visual_foundation_denial;
mod visual_foundation_receipt;

pub use app_registration::HarnessVisualFoundationRegistration;
pub use command_visual_projections::HarnessCommandProjectionVisualRole;
pub use runtime_outcome_visuals::HarnessRuntimeOutcomeVisualRole;
pub use visual_foundation_bundle::{
    HarnessVisualFoundationBundle, PreparedHarnessVisualFoundation,
};
pub use visual_foundation_denial::HarnessVisualFoundationDenial;
pub use visual_foundation_receipt::HarnessVisualFoundationReceipt;

pub(crate) use command_visual_projections::harness_command_visual_projections;
pub(crate) use icon_catalog::harness_icon_descriptors;
pub(crate) use runtime_outcome_visuals::harness_runtime_outcome_projections;
