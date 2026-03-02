//! Acceptance tests for the pipeline infrastructure.

use std::collections::HashMap;

use crate::registry::facade::CommandDispatcher;
use crate::geometry::facade::GeometryStore;
use crate::configuration::facade::KernelConfig;
use crate::engine::facade::FeaturePipeline;
use crate::engine::facade::FeatureOutput;
use crate::engine::facade::FeatureTree;
use crate::operations::primitives::MakePrimitiveFeature;
use forge_core::PolicyKind;
use forge_schema::{Command, EntityRef};
use forge_signal::facade::NodeId;

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
    assert!(
        result_id.is_ok(),
        "boolean dispatch should succeed: {:?}",
        result_id.err()
    );
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
    assert!(
        result_id.is_ok(),
        "union dispatch should succeed: {:?}",
        result_id.err()
    );
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
    use crate::engine::facade::*;
    use crate::engine::facade::Feature;

    /// Test feature that requires a non-existent policy.
    #[derive(Debug)]
    struct PolicyHungryFeature;

    struct EmptyInputs;
    impl FeatureInputs for EmptyInputs {
        fn validate(&self) -> Result<(), forge_core::KernelError> {
            Ok(())
        }
    }

    impl FeatureContract for PolicyHungryFeature {
        fn feature_kind(&self) -> &'static str {
            "test_policy_hungry"
        }
        fn required_policies(&self) -> &[PolicyKind] {
            &[PolicyKind::NearTangency]
        }
        fn entity_origins(&self) -> &[EntityOriginKind] {
            &[]
        }
        fn euler_ops(&self) -> &[crate::engine::facade::EulerOpKind] {
            &[]
        }
        fn surface_types(&self) -> &[crate::engine::facade::SurfaceKind] {
            &[]
        }
        fn post_invariants(&self) -> &[InvariantKind] {
            &[]
        }
        fn audit_level(&self) -> AuditLevel {
            AuditLevel::None
        }
    }

    impl Feature for PolicyHungryFeature {
        type Inputs = EmptyInputs;
        fn parse_inputs(
            &self,
            _raw: &HashMap<NodeId, FeatureOutput>,
        ) -> Result<EmptyInputs, forge_core::KernelError> {
            Ok(EmptyInputs)
        }
        fn execute_typed(
            &self,
            _inputs: &EmptyInputs,
            _config: &crate::configuration::facade::ResolvedConfig,
        ) -> Result<FeatureOutput, forge_core::KernelError> {
            unreachable!("should not reach execution with missing policy")
        }
        fn dependencies(&self) -> Vec<NodeId> {
            Vec::new()
        }
        fn name(&self) -> &str {
            "test_policy_hungry"
        }
    }

    let feature = PolicyHungryFeature;
    let inputs = HashMap::new();
    let result = FeaturePipeline::execute(&feature, &inputs, &KernelConfig::default());
    assert!(
        result.is_err(),
        "pipeline should reject feature with missing policy"
    );

    let err = format!("{:?}", result.unwrap_err());
    assert!(
        err.contains("NearTangency") || err.contains("not configured"),
        "error should mention the missing policy: {}",
        err,
    );
}

#[test]
fn pipeline_validates_inputs_before_execution() {
    use crate::engine::facade::*;
    use crate::engine::facade::Feature;

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
        fn feature_kind(&self) -> &'static str {
            "test_strict"
        }
        fn required_policies(&self) -> &[PolicyKind] {
            &[]
        }
        fn entity_origins(&self) -> &[EntityOriginKind] {
            &[]
        }
        fn euler_ops(&self) -> &[crate::engine::facade::EulerOpKind] {
            &[]
        }
        fn surface_types(&self) -> &[crate::engine::facade::SurfaceKind] {
            &[]
        }
        fn post_invariants(&self) -> &[InvariantKind] {
            &[]
        }
        fn audit_level(&self) -> AuditLevel {
            AuditLevel::None
        }
    }

    impl Feature for StrictInputFeature {
        type Inputs = StrictInputs;
        fn parse_inputs(
            &self,
            _raw: &HashMap<NodeId, FeatureOutput>,
        ) -> Result<StrictInputs, forge_core::KernelError> {
            Ok(StrictInputs)
        }
        fn execute_typed(
            &self,
            _inputs: &StrictInputs,
            _config: &crate::configuration::facade::ResolvedConfig,
        ) -> Result<FeatureOutput, forge_core::KernelError> {
            unreachable!("should not reach execution with failed validation")
        }
        fn dependencies(&self) -> Vec<NodeId> {
            Vec::new()
        }
        fn name(&self) -> &str {
            "test_strict"
        }
    }

    let feature = StrictInputFeature;
    let inputs = HashMap::new();
    let result = FeaturePipeline::execute(&feature, &inputs, &KernelConfig::default());
    assert!(
        result.is_err(),
        "pipeline should reject feature with invalid inputs"
    );

    let err = format!("{:?}", result.unwrap_err());
    assert!(
        err.contains("strict validation failed"),
        "error should contain validation message: {}",
        err,
    );
}

#[test]
fn pipeline_executes_make_cube_through_full_pipeline() {
    let feature = MakePrimitiveFeature::cube("test_cube", [0.0, 0.0, 0.0], 2.0);
    let inputs = HashMap::new();
    let envelope = FeaturePipeline::execute(&feature, &inputs, &KernelConfig::default())
        .expect("MakeCube through pipeline should succeed");

    let output = envelope.get_value();
    assert_eq!(
        output.topology.arena().face_count(),
        6,
        "Cube must have 6 faces"
    );
}

#[test]
fn pipeline_skips_audit_at_none_level() {
    use crate::engine::facade::*;
    use crate::engine::facade::Feature;

    use forge_topo::transactions::TopologyState;

    #[derive(Debug)]
    struct NoAuditFeature;

    struct EmptyInputs;
    impl FeatureInputs for EmptyInputs {
        fn validate(&self) -> Result<(), forge_core::KernelError> {
            Ok(())
        }
    }

    impl FeatureContract for NoAuditFeature {
        fn feature_kind(&self) -> &'static str {
            "test_no_audit"
        }
        fn required_policies(&self) -> &[PolicyKind] {
            &[]
        }
        fn entity_origins(&self) -> &[EntityOriginKind] {
            &[]
        }
        fn euler_ops(&self) -> &[crate::engine::facade::EulerOpKind] {
            &[]
        }
        fn surface_types(&self) -> &[crate::engine::facade::SurfaceKind] {
            &[]
        }
        fn post_invariants(&self) -> &[InvariantKind] {
            &[]
        }
        fn audit_level(&self) -> AuditLevel {
            AuditLevel::None
        }
    }

    impl Feature for NoAuditFeature {
        type Inputs = EmptyInputs;
        fn parse_inputs(
            &self,
            _raw: &HashMap<NodeId, FeatureOutput>,
        ) -> Result<EmptyInputs, forge_core::KernelError> {
            Ok(EmptyInputs)
        }
        fn execute_typed(
            &self,
            _inputs: &EmptyInputs,
            _config: &crate::configuration::facade::ResolvedConfig,
        ) -> Result<FeatureOutput, forge_core::KernelError> {
            Ok(FeatureOutput {
                topology: TopologyState::empty(),
                geometry: GeometryStore::default(),
            })
        }
        fn dependencies(&self) -> Vec<NodeId> {
            Vec::new()
        }
        fn name(&self) -> &str {
            "test_no_audit"
        }
    }

    let feature = NoAuditFeature;
    let inputs = HashMap::new();
    let result = FeaturePipeline::execute(&feature, &inputs, &KernelConfig::default());
    assert!(
        result.is_ok(),
        "no-audit feature should execute successfully"
    );

    // The envelope carries the canonical decision log (drained from ctx
    // by the OperationFinalizer). Verify no audit spans were added.
    let envelope = result.unwrap();
    let log = envelope.get_decision_log();
    let events = log.get_events();
    let audit_spans: Vec<_> = events
        .iter()
        .filter(|e| match e {
            forge_core::tracing::TraceEvent::StartSpan { name, .. } => name.starts_with("audit"),
            _ => false,
        })
        .collect();
    assert!(
        audit_spans.is_empty(),
        "AuditLevel::None should produce no audit spans, got {}",
        audit_spans.len(),
    );
}

#[test]
fn pipeline_validates_post_invariants_after_execution() {
    use crate::engine::facade::*;
    use crate::engine::facade::Feature;

    use forge_topo::transactions::TopologyState;

    /// Feature that produces an empty topology but declares ManifoldEdges
    /// as a post-invariant. An empty arena passes validation (no edges to
    /// violate), so the pipeline should succeed.
    #[derive(Debug)]
    struct InvariantFeature;

    struct EmptyInputs;
    impl FeatureInputs for EmptyInputs {
        fn validate(&self) -> Result<(), forge_core::KernelError> {
            Ok(())
        }
    }

    impl FeatureContract for InvariantFeature {
        fn feature_kind(&self) -> &'static str {
            "test_invariant"
        }
        fn required_policies(&self) -> &[PolicyKind] {
            &[]
        }
        fn entity_origins(&self) -> &[EntityOriginKind] {
            &[]
        }
        fn euler_ops(&self) -> &[crate::engine::facade::EulerOpKind] {
            &[]
        }
        fn surface_types(&self) -> &[crate::engine::facade::SurfaceKind] {
            &[]
        }
        fn post_invariants(&self) -> &[InvariantKind] {
            &[InvariantKind::ManifoldEdges]
        }
        fn audit_level(&self) -> AuditLevel {
            AuditLevel::None
        }
    }

    impl Feature for InvariantFeature {
        type Inputs = EmptyInputs;
        fn parse_inputs(
            &self,
            _raw: &HashMap<NodeId, FeatureOutput>,
        ) -> Result<EmptyInputs, forge_core::KernelError> {
            Ok(EmptyInputs)
        }
        fn execute_typed(
            &self,
            _inputs: &EmptyInputs,
            _config: &crate::configuration::facade::ResolvedConfig,
        ) -> Result<FeatureOutput, forge_core::KernelError> {
            Ok(FeatureOutput {
                topology: TopologyState::empty(),
                geometry: GeometryStore::default(),
            })
        }
        fn dependencies(&self) -> Vec<NodeId> {
            Vec::new()
        }
        fn name(&self) -> &str {
            "test_invariant"
        }
    }

    let feature = InvariantFeature;
    let inputs = HashMap::new();
    // With an empty topology, ManifoldEdges passes (no edges to violate).
    let result = FeaturePipeline::execute(&feature, &inputs, &KernelConfig::default());
    assert!(
        result.is_ok(),
        "feature with ManifoldEdges invariant on empty topo should succeed"
    );

    // Now test with a real cube — ManifoldEdges should also pass for valid topology.
    let cube = MakePrimitiveFeature::cube("cube", [0.0, 0.0, 0.0], 1.0);
    let cube_result = FeaturePipeline::execute(&cube, &inputs, &KernelConfig::default());
    assert!(
        cube_result.is_ok(),
        "MakeCube with ManifoldEdges invariant should pass: {:?}",
        cube_result.err()
    );
}

#[test]
fn pipeline_emits_audit_at_full_level() {
    use crate::engine::facade::*;
    use crate::engine::facade::Feature;

    use forge_topo::transactions::TopologyState;

    #[derive(Debug)]
    struct FullAuditFeature;

    struct EmptyInputs;
    impl FeatureInputs for EmptyInputs {
        fn validate(&self) -> Result<(), forge_core::KernelError> {
            Ok(())
        }
    }

    impl FeatureContract for FullAuditFeature {
        fn feature_kind(&self) -> &'static str {
            "test_full_audit"
        }
        fn required_policies(&self) -> &[PolicyKind] {
            &[]
        }
        fn entity_origins(&self) -> &[EntityOriginKind] {
            &[]
        }
        fn euler_ops(&self) -> &[crate::engine::facade::EulerOpKind] {
            &[]
        }
        fn surface_types(&self) -> &[crate::engine::facade::SurfaceKind] {
            &[]
        }
        fn post_invariants(&self) -> &[InvariantKind] {
            &[]
        }
        fn audit_level(&self) -> AuditLevel {
            AuditLevel::Full
        }
    }

    impl Feature for FullAuditFeature {
        type Inputs = EmptyInputs;
        fn parse_inputs(
            &self,
            _raw: &HashMap<NodeId, FeatureOutput>,
        ) -> Result<EmptyInputs, forge_core::KernelError> {
            Ok(EmptyInputs)
        }
        fn execute_typed(
            &self,
            _inputs: &EmptyInputs,
            _config: &crate::configuration::facade::ResolvedConfig,
        ) -> Result<FeatureOutput, forge_core::KernelError> {
            Ok(FeatureOutput {
                topology: TopologyState::empty(),
                geometry: GeometryStore::default(),
            })
        }
        fn dependencies(&self) -> Vec<NodeId> {
            Vec::new()
        }
        fn name(&self) -> &str {
            "test_full_audit"
        }
    }

    let feature = FullAuditFeature;
    let inputs = HashMap::new();
    let envelope = FeaturePipeline::execute(&feature, &inputs, &KernelConfig::default())
        .expect("full-audit feature should succeed");

    // AuditLevel::Full should produce an audit span in the envelope's log.
    // The executor records a span named "audit/{feature_kind}".
    let log = envelope.get_decision_log();
    let events = log.get_events();
    let audit_spans: Vec<_> = events
        .iter()
        .filter(|e| match e {
            forge_core::tracing::TraceEvent::StartSpan { name, .. } => name.starts_with("audit/"),
            _ => false,
        })
        .collect();
    assert!(
        !audit_spans.is_empty(),
        "AuditLevel::Full should produce at least one audit span, got none. Events: {:?}",
        events,
    );
}

#[test]
fn typed_inputs_reject_missing_dependency() {
    use crate::engine::facade::*;
    use crate::engine::facade::Feature;

    /// Feature that requires a dependency but the input map is empty.
    #[derive(Debug)]
    struct NeedyFeature {
        dep_id: NodeId,
    }

    struct NeedyInputs;
    impl FeatureInputs for NeedyInputs {
        fn validate(&self) -> Result<(), forge_core::KernelError> {
            Ok(())
        }
    }

    impl FeatureContract for NeedyFeature {
        fn feature_kind(&self) -> &'static str {
            "test_needy"
        }
        fn required_policies(&self) -> &[PolicyKind] {
            &[]
        }
        fn entity_origins(&self) -> &[EntityOriginKind] {
            &[]
        }
        fn euler_ops(&self) -> &[crate::engine::facade::EulerOpKind] {
            &[]
        }
        fn surface_types(&self) -> &[crate::engine::facade::SurfaceKind] {
            &[]
        }
        fn post_invariants(&self) -> &[InvariantKind] {
            &[]
        }
        fn audit_level(&self) -> AuditLevel {
            AuditLevel::None
        }
    }

    impl Feature for NeedyFeature {
        type Inputs = NeedyInputs;
        fn parse_inputs(
            &self,
            raw: &HashMap<NodeId, FeatureOutput>,
        ) -> Result<NeedyInputs, forge_core::KernelError> {
            if !raw.contains_key(&self.dep_id) {
                return Err(forge_core::KernelError::InvalidInput {
                    message: format!("Missing required dependency {}", self.dep_id),
                    context: None,
                });
            }
            Ok(NeedyInputs)
        }
        fn execute_typed(
            &self,
            _inputs: &NeedyInputs,
            _config: &crate::configuration::facade::ResolvedConfig,
        ) -> Result<FeatureOutput, forge_core::KernelError> {
            unreachable!("should not reach execution with missing dependency")
        }
        fn dependencies(&self) -> Vec<NodeId> {
            vec![self.dep_id]
        }
        fn name(&self) -> &str {
            "test_needy"
        }
    }

    // Create a fake NodeId that won't exist in the input map.
    let fake_id = {
        let mut graph = forge_signal::facade::SignalGraph::new();
        graph.create_node()
    };

    let feature = NeedyFeature { dep_id: fake_id };
    let inputs = HashMap::new(); // Empty — no dependencies provided

    let result = FeaturePipeline::execute(&feature, &inputs, &KernelConfig::default());

    assert!(
        result.is_err(),
        "should fail when required dependency is missing"
    );
    let err = format!("{:?}", result.unwrap_err());
    assert!(
        err.contains("Missing required dependency") || err.contains("missing"),
        "error should mention the missing dependency: {}",
        err,
    );
}
