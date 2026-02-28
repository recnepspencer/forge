//! Compatibility shim — re-exports from `semantic_attributes` component.
//!
//! All attribute types now live in `crate::semantic_attributes`. This module
//! preserves the `crate::topology::attributes::*` import paths.

pub use crate::semantic_attributes::{EntityKey, TagValue, SemanticTag, AttributeStore};
