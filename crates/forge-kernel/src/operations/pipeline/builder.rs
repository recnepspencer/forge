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
//! DEPENDENCIES: forge-core (KernelError, DecisionSink), context (OperationScope),
//! step_contract (StepContract, StepAuditEntry, OperationAuditRecord)

use forge_core::tracing::DecisionSink;
use forge_core::KernelError;

use super::step_contract::{OperationAuditRecord, StepAuditEntry, StepContract};
use crate::configuration::facade::ResolvedConfig;
use crate::context::scope::OperationScope;

/// Sequential operation pipeline with step-scoped audit.
///
/// Wraps config and sink references (destructured from `OperationScope`),
/// collects per-step audit entries with span timing via DecisionSink.
pub struct OperationPipeline<'a> {
    config: &'a ResolvedConfig,
    sink: &'a mut dyn DecisionSink,
    steps_executed: Vec<StepAuditEntry>,
}

impl<'a> OperationPipeline<'a> {
    /// Create a new pipeline from an `OperationScope`.
    ///
    /// Destructures the scope to avoid the `&'a mut T<'a>` variance trap.
    pub fn new(scope: &'a mut OperationScope<'_>) -> Self {
        Self {
            config: scope.config,
            sink: &mut *scope.sink,
            steps_executed: Vec::new(),
        }
    }

    /// Run a step with auto-injected context.
    ///
    /// 1. Validates required policies (fail-fast)
    /// 2. Records a span via DecisionSink
    /// 3. Executes the step closure with a temporary OperationScope
    /// 4. Ends span and collects step-scoped audit entry
    pub fn run_step<S, R, F>(&mut self, step: &S, execute: F) -> Result<R, KernelError>
    where
        S: StepContract,
        F: FnOnce(&mut OperationScope<'_>) -> Result<R, KernelError>,
    {
        // 1. Validate required policies (fail-fast)
        for policy in step.policy_queries() {
            self.config.validate_policy_configured(policy)?;
        }

        // 2. Start span + execute
        let step_name = step.step_name();
        let span_id = self.sink.start_span(step_name);
        let start = std::time::Instant::now();

        let mut step_scope = OperationScope::new(self.config, &mut *self.sink);
        let result = execute(&mut step_scope);

        let duration_micros = start.elapsed().as_micros() as u64;
        self.sink.end_span(span_id, duration_micros);

        // 3. Collect step audit
        self.steps_executed.push(StepAuditEntry {
            name: step_name.to_string(),
            decision_count: 0, // TODO: snapshot decision count delta from sink
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
    pub fn start(scope: &'a mut OperationScope<'_>, initial: State) -> Self {
        Self {
            pipeline: OperationPipeline::new(scope),
            state: initial,
        }
    }

    /// Run a step that transforms the intermediate state.
    ///
    /// The step closure receives the current state by move and a mutable
    /// reference to a temporary `OperationScope`. It must return the next
    /// state or an error.
    pub fn then<S, NextState, F>(
        mut self,
        step: &S,
        transform: F,
    ) -> Result<PipelineBuilder<'a, NextState>, KernelError>
    where
        S: StepContract,
        F: FnOnce(State, &mut OperationScope<'_>) -> Result<NextState, KernelError>,
    {
        let state = self.state;
        let next = self
            .pipeline
            .run_step(step, |scope| transform(state, scope))?;
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
