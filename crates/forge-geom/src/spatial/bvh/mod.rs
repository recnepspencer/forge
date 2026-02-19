//! Bounding Volume Hierarchy (BVH) for spatial indexing.
//!
//! Uses a median-split construction strategy to build a balanced tree.
//! Supports querying for overlapping pairs of objects between two trees.

mod schema;
mod eval;

#[cfg(test)]
mod tests;

pub use schema::BvhNode;
pub use eval::query_overlapping_pairs;
