use super::super::schema_world::{contract_backed_schema_registry, demo_schema_registry};
use crate::facade::{ArtifactPolicyClass, MaintenanceMode, NodeContract, PathClass};
use crate::logic::transaction::SignalRuntime;
use crate::schema::data::SignalSchemaId;
use crate::tests::support::{ASPECT_A, ASPECT_B};

#[test]
fn runtime_builder_carries_frozen_schema_registry() {
    let registry = demo_schema_registry();

    let runtime = SignalRuntime::builder(crate::data::graph::SignalGraph::new())
        .with_kernel_defaults()
        .schema_registry(registry.clone())
        .build();

    assert_eq!(runtime.schema_registry(), &registry);
}

#[test]
fn runtime_stock_constructor_with_schema_preserves_registry() {
    let registry = demo_schema_registry();

    let runtime = SignalRuntime::build_for_with_schema::<()>(
        crate::data::graph::SignalGraph::new(),
        registry.clone(),
    );

    assert_eq!(runtime.schema_registry(), &registry);
}

#[test]
fn graph_node_builder_inherits_schema_default_contract() {
    let schema_contract = NodeContract::reads(ASPECT_A)
        .with_produces(ASPECT_B)
        .with_path_class(PathClass::Rich)
        .with_maintenance_mode(MaintenanceMode::IncrementalOnly)
        .with_artifact_policy(ArtifactPolicyClass::ForensicReconstructable);
    let registry = contract_backed_schema_registry(schema_contract.clone());
    let mut graph = crate::data::graph::SignalGraph::new().with_schema_registry(registry);

    let node = graph
        .node()
        .schema_name("signal.demo.schema-bound")
        .expect("known schema")
        .build();

    assert_eq!(
        graph.get_contract(node).expect("node contract"),
        &schema_contract
    );
    let binding = graph
        .node_schema_binding(node)
        .expect("node schema binding")
        .expect("schema binding present");
    assert_eq!(binding.schema_id(), SignalSchemaId(7));
    assert_eq!(binding.semantic_name().as_str(), "signal.demo.schema-bound");
}

#[test]
fn graph_node_builder_rejects_unknown_schema_name() {
    let mut graph =
        crate::data::graph::SignalGraph::new().with_schema_registry(demo_schema_registry());

    let err = match graph.node().schema_name("signal.demo.unknown") {
        Ok(_) => panic!("unknown schema must fail"),
        Err(err) => err,
    };

    assert!(err.to_string().contains("unknown signal schema"));
}

#[test]
fn runtime_inherits_graph_schema_registry_when_builder_registry_is_empty() {
    let registry = demo_schema_registry();
    let graph = crate::data::graph::SignalGraph::new().with_schema_registry(registry.clone());

    let runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();

    assert_eq!(runtime.schema_registry(), &registry);
}

#[test]
fn runtime_builder_validation_rejects_schema_digest_drift() {
    let source_registry = contract_backed_schema_registry(
        NodeContract::reads(ASPECT_A).with_path_class(PathClass::Rich),
    );
    let drifted_registry = contract_backed_schema_registry(
        NodeContract::reads(ASPECT_B).with_path_class(PathClass::Operational),
    );
    let mut graph = crate::data::graph::SignalGraph::new().with_schema_registry(source_registry);
    graph
        .node()
        .schema_id(SignalSchemaId(7))
        .expect("known schema")
        .build();

    let err = match SignalRuntime::builder(graph)
        .with_kernel_defaults()
        .schema_registry(drifted_registry)
        .build_validated()
    {
        Ok(_) => panic!("schema digest drift must fail validation"),
        Err(err) => err,
    };

    assert!(err.to_string().contains("schema binding digest mismatch"));
}
