//! Adaptive Boolean operation router.
//!
//! DOMAIN: Single entry point that selects the optimal pipeline for a
//! given Boolean operation: EMBER (BSP merge) for planar solids, or
//! parametric (split → classify → assemble) for general geometry.
//!
//! DEPENDENCIES: `schema` (BooleanInput), `result` (BooleanResult),
//!               `parametric` (parametric pipeline execution)
//!
//! INVARIANTS:
//! - Curved geometry always routes to parametric (EMBER is planar-only)
//! - EMBER failure triggers automatic parametric fallback
//! - The router itself adds no topology decisions — it only dispatches

use forge_core::{KernelError, OperationResult};

use super::result::BooleanResult;
use super::schema::BooleanInput;

/// Execute a Boolean via the adaptive router — the production entry point.
///
/// Routing logic:
/// 1. If either input contains curved geometry → parametric directly
/// 2. If both inputs are planar → attempt EMBER first
/// 3. If EMBER fails → fall back to parametric
///
/// Currently: always routes to parametric (EMBER not yet rebuilt).
pub fn execute_boolean_adaptive(
    input: BooleanInput,
) -> OperationResult<Result<BooleanResult, KernelError>> {
    // Future: EMBER fast-path for planar solids
    // if !input.has_curved_geometry() {
    //     match ember::try_execute(&input) {
    //         Ok(result) => return result,
    //         Err(EmberError::CurvedGeometry) | Err(EmberError::PipelineError(_)) => {
    //             // fall through to parametric
    //         }
    //     }
    // }

    super::parametric::execute(input)
}

/// Execute directly via the parametric pipeline (bypasses EMBER).
///
/// Used by tests that want to exercise the parametric path specifically,
/// and by the EMBER fallback path.
pub fn execute_boolean_direct(
    input: BooleanInput,
) -> OperationResult<Result<BooleanResult, KernelError>> {
    super::parametric::execute(input)
}
