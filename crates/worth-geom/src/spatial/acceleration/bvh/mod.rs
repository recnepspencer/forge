//! Bounding Volume Hierarchy (BVH) for spatial indexing.
//!
//! Uses a median-split construction strategy to build a balanced tree.
//! Supports querying for overlapping pairs of objects between two trees.

mod eval;
mod schema;

#[cfg(test)]
mod tests;

pub use eval::query_overlapping_pairs;
pub use schema::BvhNode;
