//! Core shared types for the Forge geometry kernel.
//!
//! This crate contains the common language that `worth-math`, `worth-geom`,
//! `forge-topo`, and `forge-kernel` all speak. It is organized into four
//! functional domains:
//!
//! - **errors** — Error taxonomy (`KernelError`, `TopologyError`, `ErrorContext`)
//! - **policy** — Three-state return types and policy queries (`PolicyResult<T>`)
//! - **tracing** — Span-based decision tracing protocol (`DecisionLog`, `TracedDecision`)
//! - **envelope** — Universal operation result envelope (`OperationResult<T>`)
//!
//! # Re-export strategy
//!
//! All public types are re-exported at the crate root.

// ── Float safety ────────────────────────────────────────────────────────
// Direct float equality is banned. Use forge_core comparison predicates:
//   approximately_equal, positions_coincident, is_effectively_zero,
//   is_degenerate_magnitude_sq — all take &dyn ToleranceProvider.
#![deny(clippy::float_cmp)]

// =========================================================================
// DOMAIN MODULES
// =========================================================================

pub mod cache;
pub mod envelope;
pub mod errors;
pub mod policy;
pub mod provenance;
pub mod storage;
pub mod tolerance;
pub mod tracing;

// =========================================================================
// CRATE-ROOT RE-EXPORTS
// =========================================================================

pub use errors::{
    AmbiguousResult, DiagnosticPayload, DiagnosticPayloadSummary, ErrorCategory, ErrorContext,
    ErrorScope, ErrorSummary, KernelError, KernelErrorSummary, MergeErrorSummary,
    SourceErrorSummary, SuggestedFix, TopologyError, TopologyErrorSummary,
};

pub use cache::{
    CacheCheckpoint, CacheDirtyState, CacheDomain, CacheRefreshMode, CacheRefreshPolicy,
    DomainImpact,
};

pub use policy::{
    applicable_mask_for,
    deferred_mask_for,
    // Topology classification
    CertificationStage,
    Closure,
    InvariantContract,
    // Invariant validation contract types
    InvariantGroup,
    InvariantId,
    InvariantRelation,
    InvariantTier,
    Manifoldness,
    // Policy types
    PolicyKind,
    PolicyQuery,
    PolicyResult,
    TopologyContext,
    TopologyKind,
    // Validation checkpoints
    ValidationCheckpoint,
    ValidatorCost,
    APPLICABLE_BY_KIND,
    CLOSED_SHEET_EXTRA,
    DEFER_SEMANTIC_TIER,
    DEFER_UNCERTIFIED,
};

// Tracing — all exports routed through `tracing::facade`
pub use tracing::*;

pub use envelope::{
    KernelWarning, LineageDelta, MutationJournalSnapshot, OperationMetrics, OperationResult,
};

pub use provenance::{
    hash_directed_snapshot_segment_transport, BoundarySegmentProvenance, MergeStepProvenance,
    SelectorOrigin, SnapshotHandleRef,
};
pub use storage::{PropertyLayer, PropertyPatch};
pub use tolerance::{
    approximately_equal, is_degenerate_magnitude_sq, is_effectively_zero, positions_coincident,
    FlatToleranceProvider, ToleranceProvider,
};
