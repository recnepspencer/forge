//! Acceptance tests for the pipeline infrastructure.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use crate::configuration::facade::KernelConfig;
use crate::engine::facade::{
    FeatureAspect, FeatureDependency, FeaturePipeline, FeatureSignalPolicy, FeatureSignalTier,
    FeatureTree, SolidEnvelope,
};
use crate::geometry::facade::{transform_geometry, GeometryStore};
use crate::operations::primitives::MakePrimitiveFeature;
use crate::registry::facade::CommandDispatcher;
use forge_core::envelope::OperationResult;
use forge_core::tracing::TraceEvent;
use forge_core::KernelError;
use forge_core::PolicyKind;
use forge_geom::facade::LocalCoordinateSpace;
use forge_schema::{Command, EntityRef};
use forge_signal::facade::{EvaluationCondition, NodeId};
use forge_signal::facade::specialist::ComparatorPolicy as VersionComparatorPolicy;
use forge_topo::transactions::TopologyState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum TestBinding {
    Topology,
    Geometry,
    Both,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum TestOutput {
    Empty,
    CubeWithGeometry { center: [f64; 3], size: f64 },
    CubeTopologyOnly { center: [f64; 3], size: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum TestFeatureRegistry {
    Source {
        name: String,
        key: String,
    },
    Consumer {
        name: String,
        dependency: NodeId,
        binding: TestBinding,
        fail: bool,
        signal_policy: Option<FeatureSignalPolicy>,
    },
}

static TEST_OUTPUTS: OnceLock<Mutex<HashMap<String, TestOutput>>> = OnceLock::new();
static TEST_EXECUTIONS: OnceLock<Mutex<HashMap<String, usize>>> = OnceLock::new();
static TEST_BASE_CUBES: OnceLock<Mutex<HashMap<u64, OperationResult<SolidEnvelope>>>> =
    OnceLock::new();

fn test_outputs() -> &'static Mutex<HashMap<String, TestOutput>> {
    TEST_OUTPUTS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn test_executions() -> &'static Mutex<HashMap<String, usize>> {
    TEST_EXECUTIONS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn test_base_cubes() -> &'static Mutex<HashMap<u64, OperationResult<SolidEnvelope>>> {
    TEST_BASE_CUBES.get_or_init(|| Mutex::new(HashMap::new()))
}

fn set_test_output(key: &str, output: TestOutput) {
    test_outputs()
        .lock()
        .expect("test output lock should not poison")
        .insert(key.to_string(), output);
}

fn ensure_test_output(key: &str, output: TestOutput) {
    test_outputs()
        .lock()
        .expect("test output lock should not poison")
        .entry(key.to_string())
        .or_insert(output);
}

fn reset_test_execution(name: &str) {
    test_executions()
        .lock()
        .expect("test execution lock should not poison")
        .insert(name.to_string(), 0);
}

fn consumer_execution_count(name: &str) -> usize {
    test_executions()
        .lock()
        .expect("test execution lock should not poison")
        .get(name)
        .copied()
        .unwrap_or(0)
}

fn increment_consumer_execution(name: &str) {
    let mut executions = test_executions()
        .lock()
        .expect("test execution lock should not poison");
    *executions.entry(name.to_string()).or_insert(0) += 1;
}

fn base_cube_output(size: f64) -> Result<OperationResult<SolidEnvelope>, KernelError> {
    let key = size.to_bits();
    if let Some(envelope) = test_base_cubes()
        .lock()
        .expect("test base cube lock should not poison")
        .get(&key)
        .cloned()
    {
        return Ok(envelope);
    }

    let envelope = FeaturePipeline::execute(
        &MakePrimitiveFeature::cube("test_cube", [0.0, 0.0, 0.0], size),
        HashMap::new(),
        &KernelConfig::default(),
    )?;
    test_base_cubes()
        .lock()
        .expect("test base cube lock should not poison")
        .insert(key, envelope.clone());
    Ok(envelope)
}

fn build_test_output(output: &TestOutput) -> Result<OperationResult<SolidEnvelope>, KernelError> {
    match output {
        TestOutput::Empty => Ok(OperationResult::new(SolidEnvelope::new(
            TopologyState::empty(),
            GeometryStore::default(),
        ))),
        TestOutput::CubeWithGeometry { center, size } => {
            let envelope = base_cube_output(*size)?;
            let (topology, mut geometry) = envelope.get_value().clone().into_parts();
            if *center != [0.0, 0.0, 0.0] {
                let space = LocalCoordinateSpace::from_points(&[[0.0, 0.0, 0.0], *center]);
                transform_geometry(&mut geometry, &space);
            }
            Ok(OperationResult::new(SolidEnvelope::new(topology, geometry)))
        }
        TestOutput::CubeTopologyOnly { center: _, size } => {
            let envelope = base_cube_output(*size)?;
            Ok(OperationResult::new(SolidEnvelope::new(
                envelope.get_value().topology().clone(),
                GeometryStore::default(),
            )))
        }
    }
}

impl TestFeatureRegistry {
    fn source(name: &str, key: &str) -> Self {
        ensure_test_output(key, TestOutput::Empty);
        Self::Source {
            name: name.to_string(),
            key: key.to_string(),
        }
    }

    fn consumer(name: &str, dependency: NodeId, binding: TestBinding) -> Self {
        reset_test_execution(name);
        Self::Consumer {
            name: name.to_string(),
            dependency,
            binding,
            fail: false,
            signal_policy: None,
        }
    }

    fn failing_consumer(name: &str, dependency: NodeId, binding: TestBinding) -> Self {
        reset_test_execution(name);
        Self::Consumer {
            name: name.to_string(),
            dependency,
            binding,
            fail: true,
            signal_policy: None,
        }
    }

    fn consumer_with_policy(
        name: &str,
        dependency: NodeId,
        binding: TestBinding,
        signal_policy: FeatureSignalPolicy,
    ) -> Self {
        reset_test_execution(name);
        Self::Consumer {
            name: name.to_string(),
            dependency,
            binding,
            fail: false,
            signal_policy: Some(signal_policy),
        }
    }
}

impl crate::engine::facade::FeatureRegistry for TestFeatureRegistry {
    fn execute_via_pipeline(
        &self,
        inputs: HashMap<NodeId, SolidEnvelope>,
        _session_config: &KernelConfig,
    ) -> Result<OperationResult<SolidEnvelope>, KernelError> {
        match self {
            Self::Source { key, .. } => {
                let output = test_outputs()
                    .lock()
                    .expect("test output lock should not poison")
                    .get(key)
                    .cloned()
                    .expect("test source output should be configured");
                build_test_output(&output)
            }
            Self::Consumer {
                name,
                dependency,
                fail,
                ..
            } => {
                increment_consumer_execution(name);
                if *fail {
                    return Err(KernelError::InternalError {
                        message: format!("forced failure for {}", name),
                        context: None,
                    });
                }

                let input = inputs
                    .get(dependency)
                    .ok_or_else(|| KernelError::InvalidInput {
                        message: format!("missing dependency {}", dependency),
                        context: None,
                    })?;
                Ok(OperationResult::new(input.clone()))
            }
        }
    }

    fn dependencies(&self) -> Vec<NodeId> {
        match self {
            Self::Source { .. } => Vec::new(),
            Self::Consumer { dependency, .. } => vec![*dependency],
        }
    }

    fn dependency_bindings(&self) -> Vec<FeatureDependency> {
        match self {
            Self::Source { .. } => Vec::new(),
            Self::Consumer {
                dependency,
                binding,
                ..
            } => vec![match binding {
                TestBinding::Topology => FeatureDependency::topology(*dependency),
                TestBinding::Geometry => FeatureDependency::geometry(*dependency),
                TestBinding::Both => FeatureDependency::topology_and_geometry(*dependency),
            }],
        }
    }

    fn name(&self) -> &str {
        match self {
            Self::Source { name, .. } | Self::Consumer { name, .. } => name,
        }
    }

    fn signal_policy(&self) -> FeatureSignalPolicy {
        match self {
            Self::Source { .. } => FeatureSignalPolicy::default(),
            Self::Consumer { signal_policy, .. } => signal_policy.clone().unwrap_or_default(),
        }
    }
}

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
        output.topology().arena().face_count() > 0,
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

// ── Kernel/Signal Contract ─────────────────────────────────────────────

#[test]
fn feature_tree_repeated_topology_changes_bump_only_topology_versions() {
    let mut tree = FeatureTree::<TestFeatureRegistry>::new();
    let source_key = "topology_source";
    set_test_output(source_key, TestOutput::Empty);

    let source = tree
        .register_feature(TestFeatureRegistry::source("source", source_key))
        .expect("source should register");
    let consumer = tree
        .register_feature(TestFeatureRegistry::consumer(
            "consumer",
            source,
            TestBinding::Both,
        ))
        .expect("consumer should register");

    tree.evaluate_feature(consumer)
        .expect("initial evaluation should succeed");
    let initial_version = tree
        .get_graph()
        .get_entry(consumer)
        .expect("consumer entry should exist")
        .get_aspect_version();
    assert_eq!(initial_version.get(forge_signal::facade::Aspect::new(0)), 1);
    assert_eq!(initial_version.get(forge_signal::facade::Aspect::new(1)), 1);

    set_test_output(
        source_key,
        TestOutput::CubeTopologyOnly {
            center: [0.0, 0.0, 0.0],
            size: 2.0,
        },
    );
    tree.mark_feature_dirty(source, FeatureAspect::Topology)
        .expect("topology invalidation should succeed");
    tree.evaluate_feature(consumer)
        .expect("topology reevaluation should succeed");

    let second_version = tree
        .get_graph()
        .get_entry(consumer)
        .expect("consumer entry should exist")
        .get_aspect_version();
    assert_eq!(second_version.get(forge_signal::facade::Aspect::new(0)), 2);
    assert_eq!(second_version.get(forge_signal::facade::Aspect::new(1)), 1);

    set_test_output(source_key, TestOutput::Empty);
    tree.mark_feature_dirty(source, FeatureAspect::Topology)
        .expect("second topology invalidation should succeed");
    tree.evaluate_feature(consumer)
        .expect("second topology reevaluation should succeed");

    let third_version = tree
        .get_graph()
        .get_entry(consumer)
        .expect("consumer entry should exist")
        .get_aspect_version();
    assert_eq!(third_version.get(forge_signal::facade::Aspect::new(0)), 3);
    assert_eq!(third_version.get(forge_signal::facade::Aspect::new(1)), 1);
}

#[test]
fn feature_tree_repeated_geometry_changes_bump_only_geometry_versions() {
    let mut tree = FeatureTree::<TestFeatureRegistry>::new();
    let source_key = "geometry_source";
    set_test_output(
        source_key,
        TestOutput::CubeWithGeometry {
            center: [0.0, 0.0, 0.0],
            size: 2.0,
        },
    );

    let source = tree
        .register_feature(TestFeatureRegistry::source("source_geom", source_key))
        .expect("source should register");
    let consumer = tree
        .register_feature(TestFeatureRegistry::consumer(
            "consumer_geom",
            source,
            TestBinding::Both,
        ))
        .expect("consumer should register");

    tree.evaluate_feature(consumer)
        .expect("initial evaluation should succeed");
    let initial_source_topology = tree
        .get_envelope(source)
        .expect("source envelope should exist")
        .get_value()
        .topology_fingerprint();
    let initial_source_geometry = tree
        .get_envelope(source)
        .expect("source envelope should exist")
        .get_value()
        .geometry_fingerprint();
    let initial_consumer_topology = tree
        .get_envelope(consumer)
        .expect("consumer envelope should exist")
        .get_value()
        .topology_fingerprint();

    set_test_output(
        source_key,
        TestOutput::CubeWithGeometry {
            center: [1.0, 0.0, 0.0],
            size: 2.0,
        },
    );
    tree.mark_feature_dirty(source, FeatureAspect::Geometry)
        .expect("geometry invalidation should succeed");
    tree.evaluate_feature(consumer)
        .expect("geometry reevaluation should succeed");
    let second_source = tree
        .get_envelope(source)
        .expect("source envelope should exist")
        .get_value();
    assert_eq!(
        second_source.topology_fingerprint(),
        initial_source_topology
    );
    assert_ne!(
        second_source.geometry_fingerprint(),
        initial_source_geometry
    );
    assert_eq!(
        tree.get_envelope(consumer)
            .expect("consumer envelope should exist")
            .get_value()
            .topology_fingerprint(),
        initial_consumer_topology
    );

    let second_version = tree
        .get_graph()
        .get_entry(consumer)
        .expect("consumer entry should exist")
        .get_aspect_version();
    assert_eq!(second_version.get(forge_signal::facade::Aspect::new(0)), 1);
    assert_eq!(second_version.get(forge_signal::facade::Aspect::new(1)), 2);

    set_test_output(
        source_key,
        TestOutput::CubeWithGeometry {
            center: [2.0, 0.0, 0.0],
            size: 2.0,
        },
    );
    tree.mark_feature_dirty(source, FeatureAspect::Geometry)
        .expect("second geometry invalidation should succeed");
    tree.evaluate_feature(consumer)
        .expect("second geometry reevaluation should succeed");

    let third_version = tree
        .get_graph()
        .get_entry(consumer)
        .expect("consumer entry should exist")
        .get_aspect_version();
    assert_eq!(third_version.get(forge_signal::facade::Aspect::new(0)), 1);
    assert_eq!(third_version.get(forge_signal::facade::Aspect::new(1)), 3);
}

#[test]
fn feature_tree_maybe_stale_topology_reader_skips_when_geometry_output_is_unchanged() {
    let mut tree = FeatureTree::<TestFeatureRegistry>::new();
    let source_key = "skip_source";
    set_test_output(
        source_key,
        TestOutput::CubeWithGeometry {
            center: [0.0, 0.0, 0.0],
            size: 2.0,
        },
    );

    let source = tree
        .register_feature(TestFeatureRegistry::source("source_skip", source_key))
        .expect("source should register");
    let consumer = tree
        .register_feature(TestFeatureRegistry::consumer(
            "consumer_skip",
            source,
            TestBinding::Topology,
        ))
        .expect("consumer should register");

    tree.evaluate_feature(consumer)
        .expect("initial evaluation should succeed");
    let baseline_count = consumer_execution_count("consumer_skip");
    let baseline_version = tree
        .get_graph()
        .get_entry(consumer)
        .expect("consumer entry should exist")
        .get_aspect_version();

    tree.mark_feature_dirty(source, FeatureAspect::Geometry)
        .expect("geometry invalidation should succeed");
    tree.evaluate_feature(source)
        .expect("source reevaluation should succeed");

    assert_eq!(
        tree.get_graph()
            .get_state(consumer)
            .expect("consumer state should exist"),
        forge_signal::facade::NodeState::MaybeStale
    );

    tree.evaluate_feature(consumer)
        .expect("consumer evaluation should succeed");

    assert_eq!(
        consumer_execution_count("consumer_skip"),
        baseline_count,
        "topology-only reader should skip recomputation on unchanged geometry output"
    );
    assert_eq!(
        tree.get_graph()
            .get_entry(consumer)
            .expect("consumer entry should exist")
            .get_aspect_version(),
        baseline_version
    );
    assert_eq!(
        tree.get_graph()
            .get_state(consumer)
            .expect("consumer state should exist"),
        forge_signal::facade::NodeState::Clean
    );
}

#[test]
fn feature_tree_topology_only_dependency_is_not_directly_dirtied_by_geometry_changes() {
    let mut tree = FeatureTree::<TestFeatureRegistry>::new();
    let source_key = "topology_reader_source";
    set_test_output(
        source_key,
        TestOutput::CubeWithGeometry {
            center: [0.0, 0.0, 0.0],
            size: 2.0,
        },
    );

    let source = tree
        .register_feature(TestFeatureRegistry::source(
            "source_topology_reader",
            source_key,
        ))
        .expect("source should register");
    let consumer = tree
        .register_feature(TestFeatureRegistry::consumer(
            "consumer_topology_reader",
            source,
            TestBinding::Topology,
        ))
        .expect("consumer should register");

    tree.evaluate_feature(consumer)
        .expect("initial evaluation should succeed");
    tree.mark_feature_dirty(source, FeatureAspect::Geometry)
        .expect("geometry invalidation should succeed");

    assert_eq!(
        tree.get_graph()
            .get_state(consumer)
            .expect("consumer state should exist"),
        forge_signal::facade::NodeState::MaybeStale
    );
}

#[test]
fn feature_tree_geometry_only_dependency_is_not_directly_dirtied_by_topology_changes() {
    let mut tree = FeatureTree::<TestFeatureRegistry>::new();
    let source_key = "geometry_reader_source";
    set_test_output(source_key, TestOutput::Empty);

    let source = tree
        .register_feature(TestFeatureRegistry::source(
            "source_geometry_reader",
            source_key,
        ))
        .expect("source should register");
    let consumer = tree
        .register_feature(TestFeatureRegistry::consumer(
            "consumer_geometry_reader",
            source,
            TestBinding::Geometry,
        ))
        .expect("consumer should register");

    tree.evaluate_feature(consumer)
        .expect("initial evaluation should succeed");
    set_test_output(
        source_key,
        TestOutput::CubeTopologyOnly {
            center: [0.0, 0.0, 0.0],
            size: 2.0,
        },
    );
    tree.mark_feature_dirty(source, FeatureAspect::Topology)
        .expect("topology invalidation should succeed");

    assert_eq!(
        tree.get_graph()
            .get_state(consumer)
            .expect("consumer state should exist"),
        forge_signal::facade::NodeState::MaybeStale
    );
}

#[test]
fn feature_tree_topology_only_inputs_do_not_materialize_geometry_payload() {
    let mut tree = FeatureTree::<TestFeatureRegistry>::new();
    let source_key = "topology_only_payload_source";
    set_test_output(
        source_key,
        TestOutput::CubeWithGeometry {
            center: [0.0, 0.0, 0.0],
            size: 2.0,
        },
    );

    let source = tree
        .register_feature(TestFeatureRegistry::source(
            "topology_only_payload_source",
            source_key,
        ))
        .expect("source should register");
    let consumer = tree
        .register_feature(TestFeatureRegistry::consumer(
            "topology_only_payload_consumer",
            source,
            TestBinding::Topology,
        ))
        .expect("consumer should register");

    tree.evaluate_feature(consumer)
        .expect("evaluation should succeed");

    let source_envelope = tree
        .get_envelope(source)
        .expect("source envelope should exist")
        .get_value();
    let consumer_envelope = tree
        .get_envelope(consumer)
        .expect("consumer envelope should exist")
        .get_value();

    assert_eq!(
        consumer_envelope.topology_fingerprint(),
        source_envelope.topology_fingerprint()
    );
    assert_eq!(consumer_envelope.geometry_fingerprint(), 0);
    assert_ne!(source_envelope.geometry_fingerprint(), 0);
}

#[test]
fn feature_tree_registers_explicit_signal_policy_on_nodes() {
    let mut tree = FeatureTree::<TestFeatureRegistry>::new();
    let source = tree
        .register_feature(TestFeatureRegistry::source(
            "policy_source",
            "policy_source",
        ))
        .expect("source should register");
    let policy = FeatureSignalPolicy::core()
        .with_condition(EvaluationCondition::AspectFilter(
            forge_signal::facade::AspectMask::from_bits(FeatureAspect::Geometry.bit()),
        ))
        .with_comparator(VersionComparatorPolicy::Tolerance { epsilon: 2 })
        .with_tier(FeatureSignalTier::Core);
    let consumer = tree
        .register_feature(TestFeatureRegistry::consumer_with_policy(
            "policy_consumer",
            source,
            TestBinding::Geometry,
            policy.clone(),
        ))
        .expect("consumer should register");

    let entry = tree
        .get_graph()
        .get_entry(consumer)
        .expect("consumer entry should exist");
    assert_eq!(entry.get_eval_config(), policy.node_config());
    assert_eq!(tree.signal_tier(consumer), Some(FeatureSignalTier::Core));
}

#[test]
fn feature_tree_rejects_unsupported_signal_policies_at_registration() {
    let mut tree = FeatureTree::<TestFeatureRegistry>::new();
    let source = tree
        .register_feature(TestFeatureRegistry::source(
            "unsupported_policy_source",
            "unsupported_policy_source",
        ))
        .expect("source should register");

    let err = tree
        .register_feature(TestFeatureRegistry::consumer_with_policy(
            "unsupported_policy_consumer",
            source,
            TestBinding::Geometry,
            FeatureSignalPolicy::core().with_condition(EvaluationCondition::OnDemand),
        ))
        .expect_err("unsupported policy should be rejected");

    assert!(
        format!("{err:?}").contains("OnDemand"),
        "error should mention unsupported signal policy: {err:?}"
    );
}

#[test]
fn feature_tree_roundtrip_restores_runtime_policy_versions_and_skip_behavior() {
    let mut tree = FeatureTree::<TestFeatureRegistry>::new();
    let source_key = "roundtrip_source";
    set_test_output(
        source_key,
        TestOutput::CubeWithGeometry {
            center: [0.0, 0.0, 0.0],
            size: 2.0,
        },
    );

    let source = tree
        .register_feature(TestFeatureRegistry::source("roundtrip_source", source_key))
        .expect("source should register");
    let topo_consumer = tree
        .register_feature(TestFeatureRegistry::consumer(
            "roundtrip_topology_consumer",
            source,
            TestBinding::Topology,
        ))
        .expect("topology consumer should register");
    let geom_policy = FeatureSignalPolicy::core()
        .with_condition(EvaluationCondition::AspectFilter(
            forge_signal::facade::AspectMask::from_bits(FeatureAspect::Geometry.bit()),
        ))
        .with_comparator(VersionComparatorPolicy::Tolerance { epsilon: 2 });
    let geom_consumer = tree
        .register_feature(TestFeatureRegistry::consumer_with_policy(
            "roundtrip_geometry_consumer",
            source,
            TestBinding::Geometry,
            geom_policy.clone(),
        ))
        .expect("geometry consumer should register");

    tree.evaluate_feature(topo_consumer)
        .expect("topology consumer should evaluate");
    tree.evaluate_feature(geom_consumer)
        .expect("geometry consumer should evaluate");

    let topo_version_before = tree
        .get_graph()
        .get_entry(topo_consumer)
        .expect("topology consumer entry should exist")
        .get_aspect_version();
    let geom_version_before = tree
        .get_graph()
        .get_entry(geom_consumer)
        .expect("geometry consumer entry should exist")
        .get_aspect_version();
    let topo_fingerprint_before = tree
        .get_envelope(topo_consumer)
        .expect("topology consumer envelope should exist")
        .get_value()
        .full_fingerprint();
    let geom_fingerprint_before = tree
        .get_envelope(geom_consumer)
        .expect("geometry consumer envelope should exist")
        .get_value()
        .full_fingerprint();

    let json = serde_json::to_string(&tree).expect("FeatureTree should serialize");
    let mut restored: FeatureTree<TestFeatureRegistry> =
        serde_json::from_str(&json).expect("FeatureTree should deserialize");
    let topo_entry = restored
        .get_graph()
        .get_entry(topo_consumer)
        .expect("topology consumer entry should exist after deserialize");

    assert_eq!(
        restored.signal_tier(topo_consumer),
        Some(FeatureSignalTier::Core)
    );
    assert_eq!(
        restored.signal_tier(geom_consumer),
        Some(FeatureSignalTier::Core)
    );
    assert_eq!(
        restored
            .get_graph()
            .get_entry(geom_consumer)
            .expect("geometry consumer entry should exist after deserialize")
            .get_eval_config(),
        geom_policy.node_config()
    );
    assert_eq!(
        restored
            .get_graph()
            .get_entry(topo_consumer)
            .expect("topology consumer entry should exist after deserialize")
            .get_aspect_version(),
        topo_version_before
    );
    assert_eq!(
        restored
            .get_graph()
            .get_entry(geom_consumer)
            .expect("geometry consumer entry should exist after deserialize")
            .get_aspect_version(),
        geom_version_before
    );
    assert_eq!(
        restored
            .get_envelope(topo_consumer)
            .expect("topology consumer envelope should exist after deserialize")
            .get_value()
            .full_fingerprint(),
        topo_fingerprint_before
    );
    assert_eq!(
        restored
            .get_envelope(geom_consumer)
            .expect("geometry consumer envelope should exist after deserialize")
            .get_value()
            .full_fingerprint(),
        geom_fingerprint_before
    );

    set_test_output(
        source_key,
        TestOutput::CubeWithGeometry {
            center: [1.0, 0.0, 0.0],
            size: 2.0,
        },
    );
    restored
        .mark_feature_dirty(source, FeatureAspect::Geometry)
        .expect("geometry invalidation should succeed after deserialize");
    restored
        .evaluate_feature(topo_consumer)
        .expect("topology consumer should evaluate cleanly after deserialize");
    restored
        .evaluate_feature(geom_consumer)
        .expect("geometry consumer should evaluate after deserialize");

    assert_eq!(consumer_execution_count("roundtrip_topology_consumer"), 1);
    assert_eq!(consumer_execution_count("roundtrip_geometry_consumer"), 2);
}

#[test]
fn feature_tree_failed_evaluation_rolls_back_graph_envelope_and_trace_visibility() {
    let mut tree = FeatureTree::<TestFeatureRegistry>::new();
    let source_key = "rollback_source";
    set_test_output(
        source_key,
        TestOutput::CubeWithGeometry {
            center: [0.0, 0.0, 0.0],
            size: 2.0,
        },
    );

    let source = tree
        .register_feature(TestFeatureRegistry::source("source_rollback", source_key))
        .expect("source should register");
    let consumer = tree
        .register_feature(TestFeatureRegistry::consumer(
            "consumer_rollback",
            source,
            TestBinding::Both,
        ))
        .expect("consumer should register");

    tree.evaluate_feature(consumer)
        .expect("initial evaluation should succeed");

    let previous_envelope = tree
        .get_envelope(consumer)
        .expect("consumer envelope should exist")
        .clone();
    let previous_trace = tree
        .get_graph()
        .get_entry(consumer)
        .expect("consumer entry should exist")
        .trace_summary();
    let previous_version = tree
        .get_graph()
        .get_entry(consumer)
        .expect("consumer entry should exist")
        .get_aspect_version();

    tree.replace_feature(
        consumer,
        TestFeatureRegistry::failing_consumer("consumer_rollback", source, TestBinding::Both),
    )
    .expect("consumer replacement should succeed");

    let error = tree.evaluate_feature(consumer);
    assert!(error.is_err(), "failing consumer should propagate an error");

    assert_eq!(
        tree.get_envelope(consumer)
            .expect("consumer envelope should remain cached")
            .get_value()
            .full_fingerprint(),
        previous_envelope.get_value().full_fingerprint()
    );
    assert_eq!(
        tree.get_graph()
            .get_entry(consumer)
            .expect("consumer entry should exist")
            .trace_summary(),
        previous_trace
    );
    assert_eq!(
        tree.get_graph()
            .get_entry(consumer)
            .expect("consumer entry should exist")
            .get_aspect_version(),
        previous_version
    );
}

#[test]
fn feature_tree_replace_rewires_aspect_dependencies_without_duplicate_subscribers() {
    let mut tree = FeatureTree::<TestFeatureRegistry>::new();
    let source_key = "rewire_source";
    set_test_output(source_key, TestOutput::Empty);

    let source = tree
        .register_feature(TestFeatureRegistry::source("source_rewire", source_key))
        .expect("source should register");
    let consumer = tree
        .register_feature(TestFeatureRegistry::consumer(
            "consumer_rewire",
            source,
            TestBinding::Both,
        ))
        .expect("consumer should register");

    let source_subscribers = tree
        .get_graph()
        .subscribers_of(source)
        .expect("source subscribers should exist");
    assert_eq!(source_subscribers, &[consumer]);

    tree.replace_feature(
        consumer,
        TestFeatureRegistry::consumer("consumer_rewire", source, TestBinding::Topology),
    )
    .expect("consumer replacement should succeed");

    let source_subscribers = tree
        .get_graph()
        .subscribers_of(source)
        .expect("source subscribers should exist");
    assert_eq!(source_subscribers, &[consumer]);

    let consumer_deps = tree
        .get_graph()
        .dependencies_of(consumer)
        .expect("consumer dependencies should exist");
    assert_eq!(consumer_deps.len(), 1);
    assert_eq!(consumer_deps[0].aspect(), forge_signal::facade::Aspect::new(0));
}

// ── Tier 1: Feature Pipeline ─────────────────────────────────────────────

#[test]
fn pipeline_rejects_feature_with_missing_policy_configuration() {
    use crate::engine::facade::Feature;
    use crate::engine::facade::*;

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
            _raw: HashMap<NodeId, SolidEnvelope>,
        ) -> Result<EmptyInputs, forge_core::KernelError> {
            Ok(EmptyInputs)
        }
        fn execute_typed(
            &self,
            _inputs: EmptyInputs,
            _scope: &mut crate::context::scope::OperationScope<'_>,
        ) -> Result<forge_core::envelope::OperationResult<SolidEnvelope>, forge_core::KernelError>
        {
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
    let result = FeaturePipeline::execute(&feature, inputs, &KernelConfig::default());
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
    use crate::engine::facade::Feature;
    use crate::engine::facade::*;

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
            _raw: HashMap<NodeId, SolidEnvelope>,
        ) -> Result<StrictInputs, forge_core::KernelError> {
            Ok(StrictInputs)
        }
        fn execute_typed(
            &self,
            _inputs: StrictInputs,
            _scope: &mut crate::context::scope::OperationScope<'_>,
        ) -> Result<forge_core::envelope::OperationResult<SolidEnvelope>, forge_core::KernelError>
        {
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
    let result = FeaturePipeline::execute(&feature, inputs, &KernelConfig::default());
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
    let envelope = FeaturePipeline::execute(&feature, inputs, &KernelConfig::default())
        .expect("MakeCube through pipeline should succeed");

    let output = envelope.get_value();
    assert_eq!(
        output.topology().arena().face_count(),
        6,
        "Cube must have 6 faces"
    );
}

#[test]
fn pipeline_skips_audit_at_none_level() {
    use crate::engine::facade::Feature;
    use crate::engine::facade::*;

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
            _raw: HashMap<NodeId, SolidEnvelope>,
        ) -> Result<EmptyInputs, forge_core::KernelError> {
            Ok(EmptyInputs)
        }
        fn execute_typed(
            &self,
            _inputs: EmptyInputs,
            _scope: &mut crate::context::scope::OperationScope<'_>,
        ) -> Result<forge_core::envelope::OperationResult<SolidEnvelope>, forge_core::KernelError>
        {
            Ok(forge_core::envelope::OperationResult::new(
                SolidEnvelope::new(TopologyState::empty(), GeometryStore::default()),
            ))
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
    let result = FeaturePipeline::execute(&feature, inputs, &KernelConfig::default());
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
    use crate::engine::facade::Feature;
    use crate::engine::facade::*;

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
            _raw: HashMap<NodeId, SolidEnvelope>,
        ) -> Result<EmptyInputs, forge_core::KernelError> {
            Ok(EmptyInputs)
        }
        fn execute_typed(
            &self,
            _inputs: EmptyInputs,
            _scope: &mut crate::context::scope::OperationScope<'_>,
        ) -> Result<forge_core::envelope::OperationResult<SolidEnvelope>, forge_core::KernelError>
        {
            Ok(forge_core::envelope::OperationResult::new(
                SolidEnvelope::new(TopologyState::empty(), GeometryStore::default()),
            ))
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
    let result = FeaturePipeline::execute(&feature, inputs, &KernelConfig::default());
    assert!(
        result.is_ok(),
        "feature with ManifoldEdges invariant on empty topo should succeed"
    );

    // Now test with a real cube — ManifoldEdges should also pass for valid topology.
    let cube = MakePrimitiveFeature::cube("cube", [0.0, 0.0, 0.0], 1.0);
    let cube_result = FeaturePipeline::execute(&cube, HashMap::new(), &KernelConfig::default());
    assert!(
        cube_result.is_ok(),
        "MakeCube with ManifoldEdges invariant should pass: {:?}",
        cube_result.err()
    );
}

#[test]
fn pipeline_emits_audit_at_full_level() {
    use crate::engine::facade::Feature;
    use crate::engine::facade::*;

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
            _raw: HashMap<NodeId, SolidEnvelope>,
        ) -> Result<EmptyInputs, forge_core::KernelError> {
            Ok(EmptyInputs)
        }
        fn execute_typed(
            &self,
            _inputs: EmptyInputs,
            _scope: &mut crate::context::scope::OperationScope<'_>,
        ) -> Result<forge_core::envelope::OperationResult<SolidEnvelope>, forge_core::KernelError>
        {
            Ok(forge_core::envelope::OperationResult::new(
                SolidEnvelope::new(TopologyState::empty(), GeometryStore::default()),
            ))
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
    let envelope = FeaturePipeline::execute(&feature, inputs, &KernelConfig::default())
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
fn pipeline_make_cube_is_deterministic_across_runs() {
    let feature = MakePrimitiveFeature::cube("det_cube", [0.0, 0.0, 0.0], 2.0);
    let run_a = FeaturePipeline::execute(&feature, HashMap::new(), &KernelConfig::default())
        .expect("first pipeline run should succeed");
    let run_b = FeaturePipeline::execute(&feature, HashMap::new(), &KernelConfig::default())
        .expect("second pipeline run should succeed");

    let topo_hash_a =
        forge_topo::transactions::compute_arena_topology_hash(run_a.get_value().topology().arena());
    let topo_hash_b =
        forge_topo::transactions::compute_arena_topology_hash(run_b.get_value().topology().arena());
    assert_eq!(topo_hash_a, topo_hash_b, "topology hash drift across runs");

    assert_eq!(
        run_a.get_state_hash_before(),
        run_b.get_state_hash_before(),
        "state_hash_before drift across runs"
    );
    assert_eq!(
        run_a.get_state_hash_after(),
        run_b.get_state_hash_after(),
        "state_hash_after drift across runs"
    );
    assert_eq!(
        run_a.get_decision_log().summary(),
        run_b.get_decision_log().summary(),
        "decision summary drift across runs"
    );
    assert_eq!(
        run_a.get_extra_summaries(),
        run_b.get_extra_summaries(),
        "audit summary drift across runs"
    );
}

#[test]
fn pipeline_nested_tiers_emit_ordered_summaries() {
    let feature = MakePrimitiveFeature::cube("audit_order_cube", [0.0, 0.0, 0.0], 2.0);
    let envelope = FeaturePipeline::execute(&feature, HashMap::new(), &KernelConfig::default())
        .expect("pipeline execution should succeed");
    let summaries = envelope.get_extra_summaries();
    assert!(
        summaries.len() >= 2,
        "expected nested-tier summaries (primitive + feature), got {:?}",
        summaries
    );

    let parse_count = |s: &str| -> Option<usize> {
        let (_, rest) = s.split_once("summary: ")?;
        let (num, _) = rest.split_once(" decisions")?;
        num.parse::<usize>().ok()
    };
    let primitive_count =
        parse_count(&summaries[0]).expect("first summary should include decision count");
    let feature_count =
        parse_count(&summaries[1]).expect("second summary should include decision count");

    assert!(
        primitive_count > 0,
        "primitive-tier summary should contain decisions, got {}",
        primitive_count
    );
    assert_eq!(
        feature_count, 0,
        "feature-tier summary should reflect no direct feature-scope decisions"
    );

    let events = envelope.get_decision_log().get_events();
    let audit_spans = events
        .iter()
        .filter(|e| matches!(e, TraceEvent::StartSpan { name, .. } if name.starts_with("audit/")))
        .count();
    assert!(audit_spans >= 1, "expected at least one audit span");
}

#[test]
fn typed_inputs_reject_missing_dependency() {
    use crate::engine::facade::Feature;
    use crate::engine::facade::*;

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
            raw: HashMap<NodeId, SolidEnvelope>,
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
            _inputs: NeedyInputs,
            _scope: &mut crate::context::scope::OperationScope<'_>,
        ) -> Result<forge_core::envelope::OperationResult<SolidEnvelope>, forge_core::KernelError>
        {
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
    let fake_id = forge_signal::facade::NodeId::new(999, 0);

    let feature = NeedyFeature { dep_id: fake_id };
    let inputs = HashMap::new(); // Empty — no dependencies provided

    let result = FeaturePipeline::execute(&feature, inputs, &KernelConfig::default());

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
