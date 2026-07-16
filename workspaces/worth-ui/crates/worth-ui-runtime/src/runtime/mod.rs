//! Runtime lifecycle lanes: launch → replacement → planning → activation → execution → host observation.

mod activation;
pub(crate) use activation::committed_allocation_attempt::{
    UiCommittedAllocationActivationAttempt, UiCommittedAllocationActivationIdentity,
};
mod active;
mod allocation_frame_dispatch;
mod allocation_receipt;
#[path = "compat_modules.rs"]
mod compat_modules;
mod drag_resize;
mod execution;
pub(crate) mod exports;
mod host_observation;
mod invalidation_narrowing;
pub(crate) use invalidation_narrowing::{
    UiAdmittedScrollInvalidationBinding, UiAllocationInvalidationAuthority,
};
mod launch;
mod measurement;
mod planning;
mod portal_anchored_allocation;
pub mod replacement;
pub(crate) mod scroll_owned_allocation;
mod source_ingress;
mod stream_policy;
mod viewport_resize;

pub use drag_resize::*;
pub use scroll_owned_allocation::*;

pub(crate) use allocation_receipt::project_allocation_preview;
pub(crate) use allocation_receipt::UiCommittedScrollActivationSource;
pub use compat_modules::*;
pub use exports::*;

#[cfg(test)]
pub(crate) use replacement::file_rust_replacement_parity;
#[cfg(test)]
pub(crate) use replacement::state_inventory::WorthUiTransientInteractionAdmission;

#[cfg(test)]
pub(crate) mod tests;
