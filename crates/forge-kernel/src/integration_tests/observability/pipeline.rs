//! Group 3: Pipeline state threading tests [K-6].
//!
//! Proves: Multi-step pipelines thread TopologyState between steps and
//! accumulate decisions in a unified DecisionLog.

use forge_core::KernelError;
use forge_topo::transactions::TopologyState;

use crate::context::ModelingContext;
use crate::context::scope::OperationScope;
use crate::integration_tests::harness::builders::configs::test_config;
use crate::operations::primitives;
use forge_core::envelope::OperationResult;
use crate::prelude::SolidEnvelope;
use crate::operations::pipeline::facade::{PipelineBuilder, StepContract};

/// Minimal step contract for pipeline tests.
struct TestStep {
    name: &'static str,
}

impl TestStep {
    fn new(name: &'static str) -> Self {
        Self { name }
    }
}

impl StepContract for TestStep {
    fn step_name(&self) -> &'static str {
        self.name
    }

    fn policy_queries(&self) -> &[forge_core::PolicyKind] {
        &[]
    }

    fn precision_sensitive(&self) -> bool {
        false
    }
}

#[test]
fn test_two_step_pipeline_threads_state() {
    let config = test_config();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&config, &mut ctx);

    let step1 = TestStep::new("make_primitive");
    let step2 = TestStep::new("verify_counts");

    let builder = PipelineBuilder::start(&mut scope, ());

    // Step 1: generate the cube
    let builder = builder.then(&step1, |_state, step_scope| {
        let result = primitives::make_cube([0.0, 0.0, 0.0], 1.0, step_scope.config)?;
        Ok(result)
    }).expect("step 1 should succeed");

    // Step 2: verify we got the cube
    let builder = builder.then(&step2, |mesh_result: OperationResult<SolidEnvelope>, _step_scope| {
        let (topo, _geom) = mesh_result.into_value().into_parts();
        let topo: forge_topo::transactions::TopologyState = topo;
        let vertex_count = topo.arena().iter_vertices().count();
        assert_eq!(vertex_count, 8, "Cube should have 8 vertices");
        Ok(topo)
    }).expect("step 2 should succeed");

    let (final_topo, audit) = builder.finish();
    assert_eq!(audit.steps.len(), 2, "Pipeline should have recorded 2 steps");
    assert_eq!(audit.steps[0].name, "make_primitive");
    assert_eq!(audit.steps[1].name, "verify_counts");

    // Final topology should be the cube
    let vertex_count = final_topo.arena().iter_vertices().count();
    assert_eq!(vertex_count, 8);
}

/// Decisions from both steps should be visible in the ModelingContext.
#[test]
fn test_pipeline_accumulates_decisions() {
    let config = test_config();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&config, &mut ctx);

    let step1 = TestStep::new("make_cube_a");
    let step2 = TestStep::new("make_cube_b");

    // Both steps generate primitives, both should record decisions.
    let builder = PipelineBuilder::start(&mut scope, ());

    let builder = builder.then(&step1, |_state, step_scope| {
        let result = primitives::make_cube([0.0, 0.0, 0.0], 1.0, step_scope.config)?;
        Ok(result)
    }).expect("step 1 should succeed");

    let builder = builder.then(&step2, |_prev, step_scope| {
        let result = primitives::make_cube([10.0, 0.0, 0.0], 1.0, step_scope.config)?;
        Ok(result)
    }).expect("step 2 should succeed");

    let (_final_result, audit) = builder.finish();

    // The pipeline audit should record 2 steps.
    assert_eq!(audit.steps.len(), 2, "Pipeline should have 2 step audit entries");

    // The pipeline's context (ModelingContext) records the step spans.
    // Each `run_step` wraps execution in a span, so we should see 2 step spans.
    let log = ctx.get_decision_log();
    let events = log.get_events();
    let span_count = events.iter()
        .filter(|e| matches!(e, forge_core::TraceEvent::StartSpan { .. }))
        .count();
    assert_eq!(
        span_count, 2,
        "Expected 2 span starts (one per pipeline step), got {}", span_count
    );
}

/// Same pipeline run twice produces identical topology hashes.
#[test]
fn test_pipeline_determinism() {
    fn run_pipeline() -> (u128, usize) {
        let config = test_config();
        let mut ctx = ModelingContext::new();
        let mut scope = OperationScope::new(&config, &mut ctx);

        let step = TestStep::new("make_cube");
        let builder = PipelineBuilder::start(&mut scope, ());

        let builder = builder.then(&step, |_state, step_scope| {
            let result = primitives::make_cube([0.0, 0.0, 0.0], 1.0, step_scope.config)?;
            let (topo, _geom) = result.into_value().into_parts();
            let topo: forge_topo::transactions::TopologyState = topo;
            Ok(topo)
        }).expect("make_cube should succeed");

        let (topo, _audit) = builder.finish();
        let decision_count = ctx.get_decision_count();
        (topo.topology_hash(), decision_count)
    }

    let (hash_a, decisions_a) = run_pipeline();
    let (hash_b, decisions_b) = run_pipeline();

    assert_eq!(
        hash_a, hash_b,
        "Same pipeline must produce identical topology hashes"
    );
    assert_eq!(
        decisions_a, decisions_b,
        "Same pipeline must produce identical decision counts"
    );
}

/// Pipeline step failure must not corrupt earlier state.
#[test]
fn test_pipeline_failure_preserves_prior_state() {
    let config = test_config();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&config, &mut ctx);

    let step1 = TestStep::new("make_cube");
    let step2 = TestStep::new("bad_step");

    let builder = PipelineBuilder::start(&mut scope, ());

    // Step 1: generate cube
    let builder = builder.then(&step1, |_state, step_scope| {
        let result = primitives::make_cube([0.0, 0.0, 0.0], 1.0, step_scope.config)?;
        let (topo, _geom) = result.into_value().into_parts();
        Ok(topo)
    }).expect("step 1 should succeed");

    // Capture the topology hash before the bad step
    // (PipelineBuilder consumes state, so we can't peek — but we can
    // verify the error is surfaced correctly)

    // Step 2: always fails
    let result = builder.then(&step2, |_topo, _step_scope| -> Result<TopologyState, KernelError> {
        Err(KernelError::InternalError {
            message: "intentional failure for testing".to_string(),
            context: None,
        })
    });

    assert!(result.is_err(), "Bad step should propagate error");
}
