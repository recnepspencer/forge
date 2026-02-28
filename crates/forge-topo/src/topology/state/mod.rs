//! Compatibility shim — re-exports from `transactions` component.

pub use crate::transactions::data::draft_configuration as draft_config;
pub use crate::transactions::data::versioned_snapshot as topology_state;
pub use crate::transactions::logic::mutable_draft as draft;

#[cfg(test)]
mod tests;

pub use draft_config::DraftConfig;
pub use topology_state::TopologyState;
pub use draft::MutableDraft;
