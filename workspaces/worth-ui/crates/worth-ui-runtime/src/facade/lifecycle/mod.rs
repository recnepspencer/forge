//! Facade lifecycle: bootstrap → declaration freeze → graph commit → graph evidence.

mod bootstrap;
mod declaration_freeze;
mod freeze;
mod graph_evidence;

pub(crate) use bootstrap::{
    WorthUiCapabilityRegistrationFreezeCore, WorthUiFacadeLifecycleBootstrap,
};
pub(crate) use graph_evidence::{build_graph_evidence_indexes, GraphEvidenceIndexes};
pub use crate::lifecycle::{WorthUiRuntimeSupportInventory, RUNTIME_SUPPORT_INVENTORY};
#[allow(deprecated)]
pub use crate::lifecycle::PHASE3_RUNTIME_SUPPORT_INVENTORY;