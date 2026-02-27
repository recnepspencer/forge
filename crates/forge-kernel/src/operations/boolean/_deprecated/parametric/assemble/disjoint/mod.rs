//! Handling for disjoint and contained solids (zero-split fast path).
//!
//! DOMAIN: When the split phase produces zero cuts, classify the spatial
//! relationship between the two solids and dispatch accordingly.

mod assemble;
mod eval;

pub use eval::compute_disjoint_scale;
pub use eval::execute_zero_split;
