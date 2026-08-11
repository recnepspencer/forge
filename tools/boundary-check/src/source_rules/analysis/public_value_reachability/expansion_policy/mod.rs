//! Fail closed when an unexpanded source surface could mint public values.

mod attributes;
mod derive_bindings;
mod item_macros;

use super::super::crate_modules::ModuleGraph;

pub(super) fn verify(graph: &ModuleGraph) -> Result<(), String> {
    attributes::verify(graph)?;
    item_macros::verify(graph)
}
