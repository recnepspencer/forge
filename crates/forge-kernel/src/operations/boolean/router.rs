use forge_core::{KernelError, OperationResult};

use super::result::BooleanResult;
use super::schema::BooleanInput;
use crate::configuration::facade::{resolve_config, KernelConfig};
use crate::context::scope::OperationScope;
use crate::context::ModelingContext;

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
    let session = KernelConfig::default();
    let resolved = match resolve_config(&session, None, None, None) {
        Ok(cfg) => cfg,
        Err(err) => return OperationResult::new(Err(err)),
    };
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&resolved, &mut ctx);

    let start = std::time::Instant::now();
    let inner_result = super::parametric::execute(&input, &mut scope);
    let metrics = forge_core::OperationMetrics {
        duration: start.elapsed(),
        ..forge_core::OperationMetrics::default()
    };

    let mut envelope = OperationResult::new(inner_result);
    envelope.set_metrics(metrics);
    envelope
}

/// Execute directly via the parametric pipeline (bypasses EMBER).
///
/// Used by tests that want to exercise the parametric path specifically,
/// and by the EMBER fallback path.
pub fn execute_boolean_direct(
    input: BooleanInput,
) -> OperationResult<Result<BooleanResult, KernelError>> {
    execute_boolean_adaptive(input)
}
