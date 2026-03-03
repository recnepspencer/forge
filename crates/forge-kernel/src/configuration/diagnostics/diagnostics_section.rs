//! Diagnostics section of the unified configuration.
//!
//! DOMAIN: Observability and debugging depth knobs for the kernel.
//! These control HOW MUCH diagnostic data the pipeline produces,
//! not WHAT correctness checks it runs (that's ValidationSection).
//!
//! Think of this as RUST_LOG for the geometry kernel — a single
//! section where you tune verbosity across multiple subsystems.

use forge_core::KernelError;
use serde::{Deserialize, Serialize};

use super::super::kernel_config::ConfigSection;

// ── Enums ────────────────────────────────────────────────────────────────

/// How much data the pipeline includes in its fingerprint hash.
///
/// Controls the trade-off between hash speed and collision resistance.
/// The pipeline stamps `hash_before` and `hash_after` on every
/// `OperationResult` — this setting controls what goes into those hashes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FingerprintDetail {
    /// Topology arenas + feature kind + conditioning mode + key tolerances.
    /// Fast. Catches structural changes and config drift.
    Standard,
    /// Standard + vertex positions + face plane normals.
    /// O(V+F) per input. For regression suites and deterministic replay.
    Full,
}

/// How many traced decisions the pipeline retains in the envelope.
///
/// Independent of `AuditLevel` (which controls what gets emitted).
/// This controls what survives after audit filtering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraceVerbosity {
    /// Keep only summary counts (decision count, warning count).
    Minimal,
    /// Keep all decisions but strip context payloads.
    Standard,
    /// Keep everything — full decision context, margin values, entity scopes.
    Full,
}

/// How deep geometry validation scans go.
///
/// Separate from `ValidationSection::include_geometric` which is a boolean
/// gate. This controls the depth of geometric checks when they ARE enabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeometryValidationDepth {
    /// Quick manifold + Euler formula check. O(E).
    Quick,
    /// Quick + sliver detection + edge length checks. O(E + F).
    Standard,
    /// Full self-intersection scan, gap detection, normal consistency.
    /// O(F²) worst case — use for release validation, not inner loops.
    Exhaustive,
}

// ── Section ──────────────────────────────────────────────────────────────

/// Diagnostic and observability knobs for the kernel pipeline.
///
/// Controls how much diagnostic data the pipeline produces and retains.
/// None of these affect correctness — they only affect observability depth
/// and performance overhead of diagnostic instrumentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticsSection {
    /// How much data goes into pipeline fingerprint hashes.
    pub fingerprint_detail: FingerprintDetail,

    /// How many traced decisions survive audit filtering.
    pub trace_verbosity: TraceVerbosity,

    /// Whether per-operation wall-clock timing is recorded in metrics.
    pub enable_performance_profiling: bool,

    /// Whether per-operation memory delta tracking is enabled.
    /// Adds overhead from allocation tracking — off by default.
    pub enable_memory_tracking: bool,

    /// How deep geometry validation scans go (when enabled).
    pub geometry_validation_depth: GeometryValidationDepth,

    /// Force deterministic iteration order across all collections.
    /// Use for replay verification and regression suites.
    /// Adds overhead from sorted iteration — off by default.
    pub deterministic_mode: bool,

    /// Dump intermediate geometry at each pipeline stage as debug files.
    /// Writes to a configured output directory. Off by default.
    pub enable_debug_geometry_export: bool,
}

impl ConfigSection for DiagnosticsSection {
    fn defaults() -> Self {
        if cfg!(debug_assertions) {
            // Debug builds: more verbose for development
            Self {
                fingerprint_detail: FingerprintDetail::Standard,
                trace_verbosity: TraceVerbosity::Full,
                enable_performance_profiling: true,
                enable_memory_tracking: false,
                geometry_validation_depth: GeometryValidationDepth::Standard,
                deterministic_mode: false,
                enable_debug_geometry_export: false,
            }
        } else {
            // Release builds: minimal overhead
            Self {
                fingerprint_detail: FingerprintDetail::Standard,
                trace_verbosity: TraceVerbosity::Standard,
                enable_performance_profiling: false,
                enable_memory_tracking: false,
                geometry_validation_depth: GeometryValidationDepth::Quick,
                deterministic_mode: false,
                enable_debug_geometry_export: false,
            }
        }
    }

    fn validate(&self) -> Result<(), KernelError> {
        Ok(())
    }
}

impl Default for DiagnosticsSection {
    fn default() -> Self {
        Self::defaults()
    }
}
