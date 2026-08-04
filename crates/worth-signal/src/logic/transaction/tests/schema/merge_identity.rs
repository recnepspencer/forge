use super::super::schema_world::demo_schema_registry;
use crate::facade::{ConflictPolicyName, IdentityMatcherName, MergeStrategyName, NodeContract};
use crate::logic::transaction::SignalRuntime;
use crate::schema::data::{
    SignalSchemaDescriptor, SignalSchemaId, SignalSchemaName, SignalSchemaRegistration,
    SignalSchemaRegistry, SignalSchemaVersion,
};

#[test]
fn runtime_builder_validation_rejects_unknown_schema_default_merge_strategy() {
    let registry = SignalSchemaRegistry::from_registrations(vec![SignalSchemaRegistration::new(
        SignalSchemaDescriptor::new_with_merge_semantics(
            SignalSchemaId(71),
            SignalSchemaName::new("signal.demo.unknown-default-strategy"),
            SignalSchemaVersion::new(1, 0),
            NodeContract::wildcard(),
            Some(MergeStrategyName::new("signal.merge.unknown")),
            None,
            None,
            None,
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
        Ok(_) => panic!("unknown schema default merge strategy must fail validation"),
        Err(err) => err,
    };

    assert!(err
        .to_string()
        .contains("references unknown default merge strategy"));
}

#[test]
fn runtime_builder_validation_rejects_unknown_schema_default_conflict_policy() {
    let registry = SignalSchemaRegistry::from_registrations(vec![SignalSchemaRegistration::new(
        SignalSchemaDescriptor::new_with_merge_semantics(
            SignalSchemaId(72),
            SignalSchemaName::new("signal.demo.unknown-default-conflict"),
            SignalSchemaVersion::new(1, 0),
            NodeContract::wildcard(),
            None,
            Some(ConflictPolicyName::new("signal.conflict.unknown")),
            None,
            None,
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
        Ok(_) => panic!("unknown schema default conflict policy must fail validation"),
        Err(err) => err,
    };

    assert!(err
        .to_string()
        .contains("references unknown default conflict policy"));
}

#[test]
fn runtime_builder_validation_rejects_unknown_schema_default_identity_matcher() {
    let registry = SignalSchemaRegistry::from_registrations(vec![SignalSchemaRegistration::new(
        SignalSchemaDescriptor::new_with_merge_semantics(
            SignalSchemaId(73),
            SignalSchemaName::new("signal.demo.unknown-default-identity"),
            SignalSchemaVersion::new(1, 0),
            NodeContract::wildcard(),
            None,
            None,
            Some(IdentityMatcherName::new("signal.identity.unknown")),
            None,
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
        Ok(_) => panic!("unknown schema default identity matcher must fail validation"),
        Err(err) => err,
    };

    assert!(err
        .to_string()
        .contains("references unknown default identity matcher"));
}

#[test]
fn runtime_builder_validation_rejects_unknown_node_merge_strategy_override() {
    let mut graph =
        crate::data::graph::SignalGraph::new().with_schema_registry(demo_schema_registry());
    graph
        .node()
        .merge_strategy_name("signal.merge.unknown")
        .build();

    let err = match SignalRuntime::builder(graph)
        .with_kernel_defaults()
        .build_validated()
    {
        Ok(_) => panic!("unknown node merge strategy override must fail validation"),
        Err(err) => err,
    };

    assert!(err
        .to_string()
        .contains("references unknown merge strategy"));
}

#[test]
fn runtime_builder_validation_rejects_unknown_node_conflict_policy_override() {
    let mut graph =
        crate::data::graph::SignalGraph::new().with_schema_registry(demo_schema_registry());
    graph
        .node()
        .conflict_policy_name("signal.conflict.unknown")
        .build();

    let err = match SignalRuntime::builder(graph)
        .with_kernel_defaults()
        .build_validated()
    {
        Ok(_) => panic!("unknown node conflict policy override must fail validation"),
        Err(err) => err,
    };

    assert!(err
        .to_string()
        .contains("references unknown conflict policy"));
}

#[test]
fn runtime_builder_validation_rejects_unknown_node_identity_matcher_override() {
    let mut graph =
        crate::data::graph::SignalGraph::new().with_schema_registry(demo_schema_registry());
    graph
        .node()
        .identity_matcher_name("signal.identity.unknown")
        .build();

    let err = match SignalRuntime::builder(graph)
        .with_kernel_defaults()
        .build_validated()
    {
        Ok(_) => panic!("unknown node identity matcher override must fail validation"),
        Err(err) => err,
    };

    assert!(err
        .to_string()
        .contains("references unknown identity matcher"));
}
