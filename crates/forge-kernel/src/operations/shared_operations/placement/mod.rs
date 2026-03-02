//! Vertex placement — observable, deduplicated vertex creation.

pub mod vertex;

#[cfg(test)]
mod tests;

pub use vertex::{place_vertex, place_vertex_exact, PlacementRegistry};
