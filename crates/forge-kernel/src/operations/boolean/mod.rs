//! DOMAIN: Boolean Operations (CSG)
//!
//! Implements union, intersection, and subtraction on polyhedra
//! using exact predicates (D3). Adaptive pipeline routing:
//!
//! - **Parametric** (`parametric/`) — split → classify → select → assemble → postprocess
//!   Handles all geometry types (planar, NURBS, analytic).
//!   Each phase is a `StepContract`-declared step with policy pre-validation.
//!
//! - **EMBER** (future) — BSP convert → merge → extract → mesh
//!   Fast exact pipeline for planar polyhedra only. Routes through `router.rs`.
//!
//! INVARIANTS:
//! - All topology decisions use `CertifiedTriSign` (D3)
//! - Operations are atomic via `KernelDraft` (D6)
//! - Result satisfies Euler's formula (V − E + F = 2)
//! - Each pipeline phase declares its `StepContract` (policies, precision sensitivity)
//!
//! DEPENDENCIES: `forge-geom` (planes, predicates), `forge-topo` (arena, operators),
//!               `geometry_state` (GeometryState), `mesh_builder` (mesh construction)
//!
//! FILES:
//! - `schema.rs` — Input types (BooleanInput, BooleanOp)
//! - `result.rs` — Output types (BooleanResult, BooleanIntrospection)
//! - `classify_schema.rs` — Classification types (FaceClassification, ClassifiedFace)
//! - `contract.rs` — Feature contracts (ParametricBooleanContract, EmberBooleanContract)
//! - `adapters.rs` — BooleanTolerances adapter
//! - `router.rs` — Adaptive pipeline dispatch (ember vs parametric)
//! - `parametric/` — Parametric pipeline (split, classify, assemble, postprocess)

pub mod adapters;
pub mod classify_schema;
pub mod contract;
pub mod parametric;
pub mod result;
pub mod router;
pub(crate) mod schema;

// _deprecated/ directory remains on disk as reference but is excluded from compilation.

#[cfg(test)]
pub mod test_helpers;
#[cfg(test)]
mod tests;

pub use classify_schema::{ClassifiedFace, FaceClassification, FaceOrigin};
pub use result::{BooleanIntrospection, BooleanResult};
pub use router::execute_boolean_direct;
pub use schema::{BooleanInput, BooleanOp};

/// Execute a Boolean operation — the production entry point.
///
/// Routes to the optimal pipeline via adaptive dispatch:
/// - **Planar geometry** → EMBER fast-path (when rebuilt)
/// - **Curved geometry** → Parametric pipeline
/// - **EMBER failure** → Automatic parametric fallback
pub fn execute_boolean(
    input: BooleanInput,
) -> forge_core::OperationResult<Result<BooleanResult, forge_core::KernelError>> {
    router::execute_boolean_adaptive(input)
}
