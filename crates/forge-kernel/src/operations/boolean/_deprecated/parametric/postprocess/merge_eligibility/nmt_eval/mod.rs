//! Sheet region merge execution engine.
//!
//! DOMAIN: Compound algorithm that orchestrates face merges using
//! `JoinFaces` (manifold) and `JoinFacesNmt` (NMT) based on radial valence.
//! Operates on `KernelDraft` for atomic topo+geom transactionality.
//!
//! STATUS: Staged for integration (test-exercised only). Not yet called from
//! production boolean postprocess flow.
//!
//! DEPENDENCIES: `KernelDraft`, `GeometryPatch`, `JoinFaces`, `JoinFacesNmt`,
//! `radial_valence`, `ModelingContext`, `TracedDecision`.
//!
//! INVARIANTS:
//!   - Drop KernelDraft = atomic rollback of topology AND geometry (D6)
//!   - Handles re-derived per step from draft arena (no stale handles)
//!   - Steps sorted by edge_index for determinism
//!   - TracedDecision emitted per step
//!   - Decisions propagated to both OperationResult and ModelingContext

pub mod execute;
pub mod plan;
pub mod resolve;
pub mod validate;

#[cfg(test)]
pub mod test_api;

pub use execute::{
    execute_sheet_region_merge, execute_sheet_region_merge_persistent,
};
pub use resolve::resolve_merge_region_selection_persistent;

#[cfg(test)]
pub(super) use test_api::NmtEvalTestApi;
