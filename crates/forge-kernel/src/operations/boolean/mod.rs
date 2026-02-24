//! DOMAIN: Planar Boolean Operations (CSG)
//!
//! Implements union, intersection, and subtraction on planar-faced
//! polyhedra using exact predicates (D3). The pipeline:
//!
//! 1. **Split** — Split faces of both solids along their mutual intersections
//! 2. **Classify** — Label each split face as inside/outside the other solid
//! 3. **Assemble** — Collect the correct faces based on operation type
//!
//! INVARIANTS:
//! - All topology decisions use `CertifiedTriSign` (D3)
//! - Operations are atomic via `MutableDraft` (D6)
//! - Result satisfies Euler's formula (V - E + F = 2)
//!
//! DEPENDENCIES: `forge-geom` (planes, predicates), `forge-topo` (arena, operators),
//!               `geometry_store` (GeometryStore), `mesh_builder` (mesh construction)

pub(crate) mod schema;
pub mod eval;
mod split;
mod classify;
mod postprocess;
pub mod assemble;
pub mod traits;
pub mod engines;
#[cfg(test)]
pub mod test_helpers;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod brutality;
mod debug;

pub use schema::{BooleanInput, BooleanOp, BooleanResult, FaceClassification, ClassifiedFace};
pub use assemble::execute_boolean_direct;
pub use assemble::execute_boolean_with_overrides;

/// Execute a Boolean operation — the production entry point.
///
/// Routes to the appropriate pipeline:
/// - **Planar geometry** → EMBER (integer grid quantization + standard pipeline)
/// - **Curved geometry** → Standard pipeline directly
/// - **EMBER failure** → Automatic standard pipeline fallback
///
/// This ensures every Boolean gets the best available pipeline without
/// callers needing to know about EMBER.
pub fn execute_boolean(
    input: BooleanInput,
) -> forge_core::OperationResult<Result<BooleanResult, forge_core::KernelError>> {
    crate::operations::ember_boolean::execute_boolean_adaptive(input)
}
