//! Epoch-versioned topology state with transactional mutation.
//!
//! DOMAIN: `TopologyState` is immutable. The ONLY way to mutate topology
//! is through `MutableDraft`, which auto-rolls back if dropped without
//! committing.
//!
//! SUBMODULES:
//! - `draft_config`: DraftConfig for transaction options
//! - `topology_state`: TopologyState immutable snapshot
//! - `draft`: MutableDraft transactional wrapper

mod draft_config;
mod topology_state;
pub(crate) mod draft;

#[cfg(test)]
mod tests;

pub use draft_config::DraftConfig;
pub use topology_state::TopologyState;
pub use draft::MutableDraft;
