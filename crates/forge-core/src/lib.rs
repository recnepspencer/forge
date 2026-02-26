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

pub mod errors;
pub mod policy;
pub mod tracing;
pub mod envelope;
pub mod tolerance;
pub mod provenance;

// =========================================================================
// CRATE-ROOT RE-EXPORTS
// =========================================================================

pub use errors::{
    ErrorScope, SuggestedFix, ErrorContext,
    KernelError, TopologyError,
    AmbiguousResult, DiagnosticPayload,
    ErrorCategory, ErrorSummary, SourceErrorSummary,
    KernelErrorSummary, MergeErrorSummary, TopologyErrorSummary, DiagnosticPayloadSummary,
};

pub use policy::{
    PolicyKind, PolicyQuery, PolicyResult,
};

pub use tracing::{
    DecisionKind, DecisionId, TracedDecision, DecisionLog,
    DecisionSummary, DecisionContext, EntityRef, EntityKind, TopologyDelta,
    SpanId, DecisionTier, TraceEvent, TraceSummary, TraceDiff, SpanSummaryEntry,
    DecisionDelta, DecisionChange, CheckpointLog, diff_decision_logs,
    DivergenceReport, DivergenceDetail, scan_for_divergences,
    EULER_OP_FEATURE_SCOPE,
    resolve_trace_dir, write_trace_file,
    LogLevel, log_level, log_result, log_decision_log, log_error,
    PolicyResolutionSource, PolicyResolutionOutcome, CandidateValueSummary,
    PolicyDecisionTracePayload, PolicyTraceConsistencyError, PolicyResolutionScopeRef,
    TraceFingerprint, compute_trace_fingerprint,
};

pub use envelope::{
    KernelWarning, OperationMetrics, LineageDelta, OperationResult,
};

pub use tolerance::{ToleranceProvider, FlatToleranceProvider};
pub use provenance::{
    SnapshotHandleRef, BoundarySegmentProvenance, MergeStepProvenance, SelectorOrigin,
    hash_directed_snapshot_segment_transport,
};

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
