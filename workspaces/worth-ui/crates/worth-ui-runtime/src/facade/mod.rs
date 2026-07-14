//! Public Worth UI runtime surfaces ordered by lifecycle capability and authority class.
//!
//! Lifecycle order: entry → lifecycle → registry → runtime_handoff → boundaries → evidence → host → inspection

pub mod admission;
mod app_inspection_closeout;
pub mod declaration;
pub mod entry;
pub mod evidence;
pub mod graph;
pub mod host_observation;
mod inspection;
pub mod inspection_bridge;
mod inspection_observation;
mod inspection_receipt;
pub mod lifecycle;
mod measurement_inspection_evidence;
#[cfg(test)]
mod measurement_inspection_query_denial_tests;
#[cfg(test)]
mod measurement_inspection_test_support;
#[cfg(test)]
mod measurement_inspection_tests;
pub mod obligations;
pub mod query_binding;
pub mod registry;
mod retained_obligation_registry;
pub mod runtime_handoff;

pub(crate) use inspection::foreign_evidence_refs_for_obligation_record;

pub use crate::runtime::exports::*;
pub use entry::{
    CapabilityRegistrationBuilder, WorthUi, WorthUiApp, WorthUiAppBuilder, WorthUiBuilder,
};
pub use lifecycle::{WorthUiRuntimeSupportInventory, RUNTIME_SUPPORT_INVENTORY};
pub use worth_ui_dsl::WorthUiDslPackage;
