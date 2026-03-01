//! Acceptance tests for the operation pipeline (Tier 2).

use forge_core::{KernelError, PolicyKind};

use crate::context::facade::ModelingContext;
use crate::operations::pipeline::builder::{OperationPipeline, PipelineBuilder};
use crate::operations::pipeline::step_contract::StepContract;

// ── Step Fixtures ────────────────────────────────────────────────────────

/// A step with no policies and no precision sensitivity.
struct SimpleStep;

crate::declare_step!(SimpleStep,
    name: "simple_step",
    policies: [],
    precision_sensitive: false,
);

/// A step requiring NearTangency policy (not in default registry).
struct PolicyRequiringStep;

crate::declare_step!(PolicyRequiringStep,
    name: "policy_requiring_step",
    policies: [PolicyKind::NearTangency],
    precision_sensitive: true,
);

/// A step that just labels itself.
struct LabelStep;

crate::declare_step!(LabelStep,
    name: "label_step",
    policies: [],
    precision_sensitive: false,
);

/// A precision-sensitive step.
struct PrecisionStep;

crate::declare_step!(PrecisionStep,
    name: "precision_step",
    policies: [],
    precision_sensitive: true,
);

// ── Tests ────────────────────────────────────────────────────────────────

#[test]
fn run_step_rejects_missing_policy_before_execution() {
    let mut ctx = ModelingContext::new();
    let mut pipeline = OperationPipeline::new(&mut ctx);

    let result: Result<(), KernelError> = pipeline.run_step(&PolicyRequiringStep, |_ctx| {
        unreachable!("step should not execute with missing policy")
    });

    assert!(result.is_err(), "run_step should reject missing policy");
    let err = format!("{:?}", result.unwrap_err());
    assert!(
        err.contains("NearTangency") || err.contains("not configured"),
        "error should mention NearTangency: {}",
        err,
    );
}

#[test]
fn run_step_collects_step_scoped_decision_counts() {
    let mut ctx = ModelingContext::new();
    let mut pipeline = OperationPipeline::new(&mut ctx);

    // Step 1: no decisions
    pipeline
        .run_step(&SimpleStep, |_ctx| Ok(42u32))
        .expect("simple step should succeed");

    // Step 2: also no decisions (but records the entry)
    pipeline
        .run_step(&LabelStep, |_ctx| Ok("labeled"))
        .expect("label step should succeed");

    let audit = pipeline.finalize();

    assert_eq!(audit.step_count(), 2, "two steps should be recorded");
    assert_eq!(audit.steps[0].name, "simple_step");
    assert_eq!(audit.steps[1].name, "label_step");
    assert_eq!(audit.steps[0].decision_count, 0);
    assert_eq!(audit.steps[1].decision_count, 0);
    assert!(!audit.steps[0].precision_sensitive);
    assert!(!audit.steps[1].precision_sensitive);
}

#[test]
fn pipeline_builder_threads_typed_state_through_steps() {
    let mut ctx = ModelingContext::new();

    // Pipeline: u32 → String → Vec<String>
    let (result, audit) = PipelineBuilder::start(&mut ctx, 42u32)
        .then(&SimpleStep, |n, _ctx| Ok(format!("number-{}", n)))
        .expect("step 1")
        .then(&LabelStep, |s, _ctx| {
            Ok(vec![s.clone(), format!("{}-copy", s)])
        })
        .expect("step 2")
        .finish();

    assert_eq!(
        result,
        vec!["number-42".to_string(), "number-42-copy".to_string()]
    );
    assert_eq!(audit.step_count(), 2);
    assert_eq!(audit.steps[0].name, "simple_step");
    assert_eq!(audit.steps[1].name, "label_step");
}

#[test]
fn multi_step_pipeline_sequences_audit_entries_in_order() {
    let mut ctx = ModelingContext::new();
    let mut pipeline = OperationPipeline::new(&mut ctx);

    // Use distinct step structs for each name
    struct AlphaStep;
    crate::declare_step!(AlphaStep, name: "alpha", policies: [], precision_sensitive: true);
    struct BetaStep;
    crate::declare_step!(BetaStep, name: "beta", policies: [], precision_sensitive: false);
    struct GammaStep;
    crate::declare_step!(GammaStep, name: "gamma", policies: [], precision_sensitive: true);
    struct DeltaStep;
    crate::declare_step!(DeltaStep, name: "delta", policies: [], precision_sensitive: false);

    pipeline.run_step(&AlphaStep, |_ctx| Ok(())).expect("alpha");
    pipeline.run_step(&BetaStep, |_ctx| Ok(())).expect("beta");
    pipeline.run_step(&GammaStep, |_ctx| Ok(())).expect("gamma");
    pipeline.run_step(&DeltaStep, |_ctx| Ok(())).expect("delta");

    let audit = pipeline.finalize();

    let step_names = ["alpha", "beta", "gamma", "delta"];
    for (i, expected) in step_names.iter().enumerate() {
        assert_eq!(
            audit.steps[i].name, *expected,
            "step {} should be '{}', got '{}'",
            i, expected, audit.steps[i].name,
        );
    }

    assert_eq!(audit.step_count(), 4);

    // Verify precision_sensitive flags
    let prec_flags: Vec<bool> = audit.steps.iter().map(|s| s.precision_sensitive).collect();
    assert_eq!(prec_flags, vec![true, false, true, false]);
}

#[test]
fn pipeline_builder_propagates_error_from_step() {
    let mut ctx = ModelingContext::new();

    let result = PipelineBuilder::start(&mut ctx, 100u32).then(
        &SimpleStep,
        |_n, _ctx| -> Result<u32, KernelError> {
            Err(KernelError::InvalidInput {
                message: "intentional failure".into(),
                context: None,
            })
        },
    );

    match result {
        Ok(_) => panic!("builder should propagate step error"),
        Err(e) => {
            let err = format!("{:?}", e);
            assert!(
                err.contains("intentional failure"),
                "error should contain failure message: {}",
                err,
            );
        }
    }
}

#[test]
fn pipeline_builder_six_step_chain_compiles_and_runs() {
    let mut ctx = ModelingContext::new();

    // Simulate a fillet-like 6-step pipeline with type changes at each step
    let (result, audit) = PipelineBuilder::start(&mut ctx, vec![1u32, 2, 3])
        .then(&SimpleStep, |edges, _ctx| {
            // Step 1: "resolve selection"
            Ok(edges.len())
        })
        .expect("resolve")
        .then(&PrecisionStep, |count, _ctx| {
            // Step 2: "classify convexity"
            Ok((count, true)) // (edge_count, all_convex)
        })
        .expect("classify")
        .then(&PrecisionStep, |(_count, convex), _ctx| {
            // Step 3: "construct surface"
            Ok(format!("surface-convex={}", convex))
        })
        .expect("construct")
        .then(&SimpleStep, |surface, _ctx| {
            // Step 4: "apply euler ops"
            Ok((surface, 12usize)) // (surface, face_count)
        })
        .expect("euler")
        .then(&LabelStep, |(_surface, faces), _ctx| {
            // Step 5: "validate manifold"
            assert!(faces > 0);
            Ok(faces)
        })
        .expect("validate")
        .then(&PrecisionStep, |faces, _ctx| {
            // Step 6: "detect slivers"
            Ok(format!("{} faces, 0 slivers", faces))
        })
        .expect("detect")
        .finish();

    assert_eq!(result, "12 faces, 0 slivers");
    assert_eq!(audit.step_count(), 6, "should have 6 steps");

    // Verify precision_sensitive flags
    let prec_flags: Vec<bool> = audit.steps.iter().map(|s| s.precision_sensitive).collect();
    assert_eq!(prec_flags, vec![false, true, true, false, false, true]);
}
