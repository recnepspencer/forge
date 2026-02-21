//! EMBER boolean entry point — dual-engine router.
//!
//! DOMAIN: Routes Boolean operations to either the EMBER exact integer
//! grid pipeline (for planar geometry) or the legacy heuristic pipeline
//! (for curved geometry, scale disparity, or as fallback).
//!
//! ROUTING LOGIC:
//!   1. If input has curved geometry → legacy pipeline (EMBER can't do NURBS)
//!   2. If quantization would collapse >10% of vertices → legacy pipeline
//!   3. Otherwise → EMBER: quantize → collapse → delegate to legacy with clean inputs
//!
//! INVARIANTS:
//!   - Legacy pipeline is never modified — always available as fallback.
//!   - Curved geometry detection is future-proofed (currently all planar).

use forge_core::{KernelError, OperationResult};

use crate::operations::boolean::{BooleanInput, BooleanResult, BooleanOp, execute_boolean_direct};
use crate::operations::boolean::assemble::execute_boolean_with_engine;
use crate::operations::boolean::engines::planar::planar_engine;

use super::schema::QuantizedSpace;
use super::quantize::{QuantizedVertices, collapse_coincident_vertices};

/// EMBER error types — triggers fallback to legacy pipeline.
#[derive(Debug)]
pub enum EmberError {
    /// Quantization would collapse meaningful geometry (extreme scale disparity).
    QuantizationCollapse {
        collapsed_vertices: usize,
        total_vertices: usize,
    },
    /// Input contains curved geometry that EMBER cannot handle.
    CurvedGeometry,
    /// The legacy pipeline failed even after quantization.
    PipelineError(KernelError),
}

impl From<KernelError> for EmberError {
    fn from(e: KernelError) -> Self {
        EmberError::PipelineError(e)
    }
}

/// Execute a Boolean using the EMBER exact integer grid pipeline.
///
/// Quantizes both input meshes, collapses coincident vertices, then runs
/// the existing boolean pipeline on clean topology where near-misses
/// have been eliminated.
///
/// Returns `Err(EmberError::CurvedGeometry)` if inputs contain non-planar
/// faces, and `Err(EmberError::QuantizationCollapse)` if grid snapping
/// would destroy meaningful geometry.
pub fn execute_ember_boolean(
    input: BooleanInput,
) -> Result<OperationResult<Result<BooleanResult, KernelError>>, EmberError> {
    if input.has_curved_geometry() {
        return Err(EmberError::CurvedGeometry);
    }

    let (target_topo, mut target_geom, tool_topo, mut tool_geom, operation) = input.into_parts();

    let space = QuantizedSpace::build(&target_geom, &tool_geom);

    let target_quant = QuantizedVertices::compute_keys(&target_topo, &target_geom, &space);
    let tool_quant = QuantizedVertices::compute_keys(&tool_topo, &tool_geom, &space);

    let target_collapsed_groups = target_quant.find_coincident_groups();
    let tool_collapsed_groups = tool_quant.find_coincident_groups();

    let total_target = target_quant.len();
    let total_tool = tool_quant.len();
    let collapsed_count = target_collapsed_groups.iter().map(|g| g.len() - 1).sum::<usize>()
        + tool_collapsed_groups.iter().map(|g| g.len() - 1).sum::<usize>();

    let total = total_target + total_tool;
    if total > 0 && collapsed_count as f64 / total as f64 > 0.1 {
        return Err(EmberError::QuantizationCollapse {
            collapsed_vertices: collapsed_count,
            total_vertices: total,
        });
    }

    let target_topo = collapse_coincident_vertices(
        target_topo, &mut target_geom, &target_quant, &space,
    )?;

    let tool_topo = collapse_coincident_vertices(
        tool_topo, &mut tool_geom, &tool_quant, &space,
    )?;

    let quantized_input = BooleanInput::new(
        target_topo,
        target_geom,
        tool_topo,
        tool_geom,
        operation,
    );

    let result = execute_boolean_with_engine(quantized_input, planar_engine());
    Ok(result)
}

/// Dual-engine Boolean router — the recommended production entry point.
///
/// Routes operations to the appropriate pipeline:
/// - **Curved geometry** → legacy heuristic pipeline (NURBS need Newton-Raphson)
/// - **Scale disparity** → legacy pipeline (quantization would collapse geometry)
/// - **Planar geometry** → EMBER exact integer grid pipeline
/// - **EMBER failure** → automatic legacy fallback
///
/// This ensures the engine always produces a result, even if the exact
/// pipeline encounters an edge case.
pub fn execute_boolean_adaptive(
    input: BooleanInput,
) -> OperationResult<Result<BooleanResult, KernelError>> {
    if input.has_curved_geometry() {
        return execute_boolean_direct(input);
    }

    let input_clone = input.clone();

    match execute_ember_boolean(input) {
        Ok(result) => result,
        Err(EmberError::CurvedGeometry) => execute_boolean_direct(input_clone),
        Err(EmberError::QuantizationCollapse { .. }) => execute_boolean_direct(input_clone),
        Err(EmberError::PipelineError(_)) => execute_boolean_direct(input_clone),
    }
}
