//! Re-export of PropertyLayer and PropertyPatch from forge-core.
//!
//! The canonical implementation lives in `forge_core::storage`.
//! This re-export exists so that `geometry/data/mod.rs` can expose them
//! without changing its public API.

pub use forge_core::{PropertyLayer, PropertyPatch};
