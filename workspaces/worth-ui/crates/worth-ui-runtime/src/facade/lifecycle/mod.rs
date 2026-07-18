//! Facade lifecycle: bootstrap → declaration freeze → graph commit → graph evidence.

mod application_preparation_denial;
mod application_preparation_source;
#[cfg(test)]
mod application_preparation_tests;
mod bootstrap;
mod declaration_freeze;
mod freeze;
mod graph_evidence;
mod runtime_instance_expansion;

pub use crate::lifecycle::{WorthUiRuntimeSupportInventory, RUNTIME_SUPPORT_INVENTORY};
pub use application_preparation_denial::{
    WorthUiApplicationPreparationDenial, WorthUiApplicationPreparationPhase,
};
pub(crate) use application_preparation_source::{
    WorthUiApplicationDeclarationSource, WorthUiApplicationPreparationSource,
};
pub(crate) use bootstrap::WorthUiFacadeLifecycleBootstrap;
pub(crate) use freeze::{prepare_application_authority, prepare_successor_application_authority};
pub(crate) use graph_evidence::build_graph_evidence_indexes;
