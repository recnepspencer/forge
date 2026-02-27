//! Post-operation validation boundaries.
//!
//! DOMAIN: Validation steps bridging the OperationFinalizer and topology phases.

use forge_core::KernelError;

use crate::core::ModelingContext;
use crate::operations::boolean::result::BooleanResult;
use crate::analysis::proof_validation::checkpoint::{run_checkpoint, ValidationCheckpoint};

/// Run post-boolean topology validation.
///
/// This catches validation errors inside the operation envelope rather than returning them bare.
// DEFECT(D6): No post-operation ValidationLevel::Full check is enforced.
pub fn run_post_boolean_validation(
    result: &BooleanResult,
    ctx: &ModelingContext,
) -> Result<(), KernelError> {
    let geom = result.geometry();
    let pos_fn = |vid| geom.get_vertex_position(vid).copied();
    let _validation = run_checkpoint(
        result.topology().arena(),
        &ctx.get_validation_config(),
        ValidationCheckpoint::PostBoolean,
        Some(&pos_fn),
        geom,
    )?;

    Ok(())
}
