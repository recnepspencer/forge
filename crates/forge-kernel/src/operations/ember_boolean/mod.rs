//! EMBER integer grid boolean pipeline.
//!
//! DOMAIN: Exact boolean operations using integer grid quantization.
//! All input vertices are snapped to a discrete `[i64; 3]` grid before
//! any boolean logic begins. Orientation predicates use `i128` arithmetic
//! (zero epsilon, zero BigInt allocations). Cut vertices are identified
//! by their defining plane IDs (`[usize; 3]`), not positions.
//!
//! DEPENDENCIES: forge-math (grid predicates), forge-geom (planes),
//! forge-topo (topology arena), GeometryStore.
//!
//! INVARIANTS:
//!   - All input coordinates quantized to 30-bit `[i64; 3]` before splitting.
//!   - Vertex dedup: grid verts by `[i64; 3]`, cut verts by sorted `[usize; 3]`.
//!   - No spatial NNS fallback — exact vertex-ID matching only.
//!   - Legacy pipeline preserved as fallback for scale disparity.

pub mod schema;
pub mod quantize;
pub mod eval;
pub mod classify;
#[cfg(test)]
mod tests;

pub use schema::QuantizedSpace;
pub use eval::{execute_ember_boolean, execute_boolean_adaptive, EmberError};
