//! Operation pipeline executor and typed builder.
//!
//! DOMAIN: Orchestrates multi-step operations with auto-injected context,
//! policy validation, and step-scoped audit collection.
//!
//! Two APIs depending on complexity:
//! - `OperationPipeline::run_step` — for simple operations or when steps
//!   share complex data dependencies via local variables
//! - `PipelineBuilder::then` — for linear pipelines with typed intermediate
//!   state (consumes previous state, produces next state)
//!
//! DEPENDENCIES: forge-core (KernelError), core/context (ModelingContext),
//! step_contract (StepContract, StepAuditEntry, OperationAuditRecord)

use forge_core::KernelError;

use super::step_contract::{OperationAuditRecord, StepAuditEntry, StepContract};
use crate::context::facade::ModelingContext;

/// Sequential operation pipeline with step-scoped audit.
///
/// Wraps a `ModelingContext` and collects per-step audit entries.
/// Each `run_step` call validates policies, executes the step closure,
/// and records audit metadata.
pub struct OperationPipeline<'a> {
    ctx: &'a mut ModelingContext,
    steps_executed: Vec<StepAuditEntry>,
}

impl<'a> OperationPipeline<'a> {
    /// Create a new pipeline bound to a `ModelingContext`.
    pub fn new(ctx: &'a mut ModelingContext) -> Self {
        Self {
            ctx,
            steps_executed: Vec::new(),
        }
    }

    /// Run a step with auto-injected context.
    ///
    /// 1. Validates required policies (fail-fast)
    /// 2. Records decision count checkpoint
    /// 3. Executes the step closure
    /// 4. Collects step-scoped audit entry
    pub fn run_step<S, R, F>(&mut self, step: &S, execute: F) -> Result<R, KernelError>
    where
        S: StepContract,
        F: FnOnce(&mut ModelingContext) -> Result<R, KernelError>,
    {
        // 1. Validate required policies (fail-fast)
        for policy in step.policy_queries() {
            self.ctx.validate_policy_configured(policy)?;
        }

        // 2. Checkpoint for step-scoped audit
        let checkpoint = self.ctx.get_decision_count();

        // 3. Execute with trace span
        let span_id = self.ctx.get_decision_log_mut().start_span(step.step_name());
        let start = std::time::Instant::now();
        let result = execute(self.ctx);
        let duration_micros = start.elapsed().as_micros() as u64;
        self.ctx
            .get_decision_log_mut()
            .end_span(span_id, duration_micros);

        // 4. Collect step audit (regardless of success/failure)
        let decisions_count = self.ctx.get_decision_count() - checkpoint;
        self.steps_executed.push(StepAuditEntry {
            name: step.step_name().to_string(),
            decision_count: decisions_count,
            precision_sensitive: step.precision_sensitive(),
        });

        result
    }

    /// Finalize the pipeline and return the audit record.
    pub fn finalize(self) -> OperationAuditRecord {
        OperationAuditRecord {
            steps: self.steps_executed,
        }
    }

    /// Read-only access to the audit entries collected so far.
    pub fn steps_executed(&self) -> &[StepAuditEntry] {
        &self.steps_executed
    }
}

/// Typed state pipeline builder for linear multi-step operations.
///
/// Each `.then()` call consumes the current state and produces a new state.
/// The state type changes at each step, providing compile-time type safety
/// for the pipeline's data flow.
///
/// For operations where step N needs results from step N-2, use a composite
/// state struct that carries forward all needed values.
pub struct PipelineBuilder<'a, State> {
    pipeline: OperationPipeline<'a>,
    state: State,
}

impl<'a, State> PipelineBuilder<'a, State> {
    /// Start a new typed pipeline with initial state.
    pub fn start(ctx: &'a mut ModelingContext, initial: State) -> Self {
        Self {
            pipeline: OperationPipeline::new(ctx),
            state: initial,
        }
    }

    /// Run a step that transforms the intermediate state.
    ///
    /// The step closure receives the current state by move and a mutable
    /// reference to the `ModelingContext`. It must return the next state
    /// or an error.
    pub fn then<S, NextState, F>(
        mut self,
        step: &S,
        transform: F,
    ) -> Result<PipelineBuilder<'a, NextState>, KernelError>
    where
        S: StepContract,
        F: FnOnce(State, &mut ModelingContext) -> Result<NextState, KernelError>,
    {
        let state = self.state;
        let next = self.pipeline.run_step(step, |ctx| transform(state, ctx))?;
        Ok(PipelineBuilder {
            pipeline: self.pipeline,
            state: next,
        })
    }

    /// Finalize and return the final state + audit record.
    pub fn finish(self) -> (State, OperationAuditRecord) {
        let audit = self.pipeline.finalize();
        (self.state, audit)
    }
}
