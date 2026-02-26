//! Acceptance tests for the pipeline infrastructure.

use std::collections::HashMap;

use forge_core::PolicyKind;
use forge_schema::{Command, EntityRef};
use forge_signal::handles::NodeId;
use crate::core::ModelingContext;
use crate::features::tree::FeatureTree;
use crate::features::traits::FeatureOutput;
use crate::features::pipeline::executor::FeaturePipeline;
use crate::features::wrappers::{MakeCubeFeature, BooleanFeature};
use super::dispatch::CommandDispatcher;

// ── Tier 0: Command Dispatch ─────────────────────────────────────────────

#[test]
fn dispatch_add_block_creates_make_cube_feature() {
    let mut tree = FeatureTree::new();
    let mut dispatcher = CommandDispatcher::new(&mut tree);

    let cmd = Command::AddBlock {
        origin: [0.0, 0.0, 0.0],
        dimensions: [10.0, 5.0, 3.0],
    };

    let node_id = dispatcher.dispatch(&cmd).expect("dispatch should succeed");

    let output = tree
        .evaluate_feature(node_id)
        .expect("evaluation should succeed");

    assert!(
        output.topology.arena().face_count() > 0,
        "block should produce faces"
    );
}

#[test]
fn dispatch_boolean_subtract_resolves_entity_refs() {
    let mut tree = FeatureTree::new();
    let mut dispatcher = CommandDispatcher::new(&mut tree);

    let block_a = Command::AddBlock {
        origin: [0.0, 0.0, 0.0],
        dimensions: [10.0, 10.0, 10.0],
    };
    let block_b = Command::AddBlock {
        origin: [2.0, 2.0, 2.0],
        dimensions: [4.0, 4.0, 4.0],
    };

    dispatcher.dispatch(&block_a).expect("block A");
    dispatcher.dispatch(&block_b).expect("block B");

    let subtract = Command::BooleanSubtract {
        target: EntityRef::ByIndex { index: 0 },
        tool: EntityRef::ByIndex { index: 1 },
    };

    let result_id = dispatcher.dispatch(&subtract);
    assert!(result_id.is_ok(), "boolean dispatch should succeed: {:?}", result_id.err());
}

#[test]
fn dispatch_boolean_union_resolves_entity_refs() {
    let mut tree = FeatureTree::new();
    let mut dispatcher = CommandDispatcher::new(&mut tree);

    let block_a = Command::AddBlock {
        origin: [0.0, 0.0, 0.0],
        dimensions: [10.0, 10.0, 10.0],
    };
    let block_b = Command::AddBlock {
        origin: [5.0, 0.0, 0.0],
        dimensions: [10.0, 10.0, 10.0],
    };

    dispatcher.dispatch(&block_a).expect("block A");
    dispatcher.dispatch(&block_b).expect("block B");

    let union = Command::BooleanUnion {
        target: EntityRef::ByIndex { index: 0 },
        tool: EntityRef::ByIndex { index: 1 },
    };

    let result_id = dispatcher.dispatch(&union);
    assert!(result_id.is_ok(), "union dispatch should succeed: {:?}", result_id.err());
}

#[test]
fn dispatch_unknown_entity_ref_returns_error() {
    let mut tree = FeatureTree::new();
    let mut dispatcher = CommandDispatcher::new(&mut tree);

    let subtract = Command::BooleanSubtract {
        target: EntityRef::ByFeature {
            feature_name: "nonexistent".to_string(),
            selector: None,
        },
        tool: EntityRef::ByIndex { index: 99 },
    };

    let result = dispatcher.dispatch(&subtract);
    assert!(result.is_err(), "should fail on unknown entity ref");

    let err_msg = format!("{:?}", result.unwrap_err());
    assert!(
        err_msg.contains("not found") || err_msg.contains("out of range"),
        "error should mention resolution failure: {}",
        err_msg
    );
}

// ── Tier 1: Feature Pipeline ─────────────────────────────────────────────

#[test]
fn pipeline_rejects_feature_with_missing_policy_configuration() {
    use crate::features::pipeline::contract::*;
    use crate::features::traits::Feature;

    /// Test feature that requires a non-existent policy.
    #[derive(Debug)]
    struct PolicyHungryFeature;

    struct EmptyInputs;
    impl FeatureInputs for EmptyInputs {
        fn validate(&self) -> Result<(), forge_core::KernelError> { Ok(()) }
    }

    impl FeatureContract for PolicyHungryFeature {
        fn feature_kind(&self) -> &str { "test_policy_hungry" }
        fn required_policies(&self) -> &[PolicyKind] {
            &[PolicyKind::NearTangency]
        }
        fn entity_origins(&self) -> &[EntityOriginKind] { &[] }
        fn post_invariants(&self) -> &[InvariantKind] { &[] }
        fn audit_level(&self) -> AuditLevel { AuditLevel::None }
    }

    impl Feature for PolicyHungryFeature {
        type Inputs = EmptyInputs;
        fn parse_inputs(&self, _raw: &HashMap<NodeId, FeatureOutput>) -> Result<EmptyInputs, forge_core::KernelError> {
            Ok(EmptyInputs)
        }
        fn execute_typed(&self, _inputs: &EmptyInputs, _ctx: &mut ModelingContext) -> Result<FeatureOutput, forge_core::KernelError> {
            unreachable!("should not reach execution with missing policy")
        }
        fn dependencies(&self) -> Vec<NodeId> { Vec::new() }
        fn name(&self) -> &str { "test_policy_hungry" }
    }

    let feature = PolicyHungryFeature;
    let inputs = HashMap::new();
    let mut ctx = ModelingContext::new();

    let result = FeaturePipeline::execute(&feature, &inputs, &mut ctx);
    assert!(result.is_err(), "pipeline should reject feature with missing policy");

    let err = format!("{:?}", result.unwrap_err());
    assert!(
        err.contains("NearTangency") || err.contains("not configured"),
        "error should mention the missing policy: {}",
        err,
    );
}

#[test]
fn pipeline_validates_inputs_before_execution() {
    use crate::features::pipeline::contract::*;
    use crate::features::traits::Feature;

    #[derive(Debug)]
    struct StrictInputFeature;

    struct StrictInputs;
    impl FeatureInputs for StrictInputs {
        fn validate(&self) -> Result<(), forge_core::KernelError> {
            Err(forge_core::KernelError::InvalidInput {
                message: "strict validation failed".into(),
                context: None,
            })
        }
    }

    impl FeatureContract for StrictInputFeature {
        fn feature_kind(&self) -> &str { "test_strict" }
        fn required_policies(&self) -> &[PolicyKind] { &[] }
        fn entity_origins(&self) -> &[EntityOriginKind] { &[] }
        fn post_invariants(&self) -> &[InvariantKind] { &[] }
        fn audit_level(&self) -> AuditLevel { AuditLevel::None }
    }

    impl Feature for StrictInputFeature {
        type Inputs = StrictInputs;
        fn parse_inputs(&self, _raw: &HashMap<NodeId, FeatureOutput>) -> Result<StrictInputs, forge_core::KernelError> {
            Ok(StrictInputs)
        }
        fn execute_typed(&self, _inputs: &StrictInputs, _ctx: &mut ModelingContext) -> Result<FeatureOutput, forge_core::KernelError> {
            unreachable!("should not reach execution with failed validation")
        }
        fn dependencies(&self) -> Vec<NodeId> { Vec::new() }
        fn name(&self) -> &str { "test_strict" }
    }

    let feature = StrictInputFeature;
    let inputs = HashMap::new();
    let mut ctx = ModelingContext::new();

    let result = FeaturePipeline::execute(&feature, &inputs, &mut ctx);
    assert!(result.is_err(), "pipeline should reject feature with invalid inputs");

    let err = format!("{:?}", result.unwrap_err());
    assert!(
        err.contains("strict validation failed"),
        "error should contain validation message: {}",
        err,
    );
}

#[test]
fn pipeline_executes_make_cube_through_full_pipeline() {
    let feature = MakeCubeFeature::new("test_cube", [0.0, 0.0, 0.0], 2.0);
    let inputs = HashMap::new();
    let mut ctx = ModelingContext::new();

    let output = FeaturePipeline::execute(&feature, &inputs, &mut ctx)
        .expect("MakeCube through pipeline should succeed");

    assert_eq!(
        output.topology.arena().face_count(), 6,
        "Cube must have 6 faces"
    );
}

#[test]
fn pipeline_skips_audit_at_none_level() {
    use crate::features::pipeline::contract::*;
    use crate::features::traits::Feature;
    use forge_topo::state::TopologyState;
    use crate::geometry_state::GeometryState;

    #[derive(Debug)]
    struct NoAuditFeature;

    struct EmptyInputs;
    impl FeatureInputs for EmptyInputs {
        fn validate(&self) -> Result<(), forge_core::KernelError> { Ok(()) }
    }

    impl FeatureContract for NoAuditFeature {
        fn feature_kind(&self) -> &str { "test_no_audit" }
        fn required_policies(&self) -> &[PolicyKind] { &[] }
        fn entity_origins(&self) -> &[EntityOriginKind] { &[] }
        fn post_invariants(&self) -> &[InvariantKind] { &[] }
        fn audit_level(&self) -> AuditLevel { AuditLevel::None }
    }

    impl Feature for NoAuditFeature {
        type Inputs = EmptyInputs;
        fn parse_inputs(&self, _raw: &HashMap<NodeId, FeatureOutput>) -> Result<EmptyInputs, forge_core::KernelError> {
            Ok(EmptyInputs)
        }
        fn execute_typed(&self, _inputs: &EmptyInputs, _ctx: &mut ModelingContext) -> Result<FeatureOutput, forge_core::KernelError> {
            Ok(FeatureOutput {
                topology: TopologyState::empty(),
                geometry: GeometryState::new(),
            })
        }
        fn dependencies(&self) -> Vec<NodeId> { Vec::new() }
        fn name(&self) -> &str { "test_no_audit" }
    }

    let feature = NoAuditFeature;
    let inputs = HashMap::new();
    let mut ctx = ModelingContext::new();

    let result = FeaturePipeline::execute(&feature, &inputs, &mut ctx);
    assert!(result.is_ok(), "no-audit feature should execute successfully");

    let log = ctx.get_decision_log();
    let events = log.get_events();
    let audit_spans: Vec<_> = events.iter().filter(|e| match e {
        forge_core::tracing::TraceEvent::StartSpan { name, .. } => name == "audit",
        _ => false,
    }).collect();
    assert!(
        audit_spans.is_empty(),
        "AuditLevel::None should produce no audit spans, got {}",
        audit_spans.len(),
    );
}
