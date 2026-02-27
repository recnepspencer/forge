//! Curved same-support surface merge (design contracts + scaffolding).
//!
//! DOMAIN: Generalizes planar coplanar merge to curved faces sharing
//! the same geometric support surface. Entry point is
//! `execute_curved_merge`, which follows the D8/D6 execution model.
//!
//! DEPENDENCIES:
//! - forge-geom: SurfaceRelation, SurfaceData, EvaluateSurface, TrimCurveOps
//! - forge-topo: FaceId, SurfaceRef, CoedgeRef, CurveRef
//! - forge-kernel: KernelState, KernelDraft, GeometryPatch, OperationResult
//!
//! INVARIANTS:
//! - All selected faces must share SurfaceRelation::Coincident support surfaces
//! - SurfaceRelation::Undetermined is fail-closed by default
//! - GeometryPatch stages all binding updates (face→surface, he→coedge, edge→curve)
//! - On failure, KernelDraft drop discards all mutations atomically

pub mod schema;
pub mod eval;
#[cfg(test)]
pub mod tests;

pub use schema::{CurvedMergeSelection, CurvedMergePlan, CurvedMergeResult};
pub use eval::execute_curved_merge;
