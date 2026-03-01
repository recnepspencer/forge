//! Core shared types for the Forge geometry kernel.
//!
//! This crate contains the common language that `forge-math`, `forge-geom`,
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

// =========================================================================
// DOMAIN MODULES
// =========================================================================

pub mod envelope;
pub mod errors;
pub mod policy;
pub mod provenance;
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

pub use policy::{PolicyKind, PolicyQuery, PolicyResult, ValidationCheckpoint};

pub use tracing::{
    compute_trace_fingerprint, diff_decision_logs, log_decision_log, log_error,
    log_result, scan_for_divergences, CandidateValueSummary,
    CheckpointLog, DecisionChange, DecisionContext, DecisionDelta, DecisionId, DecisionKind,
    DecisionLog, DecisionSummary, DecisionTier, DivergenceDetail, DivergenceReport, EntityKind,
    EntityRef, PolicyDecisionTracePayload, PolicyResolutionOutcome,
    PolicyResolutionScopeRef, PolicyResolutionSource, PolicyTraceConsistencyError,
    ReidentificationCompatibilitySummary, ReidentificationFailureCauseSummary,
    ReidentificationModeSummary, ReidentificationOriginKindSummary, ReidentificationOutcome,
    ReidentificationTraceConsistencyError, ReidentificationTracePayload,
    ResolutionCandidateSummary, ResolutionMatchKind, ResolutionOutcome, ResolutionQuerySummary,
    ResolutionRoute, ResolutionTraceConsistencyError, ResolutionTracePayload, SpanId,
    SpanSummaryEntry, TopologyDelta, TraceDiff, TraceEvent, TraceFingerprint, TraceSummary,
    TracedDecision, EULER_OP_FEATURE_SCOPE,
};

pub use envelope::{KernelWarning, LineageDelta, OperationMetrics, OperationResult};

pub use provenance::{
    hash_directed_snapshot_segment_transport, BoundarySegmentProvenance, MergeStepProvenance,
    SelectorOrigin, SnapshotHandleRef,
};
pub use tolerance::{FlatToleranceProvider, ToleranceProvider};

// =========================================================================
// GEOMETRY SOURCE (Data-access trait, Rule 3.1)
// =========================================================================

/// Anonymous data-access trait for geometry solvers (Rule 3.1).
///
/// The geometry layer cannot import topology types (`FaceId`, etc.),
/// so it accepts `&dyn GeometrySource` for plane lookups. The kernel
/// layer provides the concrete implementation that bridges typed handles
/// to raw plane coefficients.
///
/// Use `PlaneSet` (in `forge-geom`) as the lightweight test double.
pub trait GeometrySource: std::fmt::Debug {
    /// Number of planes available.
    fn plane_count(&self) -> usize;
    /// Retrieve the plane coefficients [a, b, c, d] for a given index.
    fn get_plane(&self, index: usize) -> [f64; 4];
}
