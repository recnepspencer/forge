//! P0.5 Invariant Checkpoint System
//!
//! DOMAIN: Automatic topology validation wired into the operation pipeline.
//!
//! INVARIANTS:
//! - Every checkpoint evaluation is logged via `ValidationResult`.
//! - Geometric validation skips entities beyond `entity_limit` but logs the skip.
//! - Non-geometric validation adds < 5% overhead to operations.
//!
//! DEPENDENCIES: `forge-topo` (validate, arena), `forge-core` (KernelError)

pub use forge_core::ValidationCheckpoint;
use forge_core::{KernelError, ToleranceProvider};
use forge_spatial::{validate_geometric_invariants, GeometryContext};
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::VertexId;
use forge_topo::validate::{validate_topology, ValidationLevel};
use serde::{Deserialize, Serialize};

/// Configuration for the invariant checkpoint system.
///
/// Controls which checkpoints are enabled, whether geometric checks
/// are included (more expensive), and a performance safety valve
/// that skips validation on large models.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    /// Which checkpoints are enabled.
    pub(crate) checkpoints: Vec<ValidationCheckpoint>,
    /// Whether to include geometric invariants (zero-area, zero-length, signed volume).
    /// More expensive than structural-only validation.
    pub(crate) include_geometric: bool,
    /// Maximum entities before skipping (perf safety valve).
    /// A value of 0 means no limit (always validate).
    pub(crate) entity_limit: usize,
}

impl ValidationConfig {
    /// Debug-mode default: all checkpoints active, geometric checks on, no entity limit.
    pub fn debug_default() -> Self {
        Self {
            checkpoints: vec![
                ValidationCheckpoint::PostCommit,
                ValidationCheckpoint::PostBoolean,
                ValidationCheckpoint::PostFeature,
                ValidationCheckpoint::PostImport,
            ],
            include_geometric: true,
            entity_limit: 0,
        }
    }

    /// Release-mode default: PostBoolean + PostImport only, no geometric, 50k entity limit.
    pub fn release_default() -> Self {
        Self {
            checkpoints: vec![
                ValidationCheckpoint::PostBoolean,
                ValidationCheckpoint::PostImport,
            ],
            include_geometric: false,
            entity_limit: 50_000,
        }
    }

    /// All checkpoints active, geometric included, no entity limit.
    pub fn all_active() -> Self {
        Self {
            checkpoints: vec![
                ValidationCheckpoint::PostCommit,
                ValidationCheckpoint::PostBoolean,
                ValidationCheckpoint::PostFeature,
                ValidationCheckpoint::PostImport,
                ValidationCheckpoint::OnDemand,
            ],
            include_geometric: true,
            entity_limit: 0,
        }
    }

    /// No checkpoints active — validation fully disabled.
    pub fn disabled() -> Self {
        Self {
            checkpoints: Vec::new(),
            include_geometric: false,
            entity_limit: 0,
        }
    }

    /// Whether a given checkpoint is active.
    pub fn is_active(&self, checkpoint: ValidationCheckpoint) -> bool {
        self.checkpoints.contains(&checkpoint)
    }

    /// Whether geometric invariant checks are included.
    pub fn get_include_geometric(&self) -> bool {
        self.include_geometric
    }

    /// Set whether geometric invariant checks are included.
    pub fn set_include_geometric(&mut self, value: bool) {
        self.include_geometric = value;
    }

    /// The entity limit (0 = no limit).
    pub fn get_entity_limit(&self) -> usize {
        self.entity_limit
    }

    /// Set the entity limit.
    pub fn set_entity_limit(&mut self, limit: usize) {
        self.entity_limit = limit;
    }

    /// The active checkpoints.
    pub fn get_checkpoints(&self) -> &[ValidationCheckpoint] {
        &self.checkpoints
    }

    /// Set the active checkpoints.
    pub fn set_checkpoints(&mut self, checkpoints: Vec<ValidationCheckpoint>) {
        self.checkpoints = checkpoints;
    }

    /// Whether validation should be skipped due to entity count.
    ///
    /// Returns true when entity_limit > 0 AND total_entities >= entity_limit.
    pub fn should_skip_for_entity_count(&self, total_entities: usize) -> bool {
        self.entity_limit > 0 && total_entities >= self.entity_limit
    }
}

impl Default for ValidationConfig {
    /// Default: debug_default in debug builds, release_default in release.
    fn default() -> Self {
        if cfg!(debug_assertions) {
            Self::debug_default()
        } else {
            Self::release_default()
        }
    }
}

/// Result of a checkpoint validation run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Which checkpoint triggered this run.
    checkpoint: ValidationCheckpoint,
    /// Whether validation passed.
    passed: bool,
    /// Error description if validation failed.
    error_detail: Option<String>,
    /// Number of entities at time of validation.
    entity_count: usize,
    /// Whether validation was skipped due to entity limit.
    skipped: bool,
    /// Whether geometric checks were included in this run.
    included_geometric: bool,
    /// Duration of the validation in microseconds.
    duration_micros: u64,
}

impl ValidationResult {
    /// Create a passing result.
    pub fn passed(
        checkpoint: ValidationCheckpoint,
        entity_count: usize,
        included_geometric: bool,
        duration_micros: u64,
    ) -> Self {
        Self {
            checkpoint,
            passed: true,
            error_detail: None,
            entity_count,
            skipped: false,
            included_geometric,
            duration_micros,
        }
    }

    /// Create a failing result.
    pub fn failed(
        checkpoint: ValidationCheckpoint,
        entity_count: usize,
        detail: String,
        included_geometric: bool,
        duration_micros: u64,
    ) -> Self {
        Self {
            checkpoint,
            passed: false,
            error_detail: Some(detail),
            entity_count,
            skipped: false,
            included_geometric,
            duration_micros,
        }
    }

    /// Create a skipped result (entity limit exceeded).
    pub fn skipped(checkpoint: ValidationCheckpoint, entity_count: usize) -> Self {
        Self {
            checkpoint,
            passed: true,
            error_detail: None,
            entity_count,
            skipped: true,
            included_geometric: false,
            duration_micros: 0,
        }
    }

    /// Whether validation passed.
    pub fn is_passed(&self) -> bool {
        self.passed
    }

    /// Whether validation was skipped.
    pub fn is_skipped(&self) -> bool {
        self.skipped
    }

    /// The error detail, if any.
    pub fn error_detail(&self) -> Option<&str> {
        self.error_detail.as_deref()
    }

    /// The checkpoint that triggered this validation.
    pub fn checkpoint(&self) -> ValidationCheckpoint {
        self.checkpoint
    }

    /// Entity count at time of validation.
    pub fn entity_count(&self) -> usize {
        self.entity_count
    }

    /// Whether geometric checks were included.
    pub fn included_geometric(&self) -> bool {
        self.included_geometric
    }

    /// Duration of the validation in microseconds.
    pub fn duration_micros(&self) -> u64 {
        self.duration_micros
    }
}

/// Execute a checkpoint validation against an arena.
///
/// Respects the `ValidationConfig` — skips if the checkpoint is inactive
/// or if entity count exceeds the configured limit. Runs structural
/// validation always (when not skipped), and geometric validation only
/// when `include_geometric` is true.
///
/// Returns `Ok(ValidationResult)` on success or skip,
/// `Err(KernelError)` only on validation failure (propagated from the validators).
pub fn run_checkpoint(
    arena: &TopologyArena,
    config: &ValidationConfig,
    checkpoint: ValidationCheckpoint,
    position_fn: Option<&dyn Fn(VertexId) -> Option<[f64; 3]>>,
    tolerance_provider: &dyn ToleranceProvider,
) -> Result<ValidationResult, KernelError> {
    let total_entities =
        arena.face_count() + arena.half_edge_count() + arena.vertex_count() + arena.loop_count();

    if !config.is_active(checkpoint) {
        return Ok(ValidationResult::skipped(checkpoint, total_entities));
    }

    if config.should_skip_for_entity_count(total_entities) {
        return Ok(ValidationResult::skipped(checkpoint, total_entities));
    }

    let start = std::time::Instant::now();

    validate_topology(arena, ValidationLevel::Full)?;

    if config.get_include_geometric() {
        if let Some(pos_fn) = position_fn {
            let ctx = GeometryContext {
                position_fn: pos_fn,
                plane_fn: &|_| None,
                is_planar: &|_| true,
                curve_fn: &|_| None,
                tolerance_provider,
            };
            validate_geometric_invariants(arena, &ctx)?;
        }
    }

    let duration_micros = start.elapsed().as_micros() as u64;

    Ok(ValidationResult::passed(
        checkpoint,
        total_entities,
        config.get_include_geometric() && position_fn.is_some(),
        duration_micros,
    ))
}
