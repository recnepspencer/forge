use super::super::schema_world::demo_schema_registry;
use crate::facade::{DeletionPolicyName, NodeContract, SourceOnlyPolicyName};
use crate::logic::transaction::SignalRuntime;
use crate::schema::data::{
    SignalSchemaDescriptor, SignalSchemaId, SignalSchemaName, SignalSchemaRegistration,
    SignalSchemaRegistry, SignalSchemaVersion,
};

#[test]
fn runtime_builder_validation_rejects_unknown_schema_default_source_only_policy() {
    let registry = SignalSchemaRegistry::from_registrations(vec![SignalSchemaRegistration::new(
        SignalSchemaDescriptor::new_with_merge_semantics(
            SignalSchemaId(74),
            SignalSchemaName::new("signal.demo.unknown-default-source-only"),
            SignalSchemaVersion::new(1, 0),
            NodeContract::wildcard(),
            None,
            None,
            None,
            Some(SourceOnlyPolicyName::new("signal.source-only.unknown")),
            None,
        ),
    )
    .expect("valid schema registration")])
    .expect("valid schema registry");
    let graph = crate::data::graph::SignalGraph::new().with_schema_registry(registry);

    let err = match SignalRuntime::builder(graph)
        .with_kernel_defaults()
        .build_validated()
    {
        Ok(_) => panic!("unknown schema default source-only policy must fail validation"),
        Err(err) => err,
    };

    assert!(err
        .to_string()
        .contains("references unknown default source-only policy"));
}

#[test]
fn runtime_builder_validation_rejects_unknown_node_source_only_policy_override() {
    let mut graph =
        crate::data::graph::SignalGraph::new().with_schema_registry(demo_schema_registry());
    graph
        .node()
        .source_only_policy_name("signal.source-only.unknown")
        .build();

    let err = match SignalRuntime::builder(graph)
        .with_kernel_defaults()
        .build_validated()
    {
        Ok(_) => panic!("unknown node source-only policy override must fail validation"),
        Err(err) => err,
    };

    assert!(err
        .to_string()
        .contains("references unknown source-only policy"));
}

#[test]
fn runtime_builder_validation_rejects_unknown_schema_default_deletion_policy() {
    let registry = SignalSchemaRegistry::from_registrations(vec![SignalSchemaRegistration::new(
        SignalSchemaDescriptor::new_with_merge_semantics(
            SignalSchemaId(75),
            SignalSchemaName::new("signal.demo.unknown-default-deletion"),
            SignalSchemaVersion::new(1, 0),
            NodeContract::wildcard(),
            None,
            None,
            None,
            None,
            Some(DeletionPolicyName::new("signal.deletion.unknown")),
        ),
    )
    .expect("valid schema registration")])
    .expect("valid schema registry");
    let graph = crate::data::graph::SignalGraph::new().with_schema_registry(registry);

    let err = match SignalRuntime::builder(graph)
        .with_kernel_defaults()
        .build_validated()
    {
        Ok(_) => panic!("unknown schema default deletion policy must fail validation"),
        Err(err) => err,
    };

    assert!(err
        .to_string()
        .contains("references unknown default deletion policy"));
}

#[test]
fn runtime_builder_validation_rejects_unknown_node_deletion_policy_override() {
    let mut graph =
        crate::data::graph::SignalGraph::new().with_schema_registry(demo_schema_registry());
    graph
        .node()
        .deletion_policy_name("signal.deletion.unknown")
        .build();

    let err = match SignalRuntime::builder(graph)
        .with_kernel_defaults()
        .build_validated()
    {
        Ok(_) => panic!("unknown node deletion policy override must fail validation"),
        Err(err) => err,
    };

    assert!(err
        .to_string()
        .contains("references unknown deletion policy"));
}
