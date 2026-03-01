//! Parametric Boolean Pipeline — split → classify → select → assemble → postprocess.
//!
//! DOMAIN: General-purpose Boolean pipeline that handles all geometry types
//! (planar, NURBS, analytic surfaces). Each phase is a `StepContract`-declared
//! step, executed through `PipelineBuilder` for typed intermediate state.
//!
//! PIPELINE PHASES:
//!   1. **Validate** — Input validation and tolerance extraction
//!   2. **Split** — Split faces at intersection curves
//!   3. **Classify** — Classify each face as inside/outside the other solid
//!   4. **Select** — Choose which faces to keep based on operation type
//!   5. **Assemble** — Stitch selected faces into result topology
//!   6. **Postprocess** — Merge coplanar faces, clean slivers, close gaps
//!
//! DEPENDENCIES: `schema` (BooleanInput), `result` (BooleanResult),
//!               `adapters` (BooleanTolerances), `contract` (ParametricBooleanContract)
//!
//! INVARIANTS:
//! - All topology decisions use `CertifiedTriSign` (D3)
//! - Operations are atomic via `KernelDraft` (D6)
//! - Each phase is a `StepContract` step with policy pre-validation
//! - Result satisfies Euler's formula (V − E + F = 2)

pub mod steps;

use forge_core::{KernelError, OperationResult};

use super::result::{BooleanIntrospection, BooleanResult};
use super::schema::BooleanInput;
use crate::context::facade::ModelingContext;

/// Execute a Boolean operation via the parametric pipeline.
///
/// This is the main entry point — called by the router or directly by tests.
/// Sets up a `ModelingContext`, validates inputs, then runs the pipeline phases
/// through `OperationPipeline` with auto-injected tracing and policy checks.
pub fn execute(
    input: BooleanInput,
) -> OperationResult<Result<BooleanResult, KernelError>> {
    let mut ctx = ModelingContext::default();
    let start = std::time::Instant::now();

    let inner_result = execute_core(&input, &mut ctx);

    let metrics = forge_core::OperationMetrics {
        duration: start.elapsed(),
        ..forge_core::OperationMetrics::default()
    };

    let mut envelope = OperationResult::new(inner_result);
    envelope.set_metrics(metrics);

    envelope
}

/// Core pipeline execution — separated for testability.
///
/// Phases are executed sequentially through `OperationPipeline::run_step`,
/// each with its own `StepContract` for policy pre-validation and tracing.
fn execute_core(
    input: &BooleanInput,
    ctx: &mut ModelingContext,
) -> Result<BooleanResult, KernelError> {
    use crate::operations::pipeline::builder::OperationPipeline;

    let mut pipeline = OperationPipeline::new(ctx);

    // Phase 1: Validate inputs
    pipeline.run_step(&steps::ValidateInputs, |_ctx| {
        input.validate()
    })?;

    // Phase 2: Split (TODO — incremental build)
    // Phase 3: Classify (TODO — incremental build)
    // Phase 4: Select (TODO — incremental build)
    // Phase 5: Assemble (TODO — incremental build)
    // Phase 6: Postprocess (TODO — incremental build)

    Err(KernelError::InternalError {
        message: "Parametric boolean pipeline: phases 2-6 not yet implemented".into(),
        context: None,
    })
}

/// Execute with explicit `ModelingContext` for test injection.
///
/// Allows tests to provide a pre-configured context (custom tolerances,
/// specific policies, decision log capture).
#[cfg(test)]
pub fn execute_with_context(
    input: &BooleanInput,
    ctx: &mut ModelingContext,
) -> Result<BooleanResult, KernelError> {
    execute_core(input, ctx)
}
