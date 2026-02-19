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
//! All public types are re-exported at the crate root for convenience.
//! A backwards-compatible `result` module is provided so that existing
//! `use forge_core::result::*` imports continue to work.

// =========================================================================
// DOMAIN MODULES
// =========================================================================

pub mod errors;
pub mod policy;
pub mod tracing;
pub mod envelope;

// =========================================================================
// BACKWARDS-COMPATIBLE `result` RE-EXPORT MODULE
//
// Many crates use `use forge_core::result::{...}`, so we preserve this
// path by re-exporting everything from the new domain modules.
// =========================================================================

pub mod result {
    //! Backwards-compatible re-exports from the legacy `result` module.
    //!
    //! All types that previously lived in `result.rs` are now organized
    //! in domain-specific directories. This module re-exports them
    //! under the old path so downstream crates compile without changes.

    pub use crate::tracing::{
        EntityRef, SpanId, DecisionTier, TraceEvent,
        DecisionKind, DecisionContext, DecisionId, TracedDecision,
        EULER_OP_FEATURE_SCOPE,
        DecisionLog, DecisionSummary, SpanSummaryEntry,
        TraceSummary, TraceDiff,
        DecisionDelta, DecisionChange, CheckpointLog, diff_decision_logs,
        resolve_trace_dir, write_trace_file,
        LogLevel, log_level, log_result, log_decision_log,
    };

    pub use crate::envelope::{
        KernelWarning, OperationMetrics, LineageDelta, OperationResult,
    };
}

// =========================================================================
// CRATE-ROOT RE-EXPORTS
// =========================================================================

pub use errors::{
    ErrorScope, SuggestedFix, ErrorContext,
    KernelError, TopologyError,
    AmbiguousResult, DiagnosticPayload,
};

pub use policy::{
    PolicyKind, PolicyQuery, PolicyResult,
};

pub use tracing::{
    DecisionKind, DecisionId, TracedDecision, DecisionLog,
    DecisionSummary, DecisionContext, EntityRef,
    SpanId, DecisionTier, TraceEvent, TraceSummary, TraceDiff, SpanSummaryEntry,
    DecisionDelta, DecisionChange, CheckpointLog, diff_decision_logs,
    DivergenceReport, DivergenceDetail, scan_for_divergences,
    EULER_OP_FEATURE_SCOPE,
    resolve_trace_dir, write_trace_file,
    LogLevel, log_level, log_result, log_decision_log,
};

pub use envelope::{
    KernelWarning, OperationMetrics, LineageDelta, OperationResult,
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
