use super::super::schema_world::demo_schema_registry;
use crate::facade::{
    Aspect, AspectMergePolicyBinding, AspectMergePolicyName, ConflictIsolationPolicyName,
    NodeContract,
};
use crate::logic::transaction::SignalRuntime;
use crate::schema::data::{
    SignalSchemaDescriptor, SignalSchemaId, SignalSchemaName, SignalSchemaRegistration,
    SignalSchemaRegistry, SignalSchemaVersion,
};

#[test]
fn runtime_builder_validation_rejects_unknown_schema_default_aspect_merge_policy() {
    let registry = SignalSchemaRegistry::from_registrations(vec![SignalSchemaRegistration::new(
        SignalSchemaDescriptor::new_with_merge_semantics_and_aspects(
            SignalSchemaId(76),
            SignalSchemaName::new("signal.demo.unknown-default-aspect-policy"),
            SignalSchemaVersion::new(1, 0),
            NodeContract::wildcard(),
            None,
            None,
            None,
            None,
            None,
            vec![AspectMergePolicyBinding::new(
                Aspect::new(0),
                AspectMergePolicyName::new("signal.aspect.unknown"),
            )],
        ),
    )
    .expect("valid schema registration")])
    .expect("valid schema registry");
    let graph = crate::data::graph::SignalGraph::new().with_schema_registry(registry);

    let err = match SignalRuntime::builder(graph)
        .with_kernel_defaults()
        .build_validated()
    {
        Ok(_) => panic!("unknown schema default aspect merge policy must fail validation"),
        Err(err) => err,
    };

    assert!(err
        .to_string()
        .contains("references unknown aspect merge policy"));
}

#[test]
fn runtime_builder_validation_rejects_unknown_node_aspect_merge_policy_override() {
    let mut graph =
        crate::data::graph::SignalGraph::new().with_schema_registry(demo_schema_registry());
    graph
        .node()
        .aspect_merge_policy_name(Aspect::new(1), "signal.aspect.unknown")
        .build();

    let err = match SignalRuntime::builder(graph)
        .with_kernel_defaults()
        .build_validated()
    {
        Ok(_) => panic!("unknown node aspect merge policy override must fail validation"),
        Err(err) => err,
    };

    assert!(err
        .to_string()
        .contains("references unknown aspect merge policy"));
}

#[test]
fn runtime_builder_validation_rejects_unknown_schema_default_conflict_isolation_policy() {
    let registry = SignalSchemaRegistry::from_registrations(vec![SignalSchemaRegistration::new(
        SignalSchemaDescriptor::new_with_merge_semantics_and_isolation(
            SignalSchemaId(77),
            SignalSchemaName::new("signal.demo.unknown-default-conflict-isolation"),
            SignalSchemaVersion::new(1, 0),
            NodeContract::wildcard(),
            None,
            None,
            None,
            None,
            None,
            Some(ConflictIsolationPolicyName::new(
                "signal.conflict-isolation.unknown",
            )),
        ),
    )
    .expect("valid schema registration")])
    .expect("valid schema registry");
    let graph = crate::data::graph::SignalGraph::new().with_schema_registry(registry);

    let err = match SignalRuntime::builder(graph)
        .with_kernel_defaults()
        .build_validated()
    {
        Ok(_) => panic!("unknown schema default conflict isolation policy must fail validation"),
        Err(err) => err,
    };

    assert!(err
        .to_string()
        .contains("references unknown default conflict isolation policy"));
}

#[test]
fn runtime_builder_validation_rejects_unknown_node_conflict_isolation_policy_override() {
    let mut graph =
        crate::data::graph::SignalGraph::new().with_schema_registry(demo_schema_registry());
    graph
        .node()
        .conflict_isolation_policy_name("signal.conflict-isolation.unknown")
        .build();

    let err = match SignalRuntime::builder(graph)
        .with_kernel_defaults()
        .build_validated()
    {
        Ok(_) => panic!("unknown node conflict isolation policy override must fail validation"),
        Err(err) => err,
    };

    assert!(err
        .to_string()
        .contains("references unknown conflict isolation policy"));
}
