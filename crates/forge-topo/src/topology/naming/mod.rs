//! Compatibility shim — re-exports from `persistent_naming` component.

pub use crate::persistent_naming::data::naming_schema as schema;
pub use crate::persistent_naming::logic::name_resolution as eval;

#[cfg(test)]
mod tests;

pub use eval::{assign_name, resolve_name, resolve_selector};
pub use schema::{PersistentName, Selector};
