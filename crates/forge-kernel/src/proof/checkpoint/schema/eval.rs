use forge_core::{KernelError, ToleranceProvider};
use forge_spatial::{validate_geometric_invariants, GeometryContext};
use forge_spec::facade::SpecState;
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::VertexId;
use forge_topo::validate::{validate_topology, ValidationLevel};

use crate::engine::facade::SpecEnvelope;

use super::{ValidationCheckpoint, ValidationConfig, ValidationResult};

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

/// Execute a checkpoint validation against spec-backed kernel output.
///
/// This is the spec-native counterpart to `run_checkpoint`. It validates
/// graph truth through the projected topology path and preserves the same
/// checkpoint gating and `ValidationResult` semantics.
///
/// Geometric validation is currently not supported on the spec-native path,
/// so `included_geometric` is always reported as `false`.
pub fn run_spec_envelope_checkpoint(
    envelope: &SpecEnvelope,
    config: &ValidationConfig,
    checkpoint: ValidationCheckpoint,
) -> Result<ValidationResult, KernelError> {
    envelope.run_checkpoint(config, checkpoint)
}

/// Execute a checkpoint validation directly against graph-native spec truth.
///
/// This is a convenience wrapper that materializes a transitional
/// `SpecEnvelope` with empty geometry and delegates to
/// `run_spec_envelope_checkpoint`.
pub fn run_spec_checkpoint(
    spec: &SpecState,
    config: &ValidationConfig,
    checkpoint: ValidationCheckpoint,
) -> Result<ValidationResult, KernelError> {
    let envelope = SpecEnvelope::from_spec(spec.clone());
    run_spec_envelope_checkpoint(&envelope, config, checkpoint)
}
