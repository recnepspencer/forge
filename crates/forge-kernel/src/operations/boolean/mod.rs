//! DOMAIN: Boolean Operations (CSG)
//!
//! Implements union, intersection, and subtraction on polyhedra
//! using exact predicates (D3). Two pipeline implementations:
//!
//! - **Parametric** (`parametric/`) — split → classify → select → assemble → postprocess
//!   Handles all geometry types (planar, NURBS, analytic).
//! - **EMBER** (`ember/`) — BSP convert → merge → extract → mesh
//!   Fast exact pipeline for planar polyhedra only.
//!
//! INVARIANTS:
//! - All topology decisions use `CertifiedTriSign` (D3)
//! - Operations are atomic via `MutableDraft` (D6)
//! - Result satisfies Euler's formula (V - E + F = 2)
//!
//! DEPENDENCIES: `forge-geom` (planes, predicates), `forge-topo` (arena, operators),
//!               `geometry_state` (GeometryState), `mesh_builder` (mesh construction)
//!
//! FILES:
//! - `schema.rs` — Input types (BooleanInput, BooleanOp)
//! - `result.rs` — Output types (BooleanResult, BooleanIntrospection)
//! - `classify_schema.rs` — Classification types (FaceClassification, ClassifiedFace)
//! - `contract.rs` — Feature contracts for both pipelines
//! - `adapters.rs` — BooleanTolerances adapter
//! - `parametric/` — Parametric pipeline (split, classify, assemble, postprocess)

pub mod adapters;
#[cfg(test)]
mod brutality;
pub mod classify_schema;
pub mod contract;
mod debug;
pub mod ember;
pub mod parametric;
pub mod result;
pub(crate) mod schema;
pub(crate) mod shared;
#[cfg(test)]
pub mod test_helpers;
#[cfg(test)]
mod tests;

pub use classify_schema::{ClassifiedFace, FaceClassification, FaceOrigin};
pub use parametric::assemble::execute_boolean_direct;
pub use parametric::assemble::execute_boolean_with_overrides;
pub use result::{BooleanIntrospection, BooleanResult};
pub use schema::{BooleanInput, BooleanOp};

/// Execute a Boolean operation — the production entry point.
///
/// Routes to the appropriate pipeline:
/// - **Planar geometry** → EMBER (integer grid quantization + standard pipeline)
/// - **Curved geometry** → Parametric pipeline directly
/// - **EMBER failure** → Automatic parametric pipeline fallback
///
/// This ensures every Boolean gets the best available pipeline without
/// callers needing to know about EMBER.
pub fn execute_boolean(
    input: BooleanInput,
) -> forge_core::OperationResult<Result<BooleanResult, forge_core::KernelError>> {
    crate::operations::boolean::ember::execute_boolean_adaptive(input)
}
