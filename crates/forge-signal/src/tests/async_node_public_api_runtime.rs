use crate::facade::*;
use crate::tests::async_node_support::{
    async_node_capability_declaration, AsyncNodeTestRuntime as TestRuntime,
};
use crate::tests::support::define_keyed_computation;

#[test]
fn attach_async_capability_returns_handle_that_owns_public_intent_building() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);

    let attached = runtime
        .attach_async_capability(async_node_capability_declaration(node))
        .expect("async capability attachment should return a proof-bearing handle");
    let lowered = runtime
        .async_node_capability_bundle_for_node(node)
        .expect("attached node should still expose lowered bundle truth");

    assert_eq!(attached.node(), node);
    assert_eq!(
        attached.registry_digest().as_str(),
        lowered.registry_digest().as_str()
    );
    assert_eq!(
        attached.bundle_digest().as_str(),
        lowered.bundle_digest().as_str()
    );
    assert_eq!(
        attached.payload_contract_digest().as_str(),
        lowered.payload_contract_digest().as_str()
    );

    let request = runtime
        .admit_async_node_request(attached.request_intent())
        .expect("attached handle should build a valid request intent");
    let active = request
        .resource_admission()
        .expect("attached request should reach lifecycle substrate")
        .admitted_request()
        .handle();
    let revalidation = runtime
        .revalidate_async_node(attached.revalidation_intent_with_expected_active(active))
        .expect("attached handle should build a valid revalidation intent");

    assert_eq!(
        request.classification().class(),
        AsyncNodeAdmissionClass::AdmittedNewLineage
    );
    assert_eq!(
        revalidation.classification().class(),
        AsyncNodeAdmissionClass::AdmittedNewLineage
    );
}

#[test]
fn keyed_attach_async_capability_reads_like_node_capability_not_subsystem_switch() {
    let mut runtime = TestRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let family = define_keyed_computation(&mut runtime, "async-projection", ());
    let keyed = family.keyed("left-wing");

    let attached = keyed
        .attach_async_capability(
            &mut runtime,
            AsyncNodePayloadContract::new(AsyncNodePayloadContractId::new(9))
                .with_max_payload_bytes(2048),
        )
        .expect("keyed computation should attach async capability through a node-first handle");

    let report = runtime
        .admit_async_node_request(attached.request_intent())
        .expect("attached keyed capability should admit through the same runtime substrate");
    let looked_up = keyed
        .async_capable_node(&mut runtime)
        .expect("keyed helper should rediscover the attached async-capable handle");

    assert_eq!(attached.node(), keyed.node(&mut runtime));
    assert_eq!(looked_up.node(), attached.node());
    assert_eq!(
        report
            .resource_admission()
            .expect("attached keyed request should expose lifecycle truth")
            .lifecycle()
            .node(),
        ResourceNodeId::from_node(attached.node())
    );
}
